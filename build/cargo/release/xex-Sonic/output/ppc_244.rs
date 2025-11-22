pub fn sub_83046278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046278 size=84
    let mut pc: u32 = 0x83046278;
    'dispatch: loop {
        match pc {
            0x83046278 => {
    //   block [0x83046278..0x830462CC)
	// 83046278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304627C: 48161EED  bl 0x831a8168
	ctx.lr = 0x83046280;
	sub_831A8130(ctx, base);
	// 83046280: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83046284: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046288: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304628C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83046290: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83046294: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 83046298: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304629C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830462A0: 48045AC1  bl 0x8308bd60
	ctx.lr = 0x830462A4;
	sub_8308BD60(ctx, base);
	// 830462A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830462A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830462AC: 396BFE90  addi r11, r11, -0x170
	ctx.r[11].s64 = ctx.r[11].s64 + -368;
	// 830462B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830462B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830462B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830462BC: 480482E5  bl 0x8308e5a0
	ctx.lr = 0x830462C0;
	sub_8308E5A0(ctx, base);
	// 830462C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830462C4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830462C8: 48161EF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830462CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830462CC size=40
    let mut pc: u32 = 0x830462CC;
    'dispatch: loop {
        match pc {
            0x830462CC => {
    //   block [0x830462CC..0x830462F4)
	// 830462CC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830462D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830462D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830462D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830462DC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830462E0: 48045A19  bl 0x8308bcf8
	ctx.lr = 0x830462E4;
	sub_8308BCF8(ctx, base);
	// 830462E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830462E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830462EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830462F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830462F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830462F8 size=8
    let mut pc: u32 = 0x830462F8;
    'dispatch: loop {
        match pc {
            0x830462F8 => {
    //   block [0x830462F8..0x83046300)
	// 830462F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830462FC: 8215FF38  lwz r16, -0xc8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046300 size=104
    let mut pc: u32 = 0x83046300;
    'dispatch: loop {
        match pc {
            0x83046300 => {
    //   block [0x83046300..0x83046368)
	// 83046300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046304: 48161E5D  bl 0x831a8160
	ctx.lr = 0x83046308;
	sub_831A8130(ctx, base);
	// 83046308: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304630C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046310: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83046314: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83046318: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304631C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046320: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046324: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83046328: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304632C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83046330: 4BF91F69  bl 0x82fd8298
	ctx.lr = 0x83046334;
	sub_82FD8298(ctx, base);
	// 83046334: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046338: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304633C: 41820020  beq 0x8304635c
	if ctx.cr[0].eq {
	pc = 0x8304635C; continue 'dispatch;
	}
	// 83046340: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83046344: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83046348: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304634C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83046350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83046354: 4BFFFF25  bl 0x83046278
	ctx.lr = 0x83046358;
	sub_83046278(ctx, base);
	// 83046358: 48000008  b 0x83046360
	pc = 0x83046360; continue 'dispatch;
	// 8304635C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046360: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83046364: 48161E4C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046368 size=44
    let mut pc: u32 = 0x83046368;
    'dispatch: loop {
        match pc {
            0x83046368 => {
    //   block [0x83046368..0x83046394)
	// 83046368: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304636C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046370: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046378: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304637C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046380: 4BF91F61  bl 0x82fd82e0
	ctx.lr = 0x83046384;
	sub_82FD82E0(ctx, base);
	// 83046384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046398 size=8
    let mut pc: u32 = 0x83046398;
    'dispatch: loop {
        match pc {
            0x83046398 => {
    //   block [0x83046398..0x830463A0)
	// 83046398: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304639C: 8215FFB4  lwz r16, -0x4c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-76 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830463A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830463A0 size=116
    let mut pc: u32 = 0x830463A0;
    'dispatch: loop {
        match pc {
            0x830463A0 => {
    //   block [0x830463A0..0x83046414)
	// 830463A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830463A4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830463A8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830463AC: 48161DC1  bl 0x831a816c
	ctx.lr = 0x830463B0;
	sub_831A8130(ctx, base);
	// 830463B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830463B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830463B8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830463BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830463C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830463C4: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 830463C8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830463CC: 4BF91ECD  bl 0x82fd8298
	ctx.lr = 0x830463D0;
	sub_82FD8298(ctx, base);
	// 830463D0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830463D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830463D8: 41820018  beq 0x830463f0
	if ctx.cr[0].eq {
	pc = 0x830463F0; continue 'dispatch;
	}
	// 830463DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830463E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830463E4: 4804983D  bl 0x8308fc20
	ctx.lr = 0x830463E8;
	sub_8308FC20(ctx, base);
	// 830463E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830463EC: 48000008  b 0x830463f4
	pc = 0x830463F4; continue 'dispatch;
	// 830463F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830463F4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830463F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830463FC: 4804A44D  bl 0x83090848
	ctx.lr = 0x83046400;
	sub_83090848(ctx, base);
	// 83046400: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046404: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304640C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046410: 48161DAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046414 size=8
    let mut pc: u32 = 0x83046414;
    'dispatch: loop {
        match pc {
            0x83046414 => {
    //   block [0x83046414..0x8304641C)
	// 83046414: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046418: 8215FFB4  lwz r16, -0x4c(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-76 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304641C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304641C size=24
    let mut pc: u32 = 0x8304641C;
    'dispatch: loop {
        match pc {
            0x8304641C => {
    //   block [0x8304641C..0x83046434)
	// 8304641C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046420: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046424: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304642C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046430: 4816A7F9  bl 0x831b0c28
	ctx.lr = 0x83046434;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304643C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304643C size=60
    let mut pc: u32 = 0x8304643C;
    'dispatch: loop {
        match pc {
            0x8304643C => {
    //   block [0x8304643C..0x83046478)
	// 8304643C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304644C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83046454: 419A0018  beq cr6, 0x8304646c
	if ctx.cr[6].eq {
	pc = 0x8304646C; continue 'dispatch;
	}
	// 83046458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304645C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83046460: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83046464: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83046468: 4E800421  bctrl
	ctx.lr = 0x8304646C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304646C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046474: 4816A7B5  bl 0x831b0c28
	ctx.lr = 0x83046478;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046478 size=44
    let mut pc: u32 = 0x83046478;
    'dispatch: loop {
        match pc {
            0x83046478 => {
    //   block [0x83046478..0x830464A4)
	// 83046478: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304647C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046480: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046488: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8304648C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046490: 4BF91E51  bl 0x82fd82e0
	ctx.lr = 0x83046494;
	sub_82FD82E0(ctx, base);
	// 83046494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304649C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830464A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830464A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830464A8 size=8
    let mut pc: u32 = 0x830464A8;
    'dispatch: loop {
        match pc {
            0x830464A8 => {
    //   block [0x830464A8..0x830464B0)
	// 830464A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830464AC: 4804A39C  b 0x83090848
	sub_83090848(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830464B0 size=8
    let mut pc: u32 = 0x830464B0;
    'dispatch: loop {
        match pc {
            0x830464B0 => {
    //   block [0x830464B0..0x830464B8)
	// 830464B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830464B4: 82160018  lwz r16, 0x18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830464B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830464B8 size=96
    let mut pc: u32 = 0x830464B8;
    'dispatch: loop {
        match pc {
            0x830464B8 => {
    //   block [0x830464B8..0x83046518)
	// 830464B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830464BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830464C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830464C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830464C8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830464CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830464D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830464D4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830464D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830464DC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830464E0: 4BF91DB9  bl 0x82fd8298
	ctx.lr = 0x830464E4;
	sub_82FD8298(ctx, base);
	// 830464E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830464E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830464EC: 41820010  beq 0x830464fc
	if ctx.cr[0].eq {
	pc = 0x830464FC; continue 'dispatch;
	}
	// 830464F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830464F4: 4BFFFD25  bl 0x83046218
	ctx.lr = 0x830464F8;
	sub_83046218(ctx, base);
	// 830464F8: 48000008  b 0x83046500
	pc = 0x83046500; continue 'dispatch;
	// 830464FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046500: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83046504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304650C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046518 size=44
    let mut pc: u32 = 0x83046518;
    'dispatch: loop {
        match pc {
            0x83046518 => {
    //   block [0x83046518..0x83046544)
	// 83046518: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304651C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046520: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046524: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046528: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304652C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046530: 4BF91DB1  bl 0x82fd82e0
	ctx.lr = 0x83046534;
	sub_82FD82E0(ctx, base);
	// 83046534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046548 size=12
    let mut pc: u32 = 0x83046548;
    'dispatch: loop {
        match pc {
            0x83046548 => {
    //   block [0x83046548..0x83046554)
	// 83046548: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304654C: 386B3374  addi r3, r11, 0x3374
	ctx.r[3].s64 = ctx.r[11].s64 + 13172;
	// 83046550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046558 size=88
    let mut pc: u32 = 0x83046558;
    'dispatch: loop {
        match pc {
            0x83046558 => {
    //   block [0x83046558..0x830465B0)
	// 83046558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83046564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304656C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83046574: 396BFE90  addi r11, r11, -0x170
	ctx.r[11].s64 = ctx.r[11].s64 + -368;
	// 83046578: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304657C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046580: 48045779  bl 0x8308bcf8
	ctx.lr = 0x83046584;
	sub_8308BCF8(ctx, base);
	// 83046584: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83046588: 4182000C  beq 0x83046594
	if ctx.cr[0].eq {
	pc = 0x83046594; continue 'dispatch;
	}
	// 8304658C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046590: 4BF91D51  bl 0x82fd82e0
	ctx.lr = 0x83046594;
	sub_82FD82E0(ctx, base);
	// 83046594: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046598: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304659C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830465A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830465A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830465A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830465AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830465B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830465B0 size=88
    let mut pc: u32 = 0x830465B0;
    'dispatch: loop {
        match pc {
            0x830465B0 => {
    //   block [0x830465B0..0x83046608)
	// 830465B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830465B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830465B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830465BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830465C0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830465C4: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 830465C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830465CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830465D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830465D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830465D8: 48045789  bl 0x8308bd60
	ctx.lr = 0x830465DC;
	sub_8308BD60(ctx, base);
	// 830465DC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830465E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830465E4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 830465E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830465EC: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 830465F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830465F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830465F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830465FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046608 size=8
    let mut pc: u32 = 0x83046608;
    'dispatch: loop {
        match pc {
            0x83046608 => {
    //   block [0x83046608..0x83046610)
	// 83046608: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304660C: 821600F0  lwz r16, 0xf0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046610 size=84
    let mut pc: u32 = 0x83046610;
    'dispatch: loop {
        match pc {
            0x83046610 => {
    //   block [0x83046610..0x83046664)
	// 83046610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046614: 48161B55  bl 0x831a8168
	ctx.lr = 0x83046618;
	sub_831A8130(ctx, base);
	// 83046618: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304661C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046620: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046624: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83046628: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304662C: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 83046630: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83046634: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83046638: 48045729  bl 0x8308bd60
	ctx.lr = 0x8304663C;
	sub_8308BD60(ctx, base);
	// 8304663C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046640: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83046644: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 83046648: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304664C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046650: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046654: 48047F4D  bl 0x8308e5a0
	ctx.lr = 0x83046658;
	sub_8308E5A0(ctx, base);
	// 83046658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304665C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046660: 48161B58  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046664 size=40
    let mut pc: u32 = 0x83046664;
    'dispatch: loop {
        match pc {
            0x83046664 => {
    //   block [0x83046664..0x8304668C)
	// 83046664: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046674: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83046678: 48045681  bl 0x8308bcf8
	ctx.lr = 0x8304667C;
	sub_8308BCF8(ctx, base);
	// 8304667C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046690 size=8
    let mut pc: u32 = 0x83046690;
    'dispatch: loop {
        match pc {
            0x83046690 => {
    //   block [0x83046690..0x83046698)
	// 83046690: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046694: 82160128  lwz r16, 0x128(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046698 size=104
    let mut pc: u32 = 0x83046698;
    'dispatch: loop {
        match pc {
            0x83046698 => {
    //   block [0x83046698..0x83046700)
	// 83046698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304669C: 48161AC5  bl 0x831a8160
	ctx.lr = 0x830466A0;
	sub_831A8130(ctx, base);
	// 830466A0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830466A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830466A8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830466AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830466B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830466B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830466B8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830466BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830466C0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830466C4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830466C8: 4BF91BD1  bl 0x82fd8298
	ctx.lr = 0x830466CC;
	sub_82FD8298(ctx, base);
	// 830466CC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830466D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830466D4: 41820020  beq 0x830466f4
	if ctx.cr[0].eq {
	pc = 0x830466F4; continue 'dispatch;
	}
	// 830466D8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830466DC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830466E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830466E4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830466E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830466EC: 4BFFFF25  bl 0x83046610
	ctx.lr = 0x830466F0;
	sub_83046610(ctx, base);
	// 830466F0: 48000008  b 0x830466f8
	pc = 0x830466F8; continue 'dispatch;
	// 830466F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830466F8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830466FC: 48161AB4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046700 size=44
    let mut pc: u32 = 0x83046700;
    'dispatch: loop {
        match pc {
            0x83046700 => {
    //   block [0x83046700..0x8304672C)
	// 83046700: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83046704: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046708: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304670C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046710: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83046714: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046718: 4BF91BC9  bl 0x82fd82e0
	ctx.lr = 0x8304671C;
	sub_82FD82E0(ctx, base);
	// 8304671C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046730 size=8
    let mut pc: u32 = 0x83046730;
    'dispatch: loop {
        match pc {
            0x83046730 => {
    //   block [0x83046730..0x83046738)
	// 83046730: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046734: 821601A4  lwz r16, 0x1a4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(420 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046738 size=116
    let mut pc: u32 = 0x83046738;
    'dispatch: loop {
        match pc {
            0x83046738 => {
    //   block [0x83046738..0x830467AC)
	// 83046738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304673C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83046740: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83046744: 48161A29  bl 0x831a816c
	ctx.lr = 0x83046748;
	sub_831A8130(ctx, base);
	// 83046748: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304674C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046750: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83046754: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83046758: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304675C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83046760: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83046764: 4BF91B35  bl 0x82fd8298
	ctx.lr = 0x83046768;
	sub_82FD8298(ctx, base);
	// 83046768: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304676C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046770: 41820018  beq 0x83046788
	if ctx.cr[0].eq {
	pc = 0x83046788; continue 'dispatch;
	}
	// 83046774: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83046778: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304677C: 480494A5  bl 0x8308fc20
	ctx.lr = 0x83046780;
	sub_8308FC20(ctx, base);
	// 83046780: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046784: 48000008  b 0x8304678c
	pc = 0x8304678C; continue 'dispatch;
	// 83046788: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304678C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83046790: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046794: 4804A16D  bl 0x83090900
	ctx.lr = 0x83046798;
	sub_83090900(ctx, base);
	// 83046798: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304679C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830467A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830467A4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830467A8: 48161A14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830467AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830467AC size=8
    let mut pc: u32 = 0x830467AC;
    'dispatch: loop {
        match pc {
            0x830467AC => {
    //   block [0x830467AC..0x830467B4)
	// 830467AC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830467B0: 821601A4  lwz r16, 0x1a4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(420 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830467B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830467B4 size=24
    let mut pc: u32 = 0x830467B4;
    'dispatch: loop {
        match pc {
            0x830467B4 => {
    //   block [0x830467B4..0x830467CC)
	// 830467B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830467B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830467BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830467C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830467C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830467C8: 4816A461  bl 0x831b0c28
	ctx.lr = 0x830467CC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830467D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830467D4 size=60
    let mut pc: u32 = 0x830467D4;
    'dispatch: loop {
        match pc {
            0x830467D4 => {
    //   block [0x830467D4..0x83046810)
	// 830467D4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830467D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830467DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830467E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830467E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830467E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830467EC: 419A0018  beq cr6, 0x83046804
	if ctx.cr[6].eq {
	pc = 0x83046804; continue 'dispatch;
	}
	// 830467F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830467F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830467F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830467FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83046800: 4E800421  bctrl
	ctx.lr = 0x83046804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83046804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304680C: 4816A41D  bl 0x831b0c28
	ctx.lr = 0x83046810;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046810 size=44
    let mut pc: u32 = 0x83046810;
    'dispatch: loop {
        match pc {
            0x83046810 => {
    //   block [0x83046810..0x8304683C)
	// 83046810: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046814: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046818: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304681C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046820: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83046824: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046828: 4BF91AB9  bl 0x82fd82e0
	ctx.lr = 0x8304682C;
	sub_82FD82E0(ctx, base);
	// 8304682C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046840 size=8
    let mut pc: u32 = 0x83046840;
    'dispatch: loop {
        match pc {
            0x83046840 => {
    //   block [0x83046840..0x83046848)
	// 83046840: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83046844: 4804A0BC  b 0x83090900
	sub_83090900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046848 size=8
    let mut pc: u32 = 0x83046848;
    'dispatch: loop {
        match pc {
            0x83046848 => {
    //   block [0x83046848..0x83046850)
	// 83046848: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304684C: 82160208  lwz r16, 0x208(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046850 size=96
    let mut pc: u32 = 0x83046850;
    'dispatch: loop {
        match pc {
            0x83046850 => {
    //   block [0x83046850..0x830468B0)
	// 83046850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304685C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046860: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83046864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046868: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304686C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046870: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046874: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83046878: 4BF91A21  bl 0x82fd8298
	ctx.lr = 0x8304687C;
	sub_82FD8298(ctx, base);
	// 8304687C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046880: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046884: 41820010  beq 0x83046894
	if ctx.cr[0].eq {
	pc = 0x83046894; continue 'dispatch;
	}
	// 83046888: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304688C: 4BFFFD25  bl 0x830465b0
	ctx.lr = 0x83046890;
	sub_830465B0(ctx, base);
	// 83046890: 48000008  b 0x83046898
	pc = 0x83046898; continue 'dispatch;
	// 83046894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046898: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304689C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830468A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830468A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830468A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830468AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830468B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830468B0 size=44
    let mut pc: u32 = 0x830468B0;
    'dispatch: loop {
        match pc {
            0x830468B0 => {
    //   block [0x830468B0..0x830468DC)
	// 830468B0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830468B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830468B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830468BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830468C0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830468C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830468C8: 4BF91A19  bl 0x82fd82e0
	ctx.lr = 0x830468CC;
	sub_82FD82E0(ctx, base);
	// 830468CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830468D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830468D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830468D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830468E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830468E0 size=12
    let mut pc: u32 = 0x830468E0;
    'dispatch: loop {
        match pc {
            0x830468E0 => {
    //   block [0x830468E0..0x830468EC)
	// 830468E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830468E4: 386B337C  addi r3, r11, 0x337c
	ctx.r[3].s64 = ctx.r[11].s64 + 13180;
	// 830468E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830468F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830468F0 size=88
    let mut pc: u32 = 0x830468F0;
    'dispatch: loop {
        match pc {
            0x830468F0 => {
    //   block [0x830468F0..0x83046948)
	// 830468F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830468F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830468F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830468FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046904: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304690C: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 83046910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83046914: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046918: 480453E1  bl 0x8308bcf8
	ctx.lr = 0x8304691C;
	sub_8308BCF8(ctx, base);
	// 8304691C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83046920: 4182000C  beq 0x8304692c
	if ctx.cr[0].eq {
	pc = 0x8304692C; continue 'dispatch;
	}
	// 83046924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046928: 4BF919B9  bl 0x82fd82e0
	ctx.lr = 0x8304692C;
	sub_82FD82E0(ctx, base);
	// 8304692C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046930: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83046934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304693C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046948 size=88
    let mut pc: u32 = 0x83046948;
    'dispatch: loop {
        match pc {
            0x83046948 => {
    //   block [0x83046948..0x830469A0)
	// 83046948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046958: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304695C: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 83046960: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83046964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83046968: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304696C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83046970: 480453F1  bl 0x8308bd60
	ctx.lr = 0x83046974;
	sub_8308BD60(ctx, base);
	// 83046974: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046978: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8304697C: 396B0268  addi r11, r11, 0x268
	ctx.r[11].s64 = ctx.r[11].s64 + 616;
	// 83046980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046984: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83046988: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304698C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830469A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830469A0 size=8
    let mut pc: u32 = 0x830469A0;
    'dispatch: loop {
        match pc {
            0x830469A0 => {
    //   block [0x830469A0..0x830469A8)
	// 830469A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830469A4: 821602D8  lwz r16, 0x2d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830469A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830469A8 size=84
    let mut pc: u32 = 0x830469A8;
    'dispatch: loop {
        match pc {
            0x830469A8 => {
    //   block [0x830469A8..0x830469FC)
	// 830469A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830469AC: 481617BD  bl 0x831a8168
	ctx.lr = 0x830469B0;
	sub_831A8130(ctx, base);
	// 830469B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830469B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830469B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830469BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830469C0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830469C4: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 830469C8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830469CC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830469D0: 48045391  bl 0x8308bd60
	ctx.lr = 0x830469D4;
	sub_8308BD60(ctx, base);
	// 830469D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830469D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830469DC: 396B0268  addi r11, r11, 0x268
	ctx.r[11].s64 = ctx.r[11].s64 + 616;
	// 830469E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830469E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830469E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830469EC: 48047BB5  bl 0x8308e5a0
	ctx.lr = 0x830469F0;
	sub_8308E5A0(ctx, base);
	// 830469F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830469F4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830469F8: 481617C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830469FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830469FC size=40
    let mut pc: u32 = 0x830469FC;
    'dispatch: loop {
        match pc {
            0x830469FC => {
    //   block [0x830469FC..0x83046A24)
	// 830469FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046A0C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83046A10: 480452E9  bl 0x8308bcf8
	ctx.lr = 0x83046A14;
	sub_8308BCF8(ctx, base);
	// 83046A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046A28 size=8
    let mut pc: u32 = 0x83046A28;
    'dispatch: loop {
        match pc {
            0x83046A28 => {
    //   block [0x83046A28..0x83046A30)
	// 83046A28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046A2C: 82160310  lwz r16, 0x310(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(784 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046A30 size=104
    let mut pc: u32 = 0x83046A30;
    'dispatch: loop {
        match pc {
            0x83046A30 => {
    //   block [0x83046A30..0x83046A98)
	// 83046A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046A34: 4816172D  bl 0x831a8160
	ctx.lr = 0x83046A38;
	sub_831A8130(ctx, base);
	// 83046A38: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83046A3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046A40: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83046A44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83046A48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83046A4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046A50: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046A54: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83046A58: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83046A5C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83046A60: 4BF91839  bl 0x82fd8298
	ctx.lr = 0x83046A64;
	sub_82FD8298(ctx, base);
	// 83046A64: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046A68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046A6C: 41820020  beq 0x83046a8c
	if ctx.cr[0].eq {
	pc = 0x83046A8C; continue 'dispatch;
	}
	// 83046A70: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83046A74: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83046A78: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83046A7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83046A80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83046A84: 4BFFFF25  bl 0x830469a8
	ctx.lr = 0x83046A88;
	sub_830469A8(ctx, base);
	// 83046A88: 48000008  b 0x83046a90
	pc = 0x83046A90; continue 'dispatch;
	// 83046A8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046A90: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83046A94: 4816171C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046A98 size=44
    let mut pc: u32 = 0x83046A98;
    'dispatch: loop {
        match pc {
            0x83046A98 => {
    //   block [0x83046A98..0x83046AC4)
	// 83046A98: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83046A9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046AA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046AA8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83046AAC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046AB0: 4BF91831  bl 0x82fd82e0
	ctx.lr = 0x83046AB4;
	sub_82FD82E0(ctx, base);
	// 83046AB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046AC8 size=8
    let mut pc: u32 = 0x83046AC8;
    'dispatch: loop {
        match pc {
            0x83046AC8 => {
    //   block [0x83046AC8..0x83046AD0)
	// 83046AC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046ACC: 8216038C  lwz r16, 0x38c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(908 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046AD0 size=116
    let mut pc: u32 = 0x83046AD0;
    'dispatch: loop {
        match pc {
            0x83046AD0 => {
    //   block [0x83046AD0..0x83046B44)
	// 83046AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046AD4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83046AD8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83046ADC: 48161691  bl 0x831a816c
	ctx.lr = 0x83046AE0;
	sub_831A8130(ctx, base);
	// 83046AE0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83046AE4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046AE8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83046AEC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83046AF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046AF4: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83046AF8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83046AFC: 4BF9179D  bl 0x82fd8298
	ctx.lr = 0x83046B00;
	sub_82FD8298(ctx, base);
	// 83046B00: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046B08: 41820018  beq 0x83046b20
	if ctx.cr[0].eq {
	pc = 0x83046B20; continue 'dispatch;
	}
	// 83046B0C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83046B10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83046B14: 4804910D  bl 0x8308fc20
	ctx.lr = 0x83046B18;
	sub_8308FC20(ctx, base);
	// 83046B18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046B1C: 48000008  b 0x83046b24
	pc = 0x83046B24; continue 'dispatch;
	// 83046B20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83046B24: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83046B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046B2C: 48049BBD  bl 0x830906e8
	ctx.lr = 0x83046B30;
	sub_830906E8(ctx, base);
	// 83046B30: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046B34: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046B38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046B3C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046B40: 4816167C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046B44 size=8
    let mut pc: u32 = 0x83046B44;
    'dispatch: loop {
        match pc {
            0x83046B44 => {
    //   block [0x83046B44..0x83046B4C)
	// 83046B44: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046B48: 8216038C  lwz r16, 0x38c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(908 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046B4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046B4C size=24
    let mut pc: u32 = 0x83046B4C;
    'dispatch: loop {
        match pc {
            0x83046B4C => {
    //   block [0x83046B4C..0x83046B64)
	// 83046B4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046B50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046B58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046B5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046B60: 4816A0C9  bl 0x831b0c28
	ctx.lr = 0x83046B64;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046B6C size=60
    let mut pc: u32 = 0x83046B6C;
    'dispatch: loop {
        match pc {
            0x83046B6C => {
    //   block [0x83046B6C..0x83046BA8)
	// 83046B6C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046B7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046B80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83046B84: 419A0018  beq cr6, 0x83046b9c
	if ctx.cr[6].eq {
	pc = 0x83046B9C; continue 'dispatch;
	}
	// 83046B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83046B8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83046B90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83046B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83046B98: 4E800421  bctrl
	ctx.lr = 0x83046B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83046B9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046BA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046BA4: 4816A085  bl 0x831b0c28
	ctx.lr = 0x83046BA8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046BA8 size=44
    let mut pc: u32 = 0x83046BA8;
    'dispatch: loop {
        match pc {
            0x83046BA8 => {
    //   block [0x83046BA8..0x83046BD4)
	// 83046BA8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046BAC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046BB0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046BB8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83046BBC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046BC0: 4BF91721  bl 0x82fd82e0
	ctx.lr = 0x83046BC4;
	sub_82FD82E0(ctx, base);
	// 83046BC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046BD8 size=8
    let mut pc: u32 = 0x83046BD8;
    'dispatch: loop {
        match pc {
            0x83046BD8 => {
    //   block [0x83046BD8..0x83046BE0)
	// 83046BD8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83046BDC: 48049B0C  b 0x830906e8
	sub_830906E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046BE0 size=8
    let mut pc: u32 = 0x83046BE0;
    'dispatch: loop {
        match pc {
            0x83046BE0 => {
    //   block [0x83046BE0..0x83046BE8)
	// 83046BE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046BE4: 821603F0  lwz r16, 0x3f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046BE8 size=96
    let mut pc: u32 = 0x83046BE8;
    'dispatch: loop {
        match pc {
            0x83046BE8 => {
    //   block [0x83046BE8..0x83046C48)
	// 83046BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046BF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83046BF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046BF8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83046BFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046C00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046C04: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046C08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046C0C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83046C10: 4BF91689  bl 0x82fd8298
	ctx.lr = 0x83046C14;
	sub_82FD8298(ctx, base);
	// 83046C14: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046C18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046C1C: 41820010  beq 0x83046c2c
	if ctx.cr[0].eq {
	pc = 0x83046C2C; continue 'dispatch;
	}
	// 83046C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046C24: 4BFFFD25  bl 0x83046948
	ctx.lr = 0x83046C28;
	sub_83046948(ctx, base);
	// 83046C28: 48000008  b 0x83046c30
	pc = 0x83046C30; continue 'dispatch;
	// 83046C2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046C30: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83046C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046C3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046C40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046C48 size=44
    let mut pc: u32 = 0x83046C48;
    'dispatch: loop {
        match pc {
            0x83046C48 => {
    //   block [0x83046C48..0x83046C74)
	// 83046C48: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83046C4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046C50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046C58: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83046C5C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046C60: 4BF91681  bl 0x82fd82e0
	ctx.lr = 0x83046C64;
	sub_82FD82E0(ctx, base);
	// 83046C64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046C78 size=12
    let mut pc: u32 = 0x83046C78;
    'dispatch: loop {
        match pc {
            0x83046C78 => {
    //   block [0x83046C78..0x83046C84)
	// 83046C78: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83046C7C: 386B3384  addi r3, r11, 0x3384
	ctx.r[3].s64 = ctx.r[11].s64 + 13188;
	// 83046C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046C88 size=88
    let mut pc: u32 = 0x83046C88;
    'dispatch: loop {
        match pc {
            0x83046C88 => {
    //   block [0x83046C88..0x83046CE0)
	// 83046C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046C90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83046C94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046C98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046C9C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83046CA4: 396B0268  addi r11, r11, 0x268
	ctx.r[11].s64 = ctx.r[11].s64 + 616;
	// 83046CA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83046CAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046CB0: 48045049  bl 0x8308bcf8
	ctx.lr = 0x83046CB4;
	sub_8308BCF8(ctx, base);
	// 83046CB4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83046CB8: 4182000C  beq 0x83046cc4
	if ctx.cr[0].eq {
	pc = 0x83046CC4; continue 'dispatch;
	}
	// 83046CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046CC0: 4BF91621  bl 0x82fd82e0
	ctx.lr = 0x83046CC4;
	sub_82FD82E0(ctx, base);
	// 83046CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83046CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046CD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046CE0 size=88
    let mut pc: u32 = 0x83046CE0;
    'dispatch: loop {
        match pc {
            0x83046CE0 => {
    //   block [0x83046CE0..0x83046D38)
	// 83046CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046CF0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83046CF4: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 83046CF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83046CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83046D00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83046D08: 48045059  bl 0x8308bd60
	ctx.lr = 0x83046D0C;
	sub_8308BD60(ctx, base);
	// 83046D0C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046D10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83046D14: 396B0450  addi r11, r11, 0x450
	ctx.r[11].s64 = ctx.r[11].s64 + 1104;
	// 83046D18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83046D1C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83046D20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046D38 size=8
    let mut pc: u32 = 0x83046D38;
    'dispatch: loop {
        match pc {
            0x83046D38 => {
    //   block [0x83046D38..0x83046D40)
	// 83046D38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046D3C: 821604C0  lwz r16, 0x4c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1216 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046D40 size=84
    let mut pc: u32 = 0x83046D40;
    'dispatch: loop {
        match pc {
            0x83046D40 => {
    //   block [0x83046D40..0x83046D94)
	// 83046D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046D44: 48161425  bl 0x831a8168
	ctx.lr = 0x83046D48;
	sub_831A8130(ctx, base);
	// 83046D48: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83046D4C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046D50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046D54: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83046D58: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83046D5C: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 83046D60: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83046D64: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83046D68: 48044FF9  bl 0x8308bd60
	ctx.lr = 0x83046D6C;
	sub_8308BD60(ctx, base);
	// 83046D6C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83046D70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83046D74: 396B0450  addi r11, r11, 0x450
	ctx.r[11].s64 = ctx.r[11].s64 + 1104;
	// 83046D78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83046D7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046D80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83046D84: 4804781D  bl 0x8308e5a0
	ctx.lr = 0x83046D88;
	sub_8308E5A0(ctx, base);
	// 83046D88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046D8C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046D90: 48161428  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046D94 size=40
    let mut pc: u32 = 0x83046D94;
    'dispatch: loop {
        match pc {
            0x83046D94 => {
    //   block [0x83046D94..0x83046DBC)
	// 83046D94: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046DA4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83046DA8: 48044F51  bl 0x8308bcf8
	ctx.lr = 0x83046DAC;
	sub_8308BCF8(ctx, base);
	// 83046DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046DC0 size=8
    let mut pc: u32 = 0x83046DC0;
    'dispatch: loop {
        match pc {
            0x83046DC0 => {
    //   block [0x83046DC0..0x83046DC8)
	// 83046DC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046DC4: 821604F8  lwz r16, 0x4f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046DC8 size=104
    let mut pc: u32 = 0x83046DC8;
    'dispatch: loop {
        match pc {
            0x83046DC8 => {
    //   block [0x83046DC8..0x83046E30)
	// 83046DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046DCC: 48161395  bl 0x831a8160
	ctx.lr = 0x83046DD0;
	sub_831A8130(ctx, base);
	// 83046DD0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83046DD4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046DD8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83046DDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83046DE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83046DE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046DE8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046DEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83046DF0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83046DF4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83046DF8: 4BF914A1  bl 0x82fd8298
	ctx.lr = 0x83046DFC;
	sub_82FD8298(ctx, base);
	// 83046DFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046E00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046E04: 41820020  beq 0x83046e24
	if ctx.cr[0].eq {
	pc = 0x83046E24; continue 'dispatch;
	}
	// 83046E08: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83046E0C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83046E10: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83046E14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83046E18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83046E1C: 4BFFFF25  bl 0x83046d40
	ctx.lr = 0x83046E20;
	sub_83046D40(ctx, base);
	// 83046E20: 48000008  b 0x83046e28
	pc = 0x83046E28; continue 'dispatch;
	// 83046E24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046E28: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83046E2C: 48161384  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046E30 size=44
    let mut pc: u32 = 0x83046E30;
    'dispatch: loop {
        match pc {
            0x83046E30 => {
    //   block [0x83046E30..0x83046E5C)
	// 83046E30: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83046E34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046E38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046E3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046E40: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83046E44: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046E48: 4BF91499  bl 0x82fd82e0
	ctx.lr = 0x83046E4C;
	sub_82FD82E0(ctx, base);
	// 83046E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046E60 size=8
    let mut pc: u32 = 0x83046E60;
    'dispatch: loop {
        match pc {
            0x83046E60 => {
    //   block [0x83046E60..0x83046E68)
	// 83046E60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046E64: 82160574  lwz r16, 0x574(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1396 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046E68 size=116
    let mut pc: u32 = 0x83046E68;
    'dispatch: loop {
        match pc {
            0x83046E68 => {
    //   block [0x83046E68..0x83046EDC)
	// 83046E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046E6C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83046E70: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83046E74: 481612F9  bl 0x831a816c
	ctx.lr = 0x83046E78;
	sub_831A8130(ctx, base);
	// 83046E78: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83046E7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046E80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83046E84: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83046E88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046E8C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83046E90: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83046E94: 4BF91405  bl 0x82fd8298
	ctx.lr = 0x83046E98;
	sub_82FD8298(ctx, base);
	// 83046E98: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046E9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046EA0: 41820018  beq 0x83046eb8
	if ctx.cr[0].eq {
	pc = 0x83046EB8; continue 'dispatch;
	}
	// 83046EA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83046EA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83046EAC: 48048D75  bl 0x8308fc20
	ctx.lr = 0x83046EB0;
	sub_8308FC20(ctx, base);
	// 83046EB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046EB4: 48000008  b 0x83046ebc
	pc = 0x83046EBC; continue 'dispatch;
	// 83046EB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83046EBC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83046EC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046EC4: 480496ED  bl 0x830905b0
	ctx.lr = 0x83046EC8;
	sub_830905B0(ctx, base);
	// 83046EC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046ECC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83046ED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83046ED4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83046ED8: 481612E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046EDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046EDC size=8
    let mut pc: u32 = 0x83046EDC;
    'dispatch: loop {
        match pc {
            0x83046EDC => {
    //   block [0x83046EDC..0x83046EE4)
	// 83046EDC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046EE0: 82160574  lwz r16, 0x574(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1396 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046EE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046EE4 size=24
    let mut pc: u32 = 0x83046EE4;
    'dispatch: loop {
        match pc {
            0x83046EE4 => {
    //   block [0x83046EE4..0x83046EFC)
	// 83046EE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046EE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046EEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046EF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046EF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046EF8: 48169D31  bl 0x831b0c28
	ctx.lr = 0x83046EFC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046F04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046F04 size=60
    let mut pc: u32 = 0x83046F04;
    'dispatch: loop {
        match pc {
            0x83046F04 => {
    //   block [0x83046F04..0x83046F40)
	// 83046F04: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046F14: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83046F1C: 419A0018  beq cr6, 0x83046f34
	if ctx.cr[6].eq {
	pc = 0x83046F34; continue 'dispatch;
	}
	// 83046F20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83046F24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83046F28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83046F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83046F30: 4E800421  bctrl
	ctx.lr = 0x83046F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83046F34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83046F38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046F3C: 48169CED  bl 0x831b0c28
	ctx.lr = 0x83046F40;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046F40 size=44
    let mut pc: u32 = 0x83046F40;
    'dispatch: loop {
        match pc {
            0x83046F40 => {
    //   block [0x83046F40..0x83046F6C)
	// 83046F40: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83046F44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046F48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046F50: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83046F54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046F58: 4BF91389  bl 0x82fd82e0
	ctx.lr = 0x83046F5C;
	sub_82FD82E0(ctx, base);
	// 83046F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83046F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046F70 size=8
    let mut pc: u32 = 0x83046F70;
    'dispatch: loop {
        match pc {
            0x83046F70 => {
    //   block [0x83046F70..0x83046F78)
	// 83046F70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83046F74: 4804963C  b 0x830905b0
	sub_830905B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83046F78 size=8
    let mut pc: u32 = 0x83046F78;
    'dispatch: loop {
        match pc {
            0x83046F78 => {
    //   block [0x83046F78..0x83046F80)
	// 83046F78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83046F7C: 821605D8  lwz r16, 0x5d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046F80 size=96
    let mut pc: u32 = 0x83046F80;
    'dispatch: loop {
        match pc {
            0x83046F80 => {
    //   block [0x83046F80..0x83046FE0)
	// 83046F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046F88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83046F8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83046F90: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83046F94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046F98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83046F9C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83046FA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046FA4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83046FA8: 4BF912F1  bl 0x82fd8298
	ctx.lr = 0x83046FAC;
	sub_82FD8298(ctx, base);
	// 83046FAC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83046FB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83046FB4: 41820010  beq 0x83046fc4
	if ctx.cr[0].eq {
	pc = 0x83046FC4; continue 'dispatch;
	}
	// 83046FB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83046FBC: 4BFFFD25  bl 0x83046ce0
	ctx.lr = 0x83046FC0;
	sub_83046CE0(ctx, base);
	// 83046FC0: 48000008  b 0x83046fc8
	pc = 0x83046FC8; continue 'dispatch;
	// 83046FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83046FC8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83046FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83046FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83046FD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83046FD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83046FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83046FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83046FE0 size=44
    let mut pc: u32 = 0x83046FE0;
    'dispatch: loop {
        match pc {
            0x83046FE0 => {
    //   block [0x83046FE0..0x8304700C)
	// 83046FE0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83046FE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83046FE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83046FEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83046FF0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83046FF4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83046FF8: 4BF912E9  bl 0x82fd82e0
	ctx.lr = 0x83046FFC;
	sub_82FD82E0(ctx, base);
	// 83046FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047010 size=12
    let mut pc: u32 = 0x83047010;
    'dispatch: loop {
        match pc {
            0x83047010 => {
    //   block [0x83047010..0x8304701C)
	// 83047010: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83047014: 386B338C  addi r3, r11, 0x338c
	ctx.r[3].s64 = ctx.r[11].s64 + 13196;
	// 83047018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047020 size=4
    let mut pc: u32 = 0x83047020;
    'dispatch: loop {
        match pc {
            0x83047020 => {
    //   block [0x83047020..0x83047024)
	// 83047020: 48044EB8  b 0x8308bed8
	sub_8308BED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047028 size=88
    let mut pc: u32 = 0x83047028;
    'dispatch: loop {
        match pc {
            0x83047028 => {
    //   block [0x83047028..0x83047080)
	// 83047028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304702C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304703C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83047044: 396B0450  addi r11, r11, 0x450
	ctx.r[11].s64 = ctx.r[11].s64 + 1104;
	// 83047048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304704C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047050: 48044CA9  bl 0x8308bcf8
	ctx.lr = 0x83047054;
	sub_8308BCF8(ctx, base);
	// 83047054: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047058: 4182000C  beq 0x83047064
	if ctx.cr[0].eq {
	pc = 0x83047064; continue 'dispatch;
	}
	// 8304705C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047060: 4BF91281  bl 0x82fd82e0
	ctx.lr = 0x83047064;
	sub_82FD82E0(ctx, base);
	// 83047064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304706C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047080 size=88
    let mut pc: u32 = 0x83047080;
    'dispatch: loop {
        match pc {
            0x83047080 => {
    //   block [0x83047080..0x830470D8)
	// 83047080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304708C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047090: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83047094: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 83047098: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304709C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830470A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830470A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830470A8: 48044CB9  bl 0x8308bd60
	ctx.lr = 0x830470AC;
	sub_8308BD60(ctx, base);
	// 830470AC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830470B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830470B4: 396B0638  addi r11, r11, 0x638
	ctx.r[11].s64 = ctx.r[11].s64 + 1592;
	// 830470B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830470BC: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 830470C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830470C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830470C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830470CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830470D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830470D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830470D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830470D8 size=8
    let mut pc: u32 = 0x830470D8;
    'dispatch: loop {
        match pc {
            0x830470D8 => {
    //   block [0x830470D8..0x830470E0)
	// 830470D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830470DC: 821606A8  lwz r16, 0x6a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830470E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830470E0 size=84
    let mut pc: u32 = 0x830470E0;
    'dispatch: loop {
        match pc {
            0x830470E0 => {
    //   block [0x830470E0..0x83047134)
	// 830470E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830470E4: 48161085  bl 0x831a8168
	ctx.lr = 0x830470E8;
	sub_831A8130(ctx, base);
	// 830470E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830470EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830470F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830470F4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830470F8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830470FC: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 83047100: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83047104: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83047108: 48044C59  bl 0x8308bd60
	ctx.lr = 0x8304710C;
	sub_8308BD60(ctx, base);
	// 8304710C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047110: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83047114: 396B0638  addi r11, r11, 0x638
	ctx.r[11].s64 = ctx.r[11].s64 + 1592;
	// 83047118: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304711C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047120: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047124: 4804747D  bl 0x8308e5a0
	ctx.lr = 0x83047128;
	sub_8308E5A0(ctx, base);
	// 83047128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304712C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047130: 48161088  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047134 size=40
    let mut pc: u32 = 0x83047134;
    'dispatch: loop {
        match pc {
            0x83047134 => {
    //   block [0x83047134..0x8304715C)
	// 83047134: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047144: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83047148: 48044BB1  bl 0x8308bcf8
	ctx.lr = 0x8304714C;
	sub_8308BCF8(ctx, base);
	// 8304714C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047160 size=8
    let mut pc: u32 = 0x83047160;
    'dispatch: loop {
        match pc {
            0x83047160 => {
    //   block [0x83047160..0x83047168)
	// 83047160: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047164: 821606E0  lwz r16, 0x6e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1760 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047168 size=104
    let mut pc: u32 = 0x83047168;
    'dispatch: loop {
        match pc {
            0x83047168 => {
    //   block [0x83047168..0x830471D0)
	// 83047168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304716C: 48160FF5  bl 0x831a8160
	ctx.lr = 0x83047170;
	sub_831A8130(ctx, base);
	// 83047170: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83047174: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047178: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304717C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83047180: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83047184: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047188: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 8304718C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83047190: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83047194: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83047198: 4BF91101  bl 0x82fd8298
	ctx.lr = 0x8304719C;
	sub_82FD8298(ctx, base);
	// 8304719C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830471A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830471A4: 41820020  beq 0x830471c4
	if ctx.cr[0].eq {
	pc = 0x830471C4; continue 'dispatch;
	}
	// 830471A8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830471AC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830471B0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830471B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830471B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830471BC: 4BFFFF25  bl 0x830470e0
	ctx.lr = 0x830471C0;
	sub_830470E0(ctx, base);
	// 830471C0: 48000008  b 0x830471c8
	pc = 0x830471C8; continue 'dispatch;
	// 830471C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830471C8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830471CC: 48160FE4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830471D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830471D0 size=44
    let mut pc: u32 = 0x830471D0;
    'dispatch: loop {
        match pc {
            0x830471D0 => {
    //   block [0x830471D0..0x830471FC)
	// 830471D0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830471D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830471D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830471DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830471E0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830471E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830471E8: 4BF910F9  bl 0x82fd82e0
	ctx.lr = 0x830471EC;
	sub_82FD82E0(ctx, base);
	// 830471EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830471F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830471F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830471F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047200 size=8
    let mut pc: u32 = 0x83047200;
    'dispatch: loop {
        match pc {
            0x83047200 => {
    //   block [0x83047200..0x83047208)
	// 83047200: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047204: 8216075C  lwz r16, 0x75c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1884 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047208 size=116
    let mut pc: u32 = 0x83047208;
    'dispatch: loop {
        match pc {
            0x83047208 => {
    //   block [0x83047208..0x8304727C)
	// 83047208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304720C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83047210: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83047214: 48160F59  bl 0x831a816c
	ctx.lr = 0x83047218;
	sub_831A8130(ctx, base);
	// 83047218: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304721C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047220: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83047224: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83047228: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304722C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83047230: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83047234: 4BF91065  bl 0x82fd8298
	ctx.lr = 0x83047238;
	sub_82FD8298(ctx, base);
	// 83047238: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304723C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047240: 41820018  beq 0x83047258
	if ctx.cr[0].eq {
	pc = 0x83047258; continue 'dispatch;
	}
	// 83047244: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83047248: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304724C: 480489D5  bl 0x8308fc20
	ctx.lr = 0x83047250;
	sub_8308FC20(ctx, base);
	// 83047250: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047254: 48000008  b 0x8304725c
	pc = 0x8304725C; continue 'dispatch;
	// 83047258: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304725C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83047260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047264: 4804A5DD  bl 0x83091840
	ctx.lr = 0x83047268;
	sub_83091840(ctx, base);
	// 83047268: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304726C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047274: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047278: 48160F44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304727C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304727C size=8
    let mut pc: u32 = 0x8304727C;
    'dispatch: loop {
        match pc {
            0x8304727C => {
    //   block [0x8304727C..0x83047284)
	// 8304727C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047280: 8216075C  lwz r16, 0x75c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(1884 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047284 size=24
    let mut pc: u32 = 0x83047284;
    'dispatch: loop {
        match pc {
            0x83047284 => {
    //   block [0x83047284..0x8304729C)
	// 83047284: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047288: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304728C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047290: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83047294: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047298: 48169991  bl 0x831b0c28
	ctx.lr = 0x8304729C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830472A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830472A4 size=60
    let mut pc: u32 = 0x830472A4;
    'dispatch: loop {
        match pc {
            0x830472A4 => {
    //   block [0x830472A4..0x830472E0)
	// 830472A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830472A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830472AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830472B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830472B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830472B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830472BC: 419A0018  beq cr6, 0x830472d4
	if ctx.cr[6].eq {
	pc = 0x830472D4; continue 'dispatch;
	}
	// 830472C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830472C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830472C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830472CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830472D0: 4E800421  bctrl
	ctx.lr = 0x830472D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830472D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830472D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830472DC: 4816994D  bl 0x831b0c28
	ctx.lr = 0x830472E0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830472E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830472E0 size=44
    let mut pc: u32 = 0x830472E0;
    'dispatch: loop {
        match pc {
            0x830472E0 => {
    //   block [0x830472E0..0x8304730C)
	// 830472E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830472E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830472E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830472EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830472F0: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830472F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830472F8: 4BF90FE9  bl 0x82fd82e0
	ctx.lr = 0x830472FC;
	sub_82FD82E0(ctx, base);
	// 830472FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047310 size=8
    let mut pc: u32 = 0x83047310;
    'dispatch: loop {
        match pc {
            0x83047310 => {
    //   block [0x83047310..0x83047318)
	// 83047310: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83047314: 4804A52C  b 0x83091840
	sub_83091840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047318 size=8
    let mut pc: u32 = 0x83047318;
    'dispatch: loop {
        match pc {
            0x83047318 => {
    //   block [0x83047318..0x83047320)
	// 83047318: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304731C: 82160828  lwz r16, 0x828(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047320 size=180
    let mut pc: u32 = 0x83047320;
    'dispatch: loop {
        match pc {
            0x83047320 => {
    //   block [0x83047320..0x830473D4)
	// 83047320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047324: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83047328: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304732C: 48160E41  bl 0x831a816c
	ctx.lr = 0x83047330;
	sub_831A8130(ctx, base);
	// 83047330: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83047334: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047338: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304733C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83047340: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83047344: 409A0008  bne cr6, 0x8304734c
	if !ctx.cr[6].eq {
	pc = 0x8304734C; continue 'dispatch;
	}
	// 83047348: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304734C: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047350: 41820028  beq 0x83047378
	if ctx.cr[0].eq {
	pc = 0x83047378; continue 'dispatch;
	}
	// 83047354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83047358: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8304735C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83047360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83047364: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047368: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8304736C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83047370: 4E800421  bctrl
	ctx.lr = 0x83047374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83047374: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047378: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8304737C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047380: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047384: 4804889D  bl 0x8308fc20
	ctx.lr = 0x83047388;
	sub_8308FC20(ctx, base);
	// 83047388: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304738C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047390: 4804A4B1  bl 0x83091840
	ctx.lr = 0x83047394;
	sub_83091840(ctx, base);
	// 83047394: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047398: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304739C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830473A0: 48048DD9  bl 0x83090178
	ctx.lr = 0x830473A4;
	sub_83090178(ctx, base);
	// 830473A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830473A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830473AC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830473B0: 480478E9  bl 0x8308ec98
	ctx.lr = 0x830473B4;
	sub_8308EC98(ctx, base);
	// 830473B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830473B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830473BC: 48000010  b 0x830473cc
	pc = 0x830473CC; continue 'dispatch;
	// 830473C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830473C4: 48000008  b 0x830473cc
	pc = 0x830473CC; continue 'dispatch;
	// 830473C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830473CC: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 830473D0: 48160DEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830473D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830473D4 size=8
    let mut pc: u32 = 0x830473D4;
    'dispatch: loop {
        match pc {
            0x830473D4 => {
    //   block [0x830473D4..0x830473DC)
	// 830473D4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830473D8: 82160828  lwz r16, 0x828(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830473DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830473DC size=20
    let mut pc: u32 = 0x830473DC;
    'dispatch: loop {
        match pc {
            0x830473DC => {
    //   block [0x830473DC..0x830473F0)
	// 830473DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830473E0: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 830473E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830473E8: 386373C0  addi r3, r3, 0x73c0
	ctx.r[3].s64 = ctx.r[3].s64 + 29632;
	// 830473EC: 48160DD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830473F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830473F0 size=8
    let mut pc: u32 = 0x830473F0;
    'dispatch: loop {
        match pc {
            0x830473F0 => {
    //   block [0x830473F0..0x830473F8)
	// 830473F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830473F4: 82160828  lwz r16, 0x828(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830473F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830473F8 size=20
    let mut pc: u32 = 0x830473F8;
    'dispatch: loop {
        match pc {
            0x830473F8 => {
    //   block [0x830473F8..0x8304740C)
	// 830473F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830473FC: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83047400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047404: 386373C8  addi r3, r3, 0x73c8
	ctx.r[3].s64 = ctx.r[3].s64 + 29640;
	// 83047408: 48160DB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304740C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304740C size=40
    let mut pc: u32 = 0x8304740C;
    'dispatch: loop {
        match pc {
            0x8304740C => {
    //   block [0x8304740C..0x83047434)
	// 8304740C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83047410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304741C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047420: 48047879  bl 0x8308ec98
	ctx.lr = 0x83047424;
	sub_8308EC98(ctx, base);
	// 83047424: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304742C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047438 size=8
    let mut pc: u32 = 0x83047438;
    'dispatch: loop {
        match pc {
            0x83047438 => {
    //   block [0x83047438..0x83047440)
	// 83047438: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304743C: 821608F0  lwz r16, 0x8f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047440 size=96
    let mut pc: u32 = 0x83047440;
    'dispatch: loop {
        match pc {
            0x83047440 => {
    //   block [0x83047440..0x830474A0)
	// 83047440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304744C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047450: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83047454: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047458: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304745C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83047460: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047464: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83047468: 4BF90E31  bl 0x82fd8298
	ctx.lr = 0x8304746C;
	sub_82FD8298(ctx, base);
	// 8304746C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047470: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047474: 41820010  beq 0x83047484
	if ctx.cr[0].eq {
	pc = 0x83047484; continue 'dispatch;
	}
	// 83047478: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304747C: 4BFFFC05  bl 0x83047080
	ctx.lr = 0x83047480;
	sub_83047080(ctx, base);
	// 83047480: 48000008  b 0x83047488
	pc = 0x83047488; continue 'dispatch;
	// 83047484: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047488: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304748C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304749C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830474A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830474A0 size=44
    let mut pc: u32 = 0x830474A0;
    'dispatch: loop {
        match pc {
            0x830474A0 => {
    //   block [0x830474A0..0x830474CC)
	// 830474A0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830474A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830474A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830474AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830474B0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830474B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830474B8: 4BF90E29  bl 0x82fd82e0
	ctx.lr = 0x830474BC;
	sub_82FD82E0(ctx, base);
	// 830474BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830474C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830474C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830474C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830474D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830474D0 size=12
    let mut pc: u32 = 0x830474D0;
    'dispatch: loop {
        match pc {
            0x830474D0 => {
    //   block [0x830474D0..0x830474DC)
	// 830474D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830474D4: 386B3394  addi r3, r11, 0x3394
	ctx.r[3].s64 = ctx.r[11].s64 + 13204;
	// 830474D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830474E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830474E0 size=88
    let mut pc: u32 = 0x830474E0;
    'dispatch: loop {
        match pc {
            0x830474E0 => {
    //   block [0x830474E0..0x83047538)
	// 830474E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830474E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830474E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830474EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830474F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830474F4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830474F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830474FC: 396B0638  addi r11, r11, 0x638
	ctx.r[11].s64 = ctx.r[11].s64 + 1592;
	// 83047500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83047504: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047508: 480447F1  bl 0x8308bcf8
	ctx.lr = 0x8304750C;
	sub_8308BCF8(ctx, base);
	// 8304750C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047510: 4182000C  beq 0x8304751c
	if ctx.cr[0].eq {
	pc = 0x8304751C; continue 'dispatch;
	}
	// 83047514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047518: 4BF90DC9  bl 0x82fd82e0
	ctx.lr = 0x8304751C;
	sub_82FD82E0(ctx, base);
	// 8304751C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047520: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83047524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304752C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047538 size=88
    let mut pc: u32 = 0x83047538;
    'dispatch: loop {
        match pc {
            0x83047538 => {
    //   block [0x83047538..0x83047590)
	// 83047538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304753C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047548: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304754C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 83047550: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83047554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83047558: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304755C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83047560: 48044801  bl 0x8308bd60
	ctx.lr = 0x83047564;
	sub_8308BD60(ctx, base);
	// 83047564: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047568: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8304756C: 396B0950  addi r11, r11, 0x950
	ctx.r[11].s64 = ctx.r[11].s64 + 2384;
	// 83047570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047574: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83047578: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304757C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304758C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047590 size=8
    let mut pc: u32 = 0x83047590;
    'dispatch: loop {
        match pc {
            0x83047590 => {
    //   block [0x83047590..0x83047598)
	// 83047590: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047594: 821609C0  lwz r16, 0x9c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047598 size=84
    let mut pc: u32 = 0x83047598;
    'dispatch: loop {
        match pc {
            0x83047598 => {
    //   block [0x83047598..0x830475EC)
	// 83047598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304759C: 48160BCD  bl 0x831a8168
	ctx.lr = 0x830475A0;
	sub_831A8130(ctx, base);
	// 830475A0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830475A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830475A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830475AC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830475B0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830475B4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 830475B8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830475BC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830475C0: 480447A1  bl 0x8308bd60
	ctx.lr = 0x830475C4;
	sub_8308BD60(ctx, base);
	// 830475C4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830475C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830475CC: 396B0950  addi r11, r11, 0x950
	ctx.r[11].s64 = ctx.r[11].s64 + 2384;
	// 830475D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830475D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830475D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830475DC: 48046FC5  bl 0x8308e5a0
	ctx.lr = 0x830475E0;
	sub_8308E5A0(ctx, base);
	// 830475E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830475E4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830475E8: 48160BD0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830475EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830475EC size=40
    let mut pc: u32 = 0x830475EC;
    'dispatch: loop {
        match pc {
            0x830475EC => {
    //   block [0x830475EC..0x83047614)
	// 830475EC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830475F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830475F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830475F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830475FC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83047600: 480446F9  bl 0x8308bcf8
	ctx.lr = 0x83047604;
	sub_8308BCF8(ctx, base);
	// 83047604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304760C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047618 size=8
    let mut pc: u32 = 0x83047618;
    'dispatch: loop {
        match pc {
            0x83047618 => {
    //   block [0x83047618..0x83047620)
	// 83047618: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304761C: 821609F8  lwz r16, 0x9f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047620 size=104
    let mut pc: u32 = 0x83047620;
    'dispatch: loop {
        match pc {
            0x83047620 => {
    //   block [0x83047620..0x83047688)
	// 83047620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047624: 48160B3D  bl 0x831a8160
	ctx.lr = 0x83047628;
	sub_831A8130(ctx, base);
	// 83047628: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304762C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047630: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83047634: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83047638: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304763C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047640: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83047644: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83047648: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304764C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83047650: 4BF90C49  bl 0x82fd8298
	ctx.lr = 0x83047654;
	sub_82FD8298(ctx, base);
	// 83047654: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047658: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304765C: 41820020  beq 0x8304767c
	if ctx.cr[0].eq {
	pc = 0x8304767C; continue 'dispatch;
	}
	// 83047660: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83047664: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83047668: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304766C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83047670: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047674: 4BFFFF25  bl 0x83047598
	ctx.lr = 0x83047678;
	sub_83047598(ctx, base);
	// 83047678: 48000008  b 0x83047680
	pc = 0x83047680; continue 'dispatch;
	// 8304767C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047680: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83047684: 48160B2C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047688 size=44
    let mut pc: u32 = 0x83047688;
    'dispatch: loop {
        match pc {
            0x83047688 => {
    //   block [0x83047688..0x830476B4)
	// 83047688: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304768C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047690: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047698: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304769C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830476A0: 4BF90C41  bl 0x82fd82e0
	ctx.lr = 0x830476A4;
	sub_82FD82E0(ctx, base);
	// 830476A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830476A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830476AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830476B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830476B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830476B8 size=8
    let mut pc: u32 = 0x830476B8;
    'dispatch: loop {
        match pc {
            0x830476B8 => {
    //   block [0x830476B8..0x830476C0)
	// 830476B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830476BC: 82160A74  lwz r16, 0xa74(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2676 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830476C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830476C0 size=116
    let mut pc: u32 = 0x830476C0;
    'dispatch: loop {
        match pc {
            0x830476C0 => {
    //   block [0x830476C0..0x83047734)
	// 830476C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830476C4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830476C8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830476CC: 48160AA1  bl 0x831a816c
	ctx.lr = 0x830476D0;
	sub_831A8130(ctx, base);
	// 830476D0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830476D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830476D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830476DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830476E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830476E4: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 830476E8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830476EC: 4BF90BAD  bl 0x82fd8298
	ctx.lr = 0x830476F0;
	sub_82FD8298(ctx, base);
	// 830476F0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830476F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830476F8: 41820018  beq 0x83047710
	if ctx.cr[0].eq {
	pc = 0x83047710; continue 'dispatch;
	}
	// 830476FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83047700: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047704: 4804851D  bl 0x8308fc20
	ctx.lr = 0x83047708;
	sub_8308FC20(ctx, base);
	// 83047708: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304770C: 48000008  b 0x83047714
	pc = 0x83047714; continue 'dispatch;
	// 83047710: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83047714: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83047718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304771C: 4804970D  bl 0x83090e28
	ctx.lr = 0x83047720;
	sub_83090E28(ctx, base);
	// 83047720: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047724: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304772C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047730: 48160A8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047734 size=8
    let mut pc: u32 = 0x83047734;
    'dispatch: loop {
        match pc {
            0x83047734 => {
    //   block [0x83047734..0x8304773C)
	// 83047734: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047738: 82160A74  lwz r16, 0xa74(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2676 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304773C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304773C size=24
    let mut pc: u32 = 0x8304773C;
    'dispatch: loop {
        match pc {
            0x8304773C => {
    //   block [0x8304773C..0x83047754)
	// 8304773C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047740: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047748: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304774C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047750: 481694D9  bl 0x831b0c28
	ctx.lr = 0x83047754;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304775C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304775C size=60
    let mut pc: u32 = 0x8304775C;
    'dispatch: loop {
        match pc {
            0x8304775C => {
    //   block [0x8304775C..0x83047798)
	// 8304775C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304776C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047770: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83047774: 419A0018  beq cr6, 0x8304778c
	if ctx.cr[6].eq {
	pc = 0x8304778C; continue 'dispatch;
	}
	// 83047778: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304777C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83047780: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83047784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83047788: 4E800421  bctrl
	ctx.lr = 0x8304778C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304778C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83047790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047794: 48169495  bl 0x831b0c28
	ctx.lr = 0x83047798;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047798 size=44
    let mut pc: u32 = 0x83047798;
    'dispatch: loop {
        match pc {
            0x83047798 => {
    //   block [0x83047798..0x830477C4)
	// 83047798: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304779C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830477A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830477A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830477A8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830477AC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830477B0: 4BF90B31  bl 0x82fd82e0
	ctx.lr = 0x830477B4;
	sub_82FD82E0(ctx, base);
	// 830477B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830477B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830477BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830477C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830477C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830477C8 size=8
    let mut pc: u32 = 0x830477C8;
    'dispatch: loop {
        match pc {
            0x830477C8 => {
    //   block [0x830477C8..0x830477D0)
	// 830477C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830477CC: 4804965C  b 0x83090e28
	sub_83090E28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830477D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830477D0 size=8
    let mut pc: u32 = 0x830477D0;
    'dispatch: loop {
        match pc {
            0x830477D0 => {
    //   block [0x830477D0..0x830477D8)
	// 830477D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830477D4: 82160AD8  lwz r16, 0xad8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830477D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830477D8 size=96
    let mut pc: u32 = 0x830477D8;
    'dispatch: loop {
        match pc {
            0x830477D8 => {
    //   block [0x830477D8..0x83047838)
	// 830477D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830477DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830477E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830477E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830477E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830477EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830477F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830477F4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830477F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830477FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83047800: 4BF90A99  bl 0x82fd8298
	ctx.lr = 0x83047804;
	sub_82FD8298(ctx, base);
	// 83047804: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304780C: 41820010  beq 0x8304781c
	if ctx.cr[0].eq {
	pc = 0x8304781C; continue 'dispatch;
	}
	// 83047810: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047814: 4BFFFD25  bl 0x83047538
	ctx.lr = 0x83047818;
	sub_83047538(ctx, base);
	// 83047818: 48000008  b 0x83047820
	pc = 0x83047820; continue 'dispatch;
	// 8304781C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047820: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83047824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304782C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047838 size=44
    let mut pc: u32 = 0x83047838;
    'dispatch: loop {
        match pc {
            0x83047838 => {
    //   block [0x83047838..0x83047864)
	// 83047838: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304783C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047840: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047848: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304784C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047850: 4BF90A91  bl 0x82fd82e0
	ctx.lr = 0x83047854;
	sub_82FD82E0(ctx, base);
	// 83047854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304785C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047868 size=12
    let mut pc: u32 = 0x83047868;
    'dispatch: loop {
        match pc {
            0x83047868 => {
    //   block [0x83047868..0x83047874)
	// 83047868: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304786C: 386B339C  addi r3, r11, 0x339c
	ctx.r[3].s64 = ctx.r[11].s64 + 13212;
	// 83047870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047878 size=88
    let mut pc: u32 = 0x83047878;
    'dispatch: loop {
        match pc {
            0x83047878 => {
    //   block [0x83047878..0x830478D0)
	// 83047878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304787C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047880: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047884: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304788C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83047894: 396B0950  addi r11, r11, 0x950
	ctx.r[11].s64 = ctx.r[11].s64 + 2384;
	// 83047898: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304789C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830478A0: 48044459  bl 0x8308bcf8
	ctx.lr = 0x830478A4;
	sub_8308BCF8(ctx, base);
	// 830478A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830478A8: 4182000C  beq 0x830478b4
	if ctx.cr[0].eq {
	pc = 0x830478B4; continue 'dispatch;
	}
	// 830478AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830478B0: 4BF90A31  bl 0x82fd82e0
	ctx.lr = 0x830478B4;
	sub_82FD82E0(ctx, base);
	// 830478B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830478B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830478BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830478C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830478C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830478C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830478CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830478D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830478D0 size=88
    let mut pc: u32 = 0x830478D0;
    'dispatch: loop {
        match pc {
            0x830478D0 => {
    //   block [0x830478D0..0x83047928)
	// 830478D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830478D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830478D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830478DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830478E0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830478E4: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 830478E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830478EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830478F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830478F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830478F8: 48044469  bl 0x8308bd60
	ctx.lr = 0x830478FC;
	sub_8308BD60(ctx, base);
	// 830478FC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047900: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83047904: 396B0B40  addi r11, r11, 0xb40
	ctx.r[11].s64 = ctx.r[11].s64 + 2880;
	// 83047908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304790C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83047910: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304791C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047928 size=8
    let mut pc: u32 = 0x83047928;
    'dispatch: loop {
        match pc {
            0x83047928 => {
    //   block [0x83047928..0x83047930)
	// 83047928: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304792C: 82160BB0  lwz r16, 0xbb0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(2992 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047930 size=84
    let mut pc: u32 = 0x83047930;
    'dispatch: loop {
        match pc {
            0x83047930 => {
    //   block [0x83047930..0x83047984)
	// 83047930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047934: 48160835  bl 0x831a8168
	ctx.lr = 0x83047938;
	sub_831A8130(ctx, base);
	// 83047938: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304793C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047944: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83047948: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304794C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 83047950: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83047954: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83047958: 48044409  bl 0x8308bd60
	ctx.lr = 0x8304795C;
	sub_8308BD60(ctx, base);
	// 8304795C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047960: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83047964: 396B0B40  addi r11, r11, 0xb40
	ctx.r[11].s64 = ctx.r[11].s64 + 2880;
	// 83047968: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304796C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047970: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047974: 48046C2D  bl 0x8308e5a0
	ctx.lr = 0x83047978;
	sub_8308E5A0(ctx, base);
	// 83047978: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304797C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047980: 48160838  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047984 size=40
    let mut pc: u32 = 0x83047984;
    'dispatch: loop {
        match pc {
            0x83047984 => {
    //   block [0x83047984..0x830479AC)
	// 83047984: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047994: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83047998: 48044361  bl 0x8308bcf8
	ctx.lr = 0x8304799C;
	sub_8308BCF8(ctx, base);
	// 8304799C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830479A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830479A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830479A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830479B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830479B0 size=8
    let mut pc: u32 = 0x830479B0;
    'dispatch: loop {
        match pc {
            0x830479B0 => {
    //   block [0x830479B0..0x830479B8)
	// 830479B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830479B4: 82160BE8  lwz r16, 0xbe8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3048 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830479B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830479B8 size=104
    let mut pc: u32 = 0x830479B8;
    'dispatch: loop {
        match pc {
            0x830479B8 => {
    //   block [0x830479B8..0x83047A20)
	// 830479B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830479BC: 481607A5  bl 0x831a8160
	ctx.lr = 0x830479C0;
	sub_831A8130(ctx, base);
	// 830479C0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830479C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830479C8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830479CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830479D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830479D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830479D8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830479DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830479E0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830479E4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830479E8: 4BF908B1  bl 0x82fd8298
	ctx.lr = 0x830479EC;
	sub_82FD8298(ctx, base);
	// 830479EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830479F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830479F4: 41820020  beq 0x83047a14
	if ctx.cr[0].eq {
	pc = 0x83047A14; continue 'dispatch;
	}
	// 830479F8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830479FC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83047A00: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83047A04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83047A08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047A0C: 4BFFFF25  bl 0x83047930
	ctx.lr = 0x83047A10;
	sub_83047930(ctx, base);
	// 83047A10: 48000008  b 0x83047a18
	pc = 0x83047A18; continue 'dispatch;
	// 83047A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047A18: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83047A1C: 48160794  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047A20 size=44
    let mut pc: u32 = 0x83047A20;
    'dispatch: loop {
        match pc {
            0x83047A20 => {
    //   block [0x83047A20..0x83047A4C)
	// 83047A20: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83047A24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047A28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047A2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047A30: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83047A34: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047A38: 4BF908A9  bl 0x82fd82e0
	ctx.lr = 0x83047A3C;
	sub_82FD82E0(ctx, base);
	// 83047A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047A50 size=8
    let mut pc: u32 = 0x83047A50;
    'dispatch: loop {
        match pc {
            0x83047A50 => {
    //   block [0x83047A50..0x83047A58)
	// 83047A50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047A54: 82160C64  lwz r16, 0xc64(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3172 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047A58 size=116
    let mut pc: u32 = 0x83047A58;
    'dispatch: loop {
        match pc {
            0x83047A58 => {
    //   block [0x83047A58..0x83047ACC)
	// 83047A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047A5C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83047A60: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83047A64: 48160709  bl 0x831a816c
	ctx.lr = 0x83047A68;
	sub_831A8130(ctx, base);
	// 83047A68: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83047A6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047A70: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83047A74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83047A78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047A7C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83047A80: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83047A84: 4BF90815  bl 0x82fd8298
	ctx.lr = 0x83047A88;
	sub_82FD8298(ctx, base);
	// 83047A88: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047A8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047A90: 41820018  beq 0x83047aa8
	if ctx.cr[0].eq {
	pc = 0x83047AA8; continue 'dispatch;
	}
	// 83047A94: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83047A98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047A9C: 48048185  bl 0x8308fc20
	ctx.lr = 0x83047AA0;
	sub_8308FC20(ctx, base);
	// 83047AA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047AA4: 48000008  b 0x83047aac
	pc = 0x83047AAC; continue 'dispatch;
	// 83047AA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83047AAC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83047AB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047AB4: 48049CD5  bl 0x83091788
	ctx.lr = 0x83047AB8;
	sub_83091788(ctx, base);
	// 83047AB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047ABC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047AC4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047AC8: 481606F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047ACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047ACC size=8
    let mut pc: u32 = 0x83047ACC;
    'dispatch: loop {
        match pc {
            0x83047ACC => {
    //   block [0x83047ACC..0x83047AD4)
	// 83047ACC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047AD0: 82160C64  lwz r16, 0xc64(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3172 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047AD4 size=24
    let mut pc: u32 = 0x83047AD4;
    'dispatch: loop {
        match pc {
            0x83047AD4 => {
    //   block [0x83047AD4..0x83047AEC)
	// 83047AD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047AD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047ADC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047AE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83047AE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047AE8: 48169141  bl 0x831b0c28
	ctx.lr = 0x83047AEC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047AF4 size=60
    let mut pc: u32 = 0x83047AF4;
    'dispatch: loop {
        match pc {
            0x83047AF4 => {
    //   block [0x83047AF4..0x83047B30)
	// 83047AF4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047B04: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83047B0C: 419A0018  beq cr6, 0x83047b24
	if ctx.cr[6].eq {
	pc = 0x83047B24; continue 'dispatch;
	}
	// 83047B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83047B14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83047B18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83047B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83047B20: 4E800421  bctrl
	ctx.lr = 0x83047B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83047B24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83047B28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047B2C: 481690FD  bl 0x831b0c28
	ctx.lr = 0x83047B30;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047B30 size=44
    let mut pc: u32 = 0x83047B30;
    'dispatch: loop {
        match pc {
            0x83047B30 => {
    //   block [0x83047B30..0x83047B5C)
	// 83047B30: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047B34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047B38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047B40: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83047B44: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047B48: 4BF90799  bl 0x82fd82e0
	ctx.lr = 0x83047B4C;
	sub_82FD82E0(ctx, base);
	// 83047B4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047B60 size=8
    let mut pc: u32 = 0x83047B60;
    'dispatch: loop {
        match pc {
            0x83047B60 => {
    //   block [0x83047B60..0x83047B68)
	// 83047B60: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83047B64: 48049C24  b 0x83091788
	sub_83091788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047B68 size=8
    let mut pc: u32 = 0x83047B68;
    'dispatch: loop {
        match pc {
            0x83047B68 => {
    //   block [0x83047B68..0x83047B70)
	// 83047B68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047B6C: 82160D30  lwz r16, 0xd30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047B70 size=180
    let mut pc: u32 = 0x83047B70;
    'dispatch: loop {
        match pc {
            0x83047B70 => {
    //   block [0x83047B70..0x83047C24)
	// 83047B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047B74: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83047B78: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83047B7C: 481605F1  bl 0x831a816c
	ctx.lr = 0x83047B80;
	sub_831A8130(ctx, base);
	// 83047B80: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83047B84: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047B88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83047B8C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83047B90: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83047B94: 409A0008  bne cr6, 0x83047b9c
	if !ctx.cr[6].eq {
	pc = 0x83047B9C; continue 'dispatch;
	}
	// 83047B98: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83047B9C: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047BA0: 41820028  beq 0x83047bc8
	if ctx.cr[0].eq {
	pc = 0x83047BC8; continue 'dispatch;
	}
	// 83047BA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83047BA8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83047BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83047BB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83047BB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047BB8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83047BBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83047BC0: 4E800421  bctrl
	ctx.lr = 0x83047BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83047BC4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047BC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83047BCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047BD0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047BD4: 4804804D  bl 0x8308fc20
	ctx.lr = 0x83047BD8;
	sub_8308FC20(ctx, base);
	// 83047BD8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047BDC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047BE0: 48049BA9  bl 0x83091788
	ctx.lr = 0x83047BE4;
	sub_83091788(ctx, base);
	// 83047BE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047BE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047BEC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047BF0: 48048381  bl 0x8308ff70
	ctx.lr = 0x83047BF4;
	sub_8308FF70(ctx, base);
	// 83047BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047BF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047BFC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047C00: 48047099  bl 0x8308ec98
	ctx.lr = 0x83047C04;
	sub_8308EC98(ctx, base);
	// 83047C04: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83047C08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047C0C: 48000010  b 0x83047c1c
	pc = 0x83047C1C; continue 'dispatch;
	// 83047C10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047C14: 48000008  b 0x83047c1c
	pc = 0x83047C1C; continue 'dispatch;
	// 83047C18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047C1C: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83047C20: 4816059C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047C24 size=8
    let mut pc: u32 = 0x83047C24;
    'dispatch: loop {
        match pc {
            0x83047C24 => {
    //   block [0x83047C24..0x83047C2C)
	// 83047C24: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047C28: 82160D30  lwz r16, 0xd30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047C2C size=20
    let mut pc: u32 = 0x83047C2C;
    'dispatch: loop {
        match pc {
            0x83047C2C => {
    //   block [0x83047C2C..0x83047C40)
	// 83047C2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047C30: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83047C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047C38: 38637C10  addi r3, r3, 0x7c10
	ctx.r[3].s64 = ctx.r[3].s64 + 31760;
	// 83047C3C: 48160580  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047C40 size=8
    let mut pc: u32 = 0x83047C40;
    'dispatch: loop {
        match pc {
            0x83047C40 => {
    //   block [0x83047C40..0x83047C48)
	// 83047C40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047C44: 82160D30  lwz r16, 0xd30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047C48 size=20
    let mut pc: u32 = 0x83047C48;
    'dispatch: loop {
        match pc {
            0x83047C48 => {
    //   block [0x83047C48..0x83047C5C)
	// 83047C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047C4C: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83047C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047C54: 38637C18  addi r3, r3, 0x7c18
	ctx.r[3].s64 = ctx.r[3].s64 + 31768;
	// 83047C58: 48160564  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047C5C size=40
    let mut pc: u32 = 0x83047C5C;
    'dispatch: loop {
        match pc {
            0x83047C5C => {
    //   block [0x83047C5C..0x83047C84)
	// 83047C5C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83047C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047C6C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83047C70: 48047029  bl 0x8308ec98
	ctx.lr = 0x83047C74;
	sub_8308EC98(ctx, base);
	// 83047C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047C88 size=8
    let mut pc: u32 = 0x83047C88;
    'dispatch: loop {
        match pc {
            0x83047C88 => {
    //   block [0x83047C88..0x83047C90)
	// 83047C88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047C8C: 82160DF8  lwz r16, 0xdf8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047C90 size=96
    let mut pc: u32 = 0x83047C90;
    'dispatch: loop {
        match pc {
            0x83047C90 => {
    //   block [0x83047C90..0x83047CF0)
	// 83047C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047CA0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83047CA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047CA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047CAC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83047CB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047CB4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83047CB8: 4BF905E1  bl 0x82fd8298
	ctx.lr = 0x83047CBC;
	sub_82FD8298(ctx, base);
	// 83047CBC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047CC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047CC4: 41820010  beq 0x83047cd4
	if ctx.cr[0].eq {
	pc = 0x83047CD4; continue 'dispatch;
	}
	// 83047CC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047CCC: 4BFFFC05  bl 0x830478d0
	ctx.lr = 0x83047CD0;
	sub_830478D0(ctx, base);
	// 83047CD0: 48000008  b 0x83047cd8
	pc = 0x83047CD8; continue 'dispatch;
	// 83047CD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047CD8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83047CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047CE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047CF0 size=44
    let mut pc: u32 = 0x83047CF0;
    'dispatch: loop {
        match pc {
            0x83047CF0 => {
    //   block [0x83047CF0..0x83047D1C)
	// 83047CF0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83047CF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047CF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047CFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047D00: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83047D04: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047D08: 4BF905D9  bl 0x82fd82e0
	ctx.lr = 0x83047D0C;
	sub_82FD82E0(ctx, base);
	// 83047D0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047D20 size=12
    let mut pc: u32 = 0x83047D20;
    'dispatch: loop {
        match pc {
            0x83047D20 => {
    //   block [0x83047D20..0x83047D2C)
	// 83047D20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83047D24: 386B33A4  addi r3, r11, 0x33a4
	ctx.r[3].s64 = ctx.r[11].s64 + 13220;
	// 83047D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047D30 size=88
    let mut pc: u32 = 0x83047D30;
    'dispatch: loop {
        match pc {
            0x83047D30 => {
    //   block [0x83047D30..0x83047D88)
	// 83047D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047D44: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047D48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83047D4C: 396B0B40  addi r11, r11, 0xb40
	ctx.r[11].s64 = ctx.r[11].s64 + 2880;
	// 83047D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83047D54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047D58: 48043FA1  bl 0x8308bcf8
	ctx.lr = 0x83047D5C;
	sub_8308BCF8(ctx, base);
	// 83047D5C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047D60: 4182000C  beq 0x83047d6c
	if ctx.cr[0].eq {
	pc = 0x83047D6C; continue 'dispatch;
	}
	// 83047D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047D68: 4BF90579  bl 0x82fd82e0
	ctx.lr = 0x83047D6C;
	sub_82FD82E0(ctx, base);
	// 83047D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83047D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047D88 size=80
    let mut pc: u32 = 0x83047D88;
    'dispatch: loop {
        match pc {
            0x83047D88 => {
    //   block [0x83047D88..0x83047DD8)
	// 83047D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047D98: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83047D9C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 83047DA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83047DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83047DA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83047DAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83047DB0: 48042B19  bl 0x8308a8c8
	ctx.lr = 0x83047DB4;
	sub_8308A8C8(ctx, base);
	// 83047DB4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047DBC: 396B0E40  addi r11, r11, 0xe40
	ctx.r[11].s64 = ctx.r[11].s64 + 3648;
	// 83047DC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047DD8 size=8
    let mut pc: u32 = 0x83047DD8;
    'dispatch: loop {
        match pc {
            0x83047DD8 => {
    //   block [0x83047DD8..0x83047DE0)
	// 83047DD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047DDC: 82160EA0  lwz r16, 0xea0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047DE0 size=84
    let mut pc: u32 = 0x83047DE0;
    'dispatch: loop {
        match pc {
            0x83047DE0 => {
    //   block [0x83047DE0..0x83047E34)
	// 83047DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047DE4: 48160385  bl 0x831a8168
	ctx.lr = 0x83047DE8;
	sub_831A8130(ctx, base);
	// 83047DE8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83047DEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047DF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047DF4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83047DF8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83047DFC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 83047E00: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83047E04: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83047E08: 48042AC1  bl 0x8308a8c8
	ctx.lr = 0x83047E0C;
	sub_8308A8C8(ctx, base);
	// 83047E0C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047E10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83047E14: 396B0E40  addi r11, r11, 0xe40
	ctx.r[11].s64 = ctx.r[11].s64 + 3648;
	// 83047E18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83047E1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047E20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83047E24: 48043E35  bl 0x8308bc58
	ctx.lr = 0x83047E28;
	sub_8308BC58(ctx, base);
	// 83047E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83047E2C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83047E30: 48160388  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047E34 size=40
    let mut pc: u32 = 0x83047E34;
    'dispatch: loop {
        match pc {
            0x83047E34 => {
    //   block [0x83047E34..0x83047E5C)
	// 83047E34: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83047E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047E44: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83047E48: 48042989  bl 0x8308a7d0
	ctx.lr = 0x83047E4C;
	sub_8308A7D0(ctx, base);
	// 83047E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047E60 size=8
    let mut pc: u32 = 0x83047E60;
    'dispatch: loop {
        match pc {
            0x83047E60 => {
    //   block [0x83047E60..0x83047E68)
	// 83047E60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047E64: 82160ED8  lwz r16, 0xed8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047E68 size=104
    let mut pc: u32 = 0x83047E68;
    'dispatch: loop {
        match pc {
            0x83047E68 => {
    //   block [0x83047E68..0x83047ED0)
	// 83047E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047E6C: 481602F5  bl 0x831a8160
	ctx.lr = 0x83047E70;
	sub_831A8130(ctx, base);
	// 83047E70: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83047E74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047E78: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83047E7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83047E80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83047E84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047E88: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83047E8C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83047E90: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83047E94: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83047E98: 4BF90401  bl 0x82fd8298
	ctx.lr = 0x83047E9C;
	sub_82FD8298(ctx, base);
	// 83047E9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047EA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047EA4: 41820020  beq 0x83047ec4
	if ctx.cr[0].eq {
	pc = 0x83047EC4; continue 'dispatch;
	}
	// 83047EA8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83047EAC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83047EB0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83047EB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83047EB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83047EBC: 4BFFFF25  bl 0x83047de0
	ctx.lr = 0x83047EC0;
	sub_83047DE0(ctx, base);
	// 83047EC0: 48000008  b 0x83047ec8
	pc = 0x83047EC8; continue 'dispatch;
	// 83047EC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047EC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83047ECC: 481602E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047ED0 size=44
    let mut pc: u32 = 0x83047ED0;
    'dispatch: loop {
        match pc {
            0x83047ED0 => {
    //   block [0x83047ED0..0x83047EFC)
	// 83047ED0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83047ED4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047ED8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047EE0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83047EE4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83047EE8: 4BF903F9  bl 0x82fd82e0
	ctx.lr = 0x83047EEC;
	sub_82FD82E0(ctx, base);
	// 83047EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83047EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047F00 size=128
    let mut pc: u32 = 0x83047F00;
    'dispatch: loop {
        match pc {
            0x83047F00 => {
    //   block [0x83047F00..0x83047F80)
	// 83047F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047F10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047F14: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83047F18: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83047F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83047F20: 4BF895F1  bl 0x82fd1510
	ctx.lr = 0x83047F24;
	sub_82FD1510(ctx, base);
	// 83047F24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83047F28: 40820040  bne 0x83047f68
	if !ctx.cr[0].eq {
	pc = 0x83047F68; continue 'dispatch;
	}
	// 83047F2C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83047F30: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83047F34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83047F38: 388B0F08  addi r4, r11, 0xf08
	ctx.r[4].s64 = ctx.r[11].s64 + 3848;
	// 83047F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83047F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83047F44: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83047F48: 38C00101  li r6, 0x101
	ctx.r[6].s64 = 257;
	// 83047F4C: 38A0007C  li r5, 0x7c
	ctx.r[5].s64 = 124;
	// 83047F50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83047F54: 4BFCDC35  bl 0x83015b88
	ctx.lr = 0x83047F58;
	sub_83015B88(ctx, base);
	// 83047F58: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83047F5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83047F60: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83047F64: 48168CC5  bl 0x831b0c28
	ctx.lr = 0x83047F68;
	sub_831B0C28(ctx, base);
	// 83047F68: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83047F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047F74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047F78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83047F80 size=8
    let mut pc: u32 = 0x83047F80;
    'dispatch: loop {
        match pc {
            0x83047F80 => {
    //   block [0x83047F80..0x83047F88)
	// 83047F80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83047F84: 82160F60  lwz r16, 0xf60(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(3936 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047F88 size=96
    let mut pc: u32 = 0x83047F88;
    'dispatch: loop {
        match pc {
            0x83047F88 => {
    //   block [0x83047F88..0x83047FE8)
	// 83047F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83047F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83047F98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83047F9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047FA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83047FA4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83047FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047FAC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83047FB0: 4BF902E9  bl 0x82fd8298
	ctx.lr = 0x83047FB4;
	sub_82FD8298(ctx, base);
	// 83047FB4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83047FB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83047FBC: 41820010  beq 0x83047fcc
	if ctx.cr[0].eq {
	pc = 0x83047FCC; continue 'dispatch;
	}
	// 83047FC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83047FC4: 4BFFFDC5  bl 0x83047d88
	ctx.lr = 0x83047FC8;
	sub_83047D88(ctx, base);
	// 83047FC8: 48000008  b 0x83047fd0
	pc = 0x83047FD0; continue 'dispatch;
	// 83047FCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83047FD0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83047FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83047FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83047FDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83047FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83047FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83047FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83047FE8 size=44
    let mut pc: u32 = 0x83047FE8;
    'dispatch: loop {
        match pc {
            0x83047FE8 => {
    //   block [0x83047FE8..0x83048014)
	// 83047FE8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83047FEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83047FF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83047FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83047FF8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83047FFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048000: 4BF902E1  bl 0x82fd82e0
	ctx.lr = 0x83048004;
	sub_82FD82E0(ctx, base);
	// 83048004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304800C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048018 size=12
    let mut pc: u32 = 0x83048018;
    'dispatch: loop {
        match pc {
            0x83048018 => {
    //   block [0x83048018..0x83048024)
	// 83048018: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304801C: 386B33AC  addi r3, r11, 0x33ac
	ctx.r[3].s64 = ctx.r[11].s64 + 13228;
	// 83048020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048028 size=88
    let mut pc: u32 = 0x83048028;
    'dispatch: loop {
        match pc {
            0x83048028 => {
    //   block [0x83048028..0x83048080)
	// 83048028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83048034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83048038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304803C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83048044: 396B0E40  addi r11, r11, 0xe40
	ctx.r[11].s64 = ctx.r[11].s64 + 3648;
	// 83048048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304804C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83048050: 48042781  bl 0x8308a7d0
	ctx.lr = 0x83048054;
	sub_8308A7D0(ctx, base);
	// 83048054: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048058: 4182000C  beq 0x83048064
	if ctx.cr[0].eq {
	pc = 0x83048064; continue 'dispatch;
	}
	// 8304805C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048060: 4BF90281  bl 0x82fd82e0
	ctx.lr = 0x83048064;
	sub_82FD82E0(ctx, base);
	// 83048064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304806C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83048078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304807C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048080 size=80
    let mut pc: u32 = 0x83048080;
    'dispatch: loop {
        match pc {
            0x83048080 => {
    //   block [0x83048080..0x830480D0)
	// 83048080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304808C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048090: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83048094: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83048098: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304809C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830480A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830480A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830480A8: 48042821  bl 0x8308a8c8
	ctx.lr = 0x830480AC;
	sub_8308A8C8(ctx, base);
	// 830480AC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830480B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830480B4: 396B0FC0  addi r11, r11, 0xfc0
	ctx.r[11].s64 = ctx.r[11].s64 + 4032;
	// 830480B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830480BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830480C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830480C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830480C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830480CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830480D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830480D0 size=8
    let mut pc: u32 = 0x830480D0;
    'dispatch: loop {
        match pc {
            0x830480D0 => {
    //   block [0x830480D0..0x830480D8)
	// 830480D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830480D4: 82161064  lwz r16, 0x1064(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4196 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830480D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830480D8 size=100
    let mut pc: u32 = 0x830480D8;
    'dispatch: loop {
        match pc {
            0x830480D8 => {
    //   block [0x830480D8..0x8304813C)
	// 830480D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830480DC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830480E0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830480E4: 48160085  bl 0x831a8168
	ctx.lr = 0x830480E8;
	sub_831A8130(ctx, base);
	// 830480E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830480EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830480F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830480F4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830480F8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 830480FC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83048100: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83048104: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83048108: 480427C1  bl 0x8308a8c8
	ctx.lr = 0x8304810C;
	sub_8308A8C8(ctx, base);
	// 8304810C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048110: 396B0FC0  addi r11, r11, 0xfc0
	ctx.r[11].s64 = ctx.r[11].s64 + 4032;
	// 83048114: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83048118: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304811C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83048120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048124: 48043B35  bl 0x8308bc58
	ctx.lr = 0x83048128;
	sub_8308BC58(ctx, base);
	// 83048128: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304812C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048134: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83048138: 48160080  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304813C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304813C size=8
    let mut pc: u32 = 0x8304813C;
    'dispatch: loop {
        match pc {
            0x8304813C => {
    //   block [0x8304813C..0x83048144)
	// 8304813C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83048140: 82161064  lwz r16, 0x1064(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4196 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048144(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048144 size=24
    let mut pc: u32 = 0x83048144;
    'dispatch: loop {
        match pc {
            0x83048144 => {
    //   block [0x83048144..0x8304815C)
	// 83048144: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048148: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304814C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048150: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83048154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048158: 48168AD1  bl 0x831b0c28
	ctx.lr = 0x8304815C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048164 size=24
    let mut pc: u32 = 0x83048164;
    'dispatch: loop {
        match pc {
            0x83048164 => {
    //   block [0x83048164..0x8304817C)
	// 83048164: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048168: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304816C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048170: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83048174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048178: 48168AB1  bl 0x831b0c28
	ctx.lr = 0x8304817C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304817C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304817C size=40
    let mut pc: u32 = 0x8304817C;
    'dispatch: loop {
        match pc {
            0x8304817C => {
    //   block [0x8304817C..0x830481A4)
	// 8304817C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83048180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304818C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048190: 48042641  bl 0x8308a7d0
	ctx.lr = 0x83048194;
	sub_8308A7D0(ctx, base);
	// 83048194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304819C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830481A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830481A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830481A8 size=8
    let mut pc: u32 = 0x830481A8;
    'dispatch: loop {
        match pc {
            0x830481A8 => {
    //   block [0x830481A8..0x830481B0)
	// 830481A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830481AC: 821610C0  lwz r16, 0x10c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830481B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830481B0 size=104
    let mut pc: u32 = 0x830481B0;
    'dispatch: loop {
        match pc {
            0x830481B0 => {
    //   block [0x830481B0..0x83048218)
	// 830481B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830481B4: 4815FFAD  bl 0x831a8160
	ctx.lr = 0x830481B8;
	sub_831A8130(ctx, base);
	// 830481B8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830481BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830481C0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830481C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830481C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830481CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830481D0: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830481D4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830481D8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830481DC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830481E0: 4BF900B9  bl 0x82fd8298
	ctx.lr = 0x830481E4;
	sub_82FD8298(ctx, base);
	// 830481E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830481E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830481EC: 41820020  beq 0x8304820c
	if ctx.cr[0].eq {
	pc = 0x8304820C; continue 'dispatch;
	}
	// 830481F0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830481F4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830481F8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830481FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83048200: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048204: 4BFFFED5  bl 0x830480d8
	ctx.lr = 0x83048208;
	sub_830480D8(ctx, base);
	// 83048208: 48000008  b 0x83048210
	pc = 0x83048210; continue 'dispatch;
	// 8304820C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048210: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83048214: 4815FF9C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048218 size=44
    let mut pc: u32 = 0x83048218;
    'dispatch: loop {
        match pc {
            0x83048218 => {
    //   block [0x83048218..0x83048244)
	// 83048218: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304821C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048220: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048224: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048228: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304822C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048230: 4BF900B1  bl 0x82fd82e0
	ctx.lr = 0x83048234;
	sub_82FD82E0(ctx, base);
	// 83048234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304823C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048248 size=8
    let mut pc: u32 = 0x83048248;
    'dispatch: loop {
        match pc {
            0x83048248 => {
    //   block [0x83048248..0x83048250)
	// 83048248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304824C: 82161184  lwz r16, 0x1184(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4484 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048250 size=220
    let mut pc: u32 = 0x83048250;
    'dispatch: loop {
        match pc {
            0x83048250 => {
    //   block [0x83048250..0x8304832C)
	// 83048250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048254: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83048258: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304825C: 4815FF11  bl 0x831a816c
	ctx.lr = 0x83048260;
	sub_831A8130(ctx, base);
	// 83048260: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83048264: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048268: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304826C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83048270: 93DF00DC  stw r30, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 83048274: 93BF00E4  stw r29, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[29].u32 ) };
	// 83048278: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8304827C: 419A0034  beq cr6, 0x830482b0
	if ctx.cr[6].eq {
	pc = 0x830482B0; continue 'dispatch;
	}
	// 83048280: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048284: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048288: 41820028  beq 0x830482b0
	if ctx.cr[0].eq {
	pc = 0x830482B0; continue 'dispatch;
	}
	// 8304828C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 83048290: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048294: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048298: 4182000C  beq 0x830482a4
	if ctx.cr[0].eq {
	pc = 0x830482A4; continue 'dispatch;
	}
	// 8304829C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830482A0: 4BFFFFF0  b 0x83048290
	pc = 0x83048290; continue 'dispatch;
	// 830482A4: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830482A8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830482AC: 48000008  b 0x830482b4
	pc = 0x830482B4; continue 'dispatch;
	// 830482B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830482B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830482B8: 419A006C  beq cr6, 0x83048324
	if ctx.cr[6].eq {
	pc = 0x83048324; continue 'dispatch;
	}
	// 830482BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830482C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830482C4: 4BF94525  bl 0x82fdc7e8
	ctx.lr = 0x830482C8;
	sub_82FDC7E8(ctx, base);
	// 830482C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830482CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830482D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830482D4: 40820050  bne 0x83048324
	if !ctx.cr[0].eq {
	pc = 0x83048324; continue 'dispatch;
	}
	// 830482D8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830482DC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830482E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830482E4: 388B10F0  addi r4, r11, 0x10f0
	ctx.r[4].s64 = ctx.r[11].s64 + 4336;
	// 830482E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830482EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830482F0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830482F4: 38C00104  li r6, 0x104
	ctx.r[6].s64 = 260;
	// 830482F8: 38A000A5  li r5, 0xa5
	ctx.r[5].s64 = 165;
	// 830482FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83048300: 4BFCD889  bl 0x83015b88
	ctx.lr = 0x83048304;
	sub_83015B88(ctx, base);
	// 83048304: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048308: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304830C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048310: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83048314: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83048318: 48168911  bl 0x831b0c28
	ctx.lr = 0x8304831C;
	sub_831B0C28(ctx, base);
	// 8304831C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048320: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048324: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83048328: 4815FE94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048334 size=24
    let mut pc: u32 = 0x83048334;
    'dispatch: loop {
        match pc {
            0x83048334 => {
    //   block [0x83048334..0x8304834C)
	// 83048334: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048338: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304833C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048340: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83048344: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048348: 481688E1  bl 0x831b0c28
	ctx.lr = 0x8304834C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048354 size=80
    let mut pc: u32 = 0x83048354;
    'dispatch: loop {
        match pc {
            0x83048354 => {
    //   block [0x83048354..0x830483A4)
	// 83048354: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83048358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304835C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048364: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304836C: 80FF00DC  lwz r7, 0xdc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 83048370: 388B10F0  addi r4, r11, 0x10f0
	ctx.r[4].s64 = ctx.r[11].s64 + 4336;
	// 83048374: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83048378: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304837C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83048380: 38C00104  li r6, 0x104
	ctx.r[6].s64 = 260;
	// 83048384: 38A000B1  li r5, 0xb1
	ctx.r[5].s64 = 177;
	// 83048388: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8304838C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83048390: 4BFCD7F9  bl 0x83015b88
	ctx.lr = 0x83048394;
	sub_83015B88(ctx, base);
	// 83048394: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048398: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8304839C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830483A0: 48168889  bl 0x831b0c28
	ctx.lr = 0x830483A4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830483A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830483A8 size=8
    let mut pc: u32 = 0x830483A8;
    'dispatch: loop {
        match pc {
            0x830483A8 => {
    //   block [0x830483A8..0x830483B0)
	// 830483A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830483AC: 821611C0  lwz r16, 0x11c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830483B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830483B0 size=96
    let mut pc: u32 = 0x830483B0;
    'dispatch: loop {
        match pc {
            0x830483B0 => {
    //   block [0x830483B0..0x83048410)
	// 830483B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830483B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830483B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830483BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830483C0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830483C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830483C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830483CC: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830483D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830483D4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830483D8: 4BF8FEC1  bl 0x82fd8298
	ctx.lr = 0x830483DC;
	sub_82FD8298(ctx, base);
	// 830483DC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830483E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830483E4: 41820010  beq 0x830483f4
	if ctx.cr[0].eq {
	pc = 0x830483F4; continue 'dispatch;
	}
	// 830483E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830483EC: 4BFFFC95  bl 0x83048080
	ctx.lr = 0x830483F0;
	sub_83048080(ctx, base);
	// 830483F0: 48000008  b 0x830483f8
	pc = 0x830483F8; continue 'dispatch;
	// 830483F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830483F8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830483FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83048408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304840C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048410 size=44
    let mut pc: u32 = 0x83048410;
    'dispatch: loop {
        match pc {
            0x83048410 => {
    //   block [0x83048410..0x8304843C)
	// 83048410: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83048414: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048418: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304841C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048420: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83048424: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048428: 4BF8FEB9  bl 0x82fd82e0
	ctx.lr = 0x8304842C;
	sub_82FD82E0(ctx, base);
	// 8304842C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048440 size=12
    let mut pc: u32 = 0x83048440;
    'dispatch: loop {
        match pc {
            0x83048440 => {
    //   block [0x83048440..0x8304844C)
	// 83048440: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83048444: 386B33B4  addi r3, r11, 0x33b4
	ctx.r[3].s64 = ctx.r[11].s64 + 13236;
	// 83048448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048450 size=88
    let mut pc: u32 = 0x83048450;
    'dispatch: loop {
        match pc {
            0x83048450 => {
    //   block [0x83048450..0x830484A8)
	// 83048450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304845C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83048460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048464: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048468: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304846C: 396B0FC0  addi r11, r11, 0xfc0
	ctx.r[11].s64 = ctx.r[11].s64 + 4032;
	// 83048470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83048474: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83048478: 48042359  bl 0x8308a7d0
	ctx.lr = 0x8304847C;
	sub_8308A7D0(ctx, base);
	// 8304847C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048480: 4182000C  beq 0x8304848c
	if ctx.cr[0].eq {
	pc = 0x8304848C; continue 'dispatch;
	}
	// 83048484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048488: 4BF8FE59  bl 0x82fd82e0
	ctx.lr = 0x8304848C;
	sub_82FD82E0(ctx, base);
	// 8304848C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83048494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304849C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830484A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830484A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830484A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830484A8 size=100
    let mut pc: u32 = 0x830484A8;
    'dispatch: loop {
        match pc {
            0x830484A8 => {
    //   block [0x830484A8..0x8304850C)
	// 830484A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830484AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830484B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830484B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830484B8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830484BC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 830484C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830484C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830484C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830484CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830484D0: 4804A231  bl 0x83092700
	ctx.lr = 0x830484D4;
	sub_83092700(ctx, base);
	// 830484D4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830484D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830484DC: 394B1208  addi r10, r11, 0x1208
	ctx.r[10].s64 = ctx.r[11].s64 + 4616;
	// 830484E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830484E4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830484E8: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830484EC: 997F003D  stb r11, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 830484F0: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 830484F4: 997F003E  stb r11, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[11].u8 ) };
	// 830484F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830484FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83048508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048510 size=8
    let mut pc: u32 = 0x83048510;
    'dispatch: loop {
        match pc {
            0x83048510 => {
    //   block [0x83048510..0x83048518)
	// 83048510: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83048514: 82161268  lwz r16, 0x1268(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4712 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048518 size=84
    let mut pc: u32 = 0x83048518;
    'dispatch: loop {
        match pc {
            0x83048518 => {
    //   block [0x83048518..0x8304856C)
	// 83048518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304851C: 4815FC4D  bl 0x831a8168
	ctx.lr = 0x83048520;
	sub_831A8130(ctx, base);
	// 83048520: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83048524: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048528: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304852C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83048530: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83048534: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 83048538: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304853C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83048540: 4804A1C1  bl 0x83092700
	ctx.lr = 0x83048544;
	sub_83092700(ctx, base);
	// 83048544: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048548: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304854C: 396B1208  addi r11, r11, 0x1208
	ctx.r[11].s64 = ctx.r[11].s64 + 4616;
	// 83048550: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83048554: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048558: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304855C: 48046045  bl 0x8308e5a0
	ctx.lr = 0x83048560;
	sub_8308E5A0(ctx, base);
	// 83048560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048564: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83048568: 4815FC50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304856C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304856C size=40
    let mut pc: u32 = 0x8304856C;
    'dispatch: loop {
        match pc {
            0x8304856C => {
    //   block [0x8304856C..0x83048594)
	// 8304856C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83048570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304857C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048580: 4804A119  bl 0x83092698
	ctx.lr = 0x83048584;
	sub_83092698(ctx, base);
	// 83048584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304858C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048598 size=8
    let mut pc: u32 = 0x83048598;
    'dispatch: loop {
        match pc {
            0x83048598 => {
    //   block [0x83048598..0x830485A0)
	// 83048598: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304859C: 821612A8  lwz r16, 0x12a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830485A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830485A0 size=116
    let mut pc: u32 = 0x830485A0;
    'dispatch: loop {
        match pc {
            0x830485A0 => {
    //   block [0x830485A0..0x83048614)
	// 830485A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830485A4: 4815FBC5  bl 0x831a8168
	ctx.lr = 0x830485A8;
	sub_831A8130(ctx, base);
	// 830485A8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830485AC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830485B0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830485B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830485B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830485BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830485C0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830485C4: 4804A57D  bl 0x83092b40
	ctx.lr = 0x830485C8;
	sub_83092B40(ctx, base);
	// 830485C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830485CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830485D0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830485D4: 4804A56D  bl 0x83092b40
	ctx.lr = 0x830485D8;
	sub_83092B40(ctx, base);
	// 830485D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830485DC: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 830485E0: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 830485E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830485E8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830485EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830485F0: 4E800421  bctrl
	ctx.lr = 0x830485F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830485F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830485F8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830485FC: 4804A5B5  bl 0x83092bb0
	ctx.lr = 0x83048600;
	sub_83092BB0(ctx, base);
	// 83048600: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83048604: 4804A5AD  bl 0x83092bb0
	ctx.lr = 0x83048608;
	sub_83092BB0(ctx, base);
	// 83048608: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304860C: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 83048610: 4815FBA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048614 size=40
    let mut pc: u32 = 0x83048614;
    'dispatch: loop {
        match pc {
            0x83048614 => {
    //   block [0x83048614..0x8304863C)
	// 83048614: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83048618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304861C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048624: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83048628: 4804A589  bl 0x83092bb0
	ctx.lr = 0x8304862C;
	sub_83092BB0(ctx, base);
	// 8304862C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304863C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304863C size=40
    let mut pc: u32 = 0x8304863C;
    'dispatch: loop {
        match pc {
            0x8304863C => {
    //   block [0x8304863C..0x83048664)
	// 8304863C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83048640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304864C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83048650: 4804A561  bl 0x83092bb0
	ctx.lr = 0x83048654;
	sub_83092BB0(ctx, base);
	// 83048654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304865C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048668 size=8
    let mut pc: u32 = 0x83048668;
    'dispatch: loop {
        match pc {
            0x83048668 => {
    //   block [0x83048668..0x83048670)
	// 83048668: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304866C: 82161300  lwz r16, 0x1300(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048670 size=104
    let mut pc: u32 = 0x83048670;
    'dispatch: loop {
        match pc {
            0x83048670 => {
    //   block [0x83048670..0x830486D8)
	// 83048670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048674: 4815FAED  bl 0x831a8160
	ctx.lr = 0x83048678;
	sub_831A8130(ctx, base);
	// 83048678: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304867C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048680: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83048684: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83048688: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304868C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83048690: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83048694: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83048698: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304869C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830486A0: 4BF8FBF9  bl 0x82fd8298
	ctx.lr = 0x830486A4;
	sub_82FD8298(ctx, base);
	// 830486A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830486A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830486AC: 41820020  beq 0x830486cc
	if ctx.cr[0].eq {
	pc = 0x830486CC; continue 'dispatch;
	}
	// 830486B0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830486B4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830486B8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830486BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830486C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830486C4: 4BFFFE55  bl 0x83048518
	ctx.lr = 0x830486C8;
	sub_83048518(ctx, base);
	// 830486C8: 48000008  b 0x830486d0
	pc = 0x830486D0; continue 'dispatch;
	// 830486CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830486D0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830486D4: 4815FADC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830486D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830486D8 size=44
    let mut pc: u32 = 0x830486D8;
    'dispatch: loop {
        match pc {
            0x830486D8 => {
    //   block [0x830486D8..0x83048704)
	// 830486D8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830486DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830486E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830486E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830486E8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830486EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830486F0: 4BF8FBF1  bl 0x82fd82e0
	ctx.lr = 0x830486F4;
	sub_82FD82E0(ctx, base);
	// 830486F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830486F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830486FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048708 size=8
    let mut pc: u32 = 0x83048708;
    'dispatch: loop {
        match pc {
            0x83048708 => {
    //   block [0x83048708..0x83048710)
	// 83048708: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304870C: 82161338  lwz r16, 0x1338(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4920 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048710 size=84
    let mut pc: u32 = 0x83048710;
    'dispatch: loop {
        match pc {
            0x83048710 => {
    //   block [0x83048710..0x83048764)
	// 83048710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048714: 4815FA59  bl 0x831a816c
	ctx.lr = 0x83048718;
	sub_831A8130(ctx, base);
	// 83048718: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304871C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048720: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83048724: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83048728: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304872C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048730: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83048734: 4BF8FB65  bl 0x82fd8298
	ctx.lr = 0x83048738;
	sub_82FD8298(ctx, base);
	// 83048738: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304873C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048740: 41820014  beq 0x83048754
	if ctx.cr[0].eq {
	pc = 0x83048754; continue 'dispatch;
	}
	// 83048744: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048748: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304874C: 4804A3F5  bl 0x83092b40
	ctx.lr = 0x83048750;
	sub_83092B40(ctx, base);
	// 83048750: 48000008  b 0x83048758
	pc = 0x83048758; continue 'dispatch;
	// 83048754: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048758: 907E0048  stw r3, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 8304875C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83048760: 4815FA5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048764 size=48
    let mut pc: u32 = 0x83048764;
    'dispatch: loop {
        match pc {
            0x83048764 => {
    //   block [0x83048764..0x83048794)
	// 83048764: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83048768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048774: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048778: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304877C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048780: 4BF8FB61  bl 0x82fd82e0
	ctx.lr = 0x83048784;
	sub_82FD82E0(ctx, base);
	// 83048784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304878C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048798 size=8
    let mut pc: u32 = 0x83048798;
    'dispatch: loop {
        match pc {
            0x83048798 => {
    //   block [0x83048798..0x830487A0)
	// 83048798: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304879C: 82161370  lwz r16, 0x1370(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830487A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830487A0 size=84
    let mut pc: u32 = 0x830487A0;
    'dispatch: loop {
        match pc {
            0x830487A0 => {
    //   block [0x830487A0..0x830487F4)
	// 830487A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830487A4: 4815F9C9  bl 0x831a816c
	ctx.lr = 0x830487A8;
	sub_831A8130(ctx, base);
	// 830487A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830487AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830487B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830487B4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830487B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830487BC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830487C0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830487C4: 4BF8FAD5  bl 0x82fd8298
	ctx.lr = 0x830487C8;
	sub_82FD8298(ctx, base);
	// 830487C8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830487CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830487D0: 41820014  beq 0x830487e4
	if ctx.cr[0].eq {
	pc = 0x830487E4; continue 'dispatch;
	}
	// 830487D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830487D8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830487DC: 4804A365  bl 0x83092b40
	ctx.lr = 0x830487E0;
	sub_83092B40(ctx, base);
	// 830487E0: 48000008  b 0x830487e8
	pc = 0x830487E8; continue 'dispatch;
	// 830487E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830487E8: 907E004C  stw r3, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 830487EC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830487F0: 4815F9CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830487F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830487F4 size=48
    let mut pc: u32 = 0x830487F4;
    'dispatch: loop {
        match pc {
            0x830487F4 => {
    //   block [0x830487F4..0x83048824)
	// 830487F4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830487F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830487FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048804: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048808: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304880C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048810: 4BF8FAD1  bl 0x82fd82e0
	ctx.lr = 0x83048814;
	sub_82FD82E0(ctx, base);
	// 83048814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304881C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048828 size=8
    let mut pc: u32 = 0x83048828;
    'dispatch: loop {
        match pc {
            0x83048828 => {
    //   block [0x83048828..0x83048830)
	// 83048828: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304882C: 821613A8  lwz r16, 0x13a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5032 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048830 size=84
    let mut pc: u32 = 0x83048830;
    'dispatch: loop {
        match pc {
            0x83048830 => {
    //   block [0x83048830..0x83048884)
	// 83048830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048834: 4815F939  bl 0x831a816c
	ctx.lr = 0x83048838;
	sub_831A8130(ctx, base);
	// 83048838: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304883C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048840: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83048844: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83048848: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304884C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048850: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83048854: 4BF8FA45  bl 0x82fd8298
	ctx.lr = 0x83048858;
	sub_82FD8298(ctx, base);
	// 83048858: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304885C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048860: 41820014  beq 0x83048874
	if ctx.cr[0].eq {
	pc = 0x83048874; continue 'dispatch;
	}
	// 83048864: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048868: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304886C: 4804A2D5  bl 0x83092b40
	ctx.lr = 0x83048870;
	sub_83092B40(ctx, base);
	// 83048870: 48000008  b 0x83048878
	pc = 0x83048878; continue 'dispatch;
	// 83048874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048878: 907E0050  stw r3, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304887C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83048880: 4815F93C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048884 size=48
    let mut pc: u32 = 0x83048884;
    'dispatch: loop {
        match pc {
            0x83048884 => {
    //   block [0x83048884..0x830488B4)
	// 83048884: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83048888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048894: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048898: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304889C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830488A0: 4BF8FA41  bl 0x82fd82e0
	ctx.lr = 0x830488A4;
	sub_82FD82E0(ctx, base);
	// 830488A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830488A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830488AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830488B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830488B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830488B8 size=8
    let mut pc: u32 = 0x830488B8;
    'dispatch: loop {
        match pc {
            0x830488B8 => {
    //   block [0x830488B8..0x830488C0)
	// 830488B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830488BC: 821613E0  lwz r16, 0x13e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830488C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830488C0 size=84
    let mut pc: u32 = 0x830488C0;
    'dispatch: loop {
        match pc {
            0x830488C0 => {
    //   block [0x830488C0..0x83048914)
	// 830488C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830488C4: 4815F8A9  bl 0x831a816c
	ctx.lr = 0x830488C8;
	sub_831A8130(ctx, base);
	// 830488C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830488CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830488D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830488D4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830488D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830488DC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830488E0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830488E4: 4BF8F9B5  bl 0x82fd8298
	ctx.lr = 0x830488E8;
	sub_82FD8298(ctx, base);
	// 830488E8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830488EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830488F0: 41820014  beq 0x83048904
	if ctx.cr[0].eq {
	pc = 0x83048904; continue 'dispatch;
	}
	// 830488F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830488F8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830488FC: 4804A245  bl 0x83092b40
	ctx.lr = 0x83048900;
	sub_83092B40(ctx, base);
	// 83048900: 48000008  b 0x83048908
	pc = 0x83048908; continue 'dispatch;
	// 83048904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048908: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8304890C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83048910: 4815F8AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048914 size=48
    let mut pc: u32 = 0x83048914;
    'dispatch: loop {
        match pc {
            0x83048914 => {
    //   block [0x83048914..0x83048944)
	// 83048914: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83048918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304891C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048924: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83048928: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304892C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83048930: 4BF8F9B1  bl 0x82fd82e0
	ctx.lr = 0x83048934;
	sub_82FD82E0(ctx, base);
	// 83048934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83048938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304893C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048948 size=8
    let mut pc: u32 = 0x83048948;
    'dispatch: loop {
        match pc {
            0x83048948 => {
    //   block [0x83048948..0x83048950)
	// 83048948: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304894C: 82161418  lwz r16, 0x1418(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048950 size=96
    let mut pc: u32 = 0x83048950;
    'dispatch: loop {
        match pc {
            0x83048950 => {
    //   block [0x83048950..0x830489B0)
	// 83048950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304895C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83048960: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83048964: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048968: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304896C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83048970: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83048974: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83048978: 4BF8F921  bl 0x82fd8298
	ctx.lr = 0x8304897C;
	sub_82FD8298(ctx, base);
	// 8304897C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83048980: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048984: 41820010  beq 0x83048994
	if ctx.cr[0].eq {
	pc = 0x83048994; continue 'dispatch;
	}
	// 83048988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304898C: 4BFFFB1D  bl 0x830484a8
	ctx.lr = 0x83048990;
	sub_830484A8(ctx, base);
	// 83048990: 48000008  b 0x83048998
	pc = 0x83048998; continue 'dispatch;
	// 83048994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048998: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304899C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830489A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830489A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830489A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830489AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830489B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830489B0 size=44
    let mut pc: u32 = 0x830489B0;
    'dispatch: loop {
        match pc {
            0x830489B0 => {
    //   block [0x830489B0..0x830489DC)
	// 830489B0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830489B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830489B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830489BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830489C0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830489C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830489C8: 4BF8F919  bl 0x82fd82e0
	ctx.lr = 0x830489CC;
	sub_82FD82E0(ctx, base);
	// 830489CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830489D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830489D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830489D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830489E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830489E0 size=12
    let mut pc: u32 = 0x830489E0;
    'dispatch: loop {
        match pc {
            0x830489E0 => {
    //   block [0x830489E0..0x830489EC)
	// 830489E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830489E4: 386B33BC  addi r3, r11, 0x33bc
	ctx.r[3].s64 = ctx.r[11].s64 + 13244;
	// 830489E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830489F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830489F0 size=92
    let mut pc: u32 = 0x830489F0;
    'dispatch: loop {
        match pc {
            0x830489F0 => {
    //   block [0x830489F0..0x83048A4C)
	// 830489F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830489F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830489F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830489FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83048A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048A04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83048A08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83048A0C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83048A10: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83048A14: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048A18: 41820010  beq 0x83048a28
	if ctx.cr[0].eq {
	pc = 0x83048A28; continue 'dispatch;
	}
	// 83048A1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83048A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048A24: 4BFB08D5  bl 0x82ff92f8
	ctx.lr = 0x83048A28;
	sub_82FF92F8(ctx, base);
	// 83048A28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83048A2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048A30: 4804A101  bl 0x83092b30
	ctx.lr = 0x83048A34;
	sub_83092B30(ctx, base);
	// 83048A34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83048A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048A40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83048A44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83048A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048A50 size=88
    let mut pc: u32 = 0x83048A50;
    'dispatch: loop {
        match pc {
            0x83048A50 => {
    //   block [0x83048A50..0x83048AA8)
	// 83048A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83048A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83048A60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048A64: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048A68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83048A6C: 396B1208  addi r11, r11, 0x1208
	ctx.r[11].s64 = ctx.r[11].s64 + 4616;
	// 83048A70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83048A74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83048A78: 48049C21  bl 0x83092698
	ctx.lr = 0x83048A7C;
	sub_83092698(ctx, base);
	// 83048A7C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048A80: 4182000C  beq 0x83048a8c
	if ctx.cr[0].eq {
	pc = 0x83048A8C; continue 'dispatch;
	}
	// 83048A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048A88: 4BF8F859  bl 0x82fd82e0
	ctx.lr = 0x83048A8C;
	sub_82FD82E0(ctx, base);
	// 83048A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048A90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83048A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048A9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83048AA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83048AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048AA8 size=8
    let mut pc: u32 = 0x83048AA8;
    'dispatch: loop {
        match pc {
            0x83048AA8 => {
    //   block [0x83048AA8..0x83048AB0)
	// 83048AA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83048AAC: 82161510  lwz r16, 0x1510(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048AB0 size=492
    let mut pc: u32 = 0x83048AB0;
    'dispatch: loop {
        match pc {
            0x83048AB0 => {
    //   block [0x83048AB0..0x83048C9C)
	// 83048AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048AB4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83048AB8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83048ABC: 4815F6A1  bl 0x831a815c
	ctx.lr = 0x83048AC0;
	sub_831A8130(ctx, base);
	// 83048AC0: 3BE1FEA0  addi r31, r1, -0x160
	ctx.r[31].s64 = ctx.r[1].s64 + -352;
	// 83048AC4: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048AC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83048ACC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83048AD0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83048AD4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83048AD8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83048ADC: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 83048AE0: 935F0194  stw r26, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[26].u32 ) };
	// 83048AE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048AE8: 41820018  beq 0x83048b00
	if ctx.cr[0].eq {
	pc = 0x83048B00; continue 'dispatch;
	}
	// 83048AEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048AF0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83048AF4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83048AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83048AFC: 4E800421  bctrl
	ctx.lr = 0x83048B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83048B00: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83048B04: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048B08: 418200A4  beq 0x83048bac
	if ctx.cr[0].eq {
	pc = 0x83048BAC; continue 'dispatch;
	}
	// 83048B0C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83048B10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048B14: 40820044  bne 0x83048b58
	if !ctx.cr[0].eq {
	pc = 0x83048B58; continue 'dispatch;
	}
	// 83048B18: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 83048B1C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048B20: 4BF8F779  bl 0x82fd8298
	ctx.lr = 0x83048B24;
	sub_82FD8298(ctx, base);
	// 83048B24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048B28: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 83048B2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048B30: 41820020  beq 0x83048b50
	if ctx.cr[0].eq {
	pc = 0x83048B50; continue 'dispatch;
	}
	// 83048B34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83048B38: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83048B3C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048B40: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 83048B44: 480415A5  bl 0x8308a0e8
	ctx.lr = 0x83048B48;
	sub_8308A0E8(ctx, base);
	// 83048B48: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048B4C: 48000008  b 0x83048b54
	pc = 0x83048B54; continue 'dispatch;
	// 83048B50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83048B54: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83048B58: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83048B5C: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83048B60: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83048B64: 48041BAD  bl 0x8308a710
	ctx.lr = 0x83048B68;
	sub_8308A710(ctx, base);
	// 83048B68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048B6C: 40820040  bne 0x83048bac
	if !ctx.cr[0].eq {
	pc = 0x83048BAC; continue 'dispatch;
	}
	// 83048B70: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048B74: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83048B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83048B7C: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 83048B80: 388B1448  addi r4, r11, 0x1448
	ctx.r[4].s64 = ctx.r[11].s64 + 5192;
	// 83048B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83048B88: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83048B8C: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 83048B90: 38A00108  li r5, 0x108
	ctx.r[5].s64 = 264;
	// 83048B94: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83048B98: 4BFCCFF1  bl 0x83015b88
	ctx.lr = 0x83048B9C;
	sub_83015B88(ctx, base);
	// 83048B9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048BA0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83048BA4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83048BA8: 48168081  bl 0x831b0c28
	ctx.lr = 0x83048BAC;
	sub_831B0C28(ctx, base);
	// 83048BAC: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048BB0: 408200E4  bne 0x83048c94
	if !ctx.cr[0].eq {
	pc = 0x83048C94; continue 'dispatch;
	}
	// 83048BB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83048BB8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83048BBC: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83048BC0: 48049F81  bl 0x83092b40
	ctx.lr = 0x83048BC4;
	sub_83092B40(ctx, base);
	// 83048BC4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048BC8: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83048BCC: 3B7F00F0  addi r27, r31, 0xf0
	ctx.r[27].s64 = ctx.r[31].s64 + 240;
	// 83048BD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048BD4: 418200A0  beq 0x83048c74
	if ctx.cr[0].eq {
	pc = 0x83048C74; continue 'dispatch;
	}
	// 83048BD8: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83048BDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83048BE0: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83048BE4: 40980044  bge cr6, 0x83048c28
	if !ctx.cr[6].lt {
	pc = 0x83048C28; continue 'dispatch;
	}
	// 83048BE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048BEC: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83048BF0: 4BFE3C81  bl 0x8302c870
	ctx.lr = 0x83048BF4;
	sub_8302C870(ctx, base);
	// 83048BF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83048BF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048BFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048C00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83048C04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048C08: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83048C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83048C10: 4E800421  bctrl
	ctx.lr = 0x83048C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83048C14: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048C18: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83048C1C: 419A000C  beq cr6, 0x83048c28
	if ctx.cr[6].eq {
	pc = 0x83048C28; continue 'dispatch;
	}
	// 83048C20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83048C24: 4BFFFFBC  b 0x83048be0
	pc = 0x83048BE0; continue 'dispatch;
	// 83048C28: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83048C2C: 409A0048  bne cr6, 0x83048c74
	if !ctx.cr[6].eq {
	pc = 0x83048C74; continue 'dispatch;
	}
	// 83048C30: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048C34: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 83048C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83048C3C: 388B1448  addi r4, r11, 0x1448
	ctx.r[4].s64 = ctx.r[11].s64 + 5192;
	// 83048C40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83048C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83048C48: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83048C4C: 38C000F4  li r6, 0xf4
	ctx.r[6].s64 = 244;
	// 83048C50: 38A00120  li r5, 0x120
	ctx.r[5].s64 = 288;
	// 83048C54: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83048C58: 4BFCCF31  bl 0x83015b88
	ctx.lr = 0x83048C5C;
	sub_83015B88(ctx, base);
	// 83048C5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048C60: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048C64: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83048C68: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83048C6C: 48167FBD  bl 0x831b0c28
	ctx.lr = 0x83048C70;
	sub_831B0C28(ctx, base);
	// 83048C70: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048C74: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83048C78: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83048C7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048C80: 48049AE1  bl 0x83092760
	ctx.lr = 0x83048C84;
	sub_83092760(ctx, base);
	// 83048C84: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048C88: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83048C8C: 48049F25  bl 0x83092bb0
	ctx.lr = 0x83048C90;
	sub_83092BB0(ctx, base);
	// 83048C90: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048C94: 383F0160  addi r1, r31, 0x160
	ctx.r[1].s64 = ctx.r[31].s64 + 352;
	// 83048C98: 4815F514  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048C9C size=8
    let mut pc: u32 = 0x83048C9C;
    'dispatch: loop {
        match pc {
            0x83048C9C => {
    //   block [0x83048C9C..0x83048CA4)
	// 83048C9C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83048CA0: 82161510  lwz r16, 0x1510(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048CA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048CA4 size=88
    let mut pc: u32 = 0x83048CA4;
    'dispatch: loop {
        match pc {
            0x83048CA4 => {
    //   block [0x83048CA4..0x83048CFC)
	// 83048CA4: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 83048CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048CB4: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 83048CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83048CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83048CC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83048CC4: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83048CC8: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048CCC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048CD0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83048CD4: 388B1448  addi r4, r11, 0x1448
	ctx.r[4].s64 = ctx.r[11].s64 + 5192;
	// 83048CD8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83048CDC: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83048CE0: 38A000FE  li r5, 0xfe
	ctx.r[5].s64 = 254;
	// 83048CE4: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83048CE8: 4BFCCEA1  bl 0x83015b88
	ctx.lr = 0x83048CEC;
	sub_83015B88(ctx, base);
	// 83048CEC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048CF0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83048CF4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83048CF8: 48167F31  bl 0x831b0c28
	ctx.lr = 0x83048CFC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048D04 size=84
    let mut pc: u32 = 0x83048D04;
    'dispatch: loop {
        match pc {
            0x83048D04 => {
    //   block [0x83048D04..0x83048D58)
	// 83048D04: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 83048D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048D14: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83048D1C: 388B1448  addi r4, r11, 0x1448
	ctx.r[4].s64 = ctx.r[11].s64 + 5192;
	// 83048D20: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83048D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83048D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83048D2C: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83048D30: 38A00127  li r5, 0x127
	ctx.r[5].s64 = 295;
	// 83048D34: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83048D38: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83048D3C: 817F0194  lwz r11, 0x194(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(404 as u32) ) } as u64;
	// 83048D40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83048D44: 4BFF8DD5  bl 0x83041b18
	ctx.lr = 0x83048D48;
	sub_83041B18(ctx, base);
	// 83048D48: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048D4C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83048D50: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83048D54: 48167ED5  bl 0x831b0c28
	ctx.lr = 0x83048D58;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048D58 size=48
    let mut pc: u32 = 0x83048D58;
    'dispatch: loop {
        match pc {
            0x83048D58 => {
    //   block [0x83048D58..0x83048D88)
	// 83048D58: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 83048D5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048D60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048D64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048D68: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 83048D6C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048D70: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83048D74: 4BF8F56D  bl 0x82fd82e0
	ctx.lr = 0x83048D78;
	sub_82FD82E0(ctx, base);
	// 83048D78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83048D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048D88 size=40
    let mut pc: u32 = 0x83048D88;
    'dispatch: loop {
        match pc {
            0x83048D88 => {
    //   block [0x83048D88..0x83048DB0)
	// 83048D88: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 83048D8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048D90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83048D94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048D98: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83048D9C: 48049E15  bl 0x83092bb0
	ctx.lr = 0x83048DA0;
	sub_83092BB0(ctx, base);
	// 83048DA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83048DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83048DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83048DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048DB0 size=184
    let mut pc: u32 = 0x83048DB0;
    'dispatch: loop {
        match pc {
            0x83048DB0 => {
    //   block [0x83048DB0..0x83048E68)
	// 83048DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048DB4: 4815F3B9  bl 0x831a816c
	ctx.lr = 0x83048DB8;
	sub_831A8130(ctx, base);
	// 83048DB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048DBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83048DC0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83048DC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83048DC8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83048DCC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83048DD0: 409A000C  bne cr6, 0x83048ddc
	if !ctx.cr[6].eq {
	pc = 0x83048DDC; continue 'dispatch;
	}
	// 83048DD4: 4BFF237D  bl 0x8303b150
	ctx.lr = 0x83048DD8;
	sub_8303B150(ctx, base);
	// 83048DD8: 48000088  b 0x83048e60
	pc = 0x83048E60; continue 'dispatch;
	// 83048DDC: 40990030  ble cr6, 0x83048e0c
	if !ctx.cr[6].gt {
	pc = 0x83048E0C; continue 'dispatch;
	}
	// 83048DE0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83048DE4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83048DE8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 83048DEC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 83048DF0: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 83048DF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83048DF8: 4BF87B61  bl 0x82fd0958
	ctx.lr = 0x83048DFC;
	sub_82FD0958(ctx, base);
	// 83048DFC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83048E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83048E04: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 83048E08: 48167E21  bl 0x831b0c28
	ctx.lr = 0x83048E0C;
	sub_831B0C28(ctx, base);
	// 83048E0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83048E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83048E14: 4BFF219D  bl 0x8303afb0
	ctx.lr = 0x83048E18;
	sub_8303AFB0(ctx, base);
	// 83048E18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83048E1C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83048E20: 40990028  ble cr6, 0x83048e48
	if !ctx.cr[6].gt {
	pc = 0x83048E48; continue 'dispatch;
	}
	// 83048E24: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83048E28: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 83048E2C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83048E30: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83048E34: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 83048E38: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 83048E3C: 8109FFFC  lwz r8, -4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83048E40: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83048E44: 4082FFE8  bne 0x83048e2c
	if !ctx.cr[0].eq {
	pc = 0x83048E2C; continue 'dispatch;
	}
	// 83048E48: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83048E4C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83048E50: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 83048E54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83048E58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83048E5C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83048E60: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83048E64: 4815F358  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83048E68 size=8
    let mut pc: u32 = 0x83048E68;
    'dispatch: loop {
        match pc {
            0x83048E68 => {
    //   block [0x83048E68..0x83048E70)
	// 83048E68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83048E6C: 8216164C  lwz r16, 0x164c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5708 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83048E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83048E70 size=416
    let mut pc: u32 = 0x83048E70;
    'dispatch: loop {
        match pc {
            0x83048E70 => {
    //   block [0x83048E70..0x83049010)
	// 83048E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83048E74: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83048E78: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83048E7C: 4815F2E5  bl 0x831a8160
	ctx.lr = 0x83048E80;
	sub_831A8130(ctx, base);
	// 83048E80: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83048E84: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83048E88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83048E8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83048E90: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83048E94: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 83048E98: 935F00DC  stw r26, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[26].u32 ) };
	// 83048E9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048EA0: 41820168  beq 0x83049008
	if ctx.cr[0].eq {
	pc = 0x83049008; continue 'dispatch;
	}
	// 83048EA4: 839E001C  lwz r28, 0x1c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83048EA8: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83048EAC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83048EB0: 41820058  beq 0x83048f08
	if ctx.cr[0].eq {
	pc = 0x83048F08; continue 'dispatch;
	}
	// 83048EB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83048EB8: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83048EBC: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83048EC0: 40980048  bge cr6, 0x83048f08
	if !ctx.cr[6].lt {
	pc = 0x83048F08; continue 'dispatch;
	}
	// 83048EC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048EC8: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83048ECC: 4BFE39A5  bl 0x8302c870
	ctx.lr = 0x83048ED0;
	sub_8302C870(ctx, base);
	// 83048ED0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83048ED4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048ED8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048EDC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83048EE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83048EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83048EE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83048EEC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83048EF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83048EF4: 4E800421  bctrl
	ctx.lr = 0x83048EF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83048EF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83048EFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83048F00: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83048F04: 4BFFFFB8  b 0x83048ebc
	pc = 0x83048EBC; continue 'dispatch;
	// 83048F08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83048F0C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83048F10: 40990040  ble cr6, 0x83048f50
	if !ctx.cr[6].gt {
	pc = 0x83048F50; continue 'dispatch;
	}
	// 83048F14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048F18: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83048F1C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83048F20: 4BFE3951  bl 0x8302c870
	ctx.lr = 0x83048F24;
	sub_8302C870(ctx, base);
	// 83048F24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83048F28: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 83048F2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83048F30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83048F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83048F38: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83048F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83048F40: 4E800421  bctrl
	ctx.lr = 0x83048F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83048F44: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83048F48: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83048F4C: 4198FFC8  blt cr6, 0x83048f14
	if ctx.cr[6].lt {
	pc = 0x83048F14; continue 'dispatch;
	}
	// 83048F50: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83048F54: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048F58: 4BF8F341  bl 0x82fd8298
	ctx.lr = 0x83048F5C;
	sub_82FD8298(ctx, base);
	// 83048F5C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83048F60: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83048F64: 4182002C  beq 0x83048f90
	if ctx.cr[0].eq {
	pc = 0x83048F90; continue 'dispatch;
	}
	// 83048F68: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83048F6C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048F70: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83048F74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83048F78: 48003879  bl 0x8304c7f0
	ctx.lr = 0x83048F7C;
	sub_8304C7F0(ctx, base);
	// 83048F7C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83048F80: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 83048F84: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83048F88: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83048F8C: 48000008  b 0x83048f94
	pc = 0x83048F94; continue 'dispatch;
	// 83048F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83048F94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83048F98: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83048F9C: 917E0058  stw r11, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83048FA0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83048FA4: 995E0044  stb r10, 0x44(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 83048FA8: 40990060  ble cr6, 0x83049008
	if !ctx.cr[6].gt {
	pc = 0x83049008; continue 'dispatch;
	}
	// 83048FAC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83048FB0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048FB4: 4BF8F2E5  bl 0x82fd8298
	ctx.lr = 0x83048FB8;
	sub_82FD8298(ctx, base);
	// 83048FB8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83048FBC: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 83048FC0: 4182002C  beq 0x83048fec
	if ctx.cr[0].eq {
	pc = 0x83048FEC; continue 'dispatch;
	}
	// 83048FC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83048FC8: 835E0004  lwz r26, 4(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83048FCC: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 83048FD0: 4BFE38A1  bl 0x8302c870
	ctx.lr = 0x83048FD4;
	sub_8302C870(ctx, base);
	// 83048FD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83048FD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83048FDC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83048FE0: 48049B61  bl 0x83092b40
	ctx.lr = 0x83048FE4;
	sub_83092B40(ctx, base);
	// 83048FE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83048FE8: 48000008  b 0x83048ff0
	pc = 0x83048FF0; continue 'dispatch;
	// 83048FEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83048FF0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83048FF4: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83048FF8: 4BFFFDB9  bl 0x83048db0
	ctx.lr = 0x83048FFC;
	sub_83048DB0(ctx, base);
	// 83048FFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83049000: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83049004: 4198FFA8  blt cr6, 0x83048fac
	if ctx.cr[6].lt {
	pc = 0x83048FAC; continue 'dispatch;
	}
	// 83049008: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 8304900C: 4815F1A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049010 size=8
    let mut pc: u32 = 0x83049010;
    'dispatch: loop {
        match pc {
            0x83049010 => {
    //   block [0x83049010..0x83049018)
	// 83049010: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049014: 8216164C  lwz r16, 0x164c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5708 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049018 size=96
    let mut pc: u32 = 0x83049018;
    'dispatch: loop {
        match pc {
            0x83049018 => {
    //   block [0x83049018..0x83049078)
	// 83049018: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304901C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049020: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049024: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049028: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8304902C: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83049030: 83DF00DC  lwz r30, 0xdc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 83049034: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049038: 4BFE3839  bl 0x8302c870
	ctx.lr = 0x8304903C;
	sub_8302C870(ctx, base);
	// 8304903C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049040: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83049044: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83049048: 388B1448  addi r4, r11, 0x1448
	ctx.r[4].s64 = ctx.r[11].s64 + 5192;
	// 8304904C: 38C000B5  li r6, 0xb5
	ctx.r[6].s64 = 181;
	// 83049050: 38A000D0  li r5, 0xd0
	ctx.r[5].s64 = 208;
	// 83049054: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049058: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304905C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83049064: 4BFF8AB5  bl 0x83041b18
	ctx.lr = 0x83049068;
	sub_83041B18(ctx, base);
	// 83049068: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304906C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049070: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83049074: 48167BB5  bl 0x831b0c28
	ctx.lr = 0x83049078;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049078 size=48
    let mut pc: u32 = 0x83049078;
    'dispatch: loop {
        match pc {
            0x83049078 => {
    //   block [0x83049078..0x830490A8)
	// 83049078: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304907C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049080: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049084: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049088: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8304908C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049090: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83049094: 4BF8F24D  bl 0x82fd82e0
	ctx.lr = 0x83049098;
	sub_82FD82E0(ctx, base);
	// 83049098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304909C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830490A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830490A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830490A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830490A8 size=48
    let mut pc: u32 = 0x830490A8;
    'dispatch: loop {
        match pc {
            0x830490A8 => {
    //   block [0x830490A8..0x830490D8)
	// 830490A8: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830490AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830490B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830490B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830490B8: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 830490BC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830490C0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830490C4: 4BF8F21D  bl 0x82fd82e0
	ctx.lr = 0x830490C8;
	sub_82FD82E0(ctx, base);
	// 830490C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830490CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830490D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830490D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830490D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830490D8 size=100
    let mut pc: u32 = 0x830490D8;
    'dispatch: loop {
        match pc {
            0x830490D8 => {
    //   block [0x830490D8..0x8304913C)
	// 830490D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830490DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830490E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830490E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830490E8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 830490EC: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 830490F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830490F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830490F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830490FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83049100: 48049601  bl 0x83092700
	ctx.lr = 0x83049104;
	sub_83092700(ctx, base);
	// 83049104: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304910C: 394B16D0  addi r10, r11, 0x16d0
	ctx.r[10].s64 = ctx.r[11].s64 + 5840;
	// 83049110: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83049114: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83049118: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8304911C: 997F003D  stb r11, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 83049120: 997F003C  stb r11, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 83049124: 997F003E  stb r11, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[11].u8 ) };
	// 83049128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304912C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049140 size=8
    let mut pc: u32 = 0x83049140;
    'dispatch: loop {
        match pc {
            0x83049140 => {
    //   block [0x83049140..0x83049148)
	// 83049140: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049144: 82161730  lwz r16, 0x1730(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(5936 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049148 size=84
    let mut pc: u32 = 0x83049148;
    'dispatch: loop {
        match pc {
            0x83049148 => {
    //   block [0x83049148..0x8304919C)
	// 83049148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304914C: 4815F01D  bl 0x831a8168
	ctx.lr = 0x83049150;
	sub_831A8130(ctx, base);
	// 83049150: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83049154: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049158: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304915C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83049160: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83049164: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 83049168: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304916C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049170: 48049591  bl 0x83092700
	ctx.lr = 0x83049174;
	sub_83092700(ctx, base);
	// 83049174: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049178: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304917C: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 83049180: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83049184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049188: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304918C: 48045415  bl 0x8308e5a0
	ctx.lr = 0x83049190;
	sub_8308E5A0(ctx, base);
	// 83049190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049194: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83049198: 4815F020  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304919C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304919C size=40
    let mut pc: u32 = 0x8304919C;
    'dispatch: loop {
        match pc {
            0x8304919C => {
    //   block [0x8304919C..0x830491C4)
	// 8304919C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830491A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830491A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830491A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830491AC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830491B0: 480494E9  bl 0x83092698
	ctx.lr = 0x830491B4;
	sub_83092698(ctx, base);
	// 830491B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830491B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830491BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830491C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830491C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830491C8 size=8
    let mut pc: u32 = 0x830491C8;
    'dispatch: loop {
        match pc {
            0x830491C8 => {
    //   block [0x830491C8..0x830491D0)
	// 830491C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830491CC: 82161770  lwz r16, 0x1770(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6000 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830491D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830491D0 size=116
    let mut pc: u32 = 0x830491D0;
    'dispatch: loop {
        match pc {
            0x830491D0 => {
    //   block [0x830491D0..0x83049244)
	// 830491D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830491D4: 4815EF95  bl 0x831a8168
	ctx.lr = 0x830491D8;
	sub_831A8130(ctx, base);
	// 830491D8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830491DC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830491E0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830491E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830491E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830491EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830491F0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830491F4: 48049CBD  bl 0x83092eb0
	ctx.lr = 0x830491F8;
	sub_83092EB0(ctx, base);
	// 830491F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830491FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83049200: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83049204: 48049CAD  bl 0x83092eb0
	ctx.lr = 0x83049208;
	sub_83092EB0(ctx, base);
	// 83049208: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304920C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 83049210: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 83049214: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049218: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8304921C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83049220: 4E800421  bctrl
	ctx.lr = 0x83049224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049228: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304922C: 48049CF5  bl 0x83092f20
	ctx.lr = 0x83049230;
	sub_83092F20(ctx, base);
	// 83049230: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83049234: 48049CED  bl 0x83092f20
	ctx.lr = 0x83049238;
	sub_83092F20(ctx, base);
	// 83049238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304923C: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 83049240: 4815EF78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049244(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049244 size=40
    let mut pc: u32 = 0x83049244;
    'dispatch: loop {
        match pc {
            0x83049244 => {
    //   block [0x83049244..0x8304926C)
	// 83049244: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83049248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304924C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049254: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83049258: 48049CC9  bl 0x83092f20
	ctx.lr = 0x8304925C;
	sub_83092F20(ctx, base);
	// 8304925C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304926C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304926C size=40
    let mut pc: u32 = 0x8304926C;
    'dispatch: loop {
        match pc {
            0x8304926C => {
    //   block [0x8304926C..0x83049294)
	// 8304926C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83049270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304927C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83049280: 48049CA1  bl 0x83092f20
	ctx.lr = 0x83049284;
	sub_83092F20(ctx, base);
	// 83049284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304928C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049298 size=8
    let mut pc: u32 = 0x83049298;
    'dispatch: loop {
        match pc {
            0x83049298 => {
    //   block [0x83049298..0x830492A0)
	// 83049298: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304929C: 821617C8  lwz r16, 0x17c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830492A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830492A0 size=104
    let mut pc: u32 = 0x830492A0;
    'dispatch: loop {
        match pc {
            0x830492A0 => {
    //   block [0x830492A0..0x83049308)
	// 830492A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830492A4: 4815EEBD  bl 0x831a8160
	ctx.lr = 0x830492A8;
	sub_831A8130(ctx, base);
	// 830492A8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830492AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830492B0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830492B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830492B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830492BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830492C0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830492C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830492C8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 830492CC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 830492D0: 4BF8EFC9  bl 0x82fd8298
	ctx.lr = 0x830492D4;
	sub_82FD8298(ctx, base);
	// 830492D4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830492D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830492DC: 41820020  beq 0x830492fc
	if ctx.cr[0].eq {
	pc = 0x830492FC; continue 'dispatch;
	}
	// 830492E0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830492E4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830492E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830492EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830492F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830492F4: 4BFFFE55  bl 0x83049148
	ctx.lr = 0x830492F8;
	sub_83049148(ctx, base);
	// 830492F8: 48000008  b 0x83049300
	pc = 0x83049300; continue 'dispatch;
	// 830492FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049300: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83049304: 4815EEAC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049308 size=44
    let mut pc: u32 = 0x83049308;
    'dispatch: loop {
        match pc {
            0x83049308 => {
    //   block [0x83049308..0x83049334)
	// 83049308: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304930C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049310: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049314: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049318: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304931C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049320: 4BF8EFC1  bl 0x82fd82e0
	ctx.lr = 0x83049324;
	sub_82FD82E0(ctx, base);
	// 83049324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304932C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049338 size=16
    let mut pc: u32 = 0x83049338;
    'dispatch: loop {
        match pc {
            0x83049338 => {
    //   block [0x83049338..0x83049348)
	// 83049338: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8304933C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83049340: 80A30024  lwz r5, 0x24(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83049344: 48048C44  b 0x83091f88
	sub_83091F88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049348 size=8
    let mut pc: u32 = 0x83049348;
    'dispatch: loop {
        match pc {
            0x83049348 => {
    //   block [0x83049348..0x83049350)
	// 83049348: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304934C: 82161800  lwz r16, 0x1800(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049350 size=84
    let mut pc: u32 = 0x83049350;
    'dispatch: loop {
        match pc {
            0x83049350 => {
    //   block [0x83049350..0x830493A4)
	// 83049350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049354: 4815EE19  bl 0x831a816c
	ctx.lr = 0x83049358;
	sub_831A8130(ctx, base);
	// 83049358: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304935C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049360: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049364: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83049368: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304936C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049370: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049374: 4BF8EF25  bl 0x82fd8298
	ctx.lr = 0x83049378;
	sub_82FD8298(ctx, base);
	// 83049378: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304937C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049380: 41820014  beq 0x83049394
	if ctx.cr[0].eq {
	pc = 0x83049394; continue 'dispatch;
	}
	// 83049384: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83049388: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304938C: 48049B25  bl 0x83092eb0
	ctx.lr = 0x83049390;
	sub_83092EB0(ctx, base);
	// 83049390: 48000008  b 0x83049398
	pc = 0x83049398; continue 'dispatch;
	// 83049394: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049398: 907E0048  stw r3, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 8304939C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830493A0: 4815EE1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830493A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830493A4 size=48
    let mut pc: u32 = 0x830493A4;
    'dispatch: loop {
        match pc {
            0x830493A4 => {
    //   block [0x830493A4..0x830493D4)
	// 830493A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830493A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830493AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830493B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830493B4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830493B8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830493BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830493C0: 4BF8EF21  bl 0x82fd82e0
	ctx.lr = 0x830493C4;
	sub_82FD82E0(ctx, base);
	// 830493C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830493C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830493CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830493D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830493D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830493D8 size=8
    let mut pc: u32 = 0x830493D8;
    'dispatch: loop {
        match pc {
            0x830493D8 => {
    //   block [0x830493D8..0x830493E0)
	// 830493D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830493DC: 82161838  lwz r16, 0x1838(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830493E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830493E0 size=84
    let mut pc: u32 = 0x830493E0;
    'dispatch: loop {
        match pc {
            0x830493E0 => {
    //   block [0x830493E0..0x83049434)
	// 830493E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830493E4: 4815ED89  bl 0x831a816c
	ctx.lr = 0x830493E8;
	sub_831A8130(ctx, base);
	// 830493E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830493EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830493F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830493F4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 830493F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830493FC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049400: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049404: 4BF8EE95  bl 0x82fd8298
	ctx.lr = 0x83049408;
	sub_82FD8298(ctx, base);
	// 83049408: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304940C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049410: 41820014  beq 0x83049424
	if ctx.cr[0].eq {
	pc = 0x83049424; continue 'dispatch;
	}
	// 83049414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83049418: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304941C: 48049A95  bl 0x83092eb0
	ctx.lr = 0x83049420;
	sub_83092EB0(ctx, base);
	// 83049420: 48000008  b 0x83049428
	pc = 0x83049428; continue 'dispatch;
	// 83049424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049428: 907E004C  stw r3, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 8304942C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83049430: 4815ED8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049434 size=48
    let mut pc: u32 = 0x83049434;
    'dispatch: loop {
        match pc {
            0x83049434 => {
    //   block [0x83049434..0x83049464)
	// 83049434: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83049438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049444: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83049448: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304944C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049450: 4BF8EE91  bl 0x82fd82e0
	ctx.lr = 0x83049454;
	sub_82FD82E0(ctx, base);
	// 83049454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304945C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049468 size=8
    let mut pc: u32 = 0x83049468;
    'dispatch: loop {
        match pc {
            0x83049468 => {
    //   block [0x83049468..0x83049470)
	// 83049468: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304946C: 82161870  lwz r16, 0x1870(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6256 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049470 size=84
    let mut pc: u32 = 0x83049470;
    'dispatch: loop {
        match pc {
            0x83049470 => {
    //   block [0x83049470..0x830494C4)
	// 83049470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049474: 4815ECF9  bl 0x831a816c
	ctx.lr = 0x83049478;
	sub_831A8130(ctx, base);
	// 83049478: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304947C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049484: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83049488: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304948C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049490: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049494: 4BF8EE05  bl 0x82fd8298
	ctx.lr = 0x83049498;
	sub_82FD8298(ctx, base);
	// 83049498: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304949C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830494A0: 41820014  beq 0x830494b4
	if ctx.cr[0].eq {
	pc = 0x830494B4; continue 'dispatch;
	}
	// 830494A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830494A8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830494AC: 48049A05  bl 0x83092eb0
	ctx.lr = 0x830494B0;
	sub_83092EB0(ctx, base);
	// 830494B0: 48000008  b 0x830494b8
	pc = 0x830494B8; continue 'dispatch;
	// 830494B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830494B8: 907E0050  stw r3, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830494BC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830494C0: 4815ECFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830494C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830494C4 size=48
    let mut pc: u32 = 0x830494C4;
    'dispatch: loop {
        match pc {
            0x830494C4 => {
    //   block [0x830494C4..0x830494F4)
	// 830494C4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830494C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830494CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830494D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830494D4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830494D8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830494DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830494E0: 4BF8EE01  bl 0x82fd82e0
	ctx.lr = 0x830494E4;
	sub_82FD82E0(ctx, base);
	// 830494E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830494E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830494EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830494F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830494F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830494F8 size=8
    let mut pc: u32 = 0x830494F8;
    'dispatch: loop {
        match pc {
            0x830494F8 => {
    //   block [0x830494F8..0x83049500)
	// 830494F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830494FC: 821618A8  lwz r16, 0x18a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6312 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049500 size=84
    let mut pc: u32 = 0x83049500;
    'dispatch: loop {
        match pc {
            0x83049500 => {
    //   block [0x83049500..0x83049554)
	// 83049500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049504: 4815EC69  bl 0x831a816c
	ctx.lr = 0x83049508;
	sub_831A8130(ctx, base);
	// 83049508: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304950C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049510: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049514: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83049518: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304951C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049520: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049524: 4BF8ED75  bl 0x82fd8298
	ctx.lr = 0x83049528;
	sub_82FD8298(ctx, base);
	// 83049528: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304952C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049530: 41820014  beq 0x83049544
	if ctx.cr[0].eq {
	pc = 0x83049544; continue 'dispatch;
	}
	// 83049534: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83049538: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304953C: 48049975  bl 0x83092eb0
	ctx.lr = 0x83049540;
	sub_83092EB0(ctx, base);
	// 83049540: 48000008  b 0x83049548
	pc = 0x83049548; continue 'dispatch;
	// 83049544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049548: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8304954C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83049550: 4815EC6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049554(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049554 size=48
    let mut pc: u32 = 0x83049554;
    'dispatch: loop {
        match pc {
            0x83049554 => {
    //   block [0x83049554..0x83049584)
	// 83049554: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83049558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049564: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83049568: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304956C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049570: 4BF8ED71  bl 0x82fd82e0
	ctx.lr = 0x83049574;
	sub_82FD82E0(ctx, base);
	// 83049574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304957C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049588 size=8
    let mut pc: u32 = 0x83049588;
    'dispatch: loop {
        match pc {
            0x83049588 => {
    //   block [0x83049588..0x83049590)
	// 83049588: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304958C: 821618E0  lwz r16, 0x18e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049590 size=96
    let mut pc: u32 = 0x83049590;
    'dispatch: loop {
        match pc {
            0x83049590 => {
    //   block [0x83049590..0x830495F0)
	// 83049590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304959C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830495A0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830495A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830495A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830495AC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830495B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830495B4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830495B8: 4BF8ECE1  bl 0x82fd8298
	ctx.lr = 0x830495BC;
	sub_82FD8298(ctx, base);
	// 830495BC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830495C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830495C4: 41820010  beq 0x830495d4
	if ctx.cr[0].eq {
	pc = 0x830495D4; continue 'dispatch;
	}
	// 830495C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830495CC: 4BFFFB0D  bl 0x830490d8
	ctx.lr = 0x830495D0;
	sub_830490D8(ctx, base);
	// 830495D0: 48000008  b 0x830495d8
	pc = 0x830495D8; continue 'dispatch;
	// 830495D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830495D8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830495DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830495E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830495E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830495E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830495EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830495F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830495F0 size=44
    let mut pc: u32 = 0x830495F0;
    'dispatch: loop {
        match pc {
            0x830495F0 => {
    //   block [0x830495F0..0x8304961C)
	// 830495F0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830495F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830495F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830495FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049600: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83049604: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049608: 4BF8ECD9  bl 0x82fd82e0
	ctx.lr = 0x8304960C;
	sub_82FD82E0(ctx, base);
	// 8304960C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049620 size=12
    let mut pc: u32 = 0x83049620;
    'dispatch: loop {
        match pc {
            0x83049620 => {
    //   block [0x83049620..0x8304962C)
	// 83049620: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83049624: 386B33C4  addi r3, r11, 0x33c4
	ctx.r[3].s64 = ctx.r[11].s64 + 13252;
	// 83049628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049630 size=92
    let mut pc: u32 = 0x83049630;
    'dispatch: loop {
        match pc {
            0x83049630 => {
    //   block [0x83049630..0x8304968C)
	// 83049630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304963C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049644: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83049648: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304964C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83049650: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83049654: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83049658: 41820010  beq 0x83049668
	if ctx.cr[0].eq {
	pc = 0x83049668; continue 'dispatch;
	}
	// 8304965C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83049660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83049664: 4BFAFC95  bl 0x82ff92f8
	ctx.lr = 0x83049668;
	sub_82FF92F8(ctx, base);
	// 83049668: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304966C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049670: 480494C1  bl 0x83092b30
	ctx.lr = 0x83049674;
	sub_83092B30(ctx, base);
	// 83049674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83049678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304967C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83049684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


