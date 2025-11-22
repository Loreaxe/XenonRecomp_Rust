pub fn sub_8324F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F208 size=64
    let mut pc: u32 = 0x8324F208;
    'dispatch: loop {
        match pc {
            0x8324F208 => {
    //   block [0x8324F208..0x8324F248)
	// 8324F208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F218: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F21C: 388B0750  addi r4, r11, 0x750
	ctx.r[4].s64 = ctx.r[11].s64 + 1872;
	// 8324F220: 386A7DE4  addi r3, r10, 0x7de4
	ctx.r[3].s64 = ctx.r[10].s64 + 32228;
	// 8324F224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F228: 4AFDDCA9  bl 0x8222ced0
	ctx.lr = 0x8324F22C;
	sub_8222CED0(ctx, base);
	// 8324F22C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F230: 38698F48  addi r3, r9, -0x70b8
	ctx.r[3].s64 = ctx.r[9].s64 + -28856;
	// 8324F234: 4BA5ACED  bl 0x82ca9f20
	ctx.lr = 0x8324F238;
	sub_82CA9F20(ctx, base);
	// 8324F238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F248 size=64
    let mut pc: u32 = 0x8324F248;
    'dispatch: loop {
        match pc {
            0x8324F248 => {
    //   block [0x8324F248..0x8324F288)
	// 8324F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F25C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F260: 386A7DE8  addi r3, r10, 0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + 32232;
	// 8324F264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F268: 4AFDDC69  bl 0x8222ced0
	ctx.lr = 0x8324F26C;
	sub_8222CED0(ctx, base);
	// 8324F26C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F270: 38698F58  addi r3, r9, -0x70a8
	ctx.r[3].s64 = ctx.r[9].s64 + -28840;
	// 8324F274: 4BA5ACAD  bl 0x82ca9f20
	ctx.lr = 0x8324F278;
	sub_82CA9F20(ctx, base);
	// 8324F278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F288 size=64
    let mut pc: u32 = 0x8324F288;
    'dispatch: loop {
        match pc {
            0x8324F288 => {
    //   block [0x8324F288..0x8324F2C8)
	// 8324F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F294: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F298: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F29C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324F2A0: 386A7DEC  addi r3, r10, 0x7dec
	ctx.r[3].s64 = ctx.r[10].s64 + 32236;
	// 8324F2A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F2A8: 4AFDDC29  bl 0x8222ced0
	ctx.lr = 0x8324F2AC;
	sub_8222CED0(ctx, base);
	// 8324F2AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F2B0: 38698F68  addi r3, r9, -0x7098
	ctx.r[3].s64 = ctx.r[9].s64 + -28824;
	// 8324F2B4: 4BA5AC6D  bl 0x82ca9f20
	ctx.lr = 0x8324F2B8;
	sub_82CA9F20(ctx, base);
	// 8324F2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F2C8 size=64
    let mut pc: u32 = 0x8324F2C8;
    'dispatch: loop {
        match pc {
            0x8324F2C8 => {
    //   block [0x8324F2C8..0x8324F308)
	// 8324F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F2D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F2D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F2DC: 388B078C  addi r4, r11, 0x78c
	ctx.r[4].s64 = ctx.r[11].s64 + 1932;
	// 8324F2E0: 386A7DF0  addi r3, r10, 0x7df0
	ctx.r[3].s64 = ctx.r[10].s64 + 32240;
	// 8324F2E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F2E8: 4AFDDBE9  bl 0x8222ced0
	ctx.lr = 0x8324F2EC;
	sub_8222CED0(ctx, base);
	// 8324F2EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F2F0: 38698F78  addi r3, r9, -0x7088
	ctx.r[3].s64 = ctx.r[9].s64 + -28808;
	// 8324F2F4: 4BA5AC2D  bl 0x82ca9f20
	ctx.lr = 0x8324F2F8;
	sub_82CA9F20(ctx, base);
	// 8324F2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F308 size=64
    let mut pc: u32 = 0x8324F308;
    'dispatch: loop {
        match pc {
            0x8324F308 => {
    //   block [0x8324F308..0x8324F348)
	// 8324F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F314: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F318: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F31C: 388B07A4  addi r4, r11, 0x7a4
	ctx.r[4].s64 = ctx.r[11].s64 + 1956;
	// 8324F320: 386A7DF4  addi r3, r10, 0x7df4
	ctx.r[3].s64 = ctx.r[10].s64 + 32244;
	// 8324F324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F328: 4AFDDBA9  bl 0x8222ced0
	ctx.lr = 0x8324F32C;
	sub_8222CED0(ctx, base);
	// 8324F32C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F330: 38698F88  addi r3, r9, -0x7078
	ctx.r[3].s64 = ctx.r[9].s64 + -28792;
	// 8324F334: 4BA5ABED  bl 0x82ca9f20
	ctx.lr = 0x8324F338;
	sub_82CA9F20(ctx, base);
	// 8324F338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F348 size=64
    let mut pc: u32 = 0x8324F348;
    'dispatch: loop {
        match pc {
            0x8324F348 => {
    //   block [0x8324F348..0x8324F388)
	// 8324F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F354: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F35C: 388B07BC  addi r4, r11, 0x7bc
	ctx.r[4].s64 = ctx.r[11].s64 + 1980;
	// 8324F360: 386A7DF8  addi r3, r10, 0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + 32248;
	// 8324F364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F368: 4AFDDB69  bl 0x8222ced0
	ctx.lr = 0x8324F36C;
	sub_8222CED0(ctx, base);
	// 8324F36C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F370: 38698F98  addi r3, r9, -0x7068
	ctx.r[3].s64 = ctx.r[9].s64 + -28776;
	// 8324F374: 4BA5ABAD  bl 0x82ca9f20
	ctx.lr = 0x8324F378;
	sub_82CA9F20(ctx, base);
	// 8324F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F388 size=64
    let mut pc: u32 = 0x8324F388;
    'dispatch: loop {
        match pc {
            0x8324F388 => {
    //   block [0x8324F388..0x8324F3C8)
	// 8324F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F394: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F398: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F39C: 388B07D0  addi r4, r11, 0x7d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2000;
	// 8324F3A0: 386A7DFC  addi r3, r10, 0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 32252;
	// 8324F3A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F3A8: 4AFDDB29  bl 0x8222ced0
	ctx.lr = 0x8324F3AC;
	sub_8222CED0(ctx, base);
	// 8324F3AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F3B0: 38698FA8  addi r3, r9, -0x7058
	ctx.r[3].s64 = ctx.r[9].s64 + -28760;
	// 8324F3B4: 4BA5AB6D  bl 0x82ca9f20
	ctx.lr = 0x8324F3B8;
	sub_82CA9F20(ctx, base);
	// 8324F3B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F3C8 size=64
    let mut pc: u32 = 0x8324F3C8;
    'dispatch: loop {
        match pc {
            0x8324F3C8 => {
    //   block [0x8324F3C8..0x8324F408)
	// 8324F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F3D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F3D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F3D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F3DC: 388B07EC  addi r4, r11, 0x7ec
	ctx.r[4].s64 = ctx.r[11].s64 + 2028;
	// 8324F3E0: 386A7E00  addi r3, r10, 0x7e00
	ctx.r[3].s64 = ctx.r[10].s64 + 32256;
	// 8324F3E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F3E8: 4AFDDAE9  bl 0x8222ced0
	ctx.lr = 0x8324F3EC;
	sub_8222CED0(ctx, base);
	// 8324F3EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F3F0: 38698FB8  addi r3, r9, -0x7048
	ctx.r[3].s64 = ctx.r[9].s64 + -28744;
	// 8324F3F4: 4BA5AB2D  bl 0x82ca9f20
	ctx.lr = 0x8324F3F8;
	sub_82CA9F20(ctx, base);
	// 8324F3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F408 size=64
    let mut pc: u32 = 0x8324F408;
    'dispatch: loop {
        match pc {
            0x8324F408 => {
    //   block [0x8324F408..0x8324F448)
	// 8324F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F41C: 388B0804  addi r4, r11, 0x804
	ctx.r[4].s64 = ctx.r[11].s64 + 2052;
	// 8324F420: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 8324F424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F428: 4AFDDAA9  bl 0x8222ced0
	ctx.lr = 0x8324F42C;
	sub_8222CED0(ctx, base);
	// 8324F42C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F430: 38698FC8  addi r3, r9, -0x7038
	ctx.r[3].s64 = ctx.r[9].s64 + -28728;
	// 8324F434: 4BA5AAED  bl 0x82ca9f20
	ctx.lr = 0x8324F438;
	sub_82CA9F20(ctx, base);
	// 8324F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F448 size=64
    let mut pc: u32 = 0x8324F448;
    'dispatch: loop {
        match pc {
            0x8324F448 => {
    //   block [0x8324F448..0x8324F488)
	// 8324F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F454: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F45C: 388B0818  addi r4, r11, 0x818
	ctx.r[4].s64 = ctx.r[11].s64 + 2072;
	// 8324F460: 386A7E08  addi r3, r10, 0x7e08
	ctx.r[3].s64 = ctx.r[10].s64 + 32264;
	// 8324F464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F468: 4AFDDA69  bl 0x8222ced0
	ctx.lr = 0x8324F46C;
	sub_8222CED0(ctx, base);
	// 8324F46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F470: 38698FD8  addi r3, r9, -0x7028
	ctx.r[3].s64 = ctx.r[9].s64 + -28712;
	// 8324F474: 4BA5AAAD  bl 0x82ca9f20
	ctx.lr = 0x8324F478;
	sub_82CA9F20(ctx, base);
	// 8324F478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F488 size=64
    let mut pc: u32 = 0x8324F488;
    'dispatch: loop {
        match pc {
            0x8324F488 => {
    //   block [0x8324F488..0x8324F4C8)
	// 8324F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F494: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F49C: 388B082C  addi r4, r11, 0x82c
	ctx.r[4].s64 = ctx.r[11].s64 + 2092;
	// 8324F4A0: 386A7E0C  addi r3, r10, 0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32268;
	// 8324F4A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F4A8: 4AFDDA29  bl 0x8222ced0
	ctx.lr = 0x8324F4AC;
	sub_8222CED0(ctx, base);
	// 8324F4AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F4B0: 38698FE8  addi r3, r9, -0x7018
	ctx.r[3].s64 = ctx.r[9].s64 + -28696;
	// 8324F4B4: 4BA5AA6D  bl 0x82ca9f20
	ctx.lr = 0x8324F4B8;
	sub_82CA9F20(ctx, base);
	// 8324F4B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F4C8 size=64
    let mut pc: u32 = 0x8324F4C8;
    'dispatch: loop {
        match pc {
            0x8324F4C8 => {
    //   block [0x8324F4C8..0x8324F508)
	// 8324F4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F4D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F4D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F4D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F4DC: 388B0840  addi r4, r11, 0x840
	ctx.r[4].s64 = ctx.r[11].s64 + 2112;
	// 8324F4E0: 386A7E10  addi r3, r10, 0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + 32272;
	// 8324F4E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F4E8: 4AFDD9E9  bl 0x8222ced0
	ctx.lr = 0x8324F4EC;
	sub_8222CED0(ctx, base);
	// 8324F4EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F4F0: 38698FF8  addi r3, r9, -0x7008
	ctx.r[3].s64 = ctx.r[9].s64 + -28680;
	// 8324F4F4: 4BA5AA2D  bl 0x82ca9f20
	ctx.lr = 0x8324F4F8;
	sub_82CA9F20(ctx, base);
	// 8324F4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F508 size=64
    let mut pc: u32 = 0x8324F508;
    'dispatch: loop {
        match pc {
            0x8324F508 => {
    //   block [0x8324F508..0x8324F548)
	// 8324F508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F51C: 388B0854  addi r4, r11, 0x854
	ctx.r[4].s64 = ctx.r[11].s64 + 2132;
	// 8324F520: 386A7E14  addi r3, r10, 0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + 32276;
	// 8324F524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F528: 4AFDD9A9  bl 0x8222ced0
	ctx.lr = 0x8324F52C;
	sub_8222CED0(ctx, base);
	// 8324F52C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F530: 38699008  addi r3, r9, -0x6ff8
	ctx.r[3].s64 = ctx.r[9].s64 + -28664;
	// 8324F534: 4BA5A9ED  bl 0x82ca9f20
	ctx.lr = 0x8324F538;
	sub_82CA9F20(ctx, base);
	// 8324F538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F548 size=64
    let mut pc: u32 = 0x8324F548;
    'dispatch: loop {
        match pc {
            0x8324F548 => {
    //   block [0x8324F548..0x8324F588)
	// 8324F548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F558: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F55C: 388B0864  addi r4, r11, 0x864
	ctx.r[4].s64 = ctx.r[11].s64 + 2148;
	// 8324F560: 386A7E18  addi r3, r10, 0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + 32280;
	// 8324F564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F568: 4AFDD969  bl 0x8222ced0
	ctx.lr = 0x8324F56C;
	sub_8222CED0(ctx, base);
	// 8324F56C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F570: 38699018  addi r3, r9, -0x6fe8
	ctx.r[3].s64 = ctx.r[9].s64 + -28648;
	// 8324F574: 4BA5A9AD  bl 0x82ca9f20
	ctx.lr = 0x8324F578;
	sub_82CA9F20(ctx, base);
	// 8324F578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F57C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F588 size=64
    let mut pc: u32 = 0x8324F588;
    'dispatch: loop {
        match pc {
            0x8324F588 => {
    //   block [0x8324F588..0x8324F5C8)
	// 8324F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F594: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F598: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F59C: 388B0874  addi r4, r11, 0x874
	ctx.r[4].s64 = ctx.r[11].s64 + 2164;
	// 8324F5A0: 386A7E1C  addi r3, r10, 0x7e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32284;
	// 8324F5A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F5A8: 4AFDD929  bl 0x8222ced0
	ctx.lr = 0x8324F5AC;
	sub_8222CED0(ctx, base);
	// 8324F5AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F5B0: 38699028  addi r3, r9, -0x6fd8
	ctx.r[3].s64 = ctx.r[9].s64 + -28632;
	// 8324F5B4: 4BA5A96D  bl 0x82ca9f20
	ctx.lr = 0x8324F5B8;
	sub_82CA9F20(ctx, base);
	// 8324F5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F5C8 size=64
    let mut pc: u32 = 0x8324F5C8;
    'dispatch: loop {
        match pc {
            0x8324F5C8 => {
    //   block [0x8324F5C8..0x8324F608)
	// 8324F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F5D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F5D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F5DC: 388B0888  addi r4, r11, 0x888
	ctx.r[4].s64 = ctx.r[11].s64 + 2184;
	// 8324F5E0: 386A7E20  addi r3, r10, 0x7e20
	ctx.r[3].s64 = ctx.r[10].s64 + 32288;
	// 8324F5E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F5E8: 4AFDD8E9  bl 0x8222ced0
	ctx.lr = 0x8324F5EC;
	sub_8222CED0(ctx, base);
	// 8324F5EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F5F0: 38699038  addi r3, r9, -0x6fc8
	ctx.r[3].s64 = ctx.r[9].s64 + -28616;
	// 8324F5F4: 4BA5A92D  bl 0x82ca9f20
	ctx.lr = 0x8324F5F8;
	sub_82CA9F20(ctx, base);
	// 8324F5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F608 size=64
    let mut pc: u32 = 0x8324F608;
    'dispatch: loop {
        match pc {
            0x8324F608 => {
    //   block [0x8324F608..0x8324F648)
	// 8324F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F614: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F618: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F61C: 388B08A8  addi r4, r11, 0x8a8
	ctx.r[4].s64 = ctx.r[11].s64 + 2216;
	// 8324F620: 386A7E24  addi r3, r10, 0x7e24
	ctx.r[3].s64 = ctx.r[10].s64 + 32292;
	// 8324F624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F628: 4AFDD8A9  bl 0x8222ced0
	ctx.lr = 0x8324F62C;
	sub_8222CED0(ctx, base);
	// 8324F62C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F630: 38699048  addi r3, r9, -0x6fb8
	ctx.r[3].s64 = ctx.r[9].s64 + -28600;
	// 8324F634: 4BA5A8ED  bl 0x82ca9f20
	ctx.lr = 0x8324F638;
	sub_82CA9F20(ctx, base);
	// 8324F638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F648 size=64
    let mut pc: u32 = 0x8324F648;
    'dispatch: loop {
        match pc {
            0x8324F648 => {
    //   block [0x8324F648..0x8324F688)
	// 8324F648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F654: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F658: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F65C: 388B08BC  addi r4, r11, 0x8bc
	ctx.r[4].s64 = ctx.r[11].s64 + 2236;
	// 8324F660: 386A7E28  addi r3, r10, 0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + 32296;
	// 8324F664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F668: 4AFDD869  bl 0x8222ced0
	ctx.lr = 0x8324F66C;
	sub_8222CED0(ctx, base);
	// 8324F66C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F670: 38699058  addi r3, r9, -0x6fa8
	ctx.r[3].s64 = ctx.r[9].s64 + -28584;
	// 8324F674: 4BA5A8AD  bl 0x82ca9f20
	ctx.lr = 0x8324F678;
	sub_82CA9F20(ctx, base);
	// 8324F678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F688 size=64
    let mut pc: u32 = 0x8324F688;
    'dispatch: loop {
        match pc {
            0x8324F688 => {
    //   block [0x8324F688..0x8324F6C8)
	// 8324F688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F69C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 8324F6A0: 386A7E2C  addi r3, r10, 0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32300;
	// 8324F6A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F6A8: 4AFDD829  bl 0x8222ced0
	ctx.lr = 0x8324F6AC;
	sub_8222CED0(ctx, base);
	// 8324F6AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F6B0: 38699068  addi r3, r9, -0x6f98
	ctx.r[3].s64 = ctx.r[9].s64 + -28568;
	// 8324F6B4: 4BA5A86D  bl 0x82ca9f20
	ctx.lr = 0x8324F6B8;
	sub_82CA9F20(ctx, base);
	// 8324F6B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F6C8 size=64
    let mut pc: u32 = 0x8324F6C8;
    'dispatch: loop {
        match pc {
            0x8324F6C8 => {
    //   block [0x8324F6C8..0x8324F708)
	// 8324F6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F6D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F6D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F6DC: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 8324F6E0: 386A7E30  addi r3, r10, 0x7e30
	ctx.r[3].s64 = ctx.r[10].s64 + 32304;
	// 8324F6E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F6E8: 4AFDD7E9  bl 0x8222ced0
	ctx.lr = 0x8324F6EC;
	sub_8222CED0(ctx, base);
	// 8324F6EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F6F0: 38699078  addi r3, r9, -0x6f88
	ctx.r[3].s64 = ctx.r[9].s64 + -28552;
	// 8324F6F4: 4BA5A82D  bl 0x82ca9f20
	ctx.lr = 0x8324F6F8;
	sub_82CA9F20(ctx, base);
	// 8324F6F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F708 size=64
    let mut pc: u32 = 0x8324F708;
    'dispatch: loop {
        match pc {
            0x8324F708 => {
    //   block [0x8324F708..0x8324F748)
	// 8324F708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F71C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 8324F720: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 8324F724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F728: 4AFDD7A9  bl 0x8222ced0
	ctx.lr = 0x8324F72C;
	sub_8222CED0(ctx, base);
	// 8324F72C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F730: 38699088  addi r3, r9, -0x6f78
	ctx.r[3].s64 = ctx.r[9].s64 + -28536;
	// 8324F734: 4BA5A7ED  bl 0x82ca9f20
	ctx.lr = 0x8324F738;
	sub_82CA9F20(ctx, base);
	// 8324F738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F748 size=64
    let mut pc: u32 = 0x8324F748;
    'dispatch: loop {
        match pc {
            0x8324F748 => {
    //   block [0x8324F748..0x8324F788)
	// 8324F748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F754: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F758: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F75C: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 8324F760: 386A7E38  addi r3, r10, 0x7e38
	ctx.r[3].s64 = ctx.r[10].s64 + 32312;
	// 8324F764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F768: 4AFDD769  bl 0x8222ced0
	ctx.lr = 0x8324F76C;
	sub_8222CED0(ctx, base);
	// 8324F76C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F770: 38699098  addi r3, r9, -0x6f68
	ctx.r[3].s64 = ctx.r[9].s64 + -28520;
	// 8324F774: 4BA5A7AD  bl 0x82ca9f20
	ctx.lr = 0x8324F778;
	sub_82CA9F20(ctx, base);
	// 8324F778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F788 size=64
    let mut pc: u32 = 0x8324F788;
    'dispatch: loop {
        match pc {
            0x8324F788 => {
    //   block [0x8324F788..0x8324F7C8)
	// 8324F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F794: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F798: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F79C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F7A0: 386A7E3C  addi r3, r10, 0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32316;
	// 8324F7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F7A8: 4AFDD729  bl 0x8222ced0
	ctx.lr = 0x8324F7AC;
	sub_8222CED0(ctx, base);
	// 8324F7AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F7B0: 386990A8  addi r3, r9, -0x6f58
	ctx.r[3].s64 = ctx.r[9].s64 + -28504;
	// 8324F7B4: 4BA5A76D  bl 0x82ca9f20
	ctx.lr = 0x8324F7B8;
	sub_82CA9F20(ctx, base);
	// 8324F7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F7C8 size=64
    let mut pc: u32 = 0x8324F7C8;
    'dispatch: loop {
        match pc {
            0x8324F7C8 => {
    //   block [0x8324F7C8..0x8324F808)
	// 8324F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F7D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F7D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F7DC: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324F7E0: 386A7E40  addi r3, r10, 0x7e40
	ctx.r[3].s64 = ctx.r[10].s64 + 32320;
	// 8324F7E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F7E8: 4AFDD6E9  bl 0x8222ced0
	ctx.lr = 0x8324F7EC;
	sub_8222CED0(ctx, base);
	// 8324F7EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F7F0: 386990B8  addi r3, r9, -0x6f48
	ctx.r[3].s64 = ctx.r[9].s64 + -28488;
	// 8324F7F4: 4BA5A72D  bl 0x82ca9f20
	ctx.lr = 0x8324F7F8;
	sub_82CA9F20(ctx, base);
	// 8324F7F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F808 size=64
    let mut pc: u32 = 0x8324F808;
    'dispatch: loop {
        match pc {
            0x8324F808 => {
    //   block [0x8324F808..0x8324F848)
	// 8324F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F814: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F818: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F81C: 388B0900  addi r4, r11, 0x900
	ctx.r[4].s64 = ctx.r[11].s64 + 2304;
	// 8324F820: 386A7E44  addi r3, r10, 0x7e44
	ctx.r[3].s64 = ctx.r[10].s64 + 32324;
	// 8324F824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F828: 4AFDD6A9  bl 0x8222ced0
	ctx.lr = 0x8324F82C;
	sub_8222CED0(ctx, base);
	// 8324F82C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F830: 386990C8  addi r3, r9, -0x6f38
	ctx.r[3].s64 = ctx.r[9].s64 + -28472;
	// 8324F834: 4BA5A6ED  bl 0x82ca9f20
	ctx.lr = 0x8324F838;
	sub_82CA9F20(ctx, base);
	// 8324F838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F848 size=64
    let mut pc: u32 = 0x8324F848;
    'dispatch: loop {
        match pc {
            0x8324F848 => {
    //   block [0x8324F848..0x8324F888)
	// 8324F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F854: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F858: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F85C: 388B0914  addi r4, r11, 0x914
	ctx.r[4].s64 = ctx.r[11].s64 + 2324;
	// 8324F860: 386A7E48  addi r3, r10, 0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + 32328;
	// 8324F864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F868: 4AFDD669  bl 0x8222ced0
	ctx.lr = 0x8324F86C;
	sub_8222CED0(ctx, base);
	// 8324F86C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F870: 386990D8  addi r3, r9, -0x6f28
	ctx.r[3].s64 = ctx.r[9].s64 + -28456;
	// 8324F874: 4BA5A6AD  bl 0x82ca9f20
	ctx.lr = 0x8324F878;
	sub_82CA9F20(ctx, base);
	// 8324F878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F888 size=64
    let mut pc: u32 = 0x8324F888;
    'dispatch: loop {
        match pc {
            0x8324F888 => {
    //   block [0x8324F888..0x8324F8C8)
	// 8324F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F894: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F898: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F89C: 388B092C  addi r4, r11, 0x92c
	ctx.r[4].s64 = ctx.r[11].s64 + 2348;
	// 8324F8A0: 386A7E4C  addi r3, r10, 0x7e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32332;
	// 8324F8A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F8A8: 4AFDD629  bl 0x8222ced0
	ctx.lr = 0x8324F8AC;
	sub_8222CED0(ctx, base);
	// 8324F8AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F8B0: 386990E8  addi r3, r9, -0x6f18
	ctx.r[3].s64 = ctx.r[9].s64 + -28440;
	// 8324F8B4: 4BA5A66D  bl 0x82ca9f20
	ctx.lr = 0x8324F8B8;
	sub_82CA9F20(ctx, base);
	// 8324F8B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F8C8 size=64
    let mut pc: u32 = 0x8324F8C8;
    'dispatch: loop {
        match pc {
            0x8324F8C8 => {
    //   block [0x8324F8C8..0x8324F908)
	// 8324F8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F8D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F8D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F8D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F8DC: 388B0944  addi r4, r11, 0x944
	ctx.r[4].s64 = ctx.r[11].s64 + 2372;
	// 8324F8E0: 386A7E50  addi r3, r10, 0x7e50
	ctx.r[3].s64 = ctx.r[10].s64 + 32336;
	// 8324F8E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F8E8: 4AFDD5E9  bl 0x8222ced0
	ctx.lr = 0x8324F8EC;
	sub_8222CED0(ctx, base);
	// 8324F8EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F8F0: 386990F8  addi r3, r9, -0x6f08
	ctx.r[3].s64 = ctx.r[9].s64 + -28424;
	// 8324F8F4: 4BA5A62D  bl 0x82ca9f20
	ctx.lr = 0x8324F8F8;
	sub_82CA9F20(ctx, base);
	// 8324F8F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F908 size=64
    let mut pc: u32 = 0x8324F908;
    'dispatch: loop {
        match pc {
            0x8324F908 => {
    //   block [0x8324F908..0x8324F948)
	// 8324F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F914: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F918: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F91C: 388B095C  addi r4, r11, 0x95c
	ctx.r[4].s64 = ctx.r[11].s64 + 2396;
	// 8324F920: 386A7E54  addi r3, r10, 0x7e54
	ctx.r[3].s64 = ctx.r[10].s64 + 32340;
	// 8324F924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F928: 4AFDD5A9  bl 0x8222ced0
	ctx.lr = 0x8324F92C;
	sub_8222CED0(ctx, base);
	// 8324F92C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F930: 38699108  addi r3, r9, -0x6ef8
	ctx.r[3].s64 = ctx.r[9].s64 + -28408;
	// 8324F934: 4BA5A5ED  bl 0x82ca9f20
	ctx.lr = 0x8324F938;
	sub_82CA9F20(ctx, base);
	// 8324F938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F948 size=64
    let mut pc: u32 = 0x8324F948;
    'dispatch: loop {
        match pc {
            0x8324F948 => {
    //   block [0x8324F948..0x8324F988)
	// 8324F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F954: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F958: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F95C: 388B096C  addi r4, r11, 0x96c
	ctx.r[4].s64 = ctx.r[11].s64 + 2412;
	// 8324F960: 386A7E58  addi r3, r10, 0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + 32344;
	// 8324F964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F968: 4AFDD569  bl 0x8222ced0
	ctx.lr = 0x8324F96C;
	sub_8222CED0(ctx, base);
	// 8324F96C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F970: 38699118  addi r3, r9, -0x6ee8
	ctx.r[3].s64 = ctx.r[9].s64 + -28392;
	// 8324F974: 4BA5A5AD  bl 0x82ca9f20
	ctx.lr = 0x8324F978;
	sub_82CA9F20(ctx, base);
	// 8324F978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F988 size=64
    let mut pc: u32 = 0x8324F988;
    'dispatch: loop {
        match pc {
            0x8324F988 => {
    //   block [0x8324F988..0x8324F9C8)
	// 8324F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F994: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F99C: 388B0988  addi r4, r11, 0x988
	ctx.r[4].s64 = ctx.r[11].s64 + 2440;
	// 8324F9A0: 386A7E5C  addi r3, r10, 0x7e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32348;
	// 8324F9A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F9A8: 4AFDD529  bl 0x8222ced0
	ctx.lr = 0x8324F9AC;
	sub_8222CED0(ctx, base);
	// 8324F9AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F9B0: 38699128  addi r3, r9, -0x6ed8
	ctx.r[3].s64 = ctx.r[9].s64 + -28376;
	// 8324F9B4: 4BA5A56D  bl 0x82ca9f20
	ctx.lr = 0x8324F9B8;
	sub_82CA9F20(ctx, base);
	// 8324F9B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324F9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324F9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324F9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324F9C8 size=64
    let mut pc: u32 = 0x8324F9C8;
    'dispatch: loop {
        match pc {
            0x8324F9C8 => {
    //   block [0x8324F9C8..0x8324FA08)
	// 8324F9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324F9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324F9D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324F9D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324F9D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324F9DC: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324F9E0: 386A7E60  addi r3, r10, 0x7e60
	ctx.r[3].s64 = ctx.r[10].s64 + 32352;
	// 8324F9E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324F9E8: 4AFDD4E9  bl 0x8222ced0
	ctx.lr = 0x8324F9EC;
	sub_8222CED0(ctx, base);
	// 8324F9EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324F9F0: 38699138  addi r3, r9, -0x6ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -28360;
	// 8324F9F4: 4BA5A52D  bl 0x82ca9f20
	ctx.lr = 0x8324F9F8;
	sub_82CA9F20(ctx, base);
	// 8324F9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324F9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA08 size=64
    let mut pc: u32 = 0x8324FA08;
    'dispatch: loop {
        match pc {
            0x8324FA08 => {
    //   block [0x8324FA08..0x8324FA48)
	// 8324FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA1C: 388B09A4  addi r4, r11, 0x9a4
	ctx.r[4].s64 = ctx.r[11].s64 + 2468;
	// 8324FA20: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 8324FA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FA28: 4AFDD4A9  bl 0x8222ced0
	ctx.lr = 0x8324FA2C;
	sub_8222CED0(ctx, base);
	// 8324FA2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FA30: 38699148  addi r3, r9, -0x6eb8
	ctx.r[3].s64 = ctx.r[9].s64 + -28344;
	// 8324FA34: 4BA5A4ED  bl 0x82ca9f20
	ctx.lr = 0x8324FA38;
	sub_82CA9F20(ctx, base);
	// 8324FA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA48 size=64
    let mut pc: u32 = 0x8324FA48;
    'dispatch: loop {
        match pc {
            0x8324FA48 => {
    //   block [0x8324FA48..0x8324FA88)
	// 8324FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA5C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324FA60: 386A7E68  addi r3, r10, 0x7e68
	ctx.r[3].s64 = ctx.r[10].s64 + 32360;
	// 8324FA64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FA68: 4AFDD469  bl 0x8222ced0
	ctx.lr = 0x8324FA6C;
	sub_8222CED0(ctx, base);
	// 8324FA6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FA70: 38699158  addi r3, r9, -0x6ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -28328;
	// 8324FA74: 4BA5A4AD  bl 0x82ca9f20
	ctx.lr = 0x8324FA78;
	sub_82CA9F20(ctx, base);
	// 8324FA78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FA88 size=64
    let mut pc: u32 = 0x8324FA88;
    'dispatch: loop {
        match pc {
            0x8324FA88 => {
    //   block [0x8324FA88..0x8324FAC8)
	// 8324FA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FA90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FA94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FA98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FA9C: 388B09C0  addi r4, r11, 0x9c0
	ctx.r[4].s64 = ctx.r[11].s64 + 2496;
	// 8324FAA0: 386A7E6C  addi r3, r10, 0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 32364;
	// 8324FAA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FAA8: 4AFDD429  bl 0x8222ced0
	ctx.lr = 0x8324FAAC;
	sub_8222CED0(ctx, base);
	// 8324FAAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FAB0: 38699168  addi r3, r9, -0x6e98
	ctx.r[3].s64 = ctx.r[9].s64 + -28312;
	// 8324FAB4: 4BA5A46D  bl 0x82ca9f20
	ctx.lr = 0x8324FAB8;
	sub_82CA9F20(ctx, base);
	// 8324FAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FAC8 size=64
    let mut pc: u32 = 0x8324FAC8;
    'dispatch: loop {
        match pc {
            0x8324FAC8 => {
    //   block [0x8324FAC8..0x8324FB08)
	// 8324FAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FAD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FAD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FAD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FADC: 388B09E4  addi r4, r11, 0x9e4
	ctx.r[4].s64 = ctx.r[11].s64 + 2532;
	// 8324FAE0: 386A7E70  addi r3, r10, 0x7e70
	ctx.r[3].s64 = ctx.r[10].s64 + 32368;
	// 8324FAE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FAE8: 4AFDD3E9  bl 0x8222ced0
	ctx.lr = 0x8324FAEC;
	sub_8222CED0(ctx, base);
	// 8324FAEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FAF0: 38699178  addi r3, r9, -0x6e88
	ctx.r[3].s64 = ctx.r[9].s64 + -28296;
	// 8324FAF4: 4BA5A42D  bl 0x82ca9f20
	ctx.lr = 0x8324FAF8;
	sub_82CA9F20(ctx, base);
	// 8324FAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB08 size=64
    let mut pc: u32 = 0x8324FB08;
    'dispatch: loop {
        match pc {
            0x8324FB08 => {
    //   block [0x8324FB08..0x8324FB48)
	// 8324FB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB1C: 388B09FC  addi r4, r11, 0x9fc
	ctx.r[4].s64 = ctx.r[11].s64 + 2556;
	// 8324FB20: 386A7E74  addi r3, r10, 0x7e74
	ctx.r[3].s64 = ctx.r[10].s64 + 32372;
	// 8324FB24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FB28: 4AFDD3A9  bl 0x8222ced0
	ctx.lr = 0x8324FB2C;
	sub_8222CED0(ctx, base);
	// 8324FB2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FB30: 38699188  addi r3, r9, -0x6e78
	ctx.r[3].s64 = ctx.r[9].s64 + -28280;
	// 8324FB34: 4BA5A3ED  bl 0x82ca9f20
	ctx.lr = 0x8324FB38;
	sub_82CA9F20(ctx, base);
	// 8324FB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB48 size=64
    let mut pc: u32 = 0x8324FB48;
    'dispatch: loop {
        match pc {
            0x8324FB48 => {
    //   block [0x8324FB48..0x8324FB88)
	// 8324FB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB5C: 388B0A18  addi r4, r11, 0xa18
	ctx.r[4].s64 = ctx.r[11].s64 + 2584;
	// 8324FB60: 386A7E78  addi r3, r10, 0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + 32376;
	// 8324FB64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FB68: 4AFDD369  bl 0x8222ced0
	ctx.lr = 0x8324FB6C;
	sub_8222CED0(ctx, base);
	// 8324FB6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FB70: 38699198  addi r3, r9, -0x6e68
	ctx.r[3].s64 = ctx.r[9].s64 + -28264;
	// 8324FB74: 4BA5A3AD  bl 0x82ca9f20
	ctx.lr = 0x8324FB78;
	sub_82CA9F20(ctx, base);
	// 8324FB78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FB88 size=64
    let mut pc: u32 = 0x8324FB88;
    'dispatch: loop {
        match pc {
            0x8324FB88 => {
    //   block [0x8324FB88..0x8324FBC8)
	// 8324FB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FB90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FB94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FB98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FB9C: 388B0A38  addi r4, r11, 0xa38
	ctx.r[4].s64 = ctx.r[11].s64 + 2616;
	// 8324FBA0: 386A7E7C  addi r3, r10, 0x7e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32380;
	// 8324FBA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FBA8: 4AFDD329  bl 0x8222ced0
	ctx.lr = 0x8324FBAC;
	sub_8222CED0(ctx, base);
	// 8324FBAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FBB0: 386991A8  addi r3, r9, -0x6e58
	ctx.r[3].s64 = ctx.r[9].s64 + -28248;
	// 8324FBB4: 4BA5A36D  bl 0x82ca9f20
	ctx.lr = 0x8324FBB8;
	sub_82CA9F20(ctx, base);
	// 8324FBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FBC8 size=64
    let mut pc: u32 = 0x8324FBC8;
    'dispatch: loop {
        match pc {
            0x8324FBC8 => {
    //   block [0x8324FBC8..0x8324FC08)
	// 8324FBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FBD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FBD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FBDC: 388B0A50  addi r4, r11, 0xa50
	ctx.r[4].s64 = ctx.r[11].s64 + 2640;
	// 8324FBE0: 386A7E80  addi r3, r10, 0x7e80
	ctx.r[3].s64 = ctx.r[10].s64 + 32384;
	// 8324FBE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FBE8: 4AFDD2E9  bl 0x8222ced0
	ctx.lr = 0x8324FBEC;
	sub_8222CED0(ctx, base);
	// 8324FBEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FBF0: 386991B8  addi r3, r9, -0x6e48
	ctx.r[3].s64 = ctx.r[9].s64 + -28232;
	// 8324FBF4: 4BA5A32D  bl 0x82ca9f20
	ctx.lr = 0x8324FBF8;
	sub_82CA9F20(ctx, base);
	// 8324FBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC08 size=64
    let mut pc: u32 = 0x8324FC08;
    'dispatch: loop {
        match pc {
            0x8324FC08 => {
    //   block [0x8324FC08..0x8324FC48)
	// 8324FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC1C: 388B0A6C  addi r4, r11, 0xa6c
	ctx.r[4].s64 = ctx.r[11].s64 + 2668;
	// 8324FC20: 386A7E84  addi r3, r10, 0x7e84
	ctx.r[3].s64 = ctx.r[10].s64 + 32388;
	// 8324FC24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FC28: 4AFDD2A9  bl 0x8222ced0
	ctx.lr = 0x8324FC2C;
	sub_8222CED0(ctx, base);
	// 8324FC2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FC30: 386991C8  addi r3, r9, -0x6e38
	ctx.r[3].s64 = ctx.r[9].s64 + -28216;
	// 8324FC34: 4BA5A2ED  bl 0x82ca9f20
	ctx.lr = 0x8324FC38;
	sub_82CA9F20(ctx, base);
	// 8324FC38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC48 size=64
    let mut pc: u32 = 0x8324FC48;
    'dispatch: loop {
        match pc {
            0x8324FC48 => {
    //   block [0x8324FC48..0x8324FC88)
	// 8324FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC5C: 388B0A88  addi r4, r11, 0xa88
	ctx.r[4].s64 = ctx.r[11].s64 + 2696;
	// 8324FC60: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 8324FC64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FC68: 4AFDD269  bl 0x8222ced0
	ctx.lr = 0x8324FC6C;
	sub_8222CED0(ctx, base);
	// 8324FC6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FC70: 386991D8  addi r3, r9, -0x6e28
	ctx.r[3].s64 = ctx.r[9].s64 + -28200;
	// 8324FC74: 4BA5A2AD  bl 0x82ca9f20
	ctx.lr = 0x8324FC78;
	sub_82CA9F20(ctx, base);
	// 8324FC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FC84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FC88 size=64
    let mut pc: u32 = 0x8324FC88;
    'dispatch: loop {
        match pc {
            0x8324FC88 => {
    //   block [0x8324FC88..0x8324FCC8)
	// 8324FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FC90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FC94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FC98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FC9C: 388B0AA8  addi r4, r11, 0xaa8
	ctx.r[4].s64 = ctx.r[11].s64 + 2728;
	// 8324FCA0: 386A7E8C  addi r3, r10, 0x7e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32396;
	// 8324FCA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FCA8: 4AFDD229  bl 0x8222ced0
	ctx.lr = 0x8324FCAC;
	sub_8222CED0(ctx, base);
	// 8324FCAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FCB0: 386991E8  addi r3, r9, -0x6e18
	ctx.r[3].s64 = ctx.r[9].s64 + -28184;
	// 8324FCB4: 4BA5A26D  bl 0x82ca9f20
	ctx.lr = 0x8324FCB8;
	sub_82CA9F20(ctx, base);
	// 8324FCB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FCC8 size=64
    let mut pc: u32 = 0x8324FCC8;
    'dispatch: loop {
        match pc {
            0x8324FCC8 => {
    //   block [0x8324FCC8..0x8324FD08)
	// 8324FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FCD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FCD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FCDC: 388B0AC4  addi r4, r11, 0xac4
	ctx.r[4].s64 = ctx.r[11].s64 + 2756;
	// 8324FCE0: 386A7E90  addi r3, r10, 0x7e90
	ctx.r[3].s64 = ctx.r[10].s64 + 32400;
	// 8324FCE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FCE8: 4AFDD1E9  bl 0x8222ced0
	ctx.lr = 0x8324FCEC;
	sub_8222CED0(ctx, base);
	// 8324FCEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FCF0: 386991F8  addi r3, r9, -0x6e08
	ctx.r[3].s64 = ctx.r[9].s64 + -28168;
	// 8324FCF4: 4BA5A22D  bl 0x82ca9f20
	ctx.lr = 0x8324FCF8;
	sub_82CA9F20(ctx, base);
	// 8324FCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD08 size=64
    let mut pc: u32 = 0x8324FD08;
    'dispatch: loop {
        match pc {
            0x8324FD08 => {
    //   block [0x8324FD08..0x8324FD48)
	// 8324FD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD1C: 388B0ADC  addi r4, r11, 0xadc
	ctx.r[4].s64 = ctx.r[11].s64 + 2780;
	// 8324FD20: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 8324FD24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FD28: 4AFDD1A9  bl 0x8222ced0
	ctx.lr = 0x8324FD2C;
	sub_8222CED0(ctx, base);
	// 8324FD2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FD30: 38699208  addi r3, r9, -0x6df8
	ctx.r[3].s64 = ctx.r[9].s64 + -28152;
	// 8324FD34: 4BA5A1ED  bl 0x82ca9f20
	ctx.lr = 0x8324FD38;
	sub_82CA9F20(ctx, base);
	// 8324FD38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD48 size=64
    let mut pc: u32 = 0x8324FD48;
    'dispatch: loop {
        match pc {
            0x8324FD48 => {
    //   block [0x8324FD48..0x8324FD88)
	// 8324FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD5C: 388B0AF4  addi r4, r11, 0xaf4
	ctx.r[4].s64 = ctx.r[11].s64 + 2804;
	// 8324FD60: 386A7E98  addi r3, r10, 0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + 32408;
	// 8324FD64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FD68: 4AFDD169  bl 0x8222ced0
	ctx.lr = 0x8324FD6C;
	sub_8222CED0(ctx, base);
	// 8324FD6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FD70: 38699218  addi r3, r9, -0x6de8
	ctx.r[3].s64 = ctx.r[9].s64 + -28136;
	// 8324FD74: 4BA5A1AD  bl 0x82ca9f20
	ctx.lr = 0x8324FD78;
	sub_82CA9F20(ctx, base);
	// 8324FD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FD88 size=64
    let mut pc: u32 = 0x8324FD88;
    'dispatch: loop {
        match pc {
            0x8324FD88 => {
    //   block [0x8324FD88..0x8324FDC8)
	// 8324FD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FD94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FD98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FD9C: 388B0B0C  addi r4, r11, 0xb0c
	ctx.r[4].s64 = ctx.r[11].s64 + 2828;
	// 8324FDA0: 386A7E9C  addi r3, r10, 0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 32412;
	// 8324FDA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FDA8: 4AFDD129  bl 0x8222ced0
	ctx.lr = 0x8324FDAC;
	sub_8222CED0(ctx, base);
	// 8324FDAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FDB0: 38699228  addi r3, r9, -0x6dd8
	ctx.r[3].s64 = ctx.r[9].s64 + -28120;
	// 8324FDB4: 4BA5A16D  bl 0x82ca9f20
	ctx.lr = 0x8324FDB8;
	sub_82CA9F20(ctx, base);
	// 8324FDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FDC8 size=64
    let mut pc: u32 = 0x8324FDC8;
    'dispatch: loop {
        match pc {
            0x8324FDC8 => {
    //   block [0x8324FDC8..0x8324FE08)
	// 8324FDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FDD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FDD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FDDC: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 8324FDE0: 386A7EA0  addi r3, r10, 0x7ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 32416;
	// 8324FDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FDE8: 4AFDD0E9  bl 0x8222ced0
	ctx.lr = 0x8324FDEC;
	sub_8222CED0(ctx, base);
	// 8324FDEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FDF0: 38699238  addi r3, r9, -0x6dc8
	ctx.r[3].s64 = ctx.r[9].s64 + -28104;
	// 8324FDF4: 4BA5A12D  bl 0x82ca9f20
	ctx.lr = 0x8324FDF8;
	sub_82CA9F20(ctx, base);
	// 8324FDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE08 size=64
    let mut pc: u32 = 0x8324FE08;
    'dispatch: loop {
        match pc {
            0x8324FE08 => {
    //   block [0x8324FE08..0x8324FE48)
	// 8324FE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE1C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 8324FE20: 386A7EA4  addi r3, r10, 0x7ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 32420;
	// 8324FE24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FE28: 4AFDD0A9  bl 0x8222ced0
	ctx.lr = 0x8324FE2C;
	sub_8222CED0(ctx, base);
	// 8324FE2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FE30: 38699248  addi r3, r9, -0x6db8
	ctx.r[3].s64 = ctx.r[9].s64 + -28088;
	// 8324FE34: 4BA5A0ED  bl 0x82ca9f20
	ctx.lr = 0x8324FE38;
	sub_82CA9F20(ctx, base);
	// 8324FE38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE48 size=64
    let mut pc: u32 = 0x8324FE48;
    'dispatch: loop {
        match pc {
            0x8324FE48 => {
    //   block [0x8324FE48..0x8324FE88)
	// 8324FE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE5C: 388B0B30  addi r4, r11, 0xb30
	ctx.r[4].s64 = ctx.r[11].s64 + 2864;
	// 8324FE60: 386A7EA8  addi r3, r10, 0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 32424;
	// 8324FE64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FE68: 4AFDD069  bl 0x8222ced0
	ctx.lr = 0x8324FE6C;
	sub_8222CED0(ctx, base);
	// 8324FE6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FE70: 38699258  addi r3, r9, -0x6da8
	ctx.r[3].s64 = ctx.r[9].s64 + -28072;
	// 8324FE74: 4BA5A0AD  bl 0x82ca9f20
	ctx.lr = 0x8324FE78;
	sub_82CA9F20(ctx, base);
	// 8324FE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FE88 size=64
    let mut pc: u32 = 0x8324FE88;
    'dispatch: loop {
        match pc {
            0x8324FE88 => {
    //   block [0x8324FE88..0x8324FEC8)
	// 8324FE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FE94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FE98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FE9C: 388B0B50  addi r4, r11, 0xb50
	ctx.r[4].s64 = ctx.r[11].s64 + 2896;
	// 8324FEA0: 386A7EAC  addi r3, r10, 0x7eac
	ctx.r[3].s64 = ctx.r[10].s64 + 32428;
	// 8324FEA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FEA8: 4AFDD029  bl 0x8222ced0
	ctx.lr = 0x8324FEAC;
	sub_8222CED0(ctx, base);
	// 8324FEAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FEB0: 38699268  addi r3, r9, -0x6d98
	ctx.r[3].s64 = ctx.r[9].s64 + -28056;
	// 8324FEB4: 4BA5A06D  bl 0x82ca9f20
	ctx.lr = 0x8324FEB8;
	sub_82CA9F20(ctx, base);
	// 8324FEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FEC8 size=64
    let mut pc: u32 = 0x8324FEC8;
    'dispatch: loop {
        match pc {
            0x8324FEC8 => {
    //   block [0x8324FEC8..0x8324FF08)
	// 8324FEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FED4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FED8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FEDC: 388B0B70  addi r4, r11, 0xb70
	ctx.r[4].s64 = ctx.r[11].s64 + 2928;
	// 8324FEE0: 386A7EB0  addi r3, r10, 0x7eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 32432;
	// 8324FEE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FEE8: 4AFDCFE9  bl 0x8222ced0
	ctx.lr = 0x8324FEEC;
	sub_8222CED0(ctx, base);
	// 8324FEEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FEF0: 38699278  addi r3, r9, -0x6d88
	ctx.r[3].s64 = ctx.r[9].s64 + -28040;
	// 8324FEF4: 4BA5A02D  bl 0x82ca9f20
	ctx.lr = 0x8324FEF8;
	sub_82CA9F20(ctx, base);
	// 8324FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF08 size=64
    let mut pc: u32 = 0x8324FF08;
    'dispatch: loop {
        match pc {
            0x8324FF08 => {
    //   block [0x8324FF08..0x8324FF48)
	// 8324FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF1C: 388B0B8C  addi r4, r11, 0xb8c
	ctx.r[4].s64 = ctx.r[11].s64 + 2956;
	// 8324FF20: 386A7EB4  addi r3, r10, 0x7eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32436;
	// 8324FF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FF28: 4AFDCFA9  bl 0x8222ced0
	ctx.lr = 0x8324FF2C;
	sub_8222CED0(ctx, base);
	// 8324FF2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FF30: 38699288  addi r3, r9, -0x6d78
	ctx.r[3].s64 = ctx.r[9].s64 + -28024;
	// 8324FF34: 4BA59FED  bl 0x82ca9f20
	ctx.lr = 0x8324FF38;
	sub_82CA9F20(ctx, base);
	// 8324FF38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF48 size=64
    let mut pc: u32 = 0x8324FF48;
    'dispatch: loop {
        match pc {
            0x8324FF48 => {
    //   block [0x8324FF48..0x8324FF88)
	// 8324FF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF5C: 388B0BA8  addi r4, r11, 0xba8
	ctx.r[4].s64 = ctx.r[11].s64 + 2984;
	// 8324FF60: 386A7EB8  addi r3, r10, 0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32440;
	// 8324FF64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FF68: 4AFDCF69  bl 0x8222ced0
	ctx.lr = 0x8324FF6C;
	sub_8222CED0(ctx, base);
	// 8324FF6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FF70: 38699298  addi r3, r9, -0x6d68
	ctx.r[3].s64 = ctx.r[9].s64 + -28008;
	// 8324FF74: 4BA59FAD  bl 0x82ca9f20
	ctx.lr = 0x8324FF78;
	sub_82CA9F20(ctx, base);
	// 8324FF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FF88 size=64
    let mut pc: u32 = 0x8324FF88;
    'dispatch: loop {
        match pc {
            0x8324FF88 => {
    //   block [0x8324FF88..0x8324FFC8)
	// 8324FF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FF94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FF98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FF9C: 388B0BC8  addi r4, r11, 0xbc8
	ctx.r[4].s64 = ctx.r[11].s64 + 3016;
	// 8324FFA0: 386A7EBC  addi r3, r10, 0x7ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 32444;
	// 8324FFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FFA8: 4AFDCF29  bl 0x8222ced0
	ctx.lr = 0x8324FFAC;
	sub_8222CED0(ctx, base);
	// 8324FFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FFB0: 386992A8  addi r3, r9, -0x6d58
	ctx.r[3].s64 = ctx.r[9].s64 + -27992;
	// 8324FFB4: 4BA59F6D  bl 0x82ca9f20
	ctx.lr = 0x8324FFB8;
	sub_82CA9F20(ctx, base);
	// 8324FFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324FFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324FFC8 size=64
    let mut pc: u32 = 0x8324FFC8;
    'dispatch: loop {
        match pc {
            0x8324FFC8 => {
    //   block [0x8324FFC8..0x83250008)
	// 8324FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324FFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324FFD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324FFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324FFDC: 388B0BDC  addi r4, r11, 0xbdc
	ctx.r[4].s64 = ctx.r[11].s64 + 3036;
	// 8324FFE0: 386A7EC0  addi r3, r10, 0x7ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 32448;
	// 8324FFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324FFE8: 4AFDCEE9  bl 0x8222ced0
	ctx.lr = 0x8324FFEC;
	sub_8222CED0(ctx, base);
	// 8324FFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324FFF0: 386992B8  addi r3, r9, -0x6d48
	ctx.r[3].s64 = ctx.r[9].s64 + -27976;
	// 8324FFF4: 4BA59F2D  bl 0x82ca9f20
	ctx.lr = 0x8324FFF8;
	sub_82CA9F20(ctx, base);
	// 8324FFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324FFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250008 size=64
    let mut pc: u32 = 0x83250008;
    'dispatch: loop {
        match pc {
            0x83250008 => {
    //   block [0x83250008..0x83250048)
	// 83250008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250014: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325001C: 388B0BF0  addi r4, r11, 0xbf0
	ctx.r[4].s64 = ctx.r[11].s64 + 3056;
	// 83250020: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 83250024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250028: 4AFDCEA9  bl 0x8222ced0
	ctx.lr = 0x8325002C;
	sub_8222CED0(ctx, base);
	// 8325002C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250030: 386992C8  addi r3, r9, -0x6d38
	ctx.r[3].s64 = ctx.r[9].s64 + -27960;
	// 83250034: 4BA59EED  bl 0x82ca9f20
	ctx.lr = 0x83250038;
	sub_82CA9F20(ctx, base);
	// 83250038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325003C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250048 size=64
    let mut pc: u32 = 0x83250048;
    'dispatch: loop {
        match pc {
            0x83250048 => {
    //   block [0x83250048..0x83250088)
	// 83250048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325004C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325005C: 388B0C10  addi r4, r11, 0xc10
	ctx.r[4].s64 = ctx.r[11].s64 + 3088;
	// 83250060: 386A7EC8  addi r3, r10, 0x7ec8
	ctx.r[3].s64 = ctx.r[10].s64 + 32456;
	// 83250064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250068: 4AFDCE69  bl 0x8222ced0
	ctx.lr = 0x8325006C;
	sub_8222CED0(ctx, base);
	// 8325006C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250070: 386992D8  addi r3, r9, -0x6d28
	ctx.r[3].s64 = ctx.r[9].s64 + -27944;
	// 83250074: 4BA59EAD  bl 0x82ca9f20
	ctx.lr = 0x83250078;
	sub_82CA9F20(ctx, base);
	// 83250078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325007C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250088 size=64
    let mut pc: u32 = 0x83250088;
    'dispatch: loop {
        match pc {
            0x83250088 => {
    //   block [0x83250088..0x832500C8)
	// 83250088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325008C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325009C: 388B0C28  addi r4, r11, 0xc28
	ctx.r[4].s64 = ctx.r[11].s64 + 3112;
	// 832500A0: 386A7ECC  addi r3, r10, 0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 32460;
	// 832500A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832500A8: 4AFDCE29  bl 0x8222ced0
	ctx.lr = 0x832500AC;
	sub_8222CED0(ctx, base);
	// 832500AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832500B0: 386992E8  addi r3, r9, -0x6d18
	ctx.r[3].s64 = ctx.r[9].s64 + -27928;
	// 832500B4: 4BA59E6D  bl 0x82ca9f20
	ctx.lr = 0x832500B8;
	sub_82CA9F20(ctx, base);
	// 832500B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832500BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832500C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832500C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832500C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832500C8 size=64
    let mut pc: u32 = 0x832500C8;
    'dispatch: loop {
        match pc {
            0x832500C8 => {
    //   block [0x832500C8..0x83250108)
	// 832500C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832500CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832500D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832500D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832500D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832500DC: 388B0C40  addi r4, r11, 0xc40
	ctx.r[4].s64 = ctx.r[11].s64 + 3136;
	// 832500E0: 386A7ED0  addi r3, r10, 0x7ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 32464;
	// 832500E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832500E8: 4AFDCDE9  bl 0x8222ced0
	ctx.lr = 0x832500EC;
	sub_8222CED0(ctx, base);
	// 832500EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832500F0: 386992F8  addi r3, r9, -0x6d08
	ctx.r[3].s64 = ctx.r[9].s64 + -27912;
	// 832500F4: 4BA59E2D  bl 0x82ca9f20
	ctx.lr = 0x832500F8;
	sub_82CA9F20(ctx, base);
	// 832500F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832500FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250108 size=64
    let mut pc: u32 = 0x83250108;
    'dispatch: loop {
        match pc {
            0x83250108 => {
    //   block [0x83250108..0x83250148)
	// 83250108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250118: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325011C: 388B0C5C  addi r4, r11, 0xc5c
	ctx.r[4].s64 = ctx.r[11].s64 + 3164;
	// 83250120: 386A7ED4  addi r3, r10, 0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 32468;
	// 83250124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250128: 4AFDCDA9  bl 0x8222ced0
	ctx.lr = 0x8325012C;
	sub_8222CED0(ctx, base);
	// 8325012C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250130: 38699308  addi r3, r9, -0x6cf8
	ctx.r[3].s64 = ctx.r[9].s64 + -27896;
	// 83250134: 4BA59DED  bl 0x82ca9f20
	ctx.lr = 0x83250138;
	sub_82CA9F20(ctx, base);
	// 83250138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325013C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250148 size=64
    let mut pc: u32 = 0x83250148;
    'dispatch: loop {
        match pc {
            0x83250148 => {
    //   block [0x83250148..0x83250188)
	// 83250148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325014C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250158: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325015C: 388B0C7C  addi r4, r11, 0xc7c
	ctx.r[4].s64 = ctx.r[11].s64 + 3196;
	// 83250160: 386A7ED8  addi r3, r10, 0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 32472;
	// 83250164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250168: 4AFDCD69  bl 0x8222ced0
	ctx.lr = 0x8325016C;
	sub_8222CED0(ctx, base);
	// 8325016C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250170: 38699318  addi r3, r9, -0x6ce8
	ctx.r[3].s64 = ctx.r[9].s64 + -27880;
	// 83250174: 4BA59DAD  bl 0x82ca9f20
	ctx.lr = 0x83250178;
	sub_82CA9F20(ctx, base);
	// 83250178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250188 size=64
    let mut pc: u32 = 0x83250188;
    'dispatch: loop {
        match pc {
            0x83250188 => {
    //   block [0x83250188..0x832501C8)
	// 83250188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250198: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325019C: 388B0C9C  addi r4, r11, 0xc9c
	ctx.r[4].s64 = ctx.r[11].s64 + 3228;
	// 832501A0: 386A7EDC  addi r3, r10, 0x7edc
	ctx.r[3].s64 = ctx.r[10].s64 + 32476;
	// 832501A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832501A8: 4AFDCD29  bl 0x8222ced0
	ctx.lr = 0x832501AC;
	sub_8222CED0(ctx, base);
	// 832501AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832501B0: 38699328  addi r3, r9, -0x6cd8
	ctx.r[3].s64 = ctx.r[9].s64 + -27864;
	// 832501B4: 4BA59D6D  bl 0x82ca9f20
	ctx.lr = 0x832501B8;
	sub_82CA9F20(ctx, base);
	// 832501B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832501BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832501C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832501C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832501C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832501C8 size=64
    let mut pc: u32 = 0x832501C8;
    'dispatch: loop {
        match pc {
            0x832501C8 => {
    //   block [0x832501C8..0x83250208)
	// 832501C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832501CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832501D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832501D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832501D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832501DC: 388B0CB8  addi r4, r11, 0xcb8
	ctx.r[4].s64 = ctx.r[11].s64 + 3256;
	// 832501E0: 386A7EE0  addi r3, r10, 0x7ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 32480;
	// 832501E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832501E8: 4AFDCCE9  bl 0x8222ced0
	ctx.lr = 0x832501EC;
	sub_8222CED0(ctx, base);
	// 832501EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832501F0: 38699338  addi r3, r9, -0x6cc8
	ctx.r[3].s64 = ctx.r[9].s64 + -27848;
	// 832501F4: 4BA59D2D  bl 0x82ca9f20
	ctx.lr = 0x832501F8;
	sub_82CA9F20(ctx, base);
	// 832501F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832501FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250208 size=64
    let mut pc: u32 = 0x83250208;
    'dispatch: loop {
        match pc {
            0x83250208 => {
    //   block [0x83250208..0x83250248)
	// 83250208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250218: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325021C: 388B0CD0  addi r4, r11, 0xcd0
	ctx.r[4].s64 = ctx.r[11].s64 + 3280;
	// 83250220: 386A7EE4  addi r3, r10, 0x7ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 32484;
	// 83250224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250228: 4AFDCCA9  bl 0x8222ced0
	ctx.lr = 0x8325022C;
	sub_8222CED0(ctx, base);
	// 8325022C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250230: 38699348  addi r3, r9, -0x6cb8
	ctx.r[3].s64 = ctx.r[9].s64 + -27832;
	// 83250234: 4BA59CED  bl 0x82ca9f20
	ctx.lr = 0x83250238;
	sub_82CA9F20(ctx, base);
	// 83250238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325023C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250248 size=64
    let mut pc: u32 = 0x83250248;
    'dispatch: loop {
        match pc {
            0x83250248 => {
    //   block [0x83250248..0x83250288)
	// 83250248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325024C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325025C: 388B0CE0  addi r4, r11, 0xce0
	ctx.r[4].s64 = ctx.r[11].s64 + 3296;
	// 83250260: 386A7EE8  addi r3, r10, 0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 32488;
	// 83250264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250268: 4AFDCC69  bl 0x8222ced0
	ctx.lr = 0x8325026C;
	sub_8222CED0(ctx, base);
	// 8325026C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250270: 38699358  addi r3, r9, -0x6ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -27816;
	// 83250274: 4BA59CAD  bl 0x82ca9f20
	ctx.lr = 0x83250278;
	sub_82CA9F20(ctx, base);
	// 83250278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325027C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250288 size=64
    let mut pc: u32 = 0x83250288;
    'dispatch: loop {
        match pc {
            0x83250288 => {
    //   block [0x83250288..0x832502C8)
	// 83250288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250294: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250298: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325029C: 388B0D00  addi r4, r11, 0xd00
	ctx.r[4].s64 = ctx.r[11].s64 + 3328;
	// 832502A0: 386A7EEC  addi r3, r10, 0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + 32492;
	// 832502A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832502A8: 4AFDCC29  bl 0x8222ced0
	ctx.lr = 0x832502AC;
	sub_8222CED0(ctx, base);
	// 832502AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832502B0: 38699368  addi r3, r9, -0x6c98
	ctx.r[3].s64 = ctx.r[9].s64 + -27800;
	// 832502B4: 4BA59C6D  bl 0x82ca9f20
	ctx.lr = 0x832502B8;
	sub_82CA9F20(ctx, base);
	// 832502B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832502BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832502C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832502C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832502C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832502C8 size=64
    let mut pc: u32 = 0x832502C8;
    'dispatch: loop {
        match pc {
            0x832502C8 => {
    //   block [0x832502C8..0x83250308)
	// 832502C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832502CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832502D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832502D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832502D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832502DC: 388B0D1C  addi r4, r11, 0xd1c
	ctx.r[4].s64 = ctx.r[11].s64 + 3356;
	// 832502E0: 386A7EF0  addi r3, r10, 0x7ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 32496;
	// 832502E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832502E8: 4AFDCBE9  bl 0x8222ced0
	ctx.lr = 0x832502EC;
	sub_8222CED0(ctx, base);
	// 832502EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832502F0: 38699378  addi r3, r9, -0x6c88
	ctx.r[3].s64 = ctx.r[9].s64 + -27784;
	// 832502F4: 4BA59C2D  bl 0x82ca9f20
	ctx.lr = 0x832502F8;
	sub_82CA9F20(ctx, base);
	// 832502F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832502FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250308 size=64
    let mut pc: u32 = 0x83250308;
    'dispatch: loop {
        match pc {
            0x83250308 => {
    //   block [0x83250308..0x83250348)
	// 83250308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250314: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250318: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325031C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 83250320: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 83250324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250328: 4AFDCBA9  bl 0x8222ced0
	ctx.lr = 0x8325032C;
	sub_8222CED0(ctx, base);
	// 8325032C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250330: 38699388  addi r3, r9, -0x6c78
	ctx.r[3].s64 = ctx.r[9].s64 + -27768;
	// 83250334: 4BA59BED  bl 0x82ca9f20
	ctx.lr = 0x83250338;
	sub_82CA9F20(ctx, base);
	// 83250338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250348 size=64
    let mut pc: u32 = 0x83250348;
    'dispatch: loop {
        match pc {
            0x83250348 => {
    //   block [0x83250348..0x83250388)
	// 83250348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250354: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250358: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325035C: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 83250360: 386A7EF8  addi r3, r10, 0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 32504;
	// 83250364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250368: 4AFDCB69  bl 0x8222ced0
	ctx.lr = 0x8325036C;
	sub_8222CED0(ctx, base);
	// 8325036C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250370: 38699398  addi r3, r9, -0x6c68
	ctx.r[3].s64 = ctx.r[9].s64 + -27752;
	// 83250374: 4BA59BAD  bl 0x82ca9f20
	ctx.lr = 0x83250378;
	sub_82CA9F20(ctx, base);
	// 83250378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325037C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250388 size=64
    let mut pc: u32 = 0x83250388;
    'dispatch: loop {
        match pc {
            0x83250388 => {
    //   block [0x83250388..0x832503C8)
	// 83250388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250394: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250398: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325039C: 388B08D0  addi r4, r11, 0x8d0
	ctx.r[4].s64 = ctx.r[11].s64 + 2256;
	// 832503A0: 386A7EFC  addi r3, r10, 0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + 32508;
	// 832503A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832503A8: 4AFDCB29  bl 0x8222ced0
	ctx.lr = 0x832503AC;
	sub_8222CED0(ctx, base);
	// 832503AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832503B0: 386993A8  addi r3, r9, -0x6c58
	ctx.r[3].s64 = ctx.r[9].s64 + -27736;
	// 832503B4: 4BA59B6D  bl 0x82ca9f20
	ctx.lr = 0x832503B8;
	sub_82CA9F20(ctx, base);
	// 832503B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832503BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832503C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832503C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832503C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832503C8 size=64
    let mut pc: u32 = 0x832503C8;
    'dispatch: loop {
        match pc {
            0x832503C8 => {
    //   block [0x832503C8..0x83250408)
	// 832503C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832503CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832503D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832503D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832503D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832503DC: 388B08E8  addi r4, r11, 0x8e8
	ctx.r[4].s64 = ctx.r[11].s64 + 2280;
	// 832503E0: 386A7F00  addi r3, r10, 0x7f00
	ctx.r[3].s64 = ctx.r[10].s64 + 32512;
	// 832503E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832503E8: 4AFDCAE9  bl 0x8222ced0
	ctx.lr = 0x832503EC;
	sub_8222CED0(ctx, base);
	// 832503EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832503F0: 386993B8  addi r3, r9, -0x6c48
	ctx.r[3].s64 = ctx.r[9].s64 + -27720;
	// 832503F4: 4BA59B2D  bl 0x82ca9f20
	ctx.lr = 0x832503F8;
	sub_82CA9F20(ctx, base);
	// 832503F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832503FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250408 size=64
    let mut pc: u32 = 0x83250408;
    'dispatch: loop {
        match pc {
            0x83250408 => {
    //   block [0x83250408..0x83250448)
	// 83250408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325040C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325041C: 388B0764  addi r4, r11, 0x764
	ctx.r[4].s64 = ctx.r[11].s64 + 1892;
	// 83250420: 386A7F04  addi r3, r10, 0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + 32516;
	// 83250424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250428: 4AFDCAA9  bl 0x8222ced0
	ctx.lr = 0x8325042C;
	sub_8222CED0(ctx, base);
	// 8325042C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250430: 386993C8  addi r3, r9, -0x6c38
	ctx.r[3].s64 = ctx.r[9].s64 + -27704;
	// 83250434: 4BA59AED  bl 0x82ca9f20
	ctx.lr = 0x83250438;
	sub_82CA9F20(ctx, base);
	// 83250438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325043C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250448 size=64
    let mut pc: u32 = 0x83250448;
    'dispatch: loop {
        match pc {
            0x83250448 => {
    //   block [0x83250448..0x83250488)
	// 83250448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325044C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250454: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325045C: 388B0774  addi r4, r11, 0x774
	ctx.r[4].s64 = ctx.r[11].s64 + 1908;
	// 83250460: 386A7F08  addi r3, r10, 0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + 32520;
	// 83250464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250468: 4AFDCA69  bl 0x8222ced0
	ctx.lr = 0x8325046C;
	sub_8222CED0(ctx, base);
	// 8325046C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250470: 386993D8  addi r3, r9, -0x6c28
	ctx.r[3].s64 = ctx.r[9].s64 + -27688;
	// 83250474: 4BA59AAD  bl 0x82ca9f20
	ctx.lr = 0x83250478;
	sub_82CA9F20(ctx, base);
	// 83250478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250488 size=64
    let mut pc: u32 = 0x83250488;
    'dispatch: loop {
        match pc {
            0x83250488 => {
    //   block [0x83250488..0x832504C8)
	// 83250488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250494: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325049C: 388B0D38  addi r4, r11, 0xd38
	ctx.r[4].s64 = ctx.r[11].s64 + 3384;
	// 832504A0: 386A7F0C  addi r3, r10, 0x7f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32524;
	// 832504A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832504A8: 4AFDCA29  bl 0x8222ced0
	ctx.lr = 0x832504AC;
	sub_8222CED0(ctx, base);
	// 832504AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832504B0: 386993E8  addi r3, r9, -0x6c18
	ctx.r[3].s64 = ctx.r[9].s64 + -27672;
	// 832504B4: 4BA59A6D  bl 0x82ca9f20
	ctx.lr = 0x832504B8;
	sub_82CA9F20(ctx, base);
	// 832504B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832504BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832504C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832504C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832504C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832504C8 size=64
    let mut pc: u32 = 0x832504C8;
    'dispatch: loop {
        match pc {
            0x832504C8 => {
    //   block [0x832504C8..0x83250508)
	// 832504C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832504CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832504D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832504D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832504D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832504DC: 388B0D38  addi r4, r11, 0xd38
	ctx.r[4].s64 = ctx.r[11].s64 + 3384;
	// 832504E0: 386A7F10  addi r3, r10, 0x7f10
	ctx.r[3].s64 = ctx.r[10].s64 + 32528;
	// 832504E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832504E8: 4AFDC9E9  bl 0x8222ced0
	ctx.lr = 0x832504EC;
	sub_8222CED0(ctx, base);
	// 832504EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832504F0: 386993F8  addi r3, r9, -0x6c08
	ctx.r[3].s64 = ctx.r[9].s64 + -27656;
	// 832504F4: 4BA59A2D  bl 0x82ca9f20
	ctx.lr = 0x832504F8;
	sub_82CA9F20(ctx, base);
	// 832504F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832504FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250508 size=64
    let mut pc: u32 = 0x83250508;
    'dispatch: loop {
        match pc {
            0x83250508 => {
    //   block [0x83250508..0x83250548)
	// 83250508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250514: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325051C: 388B0D48  addi r4, r11, 0xd48
	ctx.r[4].s64 = ctx.r[11].s64 + 3400;
	// 83250520: 386A7F14  addi r3, r10, 0x7f14
	ctx.r[3].s64 = ctx.r[10].s64 + 32532;
	// 83250524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250528: 4AFDC9A9  bl 0x8222ced0
	ctx.lr = 0x8325052C;
	sub_8222CED0(ctx, base);
	// 8325052C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250530: 38699408  addi r3, r9, -0x6bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -27640;
	// 83250534: 4BA599ED  bl 0x82ca9f20
	ctx.lr = 0x83250538;
	sub_82CA9F20(ctx, base);
	// 83250538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325053C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250548 size=64
    let mut pc: u32 = 0x83250548;
    'dispatch: loop {
        match pc {
            0x83250548 => {
    //   block [0x83250548..0x83250588)
	// 83250548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325054C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250558: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325055C: 388B0D5C  addi r4, r11, 0xd5c
	ctx.r[4].s64 = ctx.r[11].s64 + 3420;
	// 83250560: 386A7F18  addi r3, r10, 0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + 32536;
	// 83250564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250568: 4AFDC969  bl 0x8222ced0
	ctx.lr = 0x8325056C;
	sub_8222CED0(ctx, base);
	// 8325056C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250570: 38699418  addi r3, r9, -0x6be8
	ctx.r[3].s64 = ctx.r[9].s64 + -27624;
	// 83250574: 4BA599AD  bl 0x82ca9f20
	ctx.lr = 0x83250578;
	sub_82CA9F20(ctx, base);
	// 83250578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325057C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250588 size=64
    let mut pc: u32 = 0x83250588;
    'dispatch: loop {
        match pc {
            0x83250588 => {
    //   block [0x83250588..0x832505C8)
	// 83250588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250594: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250598: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325059C: 388B0D6C  addi r4, r11, 0xd6c
	ctx.r[4].s64 = ctx.r[11].s64 + 3436;
	// 832505A0: 386A7F1C  addi r3, r10, 0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32540;
	// 832505A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832505A8: 4AFDC929  bl 0x8222ced0
	ctx.lr = 0x832505AC;
	sub_8222CED0(ctx, base);
	// 832505AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832505B0: 38699428  addi r3, r9, -0x6bd8
	ctx.r[3].s64 = ctx.r[9].s64 + -27608;
	// 832505B4: 4BA5996D  bl 0x82ca9f20
	ctx.lr = 0x832505B8;
	sub_82CA9F20(ctx, base);
	// 832505B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832505BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832505C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832505C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832505C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832505C8 size=64
    let mut pc: u32 = 0x832505C8;
    'dispatch: loop {
        match pc {
            0x832505C8 => {
    //   block [0x832505C8..0x83250608)
	// 832505C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832505CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832505D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832505D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832505D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832505DC: 388B0D80  addi r4, r11, 0xd80
	ctx.r[4].s64 = ctx.r[11].s64 + 3456;
	// 832505E0: 386A7F20  addi r3, r10, 0x7f20
	ctx.r[3].s64 = ctx.r[10].s64 + 32544;
	// 832505E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832505E8: 4AFDC8E9  bl 0x8222ced0
	ctx.lr = 0x832505EC;
	sub_8222CED0(ctx, base);
	// 832505EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832505F0: 38699438  addi r3, r9, -0x6bc8
	ctx.r[3].s64 = ctx.r[9].s64 + -27592;
	// 832505F4: 4BA5992D  bl 0x82ca9f20
	ctx.lr = 0x832505F8;
	sub_82CA9F20(ctx, base);
	// 832505F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832505FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250608 size=64
    let mut pc: u32 = 0x83250608;
    'dispatch: loop {
        match pc {
            0x83250608 => {
    //   block [0x83250608..0x83250648)
	// 83250608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250614: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250618: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325061C: 388B0D94  addi r4, r11, 0xd94
	ctx.r[4].s64 = ctx.r[11].s64 + 3476;
	// 83250620: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 83250624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250628: 4AFDC8A9  bl 0x8222ced0
	ctx.lr = 0x8325062C;
	sub_8222CED0(ctx, base);
	// 8325062C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250630: 38699448  addi r3, r9, -0x6bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -27576;
	// 83250634: 4BA598ED  bl 0x82ca9f20
	ctx.lr = 0x83250638;
	sub_82CA9F20(ctx, base);
	// 83250638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250648 size=64
    let mut pc: u32 = 0x83250648;
    'dispatch: loop {
        match pc {
            0x83250648 => {
    //   block [0x83250648..0x83250688)
	// 83250648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250654: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250658: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325065C: 388B0DA8  addi r4, r11, 0xda8
	ctx.r[4].s64 = ctx.r[11].s64 + 3496;
	// 83250660: 386A7F28  addi r3, r10, 0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + 32552;
	// 83250664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250668: 4AFDC869  bl 0x8222ced0
	ctx.lr = 0x8325066C;
	sub_8222CED0(ctx, base);
	// 8325066C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250670: 38699458  addi r3, r9, -0x6ba8
	ctx.r[3].s64 = ctx.r[9].s64 + -27560;
	// 83250674: 4BA598AD  bl 0x82ca9f20
	ctx.lr = 0x83250678;
	sub_82CA9F20(ctx, base);
	// 83250678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325067C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250688 size=64
    let mut pc: u32 = 0x83250688;
    'dispatch: loop {
        match pc {
            0x83250688 => {
    //   block [0x83250688..0x832506C8)
	// 83250688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325068C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325069C: 388B0DC0  addi r4, r11, 0xdc0
	ctx.r[4].s64 = ctx.r[11].s64 + 3520;
	// 832506A0: 386A7F2C  addi r3, r10, 0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32556;
	// 832506A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832506A8: 4AFDC829  bl 0x8222ced0
	ctx.lr = 0x832506AC;
	sub_8222CED0(ctx, base);
	// 832506AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832506B0: 38699468  addi r3, r9, -0x6b98
	ctx.r[3].s64 = ctx.r[9].s64 + -27544;
	// 832506B4: 4BA5986D  bl 0x82ca9f20
	ctx.lr = 0x832506B8;
	sub_82CA9F20(ctx, base);
	// 832506B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832506BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832506C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832506C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832506C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832506C8 size=64
    let mut pc: u32 = 0x832506C8;
    'dispatch: loop {
        match pc {
            0x832506C8 => {
    //   block [0x832506C8..0x83250708)
	// 832506C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832506CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832506D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832506D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832506D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832506DC: 388B0DDC  addi r4, r11, 0xddc
	ctx.r[4].s64 = ctx.r[11].s64 + 3548;
	// 832506E0: 386A7F30  addi r3, r10, 0x7f30
	ctx.r[3].s64 = ctx.r[10].s64 + 32560;
	// 832506E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832506E8: 4AFDC7E9  bl 0x8222ced0
	ctx.lr = 0x832506EC;
	sub_8222CED0(ctx, base);
	// 832506EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832506F0: 38699478  addi r3, r9, -0x6b88
	ctx.r[3].s64 = ctx.r[9].s64 + -27528;
	// 832506F4: 4BA5982D  bl 0x82ca9f20
	ctx.lr = 0x832506F8;
	sub_82CA9F20(ctx, base);
	// 832506F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832506FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250708 size=64
    let mut pc: u32 = 0x83250708;
    'dispatch: loop {
        match pc {
            0x83250708 => {
    //   block [0x83250708..0x83250748)
	// 83250708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325071C: 388B0DF8  addi r4, r11, 0xdf8
	ctx.r[4].s64 = ctx.r[11].s64 + 3576;
	// 83250720: 386A7F34  addi r3, r10, 0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + 32564;
	// 83250724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250728: 4AFDC7A9  bl 0x8222ced0
	ctx.lr = 0x8325072C;
	sub_8222CED0(ctx, base);
	// 8325072C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250730: 38699488  addi r3, r9, -0x6b78
	ctx.r[3].s64 = ctx.r[9].s64 + -27512;
	// 83250734: 4BA597ED  bl 0x82ca9f20
	ctx.lr = 0x83250738;
	sub_82CA9F20(ctx, base);
	// 83250738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325073C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250748 size=64
    let mut pc: u32 = 0x83250748;
    'dispatch: loop {
        match pc {
            0x83250748 => {
    //   block [0x83250748..0x83250788)
	// 83250748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250754: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250758: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325075C: 388B0E14  addi r4, r11, 0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + 3604;
	// 83250760: 386A7F38  addi r3, r10, 0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + 32568;
	// 83250764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250768: 4AFDC769  bl 0x8222ced0
	ctx.lr = 0x8325076C;
	sub_8222CED0(ctx, base);
	// 8325076C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250770: 38699498  addi r3, r9, -0x6b68
	ctx.r[3].s64 = ctx.r[9].s64 + -27496;
	// 83250774: 4BA597AD  bl 0x82ca9f20
	ctx.lr = 0x83250778;
	sub_82CA9F20(ctx, base);
	// 83250778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325077C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250788 size=64
    let mut pc: u32 = 0x83250788;
    'dispatch: loop {
        match pc {
            0x83250788 => {
    //   block [0x83250788..0x832507C8)
	// 83250788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250794: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250798: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325079C: 388BE190  addi r4, r11, -0x1e70
	ctx.r[4].s64 = ctx.r[11].s64 + -7792;
	// 832507A0: 386A7F3C  addi r3, r10, 0x7f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32572;
	// 832507A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832507A8: 4AFDC729  bl 0x8222ced0
	ctx.lr = 0x832507AC;
	sub_8222CED0(ctx, base);
	// 832507AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832507B0: 386994A8  addi r3, r9, -0x6b58
	ctx.r[3].s64 = ctx.r[9].s64 + -27480;
	// 832507B4: 4BA5976D  bl 0x82ca9f20
	ctx.lr = 0x832507B8;
	sub_82CA9F20(ctx, base);
	// 832507B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832507BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832507C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832507C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832507C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832507C8 size=64
    let mut pc: u32 = 0x832507C8;
    'dispatch: loop {
        match pc {
            0x832507C8 => {
    //   block [0x832507C8..0x83250808)
	// 832507C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832507CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832507D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832507D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832507D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832507DC: 388B0E28  addi r4, r11, 0xe28
	ctx.r[4].s64 = ctx.r[11].s64 + 3624;
	// 832507E0: 386A7F40  addi r3, r10, 0x7f40
	ctx.r[3].s64 = ctx.r[10].s64 + 32576;
	// 832507E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832507E8: 4AFDC6E9  bl 0x8222ced0
	ctx.lr = 0x832507EC;
	sub_8222CED0(ctx, base);
	// 832507EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832507F0: 386994B8  addi r3, r9, -0x6b48
	ctx.r[3].s64 = ctx.r[9].s64 + -27464;
	// 832507F4: 4BA5972D  bl 0x82ca9f20
	ctx.lr = 0x832507F8;
	sub_82CA9F20(ctx, base);
	// 832507F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832507FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250808 size=64
    let mut pc: u32 = 0x83250808;
    'dispatch: loop {
        match pc {
            0x83250808 => {
    //   block [0x83250808..0x83250848)
	// 83250808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250814: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250818: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325081C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83250820: 386A7F44  addi r3, r10, 0x7f44
	ctx.r[3].s64 = ctx.r[10].s64 + 32580;
	// 83250824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250828: 4AFDC6A9  bl 0x8222ced0
	ctx.lr = 0x8325082C;
	sub_8222CED0(ctx, base);
	// 8325082C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250830: 386994C8  addi r3, r9, -0x6b38
	ctx.r[3].s64 = ctx.r[9].s64 + -27448;
	// 83250834: 4BA596ED  bl 0x82ca9f20
	ctx.lr = 0x83250838;
	sub_82CA9F20(ctx, base);
	// 83250838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325083C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250848 size=64
    let mut pc: u32 = 0x83250848;
    'dispatch: loop {
        match pc {
            0x83250848 => {
    //   block [0x83250848..0x83250888)
	// 83250848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250854: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250858: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325085C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250860: 386A7F48  addi r3, r10, 0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + 32584;
	// 83250864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250868: 4AFDC669  bl 0x8222ced0
	ctx.lr = 0x8325086C;
	sub_8222CED0(ctx, base);
	// 8325086C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250870: 386994F8  addi r3, r9, -0x6b08
	ctx.r[3].s64 = ctx.r[9].s64 + -27400;
	// 83250874: 4BA596AD  bl 0x82ca9f20
	ctx.lr = 0x83250878;
	sub_82CA9F20(ctx, base);
	// 83250878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325087C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250888 size=64
    let mut pc: u32 = 0x83250888;
    'dispatch: loop {
        match pc {
            0x83250888 => {
    //   block [0x83250888..0x832508C8)
	// 83250888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250894: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250898: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325089C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832508A0: 386A7F4C  addi r3, r10, 0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32588;
	// 832508A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832508A8: 4AFDC629  bl 0x8222ced0
	ctx.lr = 0x832508AC;
	sub_8222CED0(ctx, base);
	// 832508AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832508B0: 38699508  addi r3, r9, -0x6af8
	ctx.r[3].s64 = ctx.r[9].s64 + -27384;
	// 832508B4: 4BA5966D  bl 0x82ca9f20
	ctx.lr = 0x832508B8;
	sub_82CA9F20(ctx, base);
	// 832508B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832508BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832508C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832508C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832508C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832508C8 size=64
    let mut pc: u32 = 0x832508C8;
    'dispatch: loop {
        match pc {
            0x832508C8 => {
    //   block [0x832508C8..0x83250908)
	// 832508C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832508CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832508D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832508D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832508D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832508DC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832508E0: 386A7F50  addi r3, r10, 0x7f50
	ctx.r[3].s64 = ctx.r[10].s64 + 32592;
	// 832508E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832508E8: 4AFDC5E9  bl 0x8222ced0
	ctx.lr = 0x832508EC;
	sub_8222CED0(ctx, base);
	// 832508EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832508F0: 38699518  addi r3, r9, -0x6ae8
	ctx.r[3].s64 = ctx.r[9].s64 + -27368;
	// 832508F4: 4BA5962D  bl 0x82ca9f20
	ctx.lr = 0x832508F8;
	sub_82CA9F20(ctx, base);
	// 832508F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832508FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250908 size=64
    let mut pc: u32 = 0x83250908;
    'dispatch: loop {
        match pc {
            0x83250908 => {
    //   block [0x83250908..0x83250948)
	// 83250908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250918: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325091C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250920: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 83250924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250928: 4AFDC5A9  bl 0x8222ced0
	ctx.lr = 0x8325092C;
	sub_8222CED0(ctx, base);
	// 8325092C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250930: 38699528  addi r3, r9, -0x6ad8
	ctx.r[3].s64 = ctx.r[9].s64 + -27352;
	// 83250934: 4BA595ED  bl 0x82ca9f20
	ctx.lr = 0x83250938;
	sub_82CA9F20(ctx, base);
	// 83250938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325093C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250948 size=64
    let mut pc: u32 = 0x83250948;
    'dispatch: loop {
        match pc {
            0x83250948 => {
    //   block [0x83250948..0x83250988)
	// 83250948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325094C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250958: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325095C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83250960: 386A7F58  addi r3, r10, 0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + 32600;
	// 83250964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250968: 4AFDC569  bl 0x8222ced0
	ctx.lr = 0x8325096C;
	sub_8222CED0(ctx, base);
	// 8325096C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250970: 38699538  addi r3, r9, -0x6ac8
	ctx.r[3].s64 = ctx.r[9].s64 + -27336;
	// 83250974: 4BA595AD  bl 0x82ca9f20
	ctx.lr = 0x83250978;
	sub_82CA9F20(ctx, base);
	// 83250978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325097C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250988 size=64
    let mut pc: u32 = 0x83250988;
    'dispatch: loop {
        match pc {
            0x83250988 => {
    //   block [0x83250988..0x832509C8)
	// 83250988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250998: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325099C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832509A0: 386A7F5C  addi r3, r10, 0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32604;
	// 832509A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832509A8: 4AFDC529  bl 0x8222ced0
	ctx.lr = 0x832509AC;
	sub_8222CED0(ctx, base);
	// 832509AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832509B0: 38699548  addi r3, r9, -0x6ab8
	ctx.r[3].s64 = ctx.r[9].s64 + -27320;
	// 832509B4: 4BA5956D  bl 0x82ca9f20
	ctx.lr = 0x832509B8;
	sub_82CA9F20(ctx, base);
	// 832509B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832509BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832509C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832509C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832509C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832509C8 size=52
    let mut pc: u32 = 0x832509C8;
    'dispatch: loop {
        match pc {
            0x832509C8 => {
    //   block [0x832509C8..0x832509FC)
	// 832509C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832509CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832509D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832509D4: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832509D8: C82B1A08  lfd f1, 0x1a08(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(6664 as u32) ) };
	// 832509DC: 4AFE94D5  bl 0x82239eb0
	ctx.lr = 0x832509E0;
	sub_82239EB0(ctx, base);
	// 832509E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832509E4: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 832509E8: D00A7F60  stfs f0, 0x7f60(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32608 as u32), tmp.u32 ) };
	// 832509EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832509F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832509F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832509F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250A00 size=64
    let mut pc: u32 = 0x83250A00;
    'dispatch: loop {
        match pc {
            0x83250A00 => {
    //   block [0x83250A00..0x83250A40)
	// 83250A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250A0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250A10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250A14: 388B2090  addi r4, r11, 0x2090
	ctx.r[4].s64 = ctx.r[11].s64 + 8336;
	// 83250A18: 386A7F64  addi r3, r10, 0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + 32612;
	// 83250A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250A20: 4AFDC4B1  bl 0x8222ced0
	ctx.lr = 0x83250A24;
	sub_8222CED0(ctx, base);
	// 83250A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250A28: 38699558  addi r3, r9, -0x6aa8
	ctx.r[3].s64 = ctx.r[9].s64 + -27304;
	// 83250A2C: 4BA594F5  bl 0x82ca9f20
	ctx.lr = 0x83250A30;
	sub_82CA9F20(ctx, base);
	// 83250A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250A40 size=144
    let mut pc: u32 = 0x83250A40;
    'dispatch: loop {
        match pc {
            0x83250A40 => {
    //   block [0x83250A40..0x83250AD0)
	// 83250A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250A4C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83250A50: 4AFCE809  bl 0x8221f258
	ctx.lr = 0x83250A54;
	sub_8221F258(ctx, base);
	// 83250A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83250A58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83250A5C: 419A0008  beq cr6, 0x83250a64
	if ctx.cr[6].eq {
	pc = 0x83250A64; continue 'dispatch;
	}
	// 83250A60: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83250A64: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83250A68: 41820008  beq 0x83250a70
	if ctx.cr[0].eq {
	pc = 0x83250A70; continue 'dispatch;
	}
	// 83250A6C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83250A70: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83250A74: 41820008  beq 0x83250a7c
	if ctx.cr[0].eq {
	pc = 0x83250A7C; continue 'dispatch;
	}
	// 83250A78: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83250A7C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83250A80: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83250A84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83250A88: 39097F68  addi r8, r9, 0x7f68
	ctx.r[8].s64 = ctx.r[9].s64 + 32616;
	// 83250A8C: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83250A90: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83250A94: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83250A98: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83250A9C: 38679568  addi r3, r7, -0x6a98
	ctx.r[3].s64 = ctx.r[7].s64 + -27288;
	// 83250AA0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AA4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83250AA8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AAC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83250AB0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83250AB4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83250AB8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83250ABC: 4BA59465  bl 0x82ca9f20
	ctx.lr = 0x83250AC0;
	sub_82CA9F20(ctx, base);
	// 83250AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83250AD0 size=96
    let mut pc: u32 = 0x83250AD0;
    'dispatch: loop {
        match pc {
            0x83250AD0 => {
    //   block [0x83250AD0..0x83250B30)
	// 83250AD0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250AD4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83250AD8: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 83250ADC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83250AE0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83250AE4: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83250AE8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83250AEC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83250AF0: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250B30 size=64
    let mut pc: u32 = 0x83250B30;
    'dispatch: loop {
        match pc {
            0x83250B30 => {
    //   block [0x83250B30..0x83250B70)
	// 83250B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250B3C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250B40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250B44: 388B239C  addi r4, r11, 0x239c
	ctx.r[4].s64 = ctx.r[11].s64 + 9116;
	// 83250B48: 386A7F74  addi r3, r10, 0x7f74
	ctx.r[3].s64 = ctx.r[10].s64 + 32628;
	// 83250B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250B50: 4AFDC381  bl 0x8222ced0
	ctx.lr = 0x83250B54;
	sub_8222CED0(ctx, base);
	// 83250B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250B58: 38699578  addi r3, r9, -0x6a88
	ctx.r[3].s64 = ctx.r[9].s64 + -27272;
	// 83250B5C: 4BA593C5  bl 0x82ca9f20
	ctx.lr = 0x83250B60;
	sub_82CA9F20(ctx, base);
	// 83250B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250B70 size=64
    let mut pc: u32 = 0x83250B70;
    'dispatch: loop {
        match pc {
            0x83250B70 => {
    //   block [0x83250B70..0x83250BB0)
	// 83250B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250B7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250B80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250B84: 388B23B4  addi r4, r11, 0x23b4
	ctx.r[4].s64 = ctx.r[11].s64 + 9140;
	// 83250B88: 386A7F78  addi r3, r10, 0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + 32632;
	// 83250B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250B90: 4AFDC341  bl 0x8222ced0
	ctx.lr = 0x83250B94;
	sub_8222CED0(ctx, base);
	// 83250B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250B98: 38699588  addi r3, r9, -0x6a78
	ctx.r[3].s64 = ctx.r[9].s64 + -27256;
	// 83250B9C: 4BA59385  bl 0x82ca9f20
	ctx.lr = 0x83250BA0;
	sub_82CA9F20(ctx, base);
	// 83250BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83250BB0 size=12
    let mut pc: u32 = 0x83250BB0;
    'dispatch: loop {
        match pc {
            0x83250BB0 => {
    //   block [0x83250BB0..0x83250BBC)
	// 83250BB0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83250BB4: 386B9598  addi r3, r11, -0x6a68
	ctx.r[3].s64 = ctx.r[11].s64 + -27240;
	// 83250BB8: 4BA59368  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83250BC0 size=12
    let mut pc: u32 = 0x83250BC0;
    'dispatch: loop {
        match pc {
            0x83250BC0 => {
    //   block [0x83250BC0..0x83250BCC)
	// 83250BC0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83250BC4: 386B95A8  addi r3, r11, -0x6a58
	ctx.r[3].s64 = ctx.r[11].s64 + -27224;
	// 83250BC8: 4BA59358  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250BD0 size=64
    let mut pc: u32 = 0x83250BD0;
    'dispatch: loop {
        match pc {
            0x83250BD0 => {
    //   block [0x83250BD0..0x83250C10)
	// 83250BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250BDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250BE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250BE4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83250BE8: 386A7FB0  addi r3, r10, 0x7fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 32688;
	// 83250BEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250BF0: 4AFDC2E1  bl 0x8222ced0
	ctx.lr = 0x83250BF4;
	sub_8222CED0(ctx, base);
	// 83250BF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250BF8: 386995B8  addi r3, r9, -0x6a48
	ctx.r[3].s64 = ctx.r[9].s64 + -27208;
	// 83250BFC: 4BA59325  bl 0x82ca9f20
	ctx.lr = 0x83250C00;
	sub_82CA9F20(ctx, base);
	// 83250C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C10 size=64
    let mut pc: u32 = 0x83250C10;
    'dispatch: loop {
        match pc {
            0x83250C10 => {
    //   block [0x83250C10..0x83250C50)
	// 83250C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250C20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250C24: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83250C28: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 83250C2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250C30: 4AFDC2A1  bl 0x8222ced0
	ctx.lr = 0x83250C34;
	sub_8222CED0(ctx, base);
	// 83250C34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250C38: 386995C8  addi r3, r9, -0x6a38
	ctx.r[3].s64 = ctx.r[9].s64 + -27192;
	// 83250C3C: 4BA592E5  bl 0x82ca9f20
	ctx.lr = 0x83250C40;
	sub_82CA9F20(ctx, base);
	// 83250C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C50 size=64
    let mut pc: u32 = 0x83250C50;
    'dispatch: loop {
        match pc {
            0x83250C50 => {
    //   block [0x83250C50..0x83250C90)
	// 83250C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83250C60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250C64: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83250C68: 386A7FB8  addi r3, r10, 0x7fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32696;
	// 83250C6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250C70: 4AFDC261  bl 0x8222ced0
	ctx.lr = 0x83250C74;
	sub_8222CED0(ctx, base);
	// 83250C74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250C78: 386995D8  addi r3, r9, -0x6a28
	ctx.r[3].s64 = ctx.r[9].s64 + -27176;
	// 83250C7C: 4BA592A5  bl 0x82ca9f20
	ctx.lr = 0x83250C80;
	sub_82CA9F20(ctx, base);
	// 83250C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250C90 size=56
    let mut pc: u32 = 0x83250C90;
    'dispatch: loop {
        match pc {
            0x83250C90 => {
    //   block [0x83250C90..0x83250CC8)
	// 83250C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250C9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83250CA4: 386B9D2C  addi r3, r11, -0x62d4
	ctx.r[3].s64 = ctx.r[11].s64 + -25300;
	// 83250CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83250CAC: 4AFA30AD  bl 0x821f3d58
	ctx.lr = 0x83250CB0;
	sub_821F3D58(ctx, base);
	// 83250CB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250CB4: 906A7FBC  stw r3, 0x7fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32700 as u32), ctx.r[3].u32 ) };
	// 83250CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250CC8 size=64
    let mut pc: u32 = 0x83250CC8;
    'dispatch: loop {
        match pc {
            0x83250CC8 => {
    //   block [0x83250CC8..0x83250D08)
	// 83250CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250CD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250CD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250CDC: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 83250CE0: 386A7FC0  addi r3, r10, 0x7fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 32704;
	// 83250CE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250CE8: 4AFDC1E9  bl 0x8222ced0
	ctx.lr = 0x83250CEC;
	sub_8222CED0(ctx, base);
	// 83250CEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250CF0: 386995E8  addi r3, r9, -0x6a18
	ctx.r[3].s64 = ctx.r[9].s64 + -27160;
	// 83250CF4: 4BA5922D  bl 0x82ca9f20
	ctx.lr = 0x83250CF8;
	sub_82CA9F20(ctx, base);
	// 83250CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D08 size=64
    let mut pc: u32 = 0x83250D08;
    'dispatch: loop {
        match pc {
            0x83250D08 => {
    //   block [0x83250D08..0x83250D48)
	// 83250D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D1C: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 83250D20: 386A7FC4  addi r3, r10, 0x7fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32708;
	// 83250D24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250D28: 4AFDC1A9  bl 0x8222ced0
	ctx.lr = 0x83250D2C;
	sub_8222CED0(ctx, base);
	// 83250D2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250D30: 386995F8  addi r3, r9, -0x6a08
	ctx.r[3].s64 = ctx.r[9].s64 + -27144;
	// 83250D34: 4BA591ED  bl 0x82ca9f20
	ctx.lr = 0x83250D38;
	sub_82CA9F20(ctx, base);
	// 83250D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D48 size=64
    let mut pc: u32 = 0x83250D48;
    'dispatch: loop {
        match pc {
            0x83250D48 => {
    //   block [0x83250D48..0x83250D88)
	// 83250D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D5C: 388B28E4  addi r4, r11, 0x28e4
	ctx.r[4].s64 = ctx.r[11].s64 + 10468;
	// 83250D60: 386A7FC8  addi r3, r10, 0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32712;
	// 83250D64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250D68: 4AFDC169  bl 0x8222ced0
	ctx.lr = 0x83250D6C;
	sub_8222CED0(ctx, base);
	// 83250D6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250D70: 38699608  addi r3, r9, -0x69f8
	ctx.r[3].s64 = ctx.r[9].s64 + -27128;
	// 83250D74: 4BA591AD  bl 0x82ca9f20
	ctx.lr = 0x83250D78;
	sub_82CA9F20(ctx, base);
	// 83250D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250D88 size=64
    let mut pc: u32 = 0x83250D88;
    'dispatch: loop {
        match pc {
            0x83250D88 => {
    //   block [0x83250D88..0x83250DC8)
	// 83250D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250D94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250D98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250D9C: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 83250DA0: 386A7FCC  addi r3, r10, 0x7fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 32716;
	// 83250DA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250DA8: 4AFDC129  bl 0x8222ced0
	ctx.lr = 0x83250DAC;
	sub_8222CED0(ctx, base);
	// 83250DAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250DB0: 38699618  addi r3, r9, -0x69e8
	ctx.r[3].s64 = ctx.r[9].s64 + -27112;
	// 83250DB4: 4BA5916D  bl 0x82ca9f20
	ctx.lr = 0x83250DB8;
	sub_82CA9F20(ctx, base);
	// 83250DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250DC8 size=64
    let mut pc: u32 = 0x83250DC8;
    'dispatch: loop {
        match pc {
            0x83250DC8 => {
    //   block [0x83250DC8..0x83250E08)
	// 83250DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250DD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250DD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250DDC: 388B28F8  addi r4, r11, 0x28f8
	ctx.r[4].s64 = ctx.r[11].s64 + 10488;
	// 83250DE0: 386A7FD0  addi r3, r10, 0x7fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 32720;
	// 83250DE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250DE8: 4AFDC0E9  bl 0x8222ced0
	ctx.lr = 0x83250DEC;
	sub_8222CED0(ctx, base);
	// 83250DEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250DF0: 38699628  addi r3, r9, -0x69d8
	ctx.r[3].s64 = ctx.r[9].s64 + -27096;
	// 83250DF4: 4BA5912D  bl 0x82ca9f20
	ctx.lr = 0x83250DF8;
	sub_82CA9F20(ctx, base);
	// 83250DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E08 size=64
    let mut pc: u32 = 0x83250E08;
    'dispatch: loop {
        match pc {
            0x83250E08 => {
    //   block [0x83250E08..0x83250E48)
	// 83250E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E1C: 388B2904  addi r4, r11, 0x2904
	ctx.r[4].s64 = ctx.r[11].s64 + 10500;
	// 83250E20: 386A7FD4  addi r3, r10, 0x7fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32724;
	// 83250E24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250E28: 4AFDC0A9  bl 0x8222ced0
	ctx.lr = 0x83250E2C;
	sub_8222CED0(ctx, base);
	// 83250E2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250E30: 38699638  addi r3, r9, -0x69c8
	ctx.r[3].s64 = ctx.r[9].s64 + -27080;
	// 83250E34: 4BA590ED  bl 0x82ca9f20
	ctx.lr = 0x83250E38;
	sub_82CA9F20(ctx, base);
	// 83250E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E48 size=64
    let mut pc: u32 = 0x83250E48;
    'dispatch: loop {
        match pc {
            0x83250E48 => {
    //   block [0x83250E48..0x83250E88)
	// 83250E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E5C: 388B2914  addi r4, r11, 0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + 10516;
	// 83250E60: 386A7FD8  addi r3, r10, 0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32728;
	// 83250E64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250E68: 4AFDC069  bl 0x8222ced0
	ctx.lr = 0x83250E6C;
	sub_8222CED0(ctx, base);
	// 83250E6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250E70: 38699648  addi r3, r9, -0x69b8
	ctx.r[3].s64 = ctx.r[9].s64 + -27064;
	// 83250E74: 4BA590AD  bl 0x82ca9f20
	ctx.lr = 0x83250E78;
	sub_82CA9F20(ctx, base);
	// 83250E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250E88 size=64
    let mut pc: u32 = 0x83250E88;
    'dispatch: loop {
        match pc {
            0x83250E88 => {
    //   block [0x83250E88..0x83250EC8)
	// 83250E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250E94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250E98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250E9C: 388B291C  addi r4, r11, 0x291c
	ctx.r[4].s64 = ctx.r[11].s64 + 10524;
	// 83250EA0: 386A7FDC  addi r3, r10, 0x7fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 32732;
	// 83250EA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250EA8: 4AFDC029  bl 0x8222ced0
	ctx.lr = 0x83250EAC;
	sub_8222CED0(ctx, base);
	// 83250EAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250EB0: 38699658  addi r3, r9, -0x69a8
	ctx.r[3].s64 = ctx.r[9].s64 + -27048;
	// 83250EB4: 4BA5906D  bl 0x82ca9f20
	ctx.lr = 0x83250EB8;
	sub_82CA9F20(ctx, base);
	// 83250EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250EC8 size=64
    let mut pc: u32 = 0x83250EC8;
    'dispatch: loop {
        match pc {
            0x83250EC8 => {
    //   block [0x83250EC8..0x83250F08)
	// 83250EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250ED4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250ED8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250EDC: 388B292C  addi r4, r11, 0x292c
	ctx.r[4].s64 = ctx.r[11].s64 + 10540;
	// 83250EE0: 386A7FE0  addi r3, r10, 0x7fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 32736;
	// 83250EE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250EE8: 4AFDBFE9  bl 0x8222ced0
	ctx.lr = 0x83250EEC;
	sub_8222CED0(ctx, base);
	// 83250EEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250EF0: 38699668  addi r3, r9, -0x6998
	ctx.r[3].s64 = ctx.r[9].s64 + -27032;
	// 83250EF4: 4BA5902D  bl 0x82ca9f20
	ctx.lr = 0x83250EF8;
	sub_82CA9F20(ctx, base);
	// 83250EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F08 size=64
    let mut pc: u32 = 0x83250F08;
    'dispatch: loop {
        match pc {
            0x83250F08 => {
    //   block [0x83250F08..0x83250F48)
	// 83250F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F1C: 388B2934  addi r4, r11, 0x2934
	ctx.r[4].s64 = ctx.r[11].s64 + 10548;
	// 83250F20: 386A7FE4  addi r3, r10, 0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 32740;
	// 83250F24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250F28: 4AFDBFA9  bl 0x8222ced0
	ctx.lr = 0x83250F2C;
	sub_8222CED0(ctx, base);
	// 83250F2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250F30: 38699678  addi r3, r9, -0x6988
	ctx.r[3].s64 = ctx.r[9].s64 + -27016;
	// 83250F34: 4BA58FED  bl 0x82ca9f20
	ctx.lr = 0x83250F38;
	sub_82CA9F20(ctx, base);
	// 83250F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F48 size=64
    let mut pc: u32 = 0x83250F48;
    'dispatch: loop {
        match pc {
            0x83250F48 => {
    //   block [0x83250F48..0x83250F88)
	// 83250F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F58: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F5C: 388B293C  addi r4, r11, 0x293c
	ctx.r[4].s64 = ctx.r[11].s64 + 10556;
	// 83250F60: 386A7FE8  addi r3, r10, 0x7fe8
	ctx.r[3].s64 = ctx.r[10].s64 + 32744;
	// 83250F64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250F68: 4AFDBF69  bl 0x8222ced0
	ctx.lr = 0x83250F6C;
	sub_8222CED0(ctx, base);
	// 83250F6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250F70: 38699688  addi r3, r9, -0x6978
	ctx.r[3].s64 = ctx.r[9].s64 + -27000;
	// 83250F74: 4BA58FAD  bl 0x82ca9f20
	ctx.lr = 0x83250F78;
	sub_82CA9F20(ctx, base);
	// 83250F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250F88 size=64
    let mut pc: u32 = 0x83250F88;
    'dispatch: loop {
        match pc {
            0x83250F88 => {
    //   block [0x83250F88..0x83250FC8)
	// 83250F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250F94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250F98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250F9C: 388B2948  addi r4, r11, 0x2948
	ctx.r[4].s64 = ctx.r[11].s64 + 10568;
	// 83250FA0: 386A7FEC  addi r3, r10, 0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + 32748;
	// 83250FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250FA8: 4AFDBF29  bl 0x8222ced0
	ctx.lr = 0x83250FAC;
	sub_8222CED0(ctx, base);
	// 83250FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250FB0: 38699698  addi r3, r9, -0x6968
	ctx.r[3].s64 = ctx.r[9].s64 + -26984;
	// 83250FB4: 4BA58F6D  bl 0x82ca9f20
	ctx.lr = 0x83250FB8;
	sub_82CA9F20(ctx, base);
	// 83250FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83250FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83250FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83250FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83250FC8 size=64
    let mut pc: u32 = 0x83250FC8;
    'dispatch: loop {
        match pc {
            0x83250FC8 => {
    //   block [0x83250FC8..0x83251008)
	// 83250FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83250FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83250FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83250FD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83250FD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83250FDC: 388B2950  addi r4, r11, 0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + 10576;
	// 83250FE0: 386A7FF0  addi r3, r10, 0x7ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 32752;
	// 83250FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83250FE8: 4AFDBEE9  bl 0x8222ced0
	ctx.lr = 0x83250FEC;
	sub_8222CED0(ctx, base);
	// 83250FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83250FF0: 386996A8  addi r3, r9, -0x6958
	ctx.r[3].s64 = ctx.r[9].s64 + -26968;
	// 83250FF4: 4BA58F2D  bl 0x82ca9f20
	ctx.lr = 0x83250FF8;
	sub_82CA9F20(ctx, base);
	// 83250FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83250FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251008 size=64
    let mut pc: u32 = 0x83251008;
    'dispatch: loop {
        match pc {
            0x83251008 => {
    //   block [0x83251008..0x83251048)
	// 83251008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325100C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251014: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251018: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325101C: 388B2958  addi r4, r11, 0x2958
	ctx.r[4].s64 = ctx.r[11].s64 + 10584;
	// 83251020: 386A7FF4  addi r3, r10, 0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 32756;
	// 83251024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251028: 4AFDBEA9  bl 0x8222ced0
	ctx.lr = 0x8325102C;
	sub_8222CED0(ctx, base);
	// 8325102C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251030: 386996B8  addi r3, r9, -0x6948
	ctx.r[3].s64 = ctx.r[9].s64 + -26952;
	// 83251034: 4BA58EED  bl 0x82ca9f20
	ctx.lr = 0x83251038;
	sub_82CA9F20(ctx, base);
	// 83251038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251048 size=64
    let mut pc: u32 = 0x83251048;
    'dispatch: loop {
        match pc {
            0x83251048 => {
    //   block [0x83251048..0x83251088)
	// 83251048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251054: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251058: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325105C: 388B2960  addi r4, r11, 0x2960
	ctx.r[4].s64 = ctx.r[11].s64 + 10592;
	// 83251060: 386A7FF8  addi r3, r10, 0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 32760;
	// 83251064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251068: 4AFDBE69  bl 0x8222ced0
	ctx.lr = 0x8325106C;
	sub_8222CED0(ctx, base);
	// 8325106C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251070: 386996C8  addi r3, r9, -0x6938
	ctx.r[3].s64 = ctx.r[9].s64 + -26936;
	// 83251074: 4BA58EAD  bl 0x82ca9f20
	ctx.lr = 0x83251078;
	sub_82CA9F20(ctx, base);
	// 83251078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325107C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251088 size=64
    let mut pc: u32 = 0x83251088;
    'dispatch: loop {
        match pc {
            0x83251088 => {
    //   block [0x83251088..0x832510C8)
	// 83251088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251094: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251098: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8325109C: 388B2968  addi r4, r11, 0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + 10600;
	// 832510A0: 386A7FFC  addi r3, r10, 0x7ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 32764;
	// 832510A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832510A8: 4AFDBE29  bl 0x8222ced0
	ctx.lr = 0x832510AC;
	sub_8222CED0(ctx, base);
	// 832510AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832510B0: 386996D8  addi r3, r9, -0x6928
	ctx.r[3].s64 = ctx.r[9].s64 + -26920;
	// 832510B4: 4BA58E6D  bl 0x82ca9f20
	ctx.lr = 0x832510B8;
	sub_82CA9F20(ctx, base);
	// 832510B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832510BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832510C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832510C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832510C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832510C8 size=64
    let mut pc: u32 = 0x832510C8;
    'dispatch: loop {
        match pc {
            0x832510C8 => {
    //   block [0x832510C8..0x83251108)
	// 832510C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832510CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832510D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832510D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832510D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832510DC: 388B2970  addi r4, r11, 0x2970
	ctx.r[4].s64 = ctx.r[11].s64 + 10608;
	// 832510E0: 386A8000  addi r3, r10, -0x8000
	ctx.r[3].s64 = ctx.r[10].s64 + -32768;
	// 832510E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832510E8: 4AFDBDE9  bl 0x8222ced0
	ctx.lr = 0x832510EC;
	sub_8222CED0(ctx, base);
	// 832510EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832510F0: 386996E8  addi r3, r9, -0x6918
	ctx.r[3].s64 = ctx.r[9].s64 + -26904;
	// 832510F4: 4BA58E2D  bl 0x82ca9f20
	ctx.lr = 0x832510F8;
	sub_82CA9F20(ctx, base);
	// 832510F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832510FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251108 size=64
    let mut pc: u32 = 0x83251108;
    'dispatch: loop {
        match pc {
            0x83251108 => {
    //   block [0x83251108..0x83251148)
	// 83251108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251114: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325111C: 388B2978  addi r4, r11, 0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + 10616;
	// 83251120: 386A8004  addi r3, r10, -0x7ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -32764;
	// 83251124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251128: 4AFDBDA9  bl 0x8222ced0
	ctx.lr = 0x8325112C;
	sub_8222CED0(ctx, base);
	// 8325112C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251130: 386996F8  addi r3, r9, -0x6908
	ctx.r[3].s64 = ctx.r[9].s64 + -26888;
	// 83251134: 4BA58DED  bl 0x82ca9f20
	ctx.lr = 0x83251138;
	sub_82CA9F20(ctx, base);
	// 83251138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251148 size=64
    let mut pc: u32 = 0x83251148;
    'dispatch: loop {
        match pc {
            0x83251148 => {
    //   block [0x83251148..0x83251188)
	// 83251148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251154: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325115C: 388B2980  addi r4, r11, 0x2980
	ctx.r[4].s64 = ctx.r[11].s64 + 10624;
	// 83251160: 386A8008  addi r3, r10, -0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -32760;
	// 83251164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251168: 4AFDBD69  bl 0x8222ced0
	ctx.lr = 0x8325116C;
	sub_8222CED0(ctx, base);
	// 8325116C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251170: 38699708  addi r3, r9, -0x68f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26872;
	// 83251174: 4BA58DAD  bl 0x82ca9f20
	ctx.lr = 0x83251178;
	sub_82CA9F20(ctx, base);
	// 83251178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251188 size=64
    let mut pc: u32 = 0x83251188;
    'dispatch: loop {
        match pc {
            0x83251188 => {
    //   block [0x83251188..0x832511C8)
	// 83251188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251194: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325119C: 388B298C  addi r4, r11, 0x298c
	ctx.r[4].s64 = ctx.r[11].s64 + 10636;
	// 832511A0: 386A800C  addi r3, r10, -0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -32756;
	// 832511A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832511A8: 4AFDBD29  bl 0x8222ced0
	ctx.lr = 0x832511AC;
	sub_8222CED0(ctx, base);
	// 832511AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832511B0: 38699718  addi r3, r9, -0x68e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26856;
	// 832511B4: 4BA58D6D  bl 0x82ca9f20
	ctx.lr = 0x832511B8;
	sub_82CA9F20(ctx, base);
	// 832511B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832511BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832511C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832511C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832511C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832511C8 size=64
    let mut pc: u32 = 0x832511C8;
    'dispatch: loop {
        match pc {
            0x832511C8 => {
    //   block [0x832511C8..0x83251208)
	// 832511C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832511CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832511D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832511D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832511D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832511DC: 388B8CD0  addi r4, r11, -0x7330
	ctx.r[4].s64 = ctx.r[11].s64 + -29488;
	// 832511E0: 386A8010  addi r3, r10, -0x7ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -32752;
	// 832511E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832511E8: 4AFDBCE9  bl 0x8222ced0
	ctx.lr = 0x832511EC;
	sub_8222CED0(ctx, base);
	// 832511EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832511F0: 38699728  addi r3, r9, -0x68d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26840;
	// 832511F4: 4BA58D2D  bl 0x82ca9f20
	ctx.lr = 0x832511F8;
	sub_82CA9F20(ctx, base);
	// 832511F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832511FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251208 size=64
    let mut pc: u32 = 0x83251208;
    'dispatch: loop {
        match pc {
            0x83251208 => {
    //   block [0x83251208..0x83251248)
	// 83251208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251214: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325121C: 388B2994  addi r4, r11, 0x2994
	ctx.r[4].s64 = ctx.r[11].s64 + 10644;
	// 83251220: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 83251224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251228: 4AFDBCA9  bl 0x8222ced0
	ctx.lr = 0x8325122C;
	sub_8222CED0(ctx, base);
	// 8325122C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251230: 38699738  addi r3, r9, -0x68c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26824;
	// 83251234: 4BA58CED  bl 0x82ca9f20
	ctx.lr = 0x83251238;
	sub_82CA9F20(ctx, base);
	// 83251238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325123C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251248 size=56
    let mut pc: u32 = 0x83251248;
    'dispatch: loop {
        match pc {
            0x83251248 => {
    //   block [0x83251248..0x83251280)
	// 83251248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325124C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251254: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325125C: 386B0478  addi r3, r11, 0x478
	ctx.r[3].s64 = ctx.r[11].s64 + 1144;
	// 83251260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251264: 4AFA2AF5  bl 0x821f3d58
	ctx.lr = 0x83251268;
	sub_821F3D58(ctx, base);
	// 83251268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325126C: 906A8018  stw r3, -0x7fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32744 as u32), ctx.r[3].u32 ) };
	// 83251270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83251280 size=12
    let mut pc: u32 = 0x83251280;
    'dispatch: loop {
        match pc {
            0x83251280 => {
    //   block [0x83251280..0x8325128C)
	// 83251280: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251284: 386B9748  addi r3, r11, -0x68b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26808;
	// 83251288: 4BA58C98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251290 size=192
    let mut pc: u32 = 0x83251290;
    'dispatch: loop {
        match pc {
            0x83251290 => {
    //   block [0x83251290..0x83251350)
	// 83251290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325129C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832512A0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832512A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832512A8: 388B2EF0  addi r4, r11, 0x2ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 12016;
	// 832512AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832512B0: 4AFDBC21  bl 0x8222ced0
	ctx.lr = 0x832512B4;
	sub_8222CED0(ctx, base);
	// 832512B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832512B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512BC: 4AF9D87D  bl 0x821eeb38
	ctx.lr = 0x832512C0;
	sub_821EEB38(ctx, base);
	// 832512C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512C4: 4B9B252D  bl 0x82c037f0
	ctx.lr = 0x832512C8;
	sub_82C037F0(ctx, base);
	// 832512C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832512CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832512D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832512D4: 916A802C  stw r11, -0x7fd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32724 as u32), ctx.r[11].u32 ) };
	// 832512D8: 4AF75491  bl 0x821c6768
	ctx.lr = 0x832512DC;
	sub_821C6768(ctx, base);
	// 832512DC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832512E0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832512E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 832512E8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832512EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832512F0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832512F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832512F8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832512FC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251300: 4082FFE8  bne 0x832512e8
	if !ctx.cr[0].eq {
	pc = 0x832512E8; continue 'dispatch;
	}
	// 83251304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251308: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325130C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251310: 4AF75459  bl 0x821c6768
	ctx.lr = 0x83251314;
	sub_821C6768(ctx, base);
	// 83251314: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251318: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325131C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251320: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251324: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251328: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325132C: 4082FFE8  bne 0x83251314
	if !ctx.cr[0].eq {
	pc = 0x83251314; continue 'dispatch;
	}
	// 83251330: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251334: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 83251338: 4BA58BE9  bl 0x82ca9f20
	ctx.lr = 0x8325133C;
	sub_82CA9F20(ctx, base);
	// 8325133C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325134C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251350 size=192
    let mut pc: u32 = 0x83251350;
    'dispatch: loop {
        match pc {
            0x83251350 => {
    //   block [0x83251350..0x83251410)
	// 83251350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325135C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251360: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251368: 388B2EF0  addi r4, r11, 0x2ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 12016;
	// 8325136C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251370: 4AFDBB61  bl 0x8222ced0
	ctx.lr = 0x83251374;
	sub_8222CED0(ctx, base);
	// 83251374: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325137C: 4AF9D7BD  bl 0x821eeb38
	ctx.lr = 0x83251380;
	sub_821EEB38(ctx, base);
	// 83251380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251384: 4B9B246D  bl 0x82c037f0
	ctx.lr = 0x83251388;
	sub_82C037F0(ctx, base);
	// 83251388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325138C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251394: 916A8030  stw r11, -0x7fd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32720 as u32), ctx.r[11].u32 ) };
	// 83251398: 4AF753D1  bl 0x821c6768
	ctx.lr = 0x8325139C;
	sub_821C6768(ctx, base);
	// 8325139C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832513A0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832513A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 832513A8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832513AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513B0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832513B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832513B8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832513BC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513C0: 4082FFE8  bne 0x832513a8
	if !ctx.cr[0].eq {
	pc = 0x832513A8; continue 'dispatch;
	}
	// 832513C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832513C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832513CC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832513D0: 4AF75399  bl 0x821c6768
	ctx.lr = 0x832513D4;
	sub_821C6768(ctx, base);
	// 832513D4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832513D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513DC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832513E0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832513E4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832513E8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832513EC: 4082FFE8  bne 0x832513d4
	if !ctx.cr[0].eq {
	pc = 0x832513D4; continue 'dispatch;
	}
	// 832513F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832513F4: 386B9760  addi r3, r11, -0x68a0
	ctx.r[3].s64 = ctx.r[11].s64 + -26784;
	// 832513F8: 4BA58B29  bl 0x82ca9f20
	ctx.lr = 0x832513FC;
	sub_82CA9F20(ctx, base);
	// 832513FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325140C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251410 size=192
    let mut pc: u32 = 0x83251410;
    'dispatch: loop {
        match pc {
            0x83251410 => {
    //   block [0x83251410..0x832514D0)
	// 83251410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325141C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251420: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251428: 388B2F14  addi r4, r11, 0x2f14
	ctx.r[4].s64 = ctx.r[11].s64 + 12052;
	// 8325142C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251430: 4AFDBAA1  bl 0x8222ced0
	ctx.lr = 0x83251434;
	sub_8222CED0(ctx, base);
	// 83251434: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325143C: 4AF9D6FD  bl 0x821eeb38
	ctx.lr = 0x83251440;
	sub_821EEB38(ctx, base);
	// 83251440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251444: 4B9B23AD  bl 0x82c037f0
	ctx.lr = 0x83251448;
	sub_82C037F0(ctx, base);
	// 83251448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325144C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251454: 916A8034  stw r11, -0x7fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32716 as u32), ctx.r[11].u32 ) };
	// 83251458: 4AF75311  bl 0x821c6768
	ctx.lr = 0x8325145C;
	sub_821C6768(ctx, base);
	// 8325145C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83251460: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83251464: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83251468: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325146C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251470: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83251474: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83251478: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325147C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251480: 4082FFE8  bne 0x83251468
	if !ctx.cr[0].eq {
	pc = 0x83251468; continue 'dispatch;
	}
	// 83251484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251488: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325148C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251490: 4AF752D9  bl 0x821c6768
	ctx.lr = 0x83251494;
	sub_821C6768(ctx, base);
	// 83251494: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251498: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325149C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832514A0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832514A4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832514A8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832514AC: 4082FFE8  bne 0x83251494
	if !ctx.cr[0].eq {
	pc = 0x83251494; continue 'dispatch;
	}
	// 832514B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832514B4: 386B9768  addi r3, r11, -0x6898
	ctx.r[3].s64 = ctx.r[11].s64 + -26776;
	// 832514B8: 4BA58A69  bl 0x82ca9f20
	ctx.lr = 0x832514BC;
	sub_82CA9F20(ctx, base);
	// 832514BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832514C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832514C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832514C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832514CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832514D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832514D0 size=192
    let mut pc: u32 = 0x832514D0;
    'dispatch: loop {
        match pc {
            0x832514D0 => {
    //   block [0x832514D0..0x83251590)
	// 832514D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832514D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832514D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832514DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832514E0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832514E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832514E8: 388B2F38  addi r4, r11, 0x2f38
	ctx.r[4].s64 = ctx.r[11].s64 + 12088;
	// 832514EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832514F0: 4AFDB9E1  bl 0x8222ced0
	ctx.lr = 0x832514F4;
	sub_8222CED0(ctx, base);
	// 832514F4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832514F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832514FC: 4AF9D63D  bl 0x821eeb38
	ctx.lr = 0x83251500;
	sub_821EEB38(ctx, base);
	// 83251500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251504: 4B9B22ED  bl 0x82c037f0
	ctx.lr = 0x83251508;
	sub_82C037F0(ctx, base);
	// 83251508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325150C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251514: 916A8038  stw r11, -0x7fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32712 as u32), ctx.r[11].u32 ) };
	// 83251518: 4AF75251  bl 0x821c6768
	ctx.lr = 0x8325151C;
	sub_821C6768(ctx, base);
	// 8325151C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83251520: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83251524: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83251528: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325152C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251530: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83251534: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83251538: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325153C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251540: 4082FFE8  bne 0x83251528
	if !ctx.cr[0].eq {
	pc = 0x83251528; continue 'dispatch;
	}
	// 83251544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251548: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325154C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251550: 4AF75219  bl 0x821c6768
	ctx.lr = 0x83251554;
	sub_821C6768(ctx, base);
	// 83251554: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251558: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325155C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251560: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251564: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251568: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325156C: 4082FFE8  bne 0x83251554
	if !ctx.cr[0].eq {
	pc = 0x83251554; continue 'dispatch;
	}
	// 83251570: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251574: 386B9770  addi r3, r11, -0x6890
	ctx.r[3].s64 = ctx.r[11].s64 + -26768;
	// 83251578: 4BA589A9  bl 0x82ca9f20
	ctx.lr = 0x8325157C;
	sub_82CA9F20(ctx, base);
	// 8325157C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251590 size=192
    let mut pc: u32 = 0x83251590;
    'dispatch: loop {
        match pc {
            0x83251590 => {
    //   block [0x83251590..0x83251650)
	// 83251590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325159C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832515A0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832515A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832515A8: 388B2F58  addi r4, r11, 0x2f58
	ctx.r[4].s64 = ctx.r[11].s64 + 12120;
	// 832515AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832515B0: 4AFDB921  bl 0x8222ced0
	ctx.lr = 0x832515B4;
	sub_8222CED0(ctx, base);
	// 832515B4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832515B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515BC: 4AF9D57D  bl 0x821eeb38
	ctx.lr = 0x832515C0;
	sub_821EEB38(ctx, base);
	// 832515C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515C4: 4B9B222D  bl 0x82c037f0
	ctx.lr = 0x832515C8;
	sub_82C037F0(ctx, base);
	// 832515C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832515CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832515D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832515D4: 916A803C  stw r11, -0x7fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32708 as u32), ctx.r[11].u32 ) };
	// 832515D8: 4AF75191  bl 0x821c6768
	ctx.lr = 0x832515DC;
	sub_821C6768(ctx, base);
	// 832515DC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832515E0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832515E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 832515E8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832515EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832515F0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832515F4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832515F8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832515FC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83251600: 4082FFE8  bne 0x832515e8
	if !ctx.cr[0].eq {
	pc = 0x832515E8; continue 'dispatch;
	}
	// 83251604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83251608: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325160C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83251610: 4AF75159  bl 0x821c6768
	ctx.lr = 0x83251614;
	sub_821C6768(ctx, base);
	// 83251614: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83251618: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325161C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83251620: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 83251624: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83251628: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325162C: 4082FFE8  bne 0x83251614
	if !ctx.cr[0].eq {
	pc = 0x83251614; continue 'dispatch;
	}
	// 83251630: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83251634: 386B9778  addi r3, r11, -0x6888
	ctx.r[3].s64 = ctx.r[11].s64 + -26760;
	// 83251638: 4BA588E9  bl 0x82ca9f20
	ctx.lr = 0x8325163C;
	sub_82CA9F20(ctx, base);
	// 8325163C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325164C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251650 size=192
    let mut pc: u32 = 0x83251650;
    'dispatch: loop {
        match pc {
            0x83251650 => {
    //   block [0x83251650..0x83251710)
	// 83251650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251658: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325165C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251660: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251668: 388B2F78  addi r4, r11, 0x2f78
	ctx.r[4].s64 = ctx.r[11].s64 + 12152;
	// 8325166C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83251670: 4AFDB861  bl 0x8222ced0
	ctx.lr = 0x83251674;
	sub_8222CED0(ctx, base);
	// 83251674: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83251678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325167C: 4AF9D4BD  bl 0x821eeb38
	ctx.lr = 0x83251680;
	sub_821EEB38(ctx, base);
	// 83251680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251684: 4B9B216D  bl 0x82c037f0
	ctx.lr = 0x83251688;
	sub_82C037F0(ctx, base);
	// 83251688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325168C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83251690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83251694: 916A8040  stw r11, -0x7fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32704 as u32), ctx.r[11].u32 ) };
	// 83251698: 4AF750D1  bl 0x821c6768
	ctx.lr = 0x8325169C;
	sub_821C6768(ctx, base);
	// 8325169C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832516A0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832516A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 832516A8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832516AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516B0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832516B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832516B8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832516BC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516C0: 4082FFE8  bne 0x832516a8
	if !ctx.cr[0].eq {
	pc = 0x832516A8; continue 'dispatch;
	}
	// 832516C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832516C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832516CC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832516D0: 4AF75099  bl 0x821c6768
	ctx.lr = 0x832516D4;
	sub_821C6768(ctx, base);
	// 832516D4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 832516D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516DC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832516E0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832516E4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832516E8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832516EC: 4082FFE8  bne 0x832516d4
	if !ctx.cr[0].eq {
	pc = 0x832516D4; continue 'dispatch;
	}
	// 832516F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832516F4: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 832516F8: 4BA58829  bl 0x82ca9f20
	ctx.lr = 0x832516FC;
	sub_82CA9F20(ctx, base);
	// 832516FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83251700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325170C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251710 size=56
    let mut pc: u32 = 0x83251710;
    'dispatch: loop {
        match pc {
            0x83251710 => {
    //   block [0x83251710..0x83251748)
	// 83251710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325171C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251724: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83251728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325172C: 4AFA262D  bl 0x821f3d58
	ctx.lr = 0x83251730;
	sub_821F3D58(ctx, base);
	// 83251730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251734: 906A8044  stw r3, -0x7fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32700 as u32), ctx.r[3].u32 ) };
	// 83251738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325173C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251748 size=56
    let mut pc: u32 = 0x83251748;
    'dispatch: loop {
        match pc {
            0x83251748 => {
    //   block [0x83251748..0x83251780)
	// 83251748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325174C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325175C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83251760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251764: 4AFA25F5  bl 0x821f3d58
	ctx.lr = 0x83251768;
	sub_821F3D58(ctx, base);
	// 83251768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325176C: 906A8048  stw r3, -0x7fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32696 as u32), ctx.r[3].u32 ) };
	// 83251770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251780 size=56
    let mut pc: u32 = 0x83251780;
    'dispatch: loop {
        match pc {
            0x83251780 => {
    //   block [0x83251780..0x832517B8)
	// 83251780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325178C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251794: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83251798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325179C: 4AFA25BD  bl 0x821f3d58
	ctx.lr = 0x832517A0;
	sub_821F3D58(ctx, base);
	// 832517A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832517A4: 906A804C  stw r3, -0x7fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32692 as u32), ctx.r[3].u32 ) };
	// 832517A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832517AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832517B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832517B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832517B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832517B8 size=56
    let mut pc: u32 = 0x832517B8;
    'dispatch: loop {
        match pc {
            0x832517B8 => {
    //   block [0x832517B8..0x832517F0)
	// 832517B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832517BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832517C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832517C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832517C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832517CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832517D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832517D4: 4AFA2585  bl 0x821f3d58
	ctx.lr = 0x832517D8;
	sub_821F3D58(ctx, base);
	// 832517D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832517DC: 906A8050  stw r3, -0x7fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32688 as u32), ctx.r[3].u32 ) };
	// 832517E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832517E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832517E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832517EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832517F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832517F0 size=56
    let mut pc: u32 = 0x832517F0;
    'dispatch: loop {
        match pc {
            0x832517F0 => {
    //   block [0x832517F0..0x83251828)
	// 832517F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832517F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832517F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832517FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251804: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83251808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325180C: 4AFA254D  bl 0x821f3d58
	ctx.lr = 0x83251810;
	sub_821F3D58(ctx, base);
	// 83251810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251814: 906A8054  stw r3, -0x7fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32684 as u32), ctx.r[3].u32 ) };
	// 83251818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251828 size=56
    let mut pc: u32 = 0x83251828;
    'dispatch: loop {
        match pc {
            0x83251828 => {
    //   block [0x83251828..0x83251860)
	// 83251828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325183C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83251840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251844: 4AFA2515  bl 0x821f3d58
	ctx.lr = 0x83251848;
	sub_821F3D58(ctx, base);
	// 83251848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325184C: 906A8058  stw r3, -0x7fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32680 as u32), ctx.r[3].u32 ) };
	// 83251850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325185C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251860 size=56
    let mut pc: u32 = 0x83251860;
    'dispatch: loop {
        match pc {
            0x83251860 => {
    //   block [0x83251860..0x83251898)
	// 83251860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325186C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251874: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83251878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325187C: 4AFA24DD  bl 0x821f3d58
	ctx.lr = 0x83251880;
	sub_821F3D58(ctx, base);
	// 83251880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251884: 906A805C  stw r3, -0x7fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32676 as u32), ctx.r[3].u32 ) };
	// 83251888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251898 size=56
    let mut pc: u32 = 0x83251898;
    'dispatch: loop {
        match pc {
            0x83251898 => {
    //   block [0x83251898..0x832518D0)
	// 83251898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832518A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832518A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832518A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832518AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832518B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832518B4: 4AFA24A5  bl 0x821f3d58
	ctx.lr = 0x832518B8;
	sub_821F3D58(ctx, base);
	// 832518B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832518BC: 906A8060  stw r3, -0x7fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32672 as u32), ctx.r[3].u32 ) };
	// 832518C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832518C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832518C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832518CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832518D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832518D0 size=56
    let mut pc: u32 = 0x832518D0;
    'dispatch: loop {
        match pc {
            0x832518D0 => {
    //   block [0x832518D0..0x83251908)
	// 832518D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832518D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832518D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832518DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832518E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832518E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832518E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832518EC: 4AFA246D  bl 0x821f3d58
	ctx.lr = 0x832518F0;
	sub_821F3D58(ctx, base);
	// 832518F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832518F4: 906A8064  stw r3, -0x7f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32668 as u32), ctx.r[3].u32 ) };
	// 832518F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832518FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251908 size=56
    let mut pc: u32 = 0x83251908;
    'dispatch: loop {
        match pc {
            0x83251908 => {
    //   block [0x83251908..0x83251940)
	// 83251908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325191C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83251920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251924: 4AFA2435  bl 0x821f3d58
	ctx.lr = 0x83251928;
	sub_821F3D58(ctx, base);
	// 83251928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325192C: 906A8068  stw r3, -0x7f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32664 as u32), ctx.r[3].u32 ) };
	// 83251930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251940 size=56
    let mut pc: u32 = 0x83251940;
    'dispatch: loop {
        match pc {
            0x83251940 => {
    //   block [0x83251940..0x83251978)
	// 83251940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325194C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251954: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83251958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325195C: 4AFA23FD  bl 0x821f3d58
	ctx.lr = 0x83251960;
	sub_821F3D58(ctx, base);
	// 83251960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251964: 906A806C  stw r3, -0x7f94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32660 as u32), ctx.r[3].u32 ) };
	// 83251968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251978 size=56
    let mut pc: u32 = 0x83251978;
    'dispatch: loop {
        match pc {
            0x83251978 => {
    //   block [0x83251978..0x832519B0)
	// 83251978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325198C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83251990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251994: 4AFA23C5  bl 0x821f3d58
	ctx.lr = 0x83251998;
	sub_821F3D58(ctx, base);
	// 83251998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325199C: 906A8070  stw r3, -0x7f90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32656 as u32), ctx.r[3].u32 ) };
	// 832519A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832519A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832519A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832519AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832519B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832519B0 size=56
    let mut pc: u32 = 0x832519B0;
    'dispatch: loop {
        match pc {
            0x832519B0 => {
    //   block [0x832519B0..0x832519E8)
	// 832519B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832519B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832519B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832519BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832519C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832519C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832519C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832519CC: 4AFA238D  bl 0x821f3d58
	ctx.lr = 0x832519D0;
	sub_821F3D58(ctx, base);
	// 832519D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832519D4: 906A8074  stw r3, -0x7f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32652 as u32), ctx.r[3].u32 ) };
	// 832519D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832519DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832519E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832519E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832519E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832519E8 size=56
    let mut pc: u32 = 0x832519E8;
    'dispatch: loop {
        match pc {
            0x832519E8 => {
    //   block [0x832519E8..0x83251A20)
	// 832519E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832519EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832519F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832519F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832519F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832519FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83251A00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A04: 4AFA2355  bl 0x821f3d58
	ctx.lr = 0x83251A08;
	sub_821F3D58(ctx, base);
	// 83251A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A0C: 906A8078  stw r3, -0x7f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32648 as u32), ctx.r[3].u32 ) };
	// 83251A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A20 size=56
    let mut pc: u32 = 0x83251A20;
    'dispatch: loop {
        match pc {
            0x83251A20 => {
    //   block [0x83251A20..0x83251A58)
	// 83251A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251A30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251A34: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83251A38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A3C: 4AFA231D  bl 0x821f3d58
	ctx.lr = 0x83251A40;
	sub_821F3D58(ctx, base);
	// 83251A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A44: 906A807C  stw r3, -0x7f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32644 as u32), ctx.r[3].u32 ) };
	// 83251A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A58 size=56
    let mut pc: u32 = 0x83251A58;
    'dispatch: loop {
        match pc {
            0x83251A58 => {
    //   block [0x83251A58..0x83251A90)
	// 83251A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251A68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251A6C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83251A70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A74: 4AFA22E5  bl 0x821f3d58
	ctx.lr = 0x83251A78;
	sub_821F3D58(ctx, base);
	// 83251A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A7C: 906A8080  stw r3, -0x7f80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32640 as u32), ctx.r[3].u32 ) };
	// 83251A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A90 size=56
    let mut pc: u32 = 0x83251A90;
    'dispatch: loop {
        match pc {
            0x83251A90 => {
    //   block [0x83251A90..0x83251AC8)
	// 83251A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251AA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251AA4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83251AA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251AAC: 4AFA22AD  bl 0x821f3d58
	ctx.lr = 0x83251AB0;
	sub_821F3D58(ctx, base);
	// 83251AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251AB4: 906A8084  stw r3, -0x7f7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32636 as u32), ctx.r[3].u32 ) };
	// 83251AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251AC8 size=56
    let mut pc: u32 = 0x83251AC8;
    'dispatch: loop {
        match pc {
            0x83251AC8 => {
    //   block [0x83251AC8..0x83251B00)
	// 83251AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251AD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251AD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251ADC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83251AE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251AE4: 4AFA2275  bl 0x821f3d58
	ctx.lr = 0x83251AE8;
	sub_821F3D58(ctx, base);
	// 83251AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251AEC: 906A8088  stw r3, -0x7f78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32632 as u32), ctx.r[3].u32 ) };
	// 83251AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B00 size=56
    let mut pc: u32 = 0x83251B00;
    'dispatch: loop {
        match pc {
            0x83251B00 => {
    //   block [0x83251B00..0x83251B38)
	// 83251B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B14: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83251B18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B1C: 4AFA223D  bl 0x821f3d58
	ctx.lr = 0x83251B20;
	sub_821F3D58(ctx, base);
	// 83251B20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B24: 906A808C  stw r3, -0x7f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32628 as u32), ctx.r[3].u32 ) };
	// 83251B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B38 size=56
    let mut pc: u32 = 0x83251B38;
    'dispatch: loop {
        match pc {
            0x83251B38 => {
    //   block [0x83251B38..0x83251B70)
	// 83251B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B4C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83251B50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B54: 4AFA2205  bl 0x821f3d58
	ctx.lr = 0x83251B58;
	sub_821F3D58(ctx, base);
	// 83251B58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B5C: 906A8090  stw r3, -0x7f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32624 as u32), ctx.r[3].u32 ) };
	// 83251B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B70 size=56
    let mut pc: u32 = 0x83251B70;
    'dispatch: loop {
        match pc {
            0x83251B70 => {
    //   block [0x83251B70..0x83251BA8)
	// 83251B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B84: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83251B88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B8C: 4AFA21CD  bl 0x821f3d58
	ctx.lr = 0x83251B90;
	sub_821F3D58(ctx, base);
	// 83251B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B94: 906A8094  stw r3, -0x7f6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32620 as u32), ctx.r[3].u32 ) };
	// 83251B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251BA8 size=64
    let mut pc: u32 = 0x83251BA8;
    'dispatch: loop {
        match pc {
            0x83251BA8 => {
    //   block [0x83251BA8..0x83251BE8)
	// 83251BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251BB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251BBC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83251BC0: 386A8098  addi r3, r10, -0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + -32616;
	// 83251BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251BC8: 4AFDB309  bl 0x8222ced0
	ctx.lr = 0x83251BCC;
	sub_8222CED0(ctx, base);
	// 83251BCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251BD0: 38699788  addi r3, r9, -0x6878
	ctx.r[3].s64 = ctx.r[9].s64 + -26744;
	// 83251BD4: 4BA5834D  bl 0x82ca9f20
	ctx.lr = 0x83251BD8;
	sub_82CA9F20(ctx, base);
	// 83251BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251BE8 size=64
    let mut pc: u32 = 0x83251BE8;
    'dispatch: loop {
        match pc {
            0x83251BE8 => {
    //   block [0x83251BE8..0x83251C28)
	// 83251BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251BFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83251C00: 386A809C  addi r3, r10, -0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + -32612;
	// 83251C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C08: 4AFDB2C9  bl 0x8222ced0
	ctx.lr = 0x83251C0C;
	sub_8222CED0(ctx, base);
	// 83251C0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C10: 38699798  addi r3, r9, -0x6868
	ctx.r[3].s64 = ctx.r[9].s64 + -26728;
	// 83251C14: 4BA5830D  bl 0x82ca9f20
	ctx.lr = 0x83251C18;
	sub_82CA9F20(ctx, base);
	// 83251C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251C28 size=64
    let mut pc: u32 = 0x83251C28;
    'dispatch: loop {
        match pc {
            0x83251C28 => {
    //   block [0x83251C28..0x83251C68)
	// 83251C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251C3C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83251C40: 386A80A0  addi r3, r10, -0x7f60
	ctx.r[3].s64 = ctx.r[10].s64 + -32608;
	// 83251C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C48: 4AFDB289  bl 0x8222ced0
	ctx.lr = 0x83251C4C;
	sub_8222CED0(ctx, base);
	// 83251C4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C50: 386997A8  addi r3, r9, -0x6858
	ctx.r[3].s64 = ctx.r[9].s64 + -26712;
	// 83251C54: 4BA582CD  bl 0x82ca9f20
	ctx.lr = 0x83251C58;
	sub_82CA9F20(ctx, base);
	// 83251C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251C68 size=64
    let mut pc: u32 = 0x83251C68;
    'dispatch: loop {
        match pc {
            0x83251C68 => {
    //   block [0x83251C68..0x83251CA8)
	// 83251C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251C7C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83251C80: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 83251C84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C88: 4AFDB249  bl 0x8222ced0
	ctx.lr = 0x83251C8C;
	sub_8222CED0(ctx, base);
	// 83251C8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C90: 386997B8  addi r3, r9, -0x6848
	ctx.r[3].s64 = ctx.r[9].s64 + -26696;
	// 83251C94: 4BA5828D  bl 0x82ca9f20
	ctx.lr = 0x83251C98;
	sub_82CA9F20(ctx, base);
	// 83251C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251CA8 size=64
    let mut pc: u32 = 0x83251CA8;
    'dispatch: loop {
        match pc {
            0x83251CA8 => {
    //   block [0x83251CA8..0x83251CE8)
	// 83251CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251CB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251CBC: 388B2F98  addi r4, r11, 0x2f98
	ctx.r[4].s64 = ctx.r[11].s64 + 12184;
	// 83251CC0: 386A80A8  addi r3, r10, -0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + -32600;
	// 83251CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251CC8: 4AFDB209  bl 0x8222ced0
	ctx.lr = 0x83251CCC;
	sub_8222CED0(ctx, base);
	// 83251CCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251CD0: 386997C8  addi r3, r9, -0x6838
	ctx.r[3].s64 = ctx.r[9].s64 + -26680;
	// 83251CD4: 4BA5824D  bl 0x82ca9f20
	ctx.lr = 0x83251CD8;
	sub_82CA9F20(ctx, base);
	// 83251CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251CE8 size=64
    let mut pc: u32 = 0x83251CE8;
    'dispatch: loop {
        match pc {
            0x83251CE8 => {
    //   block [0x83251CE8..0x83251D28)
	// 83251CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251CF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251CF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251CFC: 388B2FBC  addi r4, r11, 0x2fbc
	ctx.r[4].s64 = ctx.r[11].s64 + 12220;
	// 83251D00: 386A80AC  addi r3, r10, -0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + -32596;
	// 83251D04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D08: 4AFDB1C9  bl 0x8222ced0
	ctx.lr = 0x83251D0C;
	sub_8222CED0(ctx, base);
	// 83251D0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D10: 386997D8  addi r3, r9, -0x6828
	ctx.r[3].s64 = ctx.r[9].s64 + -26664;
	// 83251D14: 4BA5820D  bl 0x82ca9f20
	ctx.lr = 0x83251D18;
	sub_82CA9F20(ctx, base);
	// 83251D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D28 size=64
    let mut pc: u32 = 0x83251D28;
    'dispatch: loop {
        match pc {
            0x83251D28 => {
    //   block [0x83251D28..0x83251D68)
	// 83251D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251D34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251D3C: 388B2FE0  addi r4, r11, 0x2fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 12256;
	// 83251D40: 386A80B0  addi r3, r10, -0x7f50
	ctx.r[3].s64 = ctx.r[10].s64 + -32592;
	// 83251D44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D48: 4AFDB189  bl 0x8222ced0
	ctx.lr = 0x83251D4C;
	sub_8222CED0(ctx, base);
	// 83251D4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D50: 386997E8  addi r3, r9, -0x6818
	ctx.r[3].s64 = ctx.r[9].s64 + -26648;
	// 83251D54: 4BA581CD  bl 0x82ca9f20
	ctx.lr = 0x83251D58;
	sub_82CA9F20(ctx, base);
	// 83251D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D68 size=64
    let mut pc: u32 = 0x83251D68;
    'dispatch: loop {
        match pc {
            0x83251D68 => {
    //   block [0x83251D68..0x83251DA8)
	// 83251D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251D74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251D7C: 388B3000  addi r4, r11, 0x3000
	ctx.r[4].s64 = ctx.r[11].s64 + 12288;
	// 83251D80: 386A80B4  addi r3, r10, -0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32588;
	// 83251D84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D88: 4AFDB149  bl 0x8222ced0
	ctx.lr = 0x83251D8C;
	sub_8222CED0(ctx, base);
	// 83251D8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D90: 386997F8  addi r3, r9, -0x6808
	ctx.r[3].s64 = ctx.r[9].s64 + -26632;
	// 83251D94: 4BA5818D  bl 0x82ca9f20
	ctx.lr = 0x83251D98;
	sub_82CA9F20(ctx, base);
	// 83251D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DA8 size=64
    let mut pc: u32 = 0x83251DA8;
    'dispatch: loop {
        match pc {
            0x83251DA8 => {
    //   block [0x83251DA8..0x83251DE8)
	// 83251DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251DB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251DBC: 388B3028  addi r4, r11, 0x3028
	ctx.r[4].s64 = ctx.r[11].s64 + 12328;
	// 83251DC0: 386A80B8  addi r3, r10, -0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + -32584;
	// 83251DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251DC8: 4AFDB109  bl 0x8222ced0
	ctx.lr = 0x83251DCC;
	sub_8222CED0(ctx, base);
	// 83251DCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251DD0: 38699808  addi r3, r9, -0x67f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26616;
	// 83251DD4: 4BA5814D  bl 0x82ca9f20
	ctx.lr = 0x83251DD8;
	sub_82CA9F20(ctx, base);
	// 83251DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DE8 size=64
    let mut pc: u32 = 0x83251DE8;
    'dispatch: loop {
        match pc {
            0x83251DE8 => {
    //   block [0x83251DE8..0x83251E28)
	// 83251DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251DF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251DFC: 388B3050  addi r4, r11, 0x3050
	ctx.r[4].s64 = ctx.r[11].s64 + 12368;
	// 83251E00: 386A80BC  addi r3, r10, -0x7f44
	ctx.r[3].s64 = ctx.r[10].s64 + -32580;
	// 83251E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E08: 4AFDB0C9  bl 0x8222ced0
	ctx.lr = 0x83251E0C;
	sub_8222CED0(ctx, base);
	// 83251E0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E10: 38699818  addi r3, r9, -0x67e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26600;
	// 83251E14: 4BA5810D  bl 0x82ca9f20
	ctx.lr = 0x83251E18;
	sub_82CA9F20(ctx, base);
	// 83251E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E28 size=64
    let mut pc: u32 = 0x83251E28;
    'dispatch: loop {
        match pc {
            0x83251E28 => {
    //   block [0x83251E28..0x83251E68)
	// 83251E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251E34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251E3C: 388B3078  addi r4, r11, 0x3078
	ctx.r[4].s64 = ctx.r[11].s64 + 12408;
	// 83251E40: 386A80C0  addi r3, r10, -0x7f40
	ctx.r[3].s64 = ctx.r[10].s64 + -32576;
	// 83251E44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E48: 4AFDB089  bl 0x8222ced0
	ctx.lr = 0x83251E4C;
	sub_8222CED0(ctx, base);
	// 83251E4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E50: 38699828  addi r3, r9, -0x67d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26584;
	// 83251E54: 4BA580CD  bl 0x82ca9f20
	ctx.lr = 0x83251E58;
	sub_82CA9F20(ctx, base);
	// 83251E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E68 size=64
    let mut pc: u32 = 0x83251E68;
    'dispatch: loop {
        match pc {
            0x83251E68 => {
    //   block [0x83251E68..0x83251EA8)
	// 83251E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251E74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251E7C: 388B3098  addi r4, r11, 0x3098
	ctx.r[4].s64 = ctx.r[11].s64 + 12440;
	// 83251E80: 386A80C4  addi r3, r10, -0x7f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32572;
	// 83251E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E88: 4AFDB049  bl 0x8222ced0
	ctx.lr = 0x83251E8C;
	sub_8222CED0(ctx, base);
	// 83251E8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E90: 38699838  addi r3, r9, -0x67c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26568;
	// 83251E94: 4BA5808D  bl 0x82ca9f20
	ctx.lr = 0x83251E98;
	sub_82CA9F20(ctx, base);
	// 83251E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EA8 size=64
    let mut pc: u32 = 0x83251EA8;
    'dispatch: loop {
        match pc {
            0x83251EA8 => {
    //   block [0x83251EA8..0x83251EE8)
	// 83251EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251EB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251EB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251EBC: 388B30BC  addi r4, r11, 0x30bc
	ctx.r[4].s64 = ctx.r[11].s64 + 12476;
	// 83251EC0: 386A80C8  addi r3, r10, -0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + -32568;
	// 83251EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251EC8: 4AFDB009  bl 0x8222ced0
	ctx.lr = 0x83251ECC;
	sub_8222CED0(ctx, base);
	// 83251ECC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251ED0: 38699848  addi r3, r9, -0x67b8
	ctx.r[3].s64 = ctx.r[9].s64 + -26552;
	// 83251ED4: 4BA5804D  bl 0x82ca9f20
	ctx.lr = 0x83251ED8;
	sub_82CA9F20(ctx, base);
	// 83251ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EE8 size=64
    let mut pc: u32 = 0x83251EE8;
    'dispatch: loop {
        match pc {
            0x83251EE8 => {
    //   block [0x83251EE8..0x83251F28)
	// 83251EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251EF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251EFC: 388B30D8  addi r4, r11, 0x30d8
	ctx.r[4].s64 = ctx.r[11].s64 + 12504;
	// 83251F00: 386A80CC  addi r3, r10, -0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + -32564;
	// 83251F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F08: 4AFDAFC9  bl 0x8222ced0
	ctx.lr = 0x83251F0C;
	sub_8222CED0(ctx, base);
	// 83251F0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F10: 38699858  addi r3, r9, -0x67a8
	ctx.r[3].s64 = ctx.r[9].s64 + -26536;
	// 83251F14: 4BA5800D  bl 0x82ca9f20
	ctx.lr = 0x83251F18;
	sub_82CA9F20(ctx, base);
	// 83251F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F28 size=64
    let mut pc: u32 = 0x83251F28;
    'dispatch: loop {
        match pc {
            0x83251F28 => {
    //   block [0x83251F28..0x83251F68)
	// 83251F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251F34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251F3C: 388B30F8  addi r4, r11, 0x30f8
	ctx.r[4].s64 = ctx.r[11].s64 + 12536;
	// 83251F40: 386A80D0  addi r3, r10, -0x7f30
	ctx.r[3].s64 = ctx.r[10].s64 + -32560;
	// 83251F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F48: 4AFDAF89  bl 0x8222ced0
	ctx.lr = 0x83251F4C;
	sub_8222CED0(ctx, base);
	// 83251F4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F50: 38699868  addi r3, r9, -0x6798
	ctx.r[3].s64 = ctx.r[9].s64 + -26520;
	// 83251F54: 4BA57FCD  bl 0x82ca9f20
	ctx.lr = 0x83251F58;
	sub_82CA9F20(ctx, base);
	// 83251F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F68 size=64
    let mut pc: u32 = 0x83251F68;
    'dispatch: loop {
        match pc {
            0x83251F68 => {
    //   block [0x83251F68..0x83251FA8)
	// 83251F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251F74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251F7C: 388B3108  addi r4, r11, 0x3108
	ctx.r[4].s64 = ctx.r[11].s64 + 12552;
	// 83251F80: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 83251F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F88: 4AFDAF49  bl 0x8222ced0
	ctx.lr = 0x83251F8C;
	sub_8222CED0(ctx, base);
	// 83251F8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F90: 38699878  addi r3, r9, -0x6788
	ctx.r[3].s64 = ctx.r[9].s64 + -26504;
	// 83251F94: 4BA57F8D  bl 0x82ca9f20
	ctx.lr = 0x83251F98;
	sub_82CA9F20(ctx, base);
	// 83251F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FA8 size=64
    let mut pc: u32 = 0x83251FA8;
    'dispatch: loop {
        match pc {
            0x83251FA8 => {
    //   block [0x83251FA8..0x83251FE8)
	// 83251FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251FB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251FBC: 388B3118  addi r4, r11, 0x3118
	ctx.r[4].s64 = ctx.r[11].s64 + 12568;
	// 83251FC0: 386A80D8  addi r3, r10, -0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + -32552;
	// 83251FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251FC8: 4AFDAF09  bl 0x8222ced0
	ctx.lr = 0x83251FCC;
	sub_8222CED0(ctx, base);
	// 83251FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251FD0: 38699888  addi r3, r9, -0x6778
	ctx.r[3].s64 = ctx.r[9].s64 + -26488;
	// 83251FD4: 4BA57F4D  bl 0x82ca9f20
	ctx.lr = 0x83251FD8;
	sub_82CA9F20(ctx, base);
	// 83251FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FE8 size=64
    let mut pc: u32 = 0x83251FE8;
    'dispatch: loop {
        match pc {
            0x83251FE8 => {
    //   block [0x83251FE8..0x83252028)
	// 83251FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251FF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251FFC: 388B3124  addi r4, r11, 0x3124
	ctx.r[4].s64 = ctx.r[11].s64 + 12580;
	// 83252000: 386A80DC  addi r3, r10, -0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + -32548;
	// 83252004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252008: 4AFDAEC9  bl 0x8222ced0
	ctx.lr = 0x8325200C;
	sub_8222CED0(ctx, base);
	// 8325200C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252010: 38699898  addi r3, r9, -0x6768
	ctx.r[3].s64 = ctx.r[9].s64 + -26472;
	// 83252014: 4BA57F0D  bl 0x82ca9f20
	ctx.lr = 0x83252018;
	sub_82CA9F20(ctx, base);
	// 83252018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252028 size=64
    let mut pc: u32 = 0x83252028;
    'dispatch: loop {
        match pc {
            0x83252028 => {
    //   block [0x83252028..0x83252068)
	// 83252028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252034: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325203C: 388B3138  addi r4, r11, 0x3138
	ctx.r[4].s64 = ctx.r[11].s64 + 12600;
	// 83252040: 386A80E0  addi r3, r10, -0x7f20
	ctx.r[3].s64 = ctx.r[10].s64 + -32544;
	// 83252044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252048: 4AFDAE89  bl 0x8222ced0
	ctx.lr = 0x8325204C;
	sub_8222CED0(ctx, base);
	// 8325204C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252050: 386998A8  addi r3, r9, -0x6758
	ctx.r[3].s64 = ctx.r[9].s64 + -26456;
	// 83252054: 4BA57ECD  bl 0x82ca9f20
	ctx.lr = 0x83252058;
	sub_82CA9F20(ctx, base);
	// 83252058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252068 size=64
    let mut pc: u32 = 0x83252068;
    'dispatch: loop {
        match pc {
            0x83252068 => {
    //   block [0x83252068..0x832520A8)
	// 83252068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252074: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325207C: 388B3154  addi r4, r11, 0x3154
	ctx.r[4].s64 = ctx.r[11].s64 + 12628;
	// 83252080: 386A80E4  addi r3, r10, -0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32540;
	// 83252084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252088: 4AFDAE49  bl 0x8222ced0
	ctx.lr = 0x8325208C;
	sub_8222CED0(ctx, base);
	// 8325208C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252090: 386998B8  addi r3, r9, -0x6748
	ctx.r[3].s64 = ctx.r[9].s64 + -26440;
	// 83252094: 4BA57E8D  bl 0x82ca9f20
	ctx.lr = 0x83252098;
	sub_82CA9F20(ctx, base);
	// 83252098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832520A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832520A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832520A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520A8 size=64
    let mut pc: u32 = 0x832520A8;
    'dispatch: loop {
        match pc {
            0x832520A8 => {
    //   block [0x832520A8..0x832520E8)
	// 832520A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832520B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832520B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832520BC: 388B317C  addi r4, r11, 0x317c
	ctx.r[4].s64 = ctx.r[11].s64 + 12668;
	// 832520C0: 386A80E8  addi r3, r10, -0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + -32536;
	// 832520C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832520C8: 4AFDAE09  bl 0x8222ced0
	ctx.lr = 0x832520CC;
	sub_8222CED0(ctx, base);
	// 832520CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832520D0: 386998C8  addi r3, r9, -0x6738
	ctx.r[3].s64 = ctx.r[9].s64 + -26424;
	// 832520D4: 4BA57E4D  bl 0x82ca9f20
	ctx.lr = 0x832520D8;
	sub_82CA9F20(ctx, base);
	// 832520D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832520DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832520E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832520E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832520E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520E8 size=64
    let mut pc: u32 = 0x832520E8;
    'dispatch: loop {
        match pc {
            0x832520E8 => {
    //   block [0x832520E8..0x83252128)
	// 832520E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832520F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832520F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832520FC: 388B3198  addi r4, r11, 0x3198
	ctx.r[4].s64 = ctx.r[11].s64 + 12696;
	// 83252100: 386A80EC  addi r3, r10, -0x7f14
	ctx.r[3].s64 = ctx.r[10].s64 + -32532;
	// 83252104: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252108: 4AFDADC9  bl 0x8222ced0
	ctx.lr = 0x8325210C;
	sub_8222CED0(ctx, base);
	// 8325210C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252110: 386998D8  addi r3, r9, -0x6728
	ctx.r[3].s64 = ctx.r[9].s64 + -26408;
	// 83252114: 4BA57E0D  bl 0x82ca9f20
	ctx.lr = 0x83252118;
	sub_82CA9F20(ctx, base);
	// 83252118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325211C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252128 size=64
    let mut pc: u32 = 0x83252128;
    'dispatch: loop {
        match pc {
            0x83252128 => {
    //   block [0x83252128..0x83252168)
	// 83252128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325212C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252134: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325213C: 388B31B8  addi r4, r11, 0x31b8
	ctx.r[4].s64 = ctx.r[11].s64 + 12728;
	// 83252140: 386A80F0  addi r3, r10, -0x7f10
	ctx.r[3].s64 = ctx.r[10].s64 + -32528;
	// 83252144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252148: 4AFDAD89  bl 0x8222ced0
	ctx.lr = 0x8325214C;
	sub_8222CED0(ctx, base);
	// 8325214C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252150: 386998E8  addi r3, r9, -0x6718
	ctx.r[3].s64 = ctx.r[9].s64 + -26392;
	// 83252154: 4BA57DCD  bl 0x82ca9f20
	ctx.lr = 0x83252158;
	sub_82CA9F20(ctx, base);
	// 83252158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252168 size=64
    let mut pc: u32 = 0x83252168;
    'dispatch: loop {
        match pc {
            0x83252168 => {
    //   block [0x83252168..0x832521A8)
	// 83252168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252174: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325217C: 388B31DC  addi r4, r11, 0x31dc
	ctx.r[4].s64 = ctx.r[11].s64 + 12764;
	// 83252180: 386A80F4  addi r3, r10, -0x7f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32524;
	// 83252184: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252188: 4AFDAD49  bl 0x8222ced0
	ctx.lr = 0x8325218C;
	sub_8222CED0(ctx, base);
	// 8325218C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252190: 386998F8  addi r3, r9, -0x6708
	ctx.r[3].s64 = ctx.r[9].s64 + -26376;
	// 83252194: 4BA57D8D  bl 0x82ca9f20
	ctx.lr = 0x83252198;
	sub_82CA9F20(ctx, base);
	// 83252198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832521A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832521A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832521A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521A8 size=64
    let mut pc: u32 = 0x832521A8;
    'dispatch: loop {
        match pc {
            0x832521A8 => {
    //   block [0x832521A8..0x832521E8)
	// 832521A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832521B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832521B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832521BC: 388B3204  addi r4, r11, 0x3204
	ctx.r[4].s64 = ctx.r[11].s64 + 12804;
	// 832521C0: 386A80F8  addi r3, r10, -0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + -32520;
	// 832521C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832521C8: 4AFDAD09  bl 0x8222ced0
	ctx.lr = 0x832521CC;
	sub_8222CED0(ctx, base);
	// 832521CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832521D0: 38699908  addi r3, r9, -0x66f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26360;
	// 832521D4: 4BA57D4D  bl 0x82ca9f20
	ctx.lr = 0x832521D8;
	sub_82CA9F20(ctx, base);
	// 832521D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832521DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832521E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832521E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832521E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521E8 size=64
    let mut pc: u32 = 0x832521E8;
    'dispatch: loop {
        match pc {
            0x832521E8 => {
    //   block [0x832521E8..0x83252228)
	// 832521E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832521F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832521F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832521FC: 388B3238  addi r4, r11, 0x3238
	ctx.r[4].s64 = ctx.r[11].s64 + 12856;
	// 83252200: 386A80FC  addi r3, r10, -0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + -32516;
	// 83252204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252208: 4AFDACC9  bl 0x8222ced0
	ctx.lr = 0x8325220C;
	sub_8222CED0(ctx, base);
	// 8325220C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252210: 38699918  addi r3, r9, -0x66e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26344;
	// 83252214: 4BA57D0D  bl 0x82ca9f20
	ctx.lr = 0x83252218;
	sub_82CA9F20(ctx, base);
	// 83252218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325221C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252228 size=64
    let mut pc: u32 = 0x83252228;
    'dispatch: loop {
        match pc {
            0x83252228 => {
    //   block [0x83252228..0x83252268)
	// 83252228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325222C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252234: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325223C: 388B3274  addi r4, r11, 0x3274
	ctx.r[4].s64 = ctx.r[11].s64 + 12916;
	// 83252240: 386A8100  addi r3, r10, -0x7f00
	ctx.r[3].s64 = ctx.r[10].s64 + -32512;
	// 83252244: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252248: 4AFDAC89  bl 0x8222ced0
	ctx.lr = 0x8325224C;
	sub_8222CED0(ctx, base);
	// 8325224C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252250: 38699928  addi r3, r9, -0x66d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26328;
	// 83252254: 4BA57CCD  bl 0x82ca9f20
	ctx.lr = 0x83252258;
	sub_82CA9F20(ctx, base);
	// 83252258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252268 size=64
    let mut pc: u32 = 0x83252268;
    'dispatch: loop {
        match pc {
            0x83252268 => {
    //   block [0x83252268..0x832522A8)
	// 83252268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252274: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325227C: 388B32A8  addi r4, r11, 0x32a8
	ctx.r[4].s64 = ctx.r[11].s64 + 12968;
	// 83252280: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 83252284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252288: 4AFDAC49  bl 0x8222ced0
	ctx.lr = 0x8325228C;
	sub_8222CED0(ctx, base);
	// 8325228C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252290: 38699938  addi r3, r9, -0x66c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26312;
	// 83252294: 4BA57C8D  bl 0x82ca9f20
	ctx.lr = 0x83252298;
	sub_82CA9F20(ctx, base);
	// 83252298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325229C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832522A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832522A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832522A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522A8 size=64
    let mut pc: u32 = 0x832522A8;
    'dispatch: loop {
        match pc {
            0x832522A8 => {
    //   block [0x832522A8..0x832522E8)
	// 832522A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832522B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832522B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832522BC: 388B32E4  addi r4, r11, 0x32e4
	ctx.r[4].s64 = ctx.r[11].s64 + 13028;
	// 832522C0: 386A8108  addi r3, r10, -0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -32504;
	// 832522C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832522C8: 4AFDAC09  bl 0x8222ced0
	ctx.lr = 0x832522CC;
	sub_8222CED0(ctx, base);
	// 832522CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832522D0: 38699948  addi r3, r9, -0x66b8
	ctx.r[3].s64 = ctx.r[9].s64 + -26296;
	// 832522D4: 4BA57C4D  bl 0x82ca9f20
	ctx.lr = 0x832522D8;
	sub_82CA9F20(ctx, base);
	// 832522D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832522DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832522E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832522E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832522E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522E8 size=64
    let mut pc: u32 = 0x832522E8;
    'dispatch: loop {
        match pc {
            0x832522E8 => {
    //   block [0x832522E8..0x83252328)
	// 832522E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832522F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832522F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832522FC: 388B330C  addi r4, r11, 0x330c
	ctx.r[4].s64 = ctx.r[11].s64 + 13068;
	// 83252300: 386A810C  addi r3, r10, -0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -32500;
	// 83252304: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252308: 4AFDABC9  bl 0x8222ced0
	ctx.lr = 0x8325230C;
	sub_8222CED0(ctx, base);
	// 8325230C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252310: 38699958  addi r3, r9, -0x66a8
	ctx.r[3].s64 = ctx.r[9].s64 + -26280;
	// 83252314: 4BA57C0D  bl 0x82ca9f20
	ctx.lr = 0x83252318;
	sub_82CA9F20(ctx, base);
	// 83252318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325231C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252328 size=64
    let mut pc: u32 = 0x83252328;
    'dispatch: loop {
        match pc {
            0x83252328 => {
    //   block [0x83252328..0x83252368)
	// 83252328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325232C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252334: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325233C: 388B3334  addi r4, r11, 0x3334
	ctx.r[4].s64 = ctx.r[11].s64 + 13108;
	// 83252340: 386A8110  addi r3, r10, -0x7ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -32496;
	// 83252344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252348: 4AFDAB89  bl 0x8222ced0
	ctx.lr = 0x8325234C;
	sub_8222CED0(ctx, base);
	// 8325234C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252350: 38699968  addi r3, r9, -0x6698
	ctx.r[3].s64 = ctx.r[9].s64 + -26264;
	// 83252354: 4BA57BCD  bl 0x82ca9f20
	ctx.lr = 0x83252358;
	sub_82CA9F20(ctx, base);
	// 83252358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325235C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252368 size=64
    let mut pc: u32 = 0x83252368;
    'dispatch: loop {
        match pc {
            0x83252368 => {
    //   block [0x83252368..0x832523A8)
	// 83252368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252374: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325237C: 388B335C  addi r4, r11, 0x335c
	ctx.r[4].s64 = ctx.r[11].s64 + 13148;
	// 83252380: 386A8114  addi r3, r10, -0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + -32492;
	// 83252384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252388: 4AFDAB49  bl 0x8222ced0
	ctx.lr = 0x8325238C;
	sub_8222CED0(ctx, base);
	// 8325238C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252390: 38699978  addi r3, r9, -0x6688
	ctx.r[3].s64 = ctx.r[9].s64 + -26248;
	// 83252394: 4BA57B8D  bl 0x82ca9f20
	ctx.lr = 0x83252398;
	sub_82CA9F20(ctx, base);
	// 83252398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832523A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832523A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832523A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832523A8 size=192
    let mut pc: u32 = 0x832523A8;
    'dispatch: loop {
        match pc {
            0x832523A8 => {
    //   block [0x832523A8..0x83252468)
	// 832523A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832523AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832523B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832523B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832523B8: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832523BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832523C0: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 832523C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832523C8: 4AFDAB09  bl 0x8222ced0
	ctx.lr = 0x832523CC;
	sub_8222CED0(ctx, base);
	// 832523CC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832523D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523D4: 4AF9C765  bl 0x821eeb38
	ctx.lr = 0x832523D8;
	sub_821EEB38(ctx, base);
	// 832523D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523DC: 4B9B1415  bl 0x82c037f0
	ctx.lr = 0x832523E0;
	sub_82C037F0(ctx, base);
	// 832523E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832523E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832523E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523EC: 916A8118  stw r11, -0x7ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32488 as u32), ctx.r[11].u32 ) };
	// 832523F0: 4AF74379  bl 0x821c6768
	ctx.lr = 0x832523F4;
	sub_821C6768(ctx, base);
	// 832523F4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832523F8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832523FC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83252400: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83252404: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252408: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325240C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83252410: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83252414: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252418: 4082FFE8  bne 0x83252400
	if !ctx.cr[0].eq {
	pc = 0x83252400; continue 'dispatch;
	}
	// 8325241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83252420: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83252424: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83252428: 4AF74341  bl 0x821c6768
	ctx.lr = 0x8325242C;
	sub_821C6768(ctx, base);
	// 8325242C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83252430: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252434: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83252438: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325243C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83252440: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252444: 4082FFE8  bne 0x8325242c
	if !ctx.cr[0].eq {
	pc = 0x8325242C; continue 'dispatch;
	}
	// 83252448: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325244C: 386B9988  addi r3, r11, -0x6678
	ctx.r[3].s64 = ctx.r[11].s64 + -26232;
	// 83252450: 4BA57AD1  bl 0x82ca9f20
	ctx.lr = 0x83252454;
	sub_82CA9F20(ctx, base);
	// 83252454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83252458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325245C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252468 size=64
    let mut pc: u32 = 0x83252468;
    'dispatch: loop {
        match pc {
            0x83252468 => {
    //   block [0x83252468..0x832524A8)
	// 83252468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325246C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252474: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325247C: 388B33AC  addi r4, r11, 0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + 13228;
	// 83252480: 386A811C  addi r3, r10, -0x7ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -32484;
	// 83252484: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252488: 4AFDAA49  bl 0x8222ced0
	ctx.lr = 0x8325248C;
	sub_8222CED0(ctx, base);
	// 8325248C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252490: 38699990  addi r3, r9, -0x6670
	ctx.r[3].s64 = ctx.r[9].s64 + -26224;
	// 83252494: 4BA57A8D  bl 0x82ca9f20
	ctx.lr = 0x83252498;
	sub_82CA9F20(ctx, base);
	// 83252498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325249C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832524A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832524A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832524A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524A8 size=64
    let mut pc: u32 = 0x832524A8;
    'dispatch: loop {
        match pc {
            0x832524A8 => {
    //   block [0x832524A8..0x832524E8)
	// 832524A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832524B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832524B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832524BC: 388B33D0  addi r4, r11, 0x33d0
	ctx.r[4].s64 = ctx.r[11].s64 + 13264;
	// 832524C0: 386A8120  addi r3, r10, -0x7ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -32480;
	// 832524C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832524C8: 4AFDAA09  bl 0x8222ced0
	ctx.lr = 0x832524CC;
	sub_8222CED0(ctx, base);
	// 832524CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832524D0: 386999A0  addi r3, r9, -0x6660
	ctx.r[3].s64 = ctx.r[9].s64 + -26208;
	// 832524D4: 4BA57A4D  bl 0x82ca9f20
	ctx.lr = 0x832524D8;
	sub_82CA9F20(ctx, base);
	// 832524D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832524DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832524E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832524E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832524E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524E8 size=64
    let mut pc: u32 = 0x832524E8;
    'dispatch: loop {
        match pc {
            0x832524E8 => {
    //   block [0x832524E8..0x83252528)
	// 832524E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832524F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832524F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832524FC: 388B33E8  addi r4, r11, 0x33e8
	ctx.r[4].s64 = ctx.r[11].s64 + 13288;
	// 83252500: 386A8124  addi r3, r10, -0x7edc
	ctx.r[3].s64 = ctx.r[10].s64 + -32476;
	// 83252504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252508: 4AFDA9C9  bl 0x8222ced0
	ctx.lr = 0x8325250C;
	sub_8222CED0(ctx, base);
	// 8325250C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252510: 386999B0  addi r3, r9, -0x6650
	ctx.r[3].s64 = ctx.r[9].s64 + -26192;
	// 83252514: 4BA57A0D  bl 0x82ca9f20
	ctx.lr = 0x83252518;
	sub_82CA9F20(ctx, base);
	// 83252518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325251C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252528 size=64
    let mut pc: u32 = 0x83252528;
    'dispatch: loop {
        match pc {
            0x83252528 => {
    //   block [0x83252528..0x83252568)
	// 83252528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252534: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325253C: 388B3404  addi r4, r11, 0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + 13316;
	// 83252540: 386A8128  addi r3, r10, -0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -32472;
	// 83252544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252548: 4AFDA989  bl 0x8222ced0
	ctx.lr = 0x8325254C;
	sub_8222CED0(ctx, base);
	// 8325254C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252550: 386999C0  addi r3, r9, -0x6640
	ctx.r[3].s64 = ctx.r[9].s64 + -26176;
	// 83252554: 4BA579CD  bl 0x82ca9f20
	ctx.lr = 0x83252558;
	sub_82CA9F20(ctx, base);
	// 83252558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325255C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252568 size=64
    let mut pc: u32 = 0x83252568;
    'dispatch: loop {
        match pc {
            0x83252568 => {
    //   block [0x83252568..0x832525A8)
	// 83252568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252574: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325257C: 388B3410  addi r4, r11, 0x3410
	ctx.r[4].s64 = ctx.r[11].s64 + 13328;
	// 83252580: 386A812C  addi r3, r10, -0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -32468;
	// 83252584: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252588: 4AFDA949  bl 0x8222ced0
	ctx.lr = 0x8325258C;
	sub_8222CED0(ctx, base);
	// 8325258C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252590: 386999D0  addi r3, r9, -0x6630
	ctx.r[3].s64 = ctx.r[9].s64 + -26160;
	// 83252594: 4BA5798D  bl 0x82ca9f20
	ctx.lr = 0x83252598;
	sub_82CA9F20(ctx, base);
	// 83252598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325259C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832525A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832525A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832525A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832525A8 size=196
    let mut pc: u32 = 0x832525A8;
    'dispatch: loop {
        match pc {
            0x832525A8 => {
    //   block [0x832525A8..0x8325266C)
	// 832525A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832525AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832525B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832525B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832525B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832525BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832525C0: 3BEB8130  addi r31, r11, -0x7ed0
	ctx.r[31].s64 = ctx.r[11].s64 + -32464;
	// 832525C4: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 832525C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832525CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525D0: 4AFDA901  bl 0x8222ced0
	ctx.lr = 0x832525D4;
	sub_8222CED0(ctx, base);
	// 832525D4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832525D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525DC: 388934F8  addi r4, r9, 0x34f8
	ctx.r[4].s64 = ctx.r[9].s64 + 13560;
	// 832525E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832525E4: 4AFDA8ED  bl 0x8222ced0
	ctx.lr = 0x832525E8;
	sub_8222CED0(ctx, base);
	// 832525E8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832525EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525F0: 388834D0  addi r4, r8, 0x34d0
	ctx.r[4].s64 = ctx.r[8].s64 + 13520;
	// 832525F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832525F8: 4AFDA8D9  bl 0x8222ced0
	ctx.lr = 0x832525FC;
	sub_8222CED0(ctx, base);
	// 832525FC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83252600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252604: 388734A8  addi r4, r7, 0x34a8
	ctx.r[4].s64 = ctx.r[7].s64 + 13480;
	// 83252608: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325260C: 4AFDA8C5  bl 0x8222ced0
	ctx.lr = 0x83252610;
	sub_8222CED0(ctx, base);
	// 83252610: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83252614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252618: 38863480  addi r4, r6, 0x3480
	ctx.r[4].s64 = ctx.r[6].s64 + 13440;
	// 8325261C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83252620: 4AFDA8B1  bl 0x8222ced0
	ctx.lr = 0x83252624;
	sub_8222CED0(ctx, base);
	// 83252624: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83252628: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325262C: 38843458  addi r4, r4, 0x3458
	ctx.r[4].s64 = ctx.r[4].s64 + 13400;
	// 83252630: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83252634: 4AFDA89D  bl 0x8222ced0
	ctx.lr = 0x83252638;
	sub_8222CED0(ctx, base);
	// 83252638: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8325263C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252640: 38833430  addi r4, r3, 0x3430
	ctx.r[4].s64 = ctx.r[3].s64 + 13360;
	// 83252644: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252648: 4AFDA889  bl 0x8222ced0
	ctx.lr = 0x8325264C;
	sub_8222CED0(ctx, base);
	// 8325264C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252650: 386B99E0  addi r3, r11, -0x6620
	ctx.r[3].s64 = ctx.r[11].s64 + -26144;
	// 83252654: 4BA578CD  bl 0x82ca9f20
	ctx.lr = 0x83252658;
	sub_82CA9F20(ctx, base);
	// 83252658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252670 size=196
    let mut pc: u32 = 0x83252670;
    'dispatch: loop {
        match pc {
            0x83252670 => {
    //   block [0x83252670..0x83252734)
	// 83252670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83252684: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83252688: 3BEB814C  addi r31, r11, -0x7eb4
	ctx.r[31].s64 = ctx.r[11].s64 + -32436;
	// 8325268C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83252690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83252694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252698: 4AFDA839  bl 0x8222ced0
	ctx.lr = 0x8325269C;
	sub_8222CED0(ctx, base);
	// 8325269C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832526A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526A4: 38893598  addi r4, r9, 0x3598
	ctx.r[4].s64 = ctx.r[9].s64 + 13720;
	// 832526A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832526AC: 4AFDA825  bl 0x8222ced0
	ctx.lr = 0x832526B0;
	sub_8222CED0(ctx, base);
	// 832526B0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832526B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526B8: 38883580  addi r4, r8, 0x3580
	ctx.r[4].s64 = ctx.r[8].s64 + 13696;
	// 832526BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832526C0: 4AFDA811  bl 0x8222ced0
	ctx.lr = 0x832526C4;
	sub_8222CED0(ctx, base);
	// 832526C4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832526C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526CC: 38873568  addi r4, r7, 0x3568
	ctx.r[4].s64 = ctx.r[7].s64 + 13672;
	// 832526D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832526D4: 4AFDA7FD  bl 0x8222ced0
	ctx.lr = 0x832526D8;
	sub_8222CED0(ctx, base);
	// 832526D8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832526DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526E0: 38863550  addi r4, r6, 0x3550
	ctx.r[4].s64 = ctx.r[6].s64 + 13648;
	// 832526E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832526E8: 4AFDA7E9  bl 0x8222ced0
	ctx.lr = 0x832526EC;
	sub_8222CED0(ctx, base);
	// 832526EC: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832526F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526F4: 38843538  addi r4, r4, 0x3538
	ctx.r[4].s64 = ctx.r[4].s64 + 13624;
	// 832526F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832526FC: 4AFDA7D5  bl 0x8222ced0
	ctx.lr = 0x83252700;
	sub_8222CED0(ctx, base);
	// 83252700: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83252704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252708: 38833520  addi r4, r3, 0x3520
	ctx.r[4].s64 = ctx.r[3].s64 + 13600;
	// 8325270C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252710: 4AFDA7C1  bl 0x8222ced0
	ctx.lr = 0x83252714;
	sub_8222CED0(ctx, base);
	// 83252714: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252718: 386B9A48  addi r3, r11, -0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26040;
	// 8325271C: 4BA57805  bl 0x82ca9f20
	ctx.lr = 0x83252720;
	sub_82CA9F20(ctx, base);
	// 83252720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325272C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252738 size=108
    let mut pc: u32 = 0x83252738;
    'dispatch: loop {
        match pc {
            0x83252738 => {
    //   block [0x83252738..0x832527A4)
	// 83252738: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325273C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252740: 392B92CC  addi r9, r11, -0x6d34
	ctx.r[9].s64 = ctx.r[11].s64 + -27956;
	// 83252744: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83252748: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8325274C: C00B92CC  lfs f0, -0x6d34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27956 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252750: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83252754: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252758: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 8325275C: C00901B8  lfs f0, 0x1b8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(440 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252760: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83252764: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832527A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832527A8 size=100
    let mut pc: u32 = 0x832527A8;
    'dispatch: loop {
        match pc {
            0x832527A8 => {
    //   block [0x832527A8..0x8325280C)
	// 832527A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832527AC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832527B0: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832527B4: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 832527B8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 832527BC: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832527C0: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832527C4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832527C8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832527CC: C1A901B0  lfs f13, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832527D0: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832527D4: 38858180  addi r4, r5, -0x7e80
	ctx.r[4].s64 = ctx.r[5].s64 + -32384;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252810 size=96
    let mut pc: u32 = 0x83252810;
    'dispatch: loop {
        match pc {
            0x83252810 => {
    //   block [0x83252810..0x83252870)
	// 83252810: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83252814: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83252818: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8325281C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83252820: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83252824: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83252828: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325282C: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252830: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252870 size=112
    let mut pc: u32 = 0x83252870;
    'dispatch: loop {
        match pc {
            0x83252870 => {
    //   block [0x83252870..0x832528E0)
	// 83252870: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252874: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83252878: 392BB480  addi r9, r11, -0x4b80
	ctx.r[9].s64 = ctx.r[11].s64 + -19328;
	// 8325287C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252880: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83252884: C00BB480  lfs f0, -0x4b80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252888: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 8325288C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252890: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832528E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832528E0 size=108
    let mut pc: u32 = 0x832528E0;
    'dispatch: loop {
        match pc {
            0x832528E0 => {
    //   block [0x832528E0..0x8325294C)
	// 832528E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832528E4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832528E8: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832528EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832528F0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832528F4: 38C1FFF4  addi r6, r1, -0xc
	ctx.r[6].s64 = ctx.r[1].s64 + -12;
	// 832528F8: C1AB92D4  lfs f13, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832528FC: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 83252900: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252904: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83252908: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252950 size=108
    let mut pc: u32 = 0x83252950;
    'dispatch: loop {
        match pc {
            0x83252950 => {
    //   block [0x83252950..0x832529BC)
	// 83252950: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252954: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252958: 392BE0DC  addi r9, r11, -0x1f24
	ctx.r[9].s64 = ctx.r[11].s64 + -7972;
	// 8325295C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83252960: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83252964: C00BE0DC  lfs f0, -0x1f24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252968: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325296C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252970: 38A1FFF8  addi r5, r1, -8
	ctx.r[5].s64 = ctx.r[1].s64 + -8;
	// 83252974: C009B3B4  lfs f0, -0x4c4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19532 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252978: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 8325297C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832529C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832529C0 size=64
    let mut pc: u32 = 0x832529C0;
    'dispatch: loop {
        match pc {
            0x832529C0 => {
    //   block [0x832529C0..0x83252A00)
	// 832529C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832529C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832529C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832529CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832529D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832529D4: 388B35B0  addi r4, r11, 0x35b0
	ctx.r[4].s64 = ctx.r[11].s64 + 13744;
	// 832529D8: 386A8168  addi r3, r10, -0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + -32408;
	// 832529DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832529E0: 4AFDA4F1  bl 0x8222ced0
	ctx.lr = 0x832529E4;
	sub_8222CED0(ctx, base);
	// 832529E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832529E8: 38699AB0  addi r3, r9, -0x6550
	ctx.r[3].s64 = ctx.r[9].s64 + -25936;
	// 832529EC: 4BA57535  bl 0x82ca9f20
	ctx.lr = 0x832529F0;
	sub_82CA9F20(ctx, base);
	// 832529F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832529F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832529F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832529FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A00 size=64
    let mut pc: u32 = 0x83252A00;
    'dispatch: loop {
        match pc {
            0x83252A00 => {
    //   block [0x83252A00..0x83252A40)
	// 83252A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A14: 388B35D4  addi r4, r11, 0x35d4
	ctx.r[4].s64 = ctx.r[11].s64 + 13780;
	// 83252A18: 386A816C  addi r3, r10, -0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + -32404;
	// 83252A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252A20: 4AFDA4B1  bl 0x8222ced0
	ctx.lr = 0x83252A24;
	sub_8222CED0(ctx, base);
	// 83252A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252A28: 38699AC0  addi r3, r9, -0x6540
	ctx.r[3].s64 = ctx.r[9].s64 + -25920;
	// 83252A2C: 4BA574F5  bl 0x82ca9f20
	ctx.lr = 0x83252A30;
	sub_82CA9F20(ctx, base);
	// 83252A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A40 size=64
    let mut pc: u32 = 0x83252A40;
    'dispatch: loop {
        match pc {
            0x83252A40 => {
    //   block [0x83252A40..0x83252A80)
	// 83252A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A54: 388B35F8  addi r4, r11, 0x35f8
	ctx.r[4].s64 = ctx.r[11].s64 + 13816;
	// 83252A58: 386A81D0  addi r3, r10, -0x7e30
	ctx.r[3].s64 = ctx.r[10].s64 + -32304;
	// 83252A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252A60: 4AFDA471  bl 0x8222ced0
	ctx.lr = 0x83252A64;
	sub_8222CED0(ctx, base);
	// 83252A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252A68: 38699AD0  addi r3, r9, -0x6530
	ctx.r[3].s64 = ctx.r[9].s64 + -25904;
	// 83252A6C: 4BA574B5  bl 0x82ca9f20
	ctx.lr = 0x83252A70;
	sub_82CA9F20(ctx, base);
	// 83252A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A80 size=64
    let mut pc: u32 = 0x83252A80;
    'dispatch: loop {
        match pc {
            0x83252A80 => {
    //   block [0x83252A80..0x83252AC0)
	// 83252A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83252A98: 386A81D4  addi r3, r10, -0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32300;
	// 83252A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252AA0: 4AFDA431  bl 0x8222ced0
	ctx.lr = 0x83252AA4;
	sub_8222CED0(ctx, base);
	// 83252AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252AA8: 38699AE0  addi r3, r9, -0x6520
	ctx.r[3].s64 = ctx.r[9].s64 + -25888;
	// 83252AAC: 4BA57475  bl 0x82ca9f20
	ctx.lr = 0x83252AB0;
	sub_82CA9F20(ctx, base);
	// 83252AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252AC0 size=64
    let mut pc: u32 = 0x83252AC0;
    'dispatch: loop {
        match pc {
            0x83252AC0 => {
    //   block [0x83252AC0..0x83252B00)
	// 83252AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252AD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83252AD8: 386A81D8  addi r3, r10, -0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + -32296;
	// 83252ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252AE0: 4AFDA3F1  bl 0x8222ced0
	ctx.lr = 0x83252AE4;
	sub_8222CED0(ctx, base);
	// 83252AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252AE8: 38699AF0  addi r3, r9, -0x6510
	ctx.r[3].s64 = ctx.r[9].s64 + -25872;
	// 83252AEC: 4BA57435  bl 0x82ca9f20
	ctx.lr = 0x83252AF0;
	sub_82CA9F20(ctx, base);
	// 83252AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B00 size=64
    let mut pc: u32 = 0x83252B00;
    'dispatch: loop {
        match pc {
            0x83252B00 => {
    //   block [0x83252B00..0x83252B40)
	// 83252B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83252B18: 386A81DC  addi r3, r10, -0x7e24
	ctx.r[3].s64 = ctx.r[10].s64 + -32292;
	// 83252B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252B20: 4AFDA3B1  bl 0x8222ced0
	ctx.lr = 0x83252B24;
	sub_8222CED0(ctx, base);
	// 83252B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252B28: 38699B00  addi r3, r9, -0x6500
	ctx.r[3].s64 = ctx.r[9].s64 + -25856;
	// 83252B2C: 4BA573F5  bl 0x82ca9f20
	ctx.lr = 0x83252B30;
	sub_82CA9F20(ctx, base);
	// 83252B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B40 size=64
    let mut pc: u32 = 0x83252B40;
    'dispatch: loop {
        match pc {
            0x83252B40 => {
    //   block [0x83252B40..0x83252B80)
	// 83252B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B54: 388B3F60  addi r4, r11, 0x3f60
	ctx.r[4].s64 = ctx.r[11].s64 + 16224;
	// 83252B58: 386A81E0  addi r3, r10, -0x7e20
	ctx.r[3].s64 = ctx.r[10].s64 + -32288;
	// 83252B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252B60: 4AFDA371  bl 0x8222ced0
	ctx.lr = 0x83252B64;
	sub_8222CED0(ctx, base);
	// 83252B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252B68: 38699B10  addi r3, r9, -0x64f0
	ctx.r[3].s64 = ctx.r[9].s64 + -25840;
	// 83252B6C: 4BA573B5  bl 0x82ca9f20
	ctx.lr = 0x83252B70;
	sub_82CA9F20(ctx, base);
	// 83252B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B80 size=64
    let mut pc: u32 = 0x83252B80;
    'dispatch: loop {
        match pc {
            0x83252B80 => {
    //   block [0x83252B80..0x83252BC0)
	// 83252B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B94: 388B3F68  addi r4, r11, 0x3f68
	ctx.r[4].s64 = ctx.r[11].s64 + 16232;
	// 83252B98: 386A81E4  addi r3, r10, -0x7e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32284;
	// 83252B9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252BA0: 4AFDA331  bl 0x8222ced0
	ctx.lr = 0x83252BA4;
	sub_8222CED0(ctx, base);
	// 83252BA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252BA8: 38699B20  addi r3, r9, -0x64e0
	ctx.r[3].s64 = ctx.r[9].s64 + -25824;
	// 83252BAC: 4BA57375  bl 0x82ca9f20
	ctx.lr = 0x83252BB0;
	sub_82CA9F20(ctx, base);
	// 83252BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252BC0 size=64
    let mut pc: u32 = 0x83252BC0;
    'dispatch: loop {
        match pc {
            0x83252BC0 => {
    //   block [0x83252BC0..0x83252C00)
	// 83252BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252BCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252BD4: 388B3F74  addi r4, r11, 0x3f74
	ctx.r[4].s64 = ctx.r[11].s64 + 16244;
	// 83252BD8: 386A81E8  addi r3, r10, -0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + -32280;
	// 83252BDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252BE0: 4AFDA2F1  bl 0x8222ced0
	ctx.lr = 0x83252BE4;
	sub_8222CED0(ctx, base);
	// 83252BE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252BE8: 38699B30  addi r3, r9, -0x64d0
	ctx.r[3].s64 = ctx.r[9].s64 + -25808;
	// 83252BEC: 4BA57335  bl 0x82ca9f20
	ctx.lr = 0x83252BF0;
	sub_82CA9F20(ctx, base);
	// 83252BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C00 size=64
    let mut pc: u32 = 0x83252C00;
    'dispatch: loop {
        match pc {
            0x83252C00 => {
    //   block [0x83252C00..0x83252C40)
	// 83252C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C14: 388B3F80  addi r4, r11, 0x3f80
	ctx.r[4].s64 = ctx.r[11].s64 + 16256;
	// 83252C18: 386A81EC  addi r3, r10, -0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + -32276;
	// 83252C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252C20: 4AFDA2B1  bl 0x8222ced0
	ctx.lr = 0x83252C24;
	sub_8222CED0(ctx, base);
	// 83252C24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252C28: 38699B40  addi r3, r9, -0x64c0
	ctx.r[3].s64 = ctx.r[9].s64 + -25792;
	// 83252C2C: 4BA572F5  bl 0x82ca9f20
	ctx.lr = 0x83252C30;
	sub_82CA9F20(ctx, base);
	// 83252C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C40 size=64
    let mut pc: u32 = 0x83252C40;
    'dispatch: loop {
        match pc {
            0x83252C40 => {
    //   block [0x83252C40..0x83252C80)
	// 83252C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C54: 388B3F90  addi r4, r11, 0x3f90
	ctx.r[4].s64 = ctx.r[11].s64 + 16272;
	// 83252C58: 386A81F0  addi r3, r10, -0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + -32272;
	// 83252C5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252C60: 4AFDA271  bl 0x8222ced0
	ctx.lr = 0x83252C64;
	sub_8222CED0(ctx, base);
	// 83252C64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252C68: 38699B50  addi r3, r9, -0x64b0
	ctx.r[3].s64 = ctx.r[9].s64 + -25776;
	// 83252C6C: 4BA572B5  bl 0x82ca9f20
	ctx.lr = 0x83252C70;
	sub_82CA9F20(ctx, base);
	// 83252C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C80 size=64
    let mut pc: u32 = 0x83252C80;
    'dispatch: loop {
        match pc {
            0x83252C80 => {
    //   block [0x83252C80..0x83252CC0)
	// 83252C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C94: 388B3F9C  addi r4, r11, 0x3f9c
	ctx.r[4].s64 = ctx.r[11].s64 + 16284;
	// 83252C98: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 83252C9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252CA0: 4AFDA231  bl 0x8222ced0
	ctx.lr = 0x83252CA4;
	sub_8222CED0(ctx, base);
	// 83252CA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252CA8: 38699B60  addi r3, r9, -0x64a0
	ctx.r[3].s64 = ctx.r[9].s64 + -25760;
	// 83252CAC: 4BA57275  bl 0x82ca9f20
	ctx.lr = 0x83252CB0;
	sub_82CA9F20(ctx, base);
	// 83252CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252CC0 size=64
    let mut pc: u32 = 0x83252CC0;
    'dispatch: loop {
        match pc {
            0x83252CC0 => {
    //   block [0x83252CC0..0x83252D00)
	// 83252CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252CCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252CD4: 388B3FAC  addi r4, r11, 0x3fac
	ctx.r[4].s64 = ctx.r[11].s64 + 16300;
	// 83252CD8: 386A81F8  addi r3, r10, -0x7e08
	ctx.r[3].s64 = ctx.r[10].s64 + -32264;
	// 83252CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252CE0: 4AFDA1F1  bl 0x8222ced0
	ctx.lr = 0x83252CE4;
	sub_8222CED0(ctx, base);
	// 83252CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252CE8: 38699B70  addi r3, r9, -0x6490
	ctx.r[3].s64 = ctx.r[9].s64 + -25744;
	// 83252CEC: 4BA57235  bl 0x82ca9f20
	ctx.lr = 0x83252CF0;
	sub_82CA9F20(ctx, base);
	// 83252CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D00 size=56
    let mut pc: u32 = 0x83252D00;
    'dispatch: loop {
        match pc {
            0x83252D00 => {
    //   block [0x83252D00..0x83252D38)
	// 83252D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83252D14: 386B3FB4  addi r3, r11, 0x3fb4
	ctx.r[3].s64 = ctx.r[11].s64 + 16308;
	// 83252D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83252D1C: 4AFA103D  bl 0x821f3d58
	ctx.lr = 0x83252D20;
	sub_821F3D58(ctx, base);
	// 83252D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D24: 906A81FC  stw r3, -0x7e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32260 as u32), ctx.r[3].u32 ) };
	// 83252D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D38 size=64
    let mut pc: u32 = 0x83252D38;
    'dispatch: loop {
        match pc {
            0x83252D38 => {
    //   block [0x83252D38..0x83252D78)
	// 83252D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D4C: 388B3FD0  addi r4, r11, 0x3fd0
	ctx.r[4].s64 = ctx.r[11].s64 + 16336;
	// 83252D50: 386A8200  addi r3, r10, -0x7e00
	ctx.r[3].s64 = ctx.r[10].s64 + -32256;
	// 83252D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252D58: 4AFDA179  bl 0x8222ced0
	ctx.lr = 0x83252D5C;
	sub_8222CED0(ctx, base);
	// 83252D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252D60: 38699B80  addi r3, r9, -0x6480
	ctx.r[3].s64 = ctx.r[9].s64 + -25728;
	// 83252D64: 4BA571BD  bl 0x82ca9f20
	ctx.lr = 0x83252D68;
	sub_82CA9F20(ctx, base);
	// 83252D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D78 size=64
    let mut pc: u32 = 0x83252D78;
    'dispatch: loop {
        match pc {
            0x83252D78 => {
    //   block [0x83252D78..0x83252DB8)
	// 83252D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D8C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83252D90: 386A8204  addi r3, r10, -0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -32252;
	// 83252D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252D98: 4AFDA139  bl 0x8222ced0
	ctx.lr = 0x83252D9C;
	sub_8222CED0(ctx, base);
	// 83252D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252DA0: 38699B90  addi r3, r9, -0x6470
	ctx.r[3].s64 = ctx.r[9].s64 + -25712;
	// 83252DA4: 4BA5717D  bl 0x82ca9f20
	ctx.lr = 0x83252DA8;
	sub_82CA9F20(ctx, base);
	// 83252DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DB8 size=64
    let mut pc: u32 = 0x83252DB8;
    'dispatch: loop {
        match pc {
            0x83252DB8 => {
    //   block [0x83252DB8..0x83252DF8)
	// 83252DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252DCC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83252DD0: 386A8208  addi r3, r10, -0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + -32248;
	// 83252DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252DD8: 4AFDA0F9  bl 0x8222ced0
	ctx.lr = 0x83252DDC;
	sub_8222CED0(ctx, base);
	// 83252DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252DE0: 38699BA0  addi r3, r9, -0x6460
	ctx.r[3].s64 = ctx.r[9].s64 + -25696;
	// 83252DE4: 4BA5713D  bl 0x82ca9f20
	ctx.lr = 0x83252DE8;
	sub_82CA9F20(ctx, base);
	// 83252DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DF8 size=64
    let mut pc: u32 = 0x83252DF8;
    'dispatch: loop {
        match pc {
            0x83252DF8 => {
    //   block [0x83252DF8..0x83252E38)
	// 83252DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252E0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83252E10: 386A820C  addi r3, r10, -0x7df4
	ctx.r[3].s64 = ctx.r[10].s64 + -32244;
	// 83252E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252E18: 4AFDA0B9  bl 0x8222ced0
	ctx.lr = 0x83252E1C;
	sub_8222CED0(ctx, base);
	// 83252E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252E20: 38699BB0  addi r3, r9, -0x6450
	ctx.r[3].s64 = ctx.r[9].s64 + -25680;
	// 83252E24: 4BA570FD  bl 0x82ca9f20
	ctx.lr = 0x83252E28;
	sub_82CA9F20(ctx, base);
	// 83252E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E38 size=64
    let mut pc: u32 = 0x83252E38;
    'dispatch: loop {
        match pc {
            0x83252E38 => {
    //   block [0x83252E38..0x83252E78)
	// 83252E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252E4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83252E50: 386A8210  addi r3, r10, -0x7df0
	ctx.r[3].s64 = ctx.r[10].s64 + -32240;
	// 83252E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252E58: 4AFDA079  bl 0x8222ced0
	ctx.lr = 0x83252E5C;
	sub_8222CED0(ctx, base);
	// 83252E5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252E60: 38699BC0  addi r3, r9, -0x6440
	ctx.r[3].s64 = ctx.r[9].s64 + -25664;
	// 83252E64: 4BA570BD  bl 0x82ca9f20
	ctx.lr = 0x83252E68;
	sub_82CA9F20(ctx, base);
	// 83252E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E78 size=144
    let mut pc: u32 = 0x83252E78;
    'dispatch: loop {
        match pc {
            0x83252E78 => {
    //   block [0x83252E78..0x83252F08)
	// 83252E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E84: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83252E88: 4AFCC3D1  bl 0x8221f258
	ctx.lr = 0x83252E8C;
	sub_8221F258(ctx, base);
	// 83252E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252E94: 419A0008  beq cr6, 0x83252e9c
	if ctx.cr[6].eq {
	pc = 0x83252E9C; continue 'dispatch;
	}
	// 83252E98: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252E9C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EA0: 41820008  beq 0x83252ea8
	if ctx.cr[0].eq {
	pc = 0x83252EA8; continue 'dispatch;
	}
	// 83252EA4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252EA8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EAC: 41820008  beq 0x83252eb4
	if ctx.cr[0].eq {
	pc = 0x83252EB4; continue 'dispatch;
	}
	// 83252EB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252EB4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252EB8: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 83252EBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252EC0: 39098214  addi r8, r9, -0x7dec
	ctx.r[8].s64 = ctx.r[9].s64 + -32236;
	// 83252EC4: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83252EC8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252ECC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252ED0: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83252ED4: 38679BD0  addi r3, r7, -0x6430
	ctx.r[3].s64 = ctx.r[7].s64 + -25648;
	// 83252ED8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EDC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252EE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EE4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252EE8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EEC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252EF0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252EF4: 4BA5702D  bl 0x82ca9f20
	ctx.lr = 0x83252EF8;
	sub_82CA9F20(ctx, base);
	// 83252EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F08 size=144
    let mut pc: u32 = 0x83252F08;
    'dispatch: loop {
        match pc {
            0x83252F08 => {
    //   block [0x83252F08..0x83252F98)
	// 83252F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252F14: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83252F18: 4AFCC341  bl 0x8221f258
	ctx.lr = 0x83252F1C;
	sub_8221F258(ctx, base);
	// 83252F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252F24: 419A0008  beq cr6, 0x83252f2c
	if ctx.cr[6].eq {
	pc = 0x83252F2C; continue 'dispatch;
	}
	// 83252F28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F2C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F30: 41820008  beq 0x83252f38
	if ctx.cr[0].eq {
	pc = 0x83252F38; continue 'dispatch;
	}
	// 83252F34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F38: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F3C: 41820008  beq 0x83252f44
	if ctx.cr[0].eq {
	pc = 0x83252F44; continue 'dispatch;
	}
	// 83252F40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83252F44: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252F48: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83252F4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252F50: 39098220  addi r8, r9, -0x7de0
	ctx.r[8].s64 = ctx.r[9].s64 + -32224;
	// 83252F54: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83252F58: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252F5C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252F60: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83252F64: 38679BE0  addi r3, r7, -0x6420
	ctx.r[3].s64 = ctx.r[7].s64 + -25632;
	// 83252F68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F6C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252F70: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F74: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252F78: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F7C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252F80: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252F84: 4BA56F9D  bl 0x82ca9f20
	ctx.lr = 0x83252F88;
	sub_82CA9F20(ctx, base);
	// 83252F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F98 size=64
    let mut pc: u32 = 0x83252F98;
    'dispatch: loop {
        match pc {
            0x83252F98 => {
    //   block [0x83252F98..0x83252FD8)
	// 83252F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252FA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252FA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252FAC: 388B6F20  addi r4, r11, 0x6f20
	ctx.r[4].s64 = ctx.r[11].s64 + 28448;
	// 83252FB0: 386A822C  addi r3, r10, -0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -32212;
	// 83252FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252FB8: 4AFD9F19  bl 0x8222ced0
	ctx.lr = 0x83252FBC;
	sub_8222CED0(ctx, base);
	// 83252FBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252FC0: 38699BF0  addi r3, r9, -0x6410
	ctx.r[3].s64 = ctx.r[9].s64 + -25616;
	// 83252FC4: 4BA56F5D  bl 0x82ca9f20
	ctx.lr = 0x83252FC8;
	sub_82CA9F20(ctx, base);
	// 83252FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252FD8 size=64
    let mut pc: u32 = 0x83252FD8;
    'dispatch: loop {
        match pc {
            0x83252FD8 => {
    //   block [0x83252FD8..0x83253018)
	// 83252FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252FE4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252FE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252FEC: 388B6F48  addi r4, r11, 0x6f48
	ctx.r[4].s64 = ctx.r[11].s64 + 28488;
	// 83252FF0: 386A8230  addi r3, r10, -0x7dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -32208;
	// 83252FF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252FF8: 4AFD9ED9  bl 0x8222ced0
	ctx.lr = 0x83252FFC;
	sub_8222CED0(ctx, base);
	// 83252FFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253000: 38699C00  addi r3, r9, -0x6400
	ctx.r[3].s64 = ctx.r[9].s64 + -25600;
	// 83253004: 4BA56F1D  bl 0x82ca9f20
	ctx.lr = 0x83253008;
	sub_82CA9F20(ctx, base);
	// 83253008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325300C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253018 size=64
    let mut pc: u32 = 0x83253018;
    'dispatch: loop {
        match pc {
            0x83253018 => {
    //   block [0x83253018..0x83253058)
	// 83253018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253024: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83253028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325302C: 388B6F64  addi r4, r11, 0x6f64
	ctx.r[4].s64 = ctx.r[11].s64 + 28516;
	// 83253030: 386A8234  addi r3, r10, -0x7dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -32204;
	// 83253034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253038: 4AFD9E99  bl 0x8222ced0
	ctx.lr = 0x8325303C;
	sub_8222CED0(ctx, base);
	// 8325303C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253040: 38699C10  addi r3, r9, -0x63f0
	ctx.r[3].s64 = ctx.r[9].s64 + -25584;
	// 83253044: 4BA56EDD  bl 0x82ca9f20
	ctx.lr = 0x83253048;
	sub_82CA9F20(ctx, base);
	// 83253048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253058 size=56
    let mut pc: u32 = 0x83253058;
    'dispatch: loop {
        match pc {
            0x83253058 => {
    //   block [0x83253058..0x83253090)
	// 83253058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325305C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325306C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83253070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253074: 4AFA0CE5  bl 0x821f3d58
	ctx.lr = 0x83253078;
	sub_821F3D58(ctx, base);
	// 83253078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325307C: 906A8238  stw r3, -0x7dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32200 as u32), ctx.r[3].u32 ) };
	// 83253080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253090 size=56
    let mut pc: u32 = 0x83253090;
    'dispatch: loop {
        match pc {
            0x83253090 => {
    //   block [0x83253090..0x832530C8)
	// 83253090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325309C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832530A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530AC: 4AFA0CAD  bl 0x821f3d58
	ctx.lr = 0x832530B0;
	sub_821F3D58(ctx, base);
	// 832530B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530B4: 906A823C  stw r3, -0x7dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32196 as u32), ctx.r[3].u32 ) };
	// 832530B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832530C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832530C8 size=56
    let mut pc: u32 = 0x832530C8;
    'dispatch: loop {
        match pc {
            0x832530C8 => {
    //   block [0x832530C8..0x83253100)
	// 832530C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832530CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832530D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832530D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832530E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530E4: 4AFA0C75  bl 0x821f3d58
	ctx.lr = 0x832530E8;
	sub_821F3D58(ctx, base);
	// 832530E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530EC: 906A8240  stw r3, -0x7dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32192 as u32), ctx.r[3].u32 ) };
	// 832530F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253100 size=56
    let mut pc: u32 = 0x83253100;
    'dispatch: loop {
        match pc {
            0x83253100 => {
    //   block [0x83253100..0x83253138)
	// 83253100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325310C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253114: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83253118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325311C: 4AFA0C3D  bl 0x821f3d58
	ctx.lr = 0x83253120;
	sub_821F3D58(ctx, base);
	// 83253120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253124: 906A8244  stw r3, -0x7dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32188 as u32), ctx.r[3].u32 ) };
	// 83253128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325312C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253138 size=56
    let mut pc: u32 = 0x83253138;
    'dispatch: loop {
        match pc {
            0x83253138 => {
    //   block [0x83253138..0x83253170)
	// 83253138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325314C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83253150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253154: 4AFA0C05  bl 0x821f3d58
	ctx.lr = 0x83253158;
	sub_821F3D58(ctx, base);
	// 83253158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325315C: 906A8248  stw r3, -0x7db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32184 as u32), ctx.r[3].u32 ) };
	// 83253160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253170 size=56
    let mut pc: u32 = 0x83253170;
    'dispatch: loop {
        match pc {
            0x83253170 => {
    //   block [0x83253170..0x832531A8)
	// 83253170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325317C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253184: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83253188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325318C: 4AFA0BCD  bl 0x821f3d58
	ctx.lr = 0x83253190;
	sub_821F3D58(ctx, base);
	// 83253190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253194: 906A824C  stw r3, -0x7db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32180 as u32), ctx.r[3].u32 ) };
	// 83253198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832531A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531A8 size=56
    let mut pc: u32 = 0x832531A8;
    'dispatch: loop {
        match pc {
            0x832531A8 => {
    //   block [0x832531A8..0x832531E0)
	// 832531A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832531C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531C4: 4AFA0B95  bl 0x821f3d58
	ctx.lr = 0x832531C8;
	sub_821F3D58(ctx, base);
	// 832531C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832531CC: 906A8250  stw r3, -0x7db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32176 as u32), ctx.r[3].u32 ) };
	// 832531D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832531D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832531E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531E0 size=56
    let mut pc: u32 = 0x832531E0;
    'dispatch: loop {
        match pc {
            0x832531E0 => {
    //   block [0x832531E0..0x83253218)
	// 832531E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832531F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531FC: 4AFA0B5D  bl 0x821f3d58
	ctx.lr = 0x83253200;
	sub_821F3D58(ctx, base);
	// 83253200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253204: 906A8254  stw r3, -0x7dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32172 as u32), ctx.r[3].u32 ) };
	// 83253208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325320C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253218 size=56
    let mut pc: u32 = 0x83253218;
    'dispatch: loop {
        match pc {
            0x83253218 => {
    //   block [0x83253218..0x83253250)
	// 83253218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253228: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325322C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83253230: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253234: 4AFA0B25  bl 0x821f3d58
	ctx.lr = 0x83253238;
	sub_821F3D58(ctx, base);
	// 83253238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325323C: 906A8258  stw r3, -0x7da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32168 as u32), ctx.r[3].u32 ) };
	// 83253240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325324C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253250 size=56
    let mut pc: u32 = 0x83253250;
    'dispatch: loop {
        match pc {
            0x83253250 => {
    //   block [0x83253250..0x83253288)
	// 83253250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325325C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253260: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253264: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83253268: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325326C: 4AFA0AED  bl 0x821f3d58
	ctx.lr = 0x83253270;
	sub_821F3D58(ctx, base);
	// 83253270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253274: 906A825C  stw r3, -0x7da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32164 as u32), ctx.r[3].u32 ) };
	// 83253278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325327C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253288 size=56
    let mut pc: u32 = 0x83253288;
    'dispatch: loop {
        match pc {
            0x83253288 => {
    //   block [0x83253288..0x832532C0)
	// 83253288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325328C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253298: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325329C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832532A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532A4: 4AFA0AB5  bl 0x821f3d58
	ctx.lr = 0x832532A8;
	sub_821F3D58(ctx, base);
	// 832532A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532AC: 906A8260  stw r3, -0x7da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32160 as u32), ctx.r[3].u32 ) };
	// 832532B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832532C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532C0 size=56
    let mut pc: u32 = 0x832532C0;
    'dispatch: loop {
        match pc {
            0x832532C0 => {
    //   block [0x832532C0..0x832532F8)
	// 832532C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832532C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832532CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832532D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832532D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832532D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532DC: 4AFA0A7D  bl 0x821f3d58
	ctx.lr = 0x832532E0;
	sub_821F3D58(ctx, base);
	// 832532E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532E4: 906A8264  stw r3, -0x7d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32156 as u32), ctx.r[3].u32 ) };
	// 832532E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832532F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532F8 size=56
    let mut pc: u32 = 0x832532F8;
    'dispatch: loop {
        match pc {
            0x832532F8 => {
    //   block [0x832532F8..0x83253330)
	// 832532F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253308: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325330C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83253310: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253314: 4AFA0A45  bl 0x821f3d58
	ctx.lr = 0x83253318;
	sub_821F3D58(ctx, base);
	// 83253318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325331C: 906A8268  stw r3, -0x7d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32152 as u32), ctx.r[3].u32 ) };
	// 83253320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325332C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253330 size=56
    let mut pc: u32 = 0x83253330;
    'dispatch: loop {
        match pc {
            0x83253330 => {
    //   block [0x83253330..0x83253368)
	// 83253330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325333C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253340: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253344: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83253348: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325334C: 4AFA0A0D  bl 0x821f3d58
	ctx.lr = 0x83253350;
	sub_821F3D58(ctx, base);
	// 83253350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253354: 906A826C  stw r3, -0x7d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32148 as u32), ctx.r[3].u32 ) };
	// 83253358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253368 size=56
    let mut pc: u32 = 0x83253368;
    'dispatch: loop {
        match pc {
            0x83253368 => {
    //   block [0x83253368..0x832533A0)
	// 83253368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325336C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253378: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325337C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83253380: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253384: 4AFA09D5  bl 0x821f3d58
	ctx.lr = 0x83253388;
	sub_821F3D58(ctx, base);
	// 83253388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325338C: 906A8270  stw r3, -0x7d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32144 as u32), ctx.r[3].u32 ) };
	// 83253390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325339C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832533A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533A0 size=56
    let mut pc: u32 = 0x832533A0;
    'dispatch: loop {
        match pc {
            0x832533A0 => {
    //   block [0x832533A0..0x832533D8)
	// 832533A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832533B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533BC: 4AFA099D  bl 0x821f3d58
	ctx.lr = 0x832533C0;
	sub_821F3D58(ctx, base);
	// 832533C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533C4: 906A8274  stw r3, -0x7d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32140 as u32), ctx.r[3].u32 ) };
	// 832533C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832533CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832533D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832533D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832533D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533D8 size=56
    let mut pc: u32 = 0x832533D8;
    'dispatch: loop {
        match pc {
            0x832533D8 => {
    //   block [0x832533D8..0x83253410)
	// 832533D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832533F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533F4: 4AFA0965  bl 0x821f3d58
	ctx.lr = 0x832533F8;
	sub_821F3D58(ctx, base);
	// 832533F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533FC: 906A8278  stw r3, -0x7d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32136 as u32), ctx.r[3].u32 ) };
	// 83253400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325340C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253410 size=56
    let mut pc: u32 = 0x83253410;
    'dispatch: loop {
        match pc {
            0x83253410 => {
    //   block [0x83253410..0x83253448)
	// 83253410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325341C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253420: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253424: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83253428: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325342C: 4AFA092D  bl 0x821f3d58
	ctx.lr = 0x83253430;
	sub_821F3D58(ctx, base);
	// 83253430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253434: 906A827C  stw r3, -0x7d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32132 as u32), ctx.r[3].u32 ) };
	// 83253438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325343C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253448 size=56
    let mut pc: u32 = 0x83253448;
    'dispatch: loop {
        match pc {
            0x83253448 => {
    //   block [0x83253448..0x83253480)
	// 83253448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253458: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325345C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83253460: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253464: 4AFA08F5  bl 0x821f3d58
	ctx.lr = 0x83253468;
	sub_821F3D58(ctx, base);
	// 83253468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325346C: 906A8280  stw r3, -0x7d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32128 as u32), ctx.r[3].u32 ) };
	// 83253470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253480 size=56
    let mut pc: u32 = 0x83253480;
    'dispatch: loop {
        match pc {
            0x83253480 => {
    //   block [0x83253480..0x832534B8)
	// 83253480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325348C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253490: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253494: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83253498: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325349C: 4AFA08BD  bl 0x821f3d58
	ctx.lr = 0x832534A0;
	sub_821F3D58(ctx, base);
	// 832534A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534A4: 906A8284  stw r3, -0x7d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32124 as u32), ctx.r[3].u32 ) };
	// 832534A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832534B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534B8 size=56
    let mut pc: u32 = 0x832534B8;
    'dispatch: loop {
        match pc {
            0x832534B8 => {
    //   block [0x832534B8..0x832534F0)
	// 832534B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832534C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832534C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832534CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832534D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832534D4: 4AFA0885  bl 0x821f3d58
	ctx.lr = 0x832534D8;
	sub_821F3D58(ctx, base);
	// 832534D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534DC: 906A8288  stw r3, -0x7d78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32120 as u32), ctx.r[3].u32 ) };
	// 832534E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832534F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534F0 size=64
    let mut pc: u32 = 0x832534F0;
    'dispatch: loop {
        match pc {
            0x832534F0 => {
    //   block [0x832534F0..0x83253530)
	// 832534F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832534FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253504: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83253508: 386A828C  addi r3, r10, -0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + -32116;
	// 8325350C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253510: 4AFD99C1  bl 0x8222ced0
	ctx.lr = 0x83253514;
	sub_8222CED0(ctx, base);
	// 83253514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253518: 38699C90  addi r3, r9, -0x6370
	ctx.r[3].s64 = ctx.r[9].s64 + -25456;
	// 8325351C: 4BA56A05  bl 0x82ca9f20
	ctx.lr = 0x83253520;
	sub_82CA9F20(ctx, base);
	// 83253520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


