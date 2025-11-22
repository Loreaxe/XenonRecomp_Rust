pub fn sub_826AA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA2C8 size=112
    let mut pc: u32 = 0x826AA2C8;
    'dispatch: loop {
        match pc {
            0x826AA2C8 => {
    //   block [0x826AA2C8..0x826AA338)
	// 826AA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA2D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA2D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA2DC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA2E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA2E4: 390BDFB0  addi r8, r11, -0x2050
	ctx.r[8].s64 = ctx.r[11].s64 + -8272;
	// 826AA2E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA2EC: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 826AA2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA300: 386A9F38  addi r3, r10, -0x60c8
	ctx.r[3].s64 = ctx.r[10].s64 + -24776;
	// 826AA304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA324: 4BDBCAFD  bl 0x82466e20
	ctx.lr = 0x826AA328;
	sub_82466E20(ctx, base);
	// 826AA328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA338 size=112
    let mut pc: u32 = 0x826AA338;
    'dispatch: loop {
        match pc {
            0x826AA338 => {
    //   block [0x826AA338..0x826AA3A8)
	// 826AA338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA34C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA354: 390BDFE0  addi r8, r11, -0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + -8224;
	// 826AA358: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA35C: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826AA360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA370: 386A9F68  addi r3, r10, -0x6098
	ctx.r[3].s64 = ctx.r[10].s64 + -24728;
	// 826AA374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA394: 4BDBCA8D  bl 0x82466e20
	ctx.lr = 0x826AA398;
	sub_82466E20(ctx, base);
	// 826AA398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA3A8 size=112
    let mut pc: u32 = 0x826AA3A8;
    'dispatch: loop {
        match pc {
            0x826AA3A8 => {
    //   block [0x826AA3A8..0x826AA418)
	// 826AA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA3B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA3B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA3BC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA3C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA3C4: 390BDFF8  addi r8, r11, -0x2008
	ctx.r[8].s64 = ctx.r[11].s64 + -8200;
	// 826AA3C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA3CC: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 826AA3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA3E0: 386A9F98  addi r3, r10, -0x6068
	ctx.r[3].s64 = ctx.r[10].s64 + -24680;
	// 826AA3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA404: 4BDBCA1D  bl 0x82466e20
	ctx.lr = 0x826AA408;
	sub_82466E20(ctx, base);
	// 826AA408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA418 size=112
    let mut pc: u32 = 0x826AA418;
    'dispatch: loop {
        match pc {
            0x826AA418 => {
    //   block [0x826AA418..0x826AA488)
	// 826AA418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA428: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA42C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA434: 390BE010  addi r8, r11, -0x1ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -8176;
	// 826AA438: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA43C: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 826AA440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA450: 386A9FC8  addi r3, r10, -0x6038
	ctx.r[3].s64 = ctx.r[10].s64 + -24632;
	// 826AA454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA474: 4BDBC9AD  bl 0x82466e20
	ctx.lr = 0x826AA478;
	sub_82466E20(ctx, base);
	// 826AA478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA488 size=112
    let mut pc: u32 = 0x826AA488;
    'dispatch: loop {
        match pc {
            0x826AA488 => {
    //   block [0x826AA488..0x826AA4F8)
	// 826AA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA498: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA49C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA4A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA4A4: 390BE058  addi r8, r11, -0x1fa8
	ctx.r[8].s64 = ctx.r[11].s64 + -8104;
	// 826AA4A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA4AC: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 826AA4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA4C0: 386A9FF8  addi r3, r10, -0x6008
	ctx.r[3].s64 = ctx.r[10].s64 + -24584;
	// 826AA4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA4E4: 4BDBC93D  bl 0x82466e20
	ctx.lr = 0x826AA4E8;
	sub_82466E20(ctx, base);
	// 826AA4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA4F8 size=112
    let mut pc: u32 = 0x826AA4F8;
    'dispatch: loop {
        match pc {
            0x826AA4F8 => {
    //   block [0x826AA4F8..0x826AA568)
	// 826AA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA508: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA50C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA514: 390BE0A0  addi r8, r11, -0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + -8032;
	// 826AA518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA51C: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 826AA520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA530: 386AA028  addi r3, r10, -0x5fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -24536;
	// 826AA534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA554: 4BDBC8CD  bl 0x82466e20
	ctx.lr = 0x826AA558;
	sub_82466E20(ctx, base);
	// 826AA558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA568 size=112
    let mut pc: u32 = 0x826AA568;
    'dispatch: loop {
        match pc {
            0x826AA568 => {
    //   block [0x826AA568..0x826AA5D8)
	// 826AA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA578: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA57C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA584: 390BE0B8  addi r8, r11, -0x1f48
	ctx.r[8].s64 = ctx.r[11].s64 + -8008;
	// 826AA588: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA58C: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826AA590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA5A0: 386AA058  addi r3, r10, -0x5fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -24488;
	// 826AA5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA5C4: 4BDBC85D  bl 0x82466e20
	ctx.lr = 0x826AA5C8;
	sub_82466E20(ctx, base);
	// 826AA5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA5D8 size=116
    let mut pc: u32 = 0x826AA5D8;
    'dispatch: loop {
        match pc {
            0x826AA5D8 => {
    //   block [0x826AA5D8..0x826AA64C)
	// 826AA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA5E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA5E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AA5EC: 390AE0E8  addi r8, r10, -0x1f18
	ctx.r[8].s64 = ctx.r[10].s64 + -7960;
	// 826AA5F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA5F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA5F8: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA5FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AA604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA60C: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 826AA610: 396BD0A0  addi r11, r11, -0x2f60
	ctx.r[11].s64 = ctx.r[11].s64 + -12128;
	// 826AA614: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA61C: 386AA088  addi r3, r10, -0x5f78
	ctx.r[3].s64 = ctx.r[10].s64 + -24440;
	// 826AA620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AA624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AA62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA638: 4BDBC7E9  bl 0x82466e20
	ctx.lr = 0x826AA63C;
	sub_82466E20(ctx, base);
	// 826AA63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA650 size=116
    let mut pc: u32 = 0x826AA650;
    'dispatch: loop {
        match pc {
            0x826AA650 => {
    //   block [0x826AA650..0x826AA6C4)
	// 826AA650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA65C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA660: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AA664: 390AE160  addi r8, r10, -0x1ea0
	ctx.r[8].s64 = ctx.r[10].s64 + -7840;
	// 826AA668: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA66C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA670: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA674: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA678: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AA67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA684: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826AA688: 396BD0B8  addi r11, r11, -0x2f48
	ctx.r[11].s64 = ctx.r[11].s64 + -12104;
	// 826AA68C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA690: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA694: 386AA0B8  addi r3, r10, -0x5f48
	ctx.r[3].s64 = ctx.r[10].s64 + -24392;
	// 826AA698: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AA69C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA6A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AA6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA6B0: 4BDBC771  bl 0x82466e20
	ctx.lr = 0x826AA6B4;
	sub_82466E20(ctx, base);
	// 826AA6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AA6C8 size=24
    let mut pc: u32 = 0x826AA6C8;
    'dispatch: loop {
        match pc {
            0x826AA6C8 => {
    //   block [0x826AA6C8..0x826AA6E0)
	// 826AA6C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA6CC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA6D0: 394A4168  addi r10, r10, 0x4168
	ctx.r[10].s64 = ctx.r[10].s64 + 16744;
	// 826AA6D4: 816BDE74  lwz r11, -0x218c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8588 as u32) ) } as u64;
	// 826AA6D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826AA6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA6E0 size=116
    let mut pc: u32 = 0x826AA6E0;
    'dispatch: loop {
        match pc {
            0x826AA6E0 => {
    //   block [0x826AA6E0..0x826AA754)
	// 826AA6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA6EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA6F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA6F4: 392BD0E4  addi r9, r11, -0x2f1c
	ctx.r[9].s64 = ctx.r[11].s64 + -12060;
	// 826AA6F8: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA6FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA700: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826AA704: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826AA708: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA70C: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 826AA710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA714: 396B4168  addi r11, r11, 0x4168
	ctx.r[11].s64 = ctx.r[11].s64 + 16744;
	// 826AA718: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AA71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA720: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AA724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA728: 386AA0E8  addi r3, r10, -0x5f18
	ctx.r[3].s64 = ctx.r[10].s64 + -24344;
	// 826AA72C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AA730: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AA734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA738: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AA73C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AA740: 4BDBC6E1  bl 0x82466e20
	ctx.lr = 0x826AA744;
	sub_82466E20(ctx, base);
	// 826AA744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA758 size=112
    let mut pc: u32 = 0x826AA758;
    'dispatch: loop {
        match pc {
            0x826AA758 => {
    //   block [0x826AA758..0x826AA7C8)
	// 826AA758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA768: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA76C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA774: 390BE1F0  addi r8, r11, -0x1e10
	ctx.r[8].s64 = ctx.r[11].s64 + -7696;
	// 826AA778: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AA77C: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826AA780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA790: 386AA118  addi r3, r10, -0x5ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -24296;
	// 826AA794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA7B4: 4BDBC66D  bl 0x82466e20
	ctx.lr = 0x826AA7B8;
	sub_82466E20(ctx, base);
	// 826AA7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA7C8 size=112
    let mut pc: u32 = 0x826AA7C8;
    'dispatch: loop {
        match pc {
            0x826AA7C8 => {
    //   block [0x826AA7C8..0x826AA838)
	// 826AA7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA7D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA7D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA7DC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA7E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA7E4: 390BE250  addi r8, r11, -0x1db0
	ctx.r[8].s64 = ctx.r[11].s64 + -7600;
	// 826AA7E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AA7EC: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 826AA7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA7F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA800: 386AA148  addi r3, r10, -0x5eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -24248;
	// 826AA804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA824: 4BDBC5FD  bl 0x82466e20
	ctx.lr = 0x826AA828;
	sub_82466E20(ctx, base);
	// 826AA828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA838 size=112
    let mut pc: u32 = 0x826AA838;
    'dispatch: loop {
        match pc {
            0x826AA838 => {
    //   block [0x826AA838..0x826AA8A8)
	// 826AA838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA848: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA84C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA854: 390BE2F8  addi r8, r11, -0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + -7432;
	// 826AA858: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AA85C: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826AA860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA870: 386AA178  addi r3, r10, -0x5e88
	ctx.r[3].s64 = ctx.r[10].s64 + -24200;
	// 826AA874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA894: 4BDBC58D  bl 0x82466e20
	ctx.lr = 0x826AA898;
	sub_82466E20(ctx, base);
	// 826AA898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA8A8 size=112
    let mut pc: u32 = 0x826AA8A8;
    'dispatch: loop {
        match pc {
            0x826AA8A8 => {
    //   block [0x826AA8A8..0x826AA918)
	// 826AA8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA8B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA8B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA8BC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA8C4: 390BE370  addi r8, r11, -0x1c90
	ctx.r[8].s64 = ctx.r[11].s64 + -7312;
	// 826AA8C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA8CC: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 826AA8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA8D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA8E0: 386AA1A8  addi r3, r10, -0x5e58
	ctx.r[3].s64 = ctx.r[10].s64 + -24152;
	// 826AA8E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA904: 4BDBC51D  bl 0x82466e20
	ctx.lr = 0x826AA908;
	sub_82466E20(ctx, base);
	// 826AA908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA918 size=112
    let mut pc: u32 = 0x826AA918;
    'dispatch: loop {
        match pc {
            0x826AA918 => {
    //   block [0x826AA918..0x826AA988)
	// 826AA918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA928: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA92C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA930: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA934: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 826AA938: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826AA93C: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 826AA940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA950: 386AA1D8  addi r3, r10, -0x5e28
	ctx.r[3].s64 = ctx.r[10].s64 + -24104;
	// 826AA954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA974: 4BDBC4AD  bl 0x82466e20
	ctx.lr = 0x826AA978;
	sub_82466E20(ctx, base);
	// 826AA978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA988 size=112
    let mut pc: u32 = 0x826AA988;
    'dispatch: loop {
        match pc {
            0x826AA988 => {
    //   block [0x826AA988..0x826AA9F8)
	// 826AA988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA998: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA99C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA9A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA9A4: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 826AA9A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AA9AC: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 826AA9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA9B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA9B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA9C0: 386AA208  addi r3, r10, -0x5df8
	ctx.r[3].s64 = ctx.r[10].s64 + -24056;
	// 826AA9C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA9E4: 4BDBC43D  bl 0x82466e20
	ctx.lr = 0x826AA9E8;
	sub_82466E20(ctx, base);
	// 826AA9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA9F8 size=112
    let mut pc: u32 = 0x826AA9F8;
    'dispatch: loop {
        match pc {
            0x826AA9F8 => {
    //   block [0x826AA9F8..0x826AAA68)
	// 826AA9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAA04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAA0C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AAA10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAA14: 390BE4A8  addi r8, r11, -0x1b58
	ctx.r[8].s64 = ctx.r[11].s64 + -7000;
	// 826AAA18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AAA1C: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 826AAA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAA24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAA30: 386AA238  addi r3, r10, -0x5dc8
	ctx.r[3].s64 = ctx.r[10].s64 + -24008;
	// 826AAA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAA54: 4BDBC3CD  bl 0x82466e20
	ctx.lr = 0x826AAA58;
	sub_82466E20(ctx, base);
	// 826AAA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAA68 size=112
    let mut pc: u32 = 0x826AAA68;
    'dispatch: loop {
        match pc {
            0x826AAA68 => {
    //   block [0x826AAA68..0x826AAAD8)
	// 826AAA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAA74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAA7C: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAA80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAA84: 390BE508  addi r8, r11, -0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + -6904;
	// 826AAA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AAA8C: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826AAA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAA94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAAA0: 386AA268  addi r3, r10, -0x5d98
	ctx.r[3].s64 = ctx.r[10].s64 + -23960;
	// 826AAAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAAC4: 4BDBC35D  bl 0x82466e20
	ctx.lr = 0x826AAAC8;
	sub_82466E20(ctx, base);
	// 826AAAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAAD8 size=112
    let mut pc: u32 = 0x826AAAD8;
    'dispatch: loop {
        match pc {
            0x826AAAD8 => {
    //   block [0x826AAAD8..0x826AAB48)
	// 826AAAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAAE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAAE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAAEC: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAAF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAAF4: 390BE538  addi r8, r11, -0x1ac8
	ctx.r[8].s64 = ctx.r[11].s64 + -6856;
	// 826AAAF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AAAFC: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 826AAB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAB04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAB10: 386AA298  addi r3, r10, -0x5d68
	ctx.r[3].s64 = ctx.r[10].s64 + -23912;
	// 826AAB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAB34: 4BDBC2ED  bl 0x82466e20
	ctx.lr = 0x826AAB38;
	sub_82466E20(ctx, base);
	// 826AAB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAB48 size=100
    let mut pc: u32 = 0x826AAB48;
    'dispatch: loop {
        match pc {
            0x826AAB48 => {
    //   block [0x826AAB48..0x826AABAC)
	// 826AAB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAB5C: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAB60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAB68: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826AAB6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAB7C: 386AA2C8  addi r3, r10, -0x5d38
	ctx.r[3].s64 = ctx.r[10].s64 + -23864;
	// 826AAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAB88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AAB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAB90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AAB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAB98: 4BDBC289  bl 0x82466e20
	ctx.lr = 0x826AAB9C;
	sub_82466E20(ctx, base);
	// 826AAB9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AABA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AABA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AABA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AABB0 size=112
    let mut pc: u32 = 0x826AABB0;
    'dispatch: loop {
        match pc {
            0x826AABB0 => {
    //   block [0x826AABB0..0x826AAC20)
	// 826AABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AABB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AABBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AABC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AABC4: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AABC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AABCC: 390BE580  addi r8, r11, -0x1a80
	ctx.r[8].s64 = ctx.r[11].s64 + -6784;
	// 826AABD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AABD4: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 826AABD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AABDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AABE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AABE8: 386AA2F8  addi r3, r10, -0x5d08
	ctx.r[3].s64 = ctx.r[10].s64 + -23816;
	// 826AABEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AABF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AABF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AABF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AABFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAC0C: 4BDBC215  bl 0x82466e20
	ctx.lr = 0x826AAC10;
	sub_82466E20(ctx, base);
	// 826AAC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAC20 size=100
    let mut pc: u32 = 0x826AAC20;
    'dispatch: loop {
        match pc {
            0x826AAC20 => {
    //   block [0x826AAC20..0x826AAC84)
	// 826AAC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAC34: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAC38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AAC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAC40: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 826AAC44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAC54: 386AA328  addi r3, r10, -0x5cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -23768;
	// 826AAC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAC60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AAC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAC68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AAC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAC70: 4BDBC1B1  bl 0x82466e20
	ctx.lr = 0x826AAC74;
	sub_82466E20(ctx, base);
	// 826AAC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAC88 size=108
    let mut pc: u32 = 0x826AAC88;
    'dispatch: loop {
        match pc {
            0x826AAC88 => {
    //   block [0x826AAC88..0x826AACF4)
	// 826AAC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAC94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAC98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAC9C: 38EBE598  addi r7, r11, -0x1a68
	ctx.r[7].s64 = ctx.r[11].s64 + -6760;
	// 826AACA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AACA4: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 826AACA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AACAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AACB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AACB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AACB8: 386AA358  addi r3, r10, -0x5ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -23720;
	// 826AACBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AACC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AACC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AACC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AACCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AACD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AACD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AACD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AACDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AACE0: 4BDBC141  bl 0x82466e20
	ctx.lr = 0x826AACE4;
	sub_82466E20(ctx, base);
	// 826AACE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AACE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AACEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AACF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AACF8 size=112
    let mut pc: u32 = 0x826AACF8;
    'dispatch: loop {
        match pc {
            0x826AACF8 => {
    //   block [0x826AACF8..0x826AAD68)
	// 826AACF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AACFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAD04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAD0C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AAD10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAD14: 390BE5E0  addi r8, r11, -0x1a20
	ctx.r[8].s64 = ctx.r[11].s64 + -6688;
	// 826AAD18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AAD1C: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 826AAD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAD30: 386AA388  addi r3, r10, -0x5c78
	ctx.r[3].s64 = ctx.r[10].s64 + -23672;
	// 826AAD34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAD38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAD54: 4BDBC0CD  bl 0x82466e20
	ctx.lr = 0x826AAD58;
	sub_82466E20(ctx, base);
	// 826AAD58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAD68 size=112
    let mut pc: u32 = 0x826AAD68;
    'dispatch: loop {
        match pc {
            0x826AAD68 => {
    //   block [0x826AAD68..0x826AADD8)
	// 826AAD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAD74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAD7C: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AAD80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAD84: 390BE640  addi r8, r11, -0x19c0
	ctx.r[8].s64 = ctx.r[11].s64 + -6592;
	// 826AAD88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AAD8C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 826AAD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAD94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AADA0: 386AA3B8  addi r3, r10, -0x5c48
	ctx.r[3].s64 = ctx.r[10].s64 + -23624;
	// 826AADA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AADA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AADAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AADB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AADB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AADB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AADBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AADC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AADC4: 4BDBC05D  bl 0x82466e20
	ctx.lr = 0x826AADC8;
	sub_82466E20(ctx, base);
	// 826AADC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AADCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AADD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AADD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AADD8 size=112
    let mut pc: u32 = 0x826AADD8;
    'dispatch: loop {
        match pc {
            0x826AADD8 => {
    //   block [0x826AADD8..0x826AAE48)
	// 826AADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AADE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AADE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AADE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AADEC: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AADF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AADF4: 390BE658  addi r8, r11, -0x19a8
	ctx.r[8].s64 = ctx.r[11].s64 + -6568;
	// 826AADF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AADFC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 826AAE00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAE04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAE10: 386AA3E8  addi r3, r10, -0x5c18
	ctx.r[3].s64 = ctx.r[10].s64 + -23576;
	// 826AAE14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAE18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAE2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAE34: 4BDBBFED  bl 0x82466e20
	ctx.lr = 0x826AAE38;
	sub_82466E20(ctx, base);
	// 826AAE38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAE48 size=112
    let mut pc: u32 = 0x826AAE48;
    'dispatch: loop {
        match pc {
            0x826AAE48 => {
    //   block [0x826AAE48..0x826AAEB8)
	// 826AAE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAE5C: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AAE60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAE64: 390BE688  addi r8, r11, -0x1978
	ctx.r[8].s64 = ctx.r[11].s64 + -6520;
	// 826AAE68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AAE6C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 826AAE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAE74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAE80: 386AA418  addi r3, r10, -0x5be8
	ctx.r[3].s64 = ctx.r[10].s64 + -23528;
	// 826AAE84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAEA4: 4BDBBF7D  bl 0x82466e20
	ctx.lr = 0x826AAEA8;
	sub_82466E20(ctx, base);
	// 826AAEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AAEB8 size=24
    let mut pc: u32 = 0x826AAEB8;
    'dispatch: loop {
        match pc {
            0x826AAEB8 => {
    //   block [0x826AAEB8..0x826AAED0)
	// 826AAEB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAEBC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AAEC0: 394A4228  addi r10, r10, 0x4228
	ctx.r[10].s64 = ctx.r[10].s64 + 16936;
	// 826AAEC4: 816BE6A0  lwz r11, -0x1960(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6496 as u32) ) } as u64;
	// 826AAEC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826AAECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAED0 size=112
    let mut pc: u32 = 0x826AAED0;
    'dispatch: loop {
        match pc {
            0x826AAED0 => {
    //   block [0x826AAED0..0x826AAF40)
	// 826AAED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAEDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AAEE0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAEE4: 392AD148  addi r9, r10, -0x2eb8
	ctx.r[9].s64 = ctx.r[10].s64 + -11960;
	// 826AAEE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAEEC: 390B4228  addi r8, r11, 0x4228
	ctx.r[8].s64 = ctx.r[11].s64 + 16936;
	// 826AAEF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826AAEF4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 826AAEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAEFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAF08: 386AA448  addi r3, r10, -0x5bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -23480;
	// 826AAF0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AAF10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AAF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAF24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AAF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAF2C: 4BDBBEF5  bl 0x82466e20
	ctx.lr = 0x826AAF30;
	sub_82466E20(ctx, base);
	// 826AAF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAF40 size=108
    let mut pc: u32 = 0x826AAF40;
    'dispatch: loop {
        match pc {
            0x826AAF40 => {
    //   block [0x826AAF40..0x826AAFAC)
	// 826AAF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAF4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAF54: 38EBE6A4  addi r7, r11, -0x195c
	ctx.r[7].s64 = ctx.r[11].s64 + -6492;
	// 826AAF58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AAF5C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 826AAF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAF64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AAF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAF70: 386AA478  addi r3, r10, -0x5b88
	ctx.r[3].s64 = ctx.r[10].s64 + -23432;
	// 826AAF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AAF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AAF98: 4BDBBE89  bl 0x82466e20
	ctx.lr = 0x826AAF9C;
	sub_82466E20(ctx, base);
	// 826AAF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAFB0 size=108
    let mut pc: u32 = 0x826AAFB0;
    'dispatch: loop {
        match pc {
            0x826AAFB0 => {
    //   block [0x826AAFB0..0x826AB01C)
	// 826AAFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAFBC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAFC4: 38EBE6C0  addi r7, r11, -0x1940
	ctx.r[7].s64 = ctx.r[11].s64 + -6464;
	// 826AAFC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AAFCC: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 826AAFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAFD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AAFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAFE0: 386AA4A8  addi r3, r10, -0x5b58
	ctx.r[3].s64 = ctx.r[10].s64 + -23384;
	// 826AAFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AAFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB008: 4BDBBE19  bl 0x82466e20
	ctx.lr = 0x826AB00C;
	sub_82466E20(ctx, base);
	// 826AB00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB020 size=116
    let mut pc: u32 = 0x826AB020;
    'dispatch: loop {
        match pc {
            0x826AB020 => {
    //   block [0x826AB020..0x826AB094)
	// 826AB020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB02C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB030: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB034: 390BE708  addi r8, r11, -0x18f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6392;
	// 826AB038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB03C: 392AD210  addi r9, r10, -0x2df0
	ctx.r[9].s64 = ctx.r[10].s64 + -11760;
	// 826AB040: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB044: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AB048: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB04C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB054: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB064: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB068: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 826AB06C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB070: 386BA4D8  addi r3, r11, -0x5b28
	ctx.r[3].s64 = ctx.r[11].s64 + -23336;
	// 826AB074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB080: 4BDBBDA1  bl 0x82466e20
	ctx.lr = 0x826AB084;
	sub_82466E20(ctx, base);
	// 826AB084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AB098 size=24
    let mut pc: u32 = 0x826AB098;
    'dispatch: loop {
        match pc {
            0x826AB098 => {
    //   block [0x826AB098..0x826AB0B0)
	// 826AB098: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB09C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AB0A0: 394A4270  addi r10, r10, 0x4270
	ctx.r[10].s64 = ctx.r[10].s64 + 17008;
	// 826AB0A4: 816BE720  lwz r11, -0x18e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6368 as u32) ) } as u64;
	// 826AB0A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826AB0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB0B0 size=116
    let mut pc: u32 = 0x826AB0B0;
    'dispatch: loop {
        match pc {
            0x826AB0B0 => {
    //   block [0x826AB0B0..0x826AB124)
	// 826AB0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB0BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB0C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB0C4: 390B4270  addi r8, r11, 0x4270
	ctx.r[8].s64 = ctx.r[11].s64 + 17008;
	// 826AB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB0CC: 392AD280  addi r9, r10, -0x2d80
	ctx.r[9].s64 = ctx.r[10].s64 + -11648;
	// 826AB0D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB0D4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826AB0D8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB0DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB0E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB0F4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB0F8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 826AB0FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB100: 386BA508  addi r3, r11, -0x5af8
	ctx.r[3].s64 = ctx.r[11].s64 + -23288;
	// 826AB104: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826AB108: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB110: 4BDBBD11  bl 0x82466e20
	ctx.lr = 0x826AB114;
	sub_82466E20(ctx, base);
	// 826AB114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB128 size=108
    let mut pc: u32 = 0x826AB128;
    'dispatch: loop {
        match pc {
            0x826AB128 => {
    //   block [0x826AB128..0x826AB194)
	// 826AB128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB134: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB13C: 38EBE730  addi r7, r11, -0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6352;
	// 826AB140: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB144: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 826AB148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB158: 386AA538  addi r3, r10, -0x5ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -23240;
	// 826AB15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB180: 4BDBBCA1  bl 0x82466e20
	ctx.lr = 0x826AB184;
	sub_82466E20(ctx, base);
	// 826AB184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB198 size=112
    let mut pc: u32 = 0x826AB198;
    'dispatch: loop {
        match pc {
            0x826AB198 => {
    //   block [0x826AB198..0x826AB208)
	// 826AB198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB1A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB1AC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB1B4: 390BE760  addi r8, r11, -0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + -6304;
	// 826AB1B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB1BC: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 826AB1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB1D0: 386AA568  addi r3, r10, -0x5a98
	ctx.r[3].s64 = ctx.r[10].s64 + -23192;
	// 826AB1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB1F4: 4BDBBC2D  bl 0x82466e20
	ctx.lr = 0x826AB1F8;
	sub_82466E20(ctx, base);
	// 826AB1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB208 size=112
    let mut pc: u32 = 0x826AB208;
    'dispatch: loop {
        match pc {
            0x826AB208 => {
    //   block [0x826AB208..0x826AB278)
	// 826AB208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB214: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB218: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB21C: 392AD2D8  addi r9, r10, -0x2d28
	ctx.r[9].s64 = ctx.r[10].s64 + -11560;
	// 826AB220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB224: 390BE780  addi r8, r11, -0x1880
	ctx.r[8].s64 = ctx.r[11].s64 + -6272;
	// 826AB228: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826AB22C: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 826AB230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB240: 386AA598  addi r3, r10, -0x5a68
	ctx.r[3].s64 = ctx.r[10].s64 + -23144;
	// 826AB244: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB248: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB25C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB264: 4BDBBBBD  bl 0x82466e20
	ctx.lr = 0x826AB268;
	sub_82466E20(ctx, base);
	// 826AB268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB278 size=112
    let mut pc: u32 = 0x826AB278;
    'dispatch: loop {
        match pc {
            0x826AB278 => {
    //   block [0x826AB278..0x826AB2E8)
	// 826AB278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB288: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB28C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB294: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 826AB298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB29C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 826AB2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB2A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB2B0: 386AA5C8  addi r3, r10, -0x5a38
	ctx.r[3].s64 = ctx.r[10].s64 + -23096;
	// 826AB2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB2D4: 4BDBBB4D  bl 0x82466e20
	ctx.lr = 0x826AB2D8;
	sub_82466E20(ctx, base);
	// 826AB2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB2E8 size=112
    let mut pc: u32 = 0x826AB2E8;
    'dispatch: loop {
        match pc {
            0x826AB2E8 => {
    //   block [0x826AB2E8..0x826AB358)
	// 826AB2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB2F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB2FC: 392AD304  addi r9, r10, -0x2cfc
	ctx.r[9].s64 = ctx.r[10].s64 + -11516;
	// 826AB300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB304: 390BE7E0  addi r8, r11, -0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + -6176;
	// 826AB308: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AB30C: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 826AB310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB320: 386AA5F8  addi r3, r10, -0x5a08
	ctx.r[3].s64 = ctx.r[10].s64 + -23048;
	// 826AB324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB328: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB344: 4BDBBADD  bl 0x82466e20
	ctx.lr = 0x826AB348;
	sub_82466E20(ctx, base);
	// 826AB348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB358 size=112
    let mut pc: u32 = 0x826AB358;
    'dispatch: loop {
        match pc {
            0x826AB358 => {
    //   block [0x826AB358..0x826AB3C8)
	// 826AB358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB368: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB36C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB374: 390BE870  addi r8, r11, -0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + -6032;
	// 826AB378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB37C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 826AB380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB390: 386AA628  addi r3, r10, -0x59d8
	ctx.r[3].s64 = ctx.r[10].s64 + -23000;
	// 826AB394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB3B4: 4BDBBA6D  bl 0x82466e20
	ctx.lr = 0x826AB3B8;
	sub_82466E20(ctx, base);
	// 826AB3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB3C8 size=112
    let mut pc: u32 = 0x826AB3C8;
    'dispatch: loop {
        match pc {
            0x826AB3C8 => {
    //   block [0x826AB3C8..0x826AB438)
	// 826AB3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB3D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB3DC: 38AAA688  addi r5, r10, -0x5978
	ctx.r[5].s64 = ctx.r[10].s64 + -22904;
	// 826AB3E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB3E4: 390BE888  addi r8, r11, -0x1778
	ctx.r[8].s64 = ctx.r[11].s64 + -6008;
	// 826AB3E8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AB3EC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826AB3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB3F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB400: 386AA658  addi r3, r10, -0x59a8
	ctx.r[3].s64 = ctx.r[10].s64 + -22952;
	// 826AB404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB424: 4BDBB9FD  bl 0x82466e20
	ctx.lr = 0x826AB428;
	sub_82466E20(ctx, base);
	// 826AB428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB438 size=100
    let mut pc: u32 = 0x826AB438;
    'dispatch: loop {
        match pc {
            0x826AB438 => {
    //   block [0x826AB438..0x826AB49C)
	// 826AB438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB44C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB458: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 826AB45C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB46C: 386AA688  addi r3, r10, -0x5978
	ctx.r[3].s64 = ctx.r[10].s64 + -22904;
	// 826AB470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AB47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AB484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB488: 4BDBB999  bl 0x82466e20
	ctx.lr = 0x826AB48C;
	sub_82466E20(ctx, base);
	// 826AB48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AB4A0 size=24
    let mut pc: u32 = 0x826AB4A0;
    'dispatch: loop {
        match pc {
            0x826AB4A0 => {
    //   block [0x826AB4A0..0x826AB4B8)
	// 826AB4A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB4A4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AB4A8: 394A4348  addi r10, r10, 0x4348
	ctx.r[10].s64 = ctx.r[10].s64 + 17224;
	// 826AB4AC: 816BE900  lwz r11, -0x1700(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5888 as u32) ) } as u64;
	// 826AB4B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826AB4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB4B8 size=116
    let mut pc: u32 = 0x826AB4B8;
    'dispatch: loop {
        match pc {
            0x826AB4B8 => {
    //   block [0x826AB4B8..0x826AB52C)
	// 826AB4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB4C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB4C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB4CC: 390B4348  addi r8, r11, 0x4348
	ctx.r[8].s64 = ctx.r[11].s64 + 17224;
	// 826AB4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB4D4: 392AD340  addi r9, r10, -0x2cc0
	ctx.r[9].s64 = ctx.r[10].s64 + -11456;
	// 826AB4D8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB4DC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AB4E0: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB4E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB4EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB4FC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB500: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826AB504: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB508: 386BA6B8  addi r3, r11, -0x5948
	ctx.r[3].s64 = ctx.r[11].s64 + -22856;
	// 826AB50C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB510: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB518: 4BDBB909  bl 0x82466e20
	ctx.lr = 0x826AB51C;
	sub_82466E20(ctx, base);
	// 826AB51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB530 size=108
    let mut pc: u32 = 0x826AB530;
    'dispatch: loop {
        match pc {
            0x826AB530 => {
    //   block [0x826AB530..0x826AB59C)
	// 826AB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB53C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB544: 38EBE904  addi r7, r11, -0x16fc
	ctx.r[7].s64 = ctx.r[11].s64 + -5884;
	// 826AB548: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB54C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 826AB550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB554: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB560: 386AA6E8  addi r3, r10, -0x5918
	ctx.r[3].s64 = ctx.r[10].s64 + -22808;
	// 826AB564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB588: 4BDBB899  bl 0x82466e20
	ctx.lr = 0x826AB58C;
	sub_82466E20(ctx, base);
	// 826AB58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB5A0 size=112
    let mut pc: u32 = 0x826AB5A0;
    'dispatch: loop {
        match pc {
            0x826AB5A0 => {
    //   block [0x826AB5A0..0x826AB610)
	// 826AB5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB5AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB5B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB5B4: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB5BC: 390BE934  addi r8, r11, -0x16cc
	ctx.r[8].s64 = ctx.r[11].s64 + -5836;
	// 826AB5C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB5C4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826AB5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB5CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB5D8: 386AA718  addi r3, r10, -0x58e8
	ctx.r[3].s64 = ctx.r[10].s64 + -22760;
	// 826AB5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB5FC: 4BDBB825  bl 0x82466e20
	ctx.lr = 0x826AB600;
	sub_82466E20(ctx, base);
	// 826AB600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB610 size=112
    let mut pc: u32 = 0x826AB610;
    'dispatch: loop {
        match pc {
            0x826AB610 => {
    //   block [0x826AB610..0x826AB680)
	// 826AB610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB61C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB620: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB624: 392AD364  addi r9, r10, -0x2c9c
	ctx.r[9].s64 = ctx.r[10].s64 + -11420;
	// 826AB628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB62C: 390BE950  addi r8, r11, -0x16b0
	ctx.r[8].s64 = ctx.r[11].s64 + -5808;
	// 826AB630: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB634: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 826AB638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB648: 386AA748  addi r3, r10, -0x58b8
	ctx.r[3].s64 = ctx.r[10].s64 + -22712;
	// 826AB64C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB650: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB66C: 4BDBB7B5  bl 0x82466e20
	ctx.lr = 0x826AB670;
	sub_82466E20(ctx, base);
	// 826AB670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB680 size=112
    let mut pc: u32 = 0x826AB680;
    'dispatch: loop {
        match pc {
            0x826AB680 => {
    //   block [0x826AB680..0x826AB6F0)
	// 826AB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB68C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB690: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB694: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB69C: 390BE9F8  addi r8, r11, -0x1608
	ctx.r[8].s64 = ctx.r[11].s64 + -5640;
	// 826AB6A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB6A4: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 826AB6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB6B8: 386AA778  addi r3, r10, -0x5888
	ctx.r[3].s64 = ctx.r[10].s64 + -22664;
	// 826AB6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB6DC: 4BDBB745  bl 0x82466e20
	ctx.lr = 0x826AB6E0;
	sub_82466E20(ctx, base);
	// 826AB6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB6F0 size=112
    let mut pc: u32 = 0x826AB6F0;
    'dispatch: loop {
        match pc {
            0x826AB6F0 => {
    //   block [0x826AB6F0..0x826AB760)
	// 826AB6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB6FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB700: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB704: 392AD3BC  addi r9, r10, -0x2c44
	ctx.r[9].s64 = ctx.r[10].s64 + -11332;
	// 826AB708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB70C: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 826AB710: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB714: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 826AB718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB728: 386AA7A8  addi r3, r10, -0x5858
	ctx.r[3].s64 = ctx.r[10].s64 + -22616;
	// 826AB72C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB730: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB74C: 4BDBB6D5  bl 0x82466e20
	ctx.lr = 0x826AB750;
	sub_82466E20(ctx, base);
	// 826AB750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB760 size=116
    let mut pc: u32 = 0x826AB760;
    'dispatch: loop {
        match pc {
            0x826AB760 => {
    //   block [0x826AB760..0x826AB7D4)
	// 826AB760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB76C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB770: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB774: 390BEAC0  addi r8, r11, -0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + -5440;
	// 826AB778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB77C: 392AD390  addi r9, r10, -0x2c70
	ctx.r[9].s64 = ctx.r[10].s64 + -11376;
	// 826AB780: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB784: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AB788: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB78C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB7A4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB7A8: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 826AB7AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB7B0: 386BA7D8  addi r3, r11, -0x5828
	ctx.r[3].s64 = ctx.r[11].s64 + -22568;
	// 826AB7B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB7B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB7C0: 4BDBB661  bl 0x82466e20
	ctx.lr = 0x826AB7C4;
	sub_82466E20(ctx, base);
	// 826AB7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB7D8 size=108
    let mut pc: u32 = 0x826AB7D8;
    'dispatch: loop {
        match pc {
            0x826AB7D8 => {
    //   block [0x826AB7D8..0x826AB844)
	// 826AB7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB7E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB7EC: 38EBEAD8  addi r7, r11, -0x1528
	ctx.r[7].s64 = ctx.r[11].s64 + -5416;
	// 826AB7F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB7F4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826AB7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB7FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB808: 386AA808  addi r3, r10, -0x57f8
	ctx.r[3].s64 = ctx.r[10].s64 + -22520;
	// 826AB80C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB830: 4BDBB5F1  bl 0x82466e20
	ctx.lr = 0x826AB834;
	sub_82466E20(ctx, base);
	// 826AB834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB848 size=112
    let mut pc: u32 = 0x826AB848;
    'dispatch: loop {
        match pc {
            0x826AB848 => {
    //   block [0x826AB848..0x826AB8B8)
	// 826AB848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB85C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB864: 390BEB08  addi r8, r11, -0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + -5368;
	// 826AB868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB86C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 826AB870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB880: 386AA838  addi r3, r10, -0x57c8
	ctx.r[3].s64 = ctx.r[10].s64 + -22472;
	// 826AB884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB8A4: 4BDBB57D  bl 0x82466e20
	ctx.lr = 0x826AB8A8;
	sub_82466E20(ctx, base);
	// 826AB8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB8B8 size=112
    let mut pc: u32 = 0x826AB8B8;
    'dispatch: loop {
        match pc {
            0x826AB8B8 => {
    //   block [0x826AB8B8..0x826AB928)
	// 826AB8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB8C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB8C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB8CC: 392AD3F0  addi r9, r10, -0x2c10
	ctx.r[9].s64 = ctx.r[10].s64 + -11280;
	// 826AB8D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB8D4: 390BEB28  addi r8, r11, -0x14d8
	ctx.r[8].s64 = ctx.r[11].s64 + -5336;
	// 826AB8D8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB8DC: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826AB8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB8F0: 386AA868  addi r3, r10, -0x5798
	ctx.r[3].s64 = ctx.r[10].s64 + -22424;
	// 826AB8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB8F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB914: 4BDBB50D  bl 0x82466e20
	ctx.lr = 0x826AB918;
	sub_82466E20(ctx, base);
	// 826AB918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB928 size=112
    let mut pc: u32 = 0x826AB928;
    'dispatch: loop {
        match pc {
            0x826AB928 => {
    //   block [0x826AB928..0x826AB998)
	// 826AB928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB93C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB944: 390BEBD0  addi r8, r11, -0x1430
	ctx.r[8].s64 = ctx.r[11].s64 + -5168;
	// 826AB948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AB94C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 826AB950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB960: 386AA898  addi r3, r10, -0x5768
	ctx.r[3].s64 = ctx.r[10].s64 + -22376;
	// 826AB964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB984: 4BDBB49D  bl 0x82466e20
	ctx.lr = 0x826AB988;
	sub_82466E20(ctx, base);
	// 826AB988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB998 size=108
    let mut pc: u32 = 0x826AB998;
    'dispatch: loop {
        match pc {
            0x826AB998 => {
    //   block [0x826AB998..0x826ABA04)
	// 826AB998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB9A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB9AC: 38EBEC18  addi r7, r11, -0x13e8
	ctx.r[7].s64 = ctx.r[11].s64 + -5096;
	// 826AB9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB9B4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826AB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB9BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB9C8: 386AA8C8  addi r3, r10, -0x5738
	ctx.r[3].s64 = ctx.r[10].s64 + -22328;
	// 826AB9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB9F0: 4BDBB431  bl 0x82466e20
	ctx.lr = 0x826AB9F4;
	sub_82466E20(ctx, base);
	// 826AB9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABA08 size=112
    let mut pc: u32 = 0x826ABA08;
    'dispatch: loop {
        match pc {
            0x826ABA08 => {
    //   block [0x826ABA08..0x826ABA78)
	// 826ABA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABA14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABA1C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABA24: 390BEC48  addi r8, r11, -0x13b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5048;
	// 826ABA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABA2C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 826ABA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABA34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABA40: 386AA8F8  addi r3, r10, -0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + -22280;
	// 826ABA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABA64: 4BDBB3BD  bl 0x82466e20
	ctx.lr = 0x826ABA68;
	sub_82466E20(ctx, base);
	// 826ABA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABA78 size=112
    let mut pc: u32 = 0x826ABA78;
    'dispatch: loop {
        match pc {
            0x826ABA78 => {
    //   block [0x826ABA78..0x826ABAE8)
	// 826ABA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABA84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABA8C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABA90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABA94: 390BEC60  addi r8, r11, -0x13a0
	ctx.r[8].s64 = ctx.r[11].s64 + -5024;
	// 826ABA98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826ABA9C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 826ABAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABAA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABAB0: 386AA928  addi r3, r10, -0x56d8
	ctx.r[3].s64 = ctx.r[10].s64 + -22232;
	// 826ABAB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABAD4: 4BDBB34D  bl 0x82466e20
	ctx.lr = 0x826ABAD8;
	sub_82466E20(ctx, base);
	// 826ABAD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABAE8 size=100
    let mut pc: u32 = 0x826ABAE8;
    'dispatch: loop {
        match pc {
            0x826ABAE8 => {
    //   block [0x826ABAE8..0x826ABB4C)
	// 826ABAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABAF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABAFC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABB08: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826ABB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABB1C: 386AA958  addi r3, r10, -0x56a8
	ctx.r[3].s64 = ctx.r[10].s64 + -22184;
	// 826ABB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABB24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABB28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ABB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABB30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ABB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABB38: 4BDBB2E9  bl 0x82466e20
	ctx.lr = 0x826ABB3C;
	sub_82466E20(ctx, base);
	// 826ABB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABB50 size=112
    let mut pc: u32 = 0x826ABB50;
    'dispatch: loop {
        match pc {
            0x826ABB50 => {
    //   block [0x826ABB50..0x826ABBC0)
	// 826ABB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABB5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABB64: 38AAA508  addi r5, r10, -0x5af8
	ctx.r[5].s64 = ctx.r[10].s64 + -23288;
	// 826ABB68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABB6C: 390BED20  addi r8, r11, -0x12e0
	ctx.r[8].s64 = ctx.r[11].s64 + -4832;
	// 826ABB70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ABB74: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 826ABB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABB88: 386AA988  addi r3, r10, -0x5678
	ctx.r[3].s64 = ctx.r[10].s64 + -22136;
	// 826ABB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABBAC: 4BDBB275  bl 0x82466e20
	ctx.lr = 0x826ABBB0;
	sub_82466E20(ctx, base);
	// 826ABBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABBC0 size=112
    let mut pc: u32 = 0x826ABBC0;
    'dispatch: loop {
        match pc {
            0x826ABBC0 => {
    //   block [0x826ABBC0..0x826ABC30)
	// 826ABBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABBCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABBD0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABBD4: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826ABBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABBDC: 390BED50  addi r8, r11, -0x12b0
	ctx.r[8].s64 = ctx.r[11].s64 + -4784;
	// 826ABBE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABBE4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826ABBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABBEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABBF8: 386AA9B8  addi r3, r10, -0x5648
	ctx.r[3].s64 = ctx.r[10].s64 + -22088;
	// 826ABBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABC1C: 4BDBB205  bl 0x82466e20
	ctx.lr = 0x826ABC20;
	sub_82466E20(ctx, base);
	// 826ABC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABC30 size=108
    let mut pc: u32 = 0x826ABC30;
    'dispatch: loop {
        match pc {
            0x826ABC30 => {
    //   block [0x826ABC30..0x826ABC9C)
	// 826ABC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABC3C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABC44: 38EBED68  addi r7, r11, -0x1298
	ctx.r[7].s64 = ctx.r[11].s64 + -4760;
	// 826ABC48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ABC4C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 826ABC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABC54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABC58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABC60: 386AA9E8  addi r3, r10, -0x5618
	ctx.r[3].s64 = ctx.r[10].s64 + -22040;
	// 826ABC64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABC84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABC88: 4BDBB199  bl 0x82466e20
	ctx.lr = 0x826ABC8C;
	sub_82466E20(ctx, base);
	// 826ABC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABCA0 size=112
    let mut pc: u32 = 0x826ABCA0;
    'dispatch: loop {
        match pc {
            0x826ABCA0 => {
    //   block [0x826ABCA0..0x826ABD10)
	// 826ABCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABCAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABCB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABCB4: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826ABCB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABCBC: 390BED98  addi r8, r11, -0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + -4712;
	// 826ABCC0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826ABCC4: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 826ABCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABCCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABCD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABCD8: 386AAA18  addi r3, r10, -0x55e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21992;
	// 826ABCDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABCFC: 4BDBB125  bl 0x82466e20
	ctx.lr = 0x826ABD00;
	sub_82466E20(ctx, base);
	// 826ABD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABD10 size=112
    let mut pc: u32 = 0x826ABD10;
    'dispatch: loop {
        match pc {
            0x826ABD10 => {
    //   block [0x826ABD10..0x826ABD80)
	// 826ABD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABD1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ABD20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABD24: 392AD41C  addi r9, r10, -0x2be4
	ctx.r[9].s64 = ctx.r[10].s64 + -11236;
	// 826ABD28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABD2C: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 826ABD30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826ABD34: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 826ABD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABD3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABD48: 386AAA48  addi r3, r10, -0x55b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21944;
	// 826ABD4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABD50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ABD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABD64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABD6C: 4BDBB0B5  bl 0x82466e20
	ctx.lr = 0x826ABD70;
	sub_82466E20(ctx, base);
	// 826ABD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABD80 size=112
    let mut pc: u32 = 0x826ABD80;
    'dispatch: loop {
        match pc {
            0x826ABD80 => {
    //   block [0x826ABD80..0x826ABDF0)
	// 826ABD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABD8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABD90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABD94: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABD98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABD9C: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 826ABDA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABDA4: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 826ABDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABDAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABDB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABDB8: 386AAA78  addi r3, r10, -0x5588
	ctx.r[3].s64 = ctx.r[10].s64 + -21896;
	// 826ABDBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABDC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABDC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABDD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABDD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABDD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABDDC: 4BDBB045  bl 0x82466e20
	ctx.lr = 0x826ABDE0;
	sub_82466E20(ctx, base);
	// 826ABDE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABDF0 size=108
    let mut pc: u32 = 0x826ABDF0;
    'dispatch: loop {
        match pc {
            0x826ABDF0 => {
    //   block [0x826ABDF0..0x826ABE5C)
	// 826ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABDFC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABE00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABE04: 38EBEE88  addi r7, r11, -0x1178
	ctx.r[7].s64 = ctx.r[11].s64 + -4472;
	// 826ABE08: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826ABE0C: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 826ABE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABE14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABE18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABE20: 386AAAA8  addi r3, r10, -0x5558
	ctx.r[3].s64 = ctx.r[10].s64 + -21848;
	// 826ABE24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABE48: 4BDBAFD9  bl 0x82466e20
	ctx.lr = 0x826ABE4C;
	sub_82466E20(ctx, base);
	// 826ABE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABE60 size=116
    let mut pc: u32 = 0x826ABE60;
    'dispatch: loop {
        match pc {
            0x826ABE60 => {
    //   block [0x826ABE60..0x826ABED4)
	// 826ABE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABE6C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ABE70: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826ABE74: 390AEF18  addi r8, r10, -0x10e8
	ctx.r[8].s64 = ctx.r[10].s64 + -4328;
	// 826ABE78: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABE7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ABE80: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826ABE84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABE88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABE90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABE94: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 826ABE98: 396BD430  addi r11, r11, -0x2bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -11216;
	// 826ABE9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABEA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABEA4: 386AAAD8  addi r3, r10, -0x5528
	ctx.r[3].s64 = ctx.r[10].s64 + -21800;
	// 826ABEA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826ABEAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABEB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826ABEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABEC0: 4BDBAF61  bl 0x82466e20
	ctx.lr = 0x826ABEC4;
	sub_82466E20(ctx, base);
	// 826ABEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABED8 size=112
    let mut pc: u32 = 0x826ABED8;
    'dispatch: loop {
        match pc {
            0x826ABED8 => {
    //   block [0x826ABED8..0x826ABF48)
	// 826ABED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABEE4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ABEE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABEEC: 392AD47C  addi r9, r10, -0x2b84
	ctx.r[9].s64 = ctx.r[10].s64 + -11140;
	// 826ABEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABEF4: 390BEFF8  addi r8, r11, -0x1008
	ctx.r[8].s64 = ctx.r[11].s64 + -4104;
	// 826ABEF8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826ABEFC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 826ABF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABF04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABF10: 386AAB08  addi r3, r10, -0x54f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21752;
	// 826ABF14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABF18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ABF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABF34: 4BDBAEED  bl 0x82466e20
	ctx.lr = 0x826ABF38;
	sub_82466E20(ctx, base);
	// 826ABF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABF48 size=112
    let mut pc: u32 = 0x826ABF48;
    'dispatch: loop {
        match pc {
            0x826ABF48 => {
    //   block [0x826ABF48..0x826ABFB8)
	// 826ABF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABF54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABF5C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABF64: 390BF058  addi r8, r11, -0xfa8
	ctx.r[8].s64 = ctx.r[11].s64 + -4008;
	// 826ABF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABF6C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 826ABF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABF74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABF80: 386AAB38  addi r3, r10, -0x54c8
	ctx.r[3].s64 = ctx.r[10].s64 + -21704;
	// 826ABF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABFA4: 4BDBAE7D  bl 0x82466e20
	ctx.lr = 0x826ABFA8;
	sub_82466E20(ctx, base);
	// 826ABFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABFB8 size=108
    let mut pc: u32 = 0x826ABFB8;
    'dispatch: loop {
        match pc {
            0x826ABFB8 => {
    //   block [0x826ABFB8..0x826AC024)
	// 826ABFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABFC4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABFC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABFCC: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 826ABFD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ABFD4: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 826ABFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABFDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABFE8: 386AAB68  addi r3, r10, -0x5498
	ctx.r[3].s64 = ctx.r[10].s64 + -21656;
	// 826ABFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC010: 4BDBAE11  bl 0x82466e20
	ctx.lr = 0x826AC014;
	sub_82466E20(ctx, base);
	// 826AC014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC028 size=112
    let mut pc: u32 = 0x826AC028;
    'dispatch: loop {
        match pc {
            0x826AC028 => {
    //   block [0x826AC028..0x826AC098)
	// 826AC028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC038: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC03C: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826AC040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC044: 390BF0B8  addi r8, r11, -0xf48
	ctx.r[8].s64 = ctx.r[11].s64 + -3912;
	// 826AC048: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AC04C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 826AC050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC060: 386AAB98  addi r3, r10, -0x5468
	ctx.r[3].s64 = ctx.r[10].s64 + -21608;
	// 826AC064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC084: 4BDBAD9D  bl 0x82466e20
	ctx.lr = 0x826AC088;
	sub_82466E20(ctx, base);
	// 826AC088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC098 size=112
    let mut pc: u32 = 0x826AC098;
    'dispatch: loop {
        match pc {
            0x826AC098 => {
    //   block [0x826AC098..0x826AC108)
	// 826AC098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC0A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC0A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC0AC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC0B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC0B4: 390BF130  addi r8, r11, -0xed0
	ctx.r[8].s64 = ctx.r[11].s64 + -3792;
	// 826AC0B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC0BC: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 826AC0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC0D0: 386AABC8  addi r3, r10, -0x5438
	ctx.r[3].s64 = ctx.r[10].s64 + -21560;
	// 826AC0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC0F4: 4BDBAD2D  bl 0x82466e20
	ctx.lr = 0x826AC0F8;
	sub_82466E20(ctx, base);
	// 826AC0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC108 size=108
    let mut pc: u32 = 0x826AC108;
    'dispatch: loop {
        match pc {
            0x826AC108 => {
    //   block [0x826AC108..0x826AC174)
	// 826AC108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC114: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC11C: 38EBF160  addi r7, r11, -0xea0
	ctx.r[7].s64 = ctx.r[11].s64 + -3744;
	// 826AC120: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AC124: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 826AC128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC138: 386AABF8  addi r3, r10, -0x5408
	ctx.r[3].s64 = ctx.r[10].s64 + -21512;
	// 826AC13C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC160: 4BDBACC1  bl 0x82466e20
	ctx.lr = 0x826AC164;
	sub_82466E20(ctx, base);
	// 826AC164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC178 size=108
    let mut pc: u32 = 0x826AC178;
    'dispatch: loop {
        match pc {
            0x826AC178 => {
    //   block [0x826AC178..0x826AC1E4)
	// 826AC178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC184: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC18C: 38EBF1C0  addi r7, r11, -0xe40
	ctx.r[7].s64 = ctx.r[11].s64 + -3648;
	// 826AC190: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AC194: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 826AC198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC1A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC1A8: 386AAC28  addi r3, r10, -0x53d8
	ctx.r[3].s64 = ctx.r[10].s64 + -21464;
	// 826AC1AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC1B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC1CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC1D0: 4BDBAC51  bl 0x82466e20
	ctx.lr = 0x826AC1D4;
	sub_82466E20(ctx, base);
	// 826AC1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC1E8 size=112
    let mut pc: u32 = 0x826AC1E8;
    'dispatch: loop {
        match pc {
            0x826AC1E8 => {
    //   block [0x826AC1E8..0x826AC258)
	// 826AC1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC1F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC1F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC1FC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC204: 390BF238  addi r8, r11, -0xdc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3528;
	// 826AC208: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AC20C: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 826AC210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC220: 386AAC58  addi r3, r10, -0x53a8
	ctx.r[3].s64 = ctx.r[10].s64 + -21416;
	// 826AC224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC244: 4BDBABDD  bl 0x82466e20
	ctx.lr = 0x826AC248;
	sub_82466E20(ctx, base);
	// 826AC248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AC258 size=24
    let mut pc: u32 = 0x826AC258;
    'dispatch: loop {
        match pc {
            0x826AC258 => {
    //   block [0x826AC258..0x826AC270)
	// 826AC258: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC25C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AC260: 394A43C0  addi r10, r10, 0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + 17344;
	// 826AC264: 816BEFF4  lwz r11, -0x100c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4108 as u32) ) } as u64;
	// 826AC268: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826AC26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC270 size=116
    let mut pc: u32 = 0x826AC270;
    'dispatch: loop {
        match pc {
            0x826AC270 => {
    //   block [0x826AC270..0x826AC2E4)
	// 826AC270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC27C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC280: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC284: 390B43C0  addi r8, r11, 0x43c0
	ctx.r[8].s64 = ctx.r[11].s64 + 17344;
	// 826AC288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC28C: 392AD4C0  addi r9, r10, -0x2b40
	ctx.r[9].s64 = ctx.r[10].s64 + -11072;
	// 826AC290: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC294: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AC298: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AC29C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC2A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC2B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AC2B8: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826AC2BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AC2C0: 386BAC88  addi r3, r11, -0x5378
	ctx.r[3].s64 = ctx.r[11].s64 + -21368;
	// 826AC2C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC2C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC2D0: 4BDBAB51  bl 0x82466e20
	ctx.lr = 0x826AC2D4;
	sub_82466E20(ctx, base);
	// 826AC2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC2E8 size=112
    let mut pc: u32 = 0x826AC2E8;
    'dispatch: loop {
        match pc {
            0x826AC2E8 => {
    //   block [0x826AC2E8..0x826AC358)
	// 826AC2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC2FC: 38AAAC88  addi r5, r10, -0x5378
	ctx.r[5].s64 = ctx.r[10].s64 + -21368;
	// 826AC300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC304: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 826AC308: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC30C: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 826AC310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC320: 386AACB8  addi r3, r10, -0x5348
	ctx.r[3].s64 = ctx.r[10].s64 + -21320;
	// 826AC324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC344: 4BDBAADD  bl 0x82466e20
	ctx.lr = 0x826AC348;
	sub_82466E20(ctx, base);
	// 826AC348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AC358 size=24
    let mut pc: u32 = 0x826AC358;
    'dispatch: loop {
        match pc {
            0x826AC358 => {
    //   block [0x826AC358..0x826AC370)
	// 826AC358: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC35C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AC360: 394A43D8  addi r10, r10, 0x43d8
	ctx.r[10].s64 = ctx.r[10].s64 + 17368;
	// 826AC364: 816BF2B0  lwz r11, -0xd50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3408 as u32) ) } as u64;
	// 826AC368: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826AC36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC370 size=116
    let mut pc: u32 = 0x826AC370;
    'dispatch: loop {
        match pc {
            0x826AC370 => {
    //   block [0x826AC370..0x826AC3E4)
	// 826AC370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC37C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC380: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC384: 390B43D8  addi r8, r11, 0x43d8
	ctx.r[8].s64 = ctx.r[11].s64 + 17368;
	// 826AC388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC38C: 392AD4FC  addi r9, r10, -0x2b04
	ctx.r[9].s64 = ctx.r[10].s64 + -11012;
	// 826AC390: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC394: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AC398: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC39C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC3B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AC3B8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826AC3BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AC3C0: 386BACE8  addi r3, r11, -0x5318
	ctx.r[3].s64 = ctx.r[11].s64 + -21272;
	// 826AC3C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC3C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC3D0: 4BDBAA51  bl 0x82466e20
	ctx.lr = 0x826AC3D4;
	sub_82466E20(ctx, base);
	// 826AC3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC3E8 size=112
    let mut pc: u32 = 0x826AC3E8;
    'dispatch: loop {
        match pc {
            0x826AC3E8 => {
    //   block [0x826AC3E8..0x826AC458)
	// 826AC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC3F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC3F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC3FC: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC404: 390BF2B8  addi r8, r11, -0xd48
	ctx.r[8].s64 = ctx.r[11].s64 + -3400;
	// 826AC408: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AC40C: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 826AC410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC414: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC420: 386AAD18  addi r3, r10, -0x52e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21224;
	// 826AC424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC444: 4BDBA9DD  bl 0x82466e20
	ctx.lr = 0x826AC448;
	sub_82466E20(ctx, base);
	// 826AC448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC458 size=112
    let mut pc: u32 = 0x826AC458;
    'dispatch: loop {
        match pc {
            0x826AC458 => {
    //   block [0x826AC458..0x826AC4C8)
	// 826AC458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC468: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC46C: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC474: 390BF318  addi r8, r11, -0xce8
	ctx.r[8].s64 = ctx.r[11].s64 + -3304;
	// 826AC478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC47C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 826AC480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC484: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC490: 386AAD48  addi r3, r10, -0x52b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21176;
	// 826AC494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC4B4: 4BDBA96D  bl 0x82466e20
	ctx.lr = 0x826AC4B8;
	sub_82466E20(ctx, base);
	// 826AC4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC4C8 size=112
    let mut pc: u32 = 0x826AC4C8;
    'dispatch: loop {
        match pc {
            0x826AC4C8 => {
    //   block [0x826AC4C8..0x826AC538)
	// 826AC4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC4D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC4D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC4DC: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC4E4: 390BF348  addi r8, r11, -0xcb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3256;
	// 826AC4E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AC4EC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826AC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC500: 386AAD78  addi r3, r10, -0x5288
	ctx.r[3].s64 = ctx.r[10].s64 + -21128;
	// 826AC504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC524: 4BDBA8FD  bl 0x82466e20
	ctx.lr = 0x826AC528;
	sub_82466E20(ctx, base);
	// 826AC528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC538 size=108
    let mut pc: u32 = 0x826AC538;
    'dispatch: loop {
        match pc {
            0x826AC538 => {
    //   block [0x826AC538..0x826AC5A4)
	// 826AC538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC544: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC54C: 38EBF390  addi r7, r11, -0xc70
	ctx.r[7].s64 = ctx.r[11].s64 + -3184;
	// 826AC550: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC554: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 826AC558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC55C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC568: 386AADA8  addi r3, r10, -0x5258
	ctx.r[3].s64 = ctx.r[10].s64 + -21080;
	// 826AC56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC590: 4BDBA891  bl 0x82466e20
	ctx.lr = 0x826AC594;
	sub_82466E20(ctx, base);
	// 826AC594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC5A8 size=112
    let mut pc: u32 = 0x826AC5A8;
    'dispatch: loop {
        match pc {
            0x826AC5A8 => {
    //   block [0x826AC5A8..0x826AC618)
	// 826AC5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC5B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC5BC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC5C4: 390BF3C0  addi r8, r11, -0xc40
	ctx.r[8].s64 = ctx.r[11].s64 + -3136;
	// 826AC5C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AC5CC: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826AC5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC5E0: 386AADD8  addi r3, r10, -0x5228
	ctx.r[3].s64 = ctx.r[10].s64 + -21032;
	// 826AC5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC604: 4BDBA81D  bl 0x82466e20
	ctx.lr = 0x826AC608;
	sub_82466E20(ctx, base);
	// 826AC608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC618 size=108
    let mut pc: u32 = 0x826AC618;
    'dispatch: loop {
        match pc {
            0x826AC618 => {
    //   block [0x826AC618..0x826AC684)
	// 826AC618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC624: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC62C: 38EBF3D8  addi r7, r11, -0xc28
	ctx.r[7].s64 = ctx.r[11].s64 + -3112;
	// 826AC630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AC634: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 826AC638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC648: 386AAE08  addi r3, r10, -0x51f8
	ctx.r[3].s64 = ctx.r[10].s64 + -20984;
	// 826AC64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC670: 4BDBA7B1  bl 0x82466e20
	ctx.lr = 0x826AC674;
	sub_82466E20(ctx, base);
	// 826AC674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC688 size=108
    let mut pc: u32 = 0x826AC688;
    'dispatch: loop {
        match pc {
            0x826AC688 => {
    //   block [0x826AC688..0x826AC6F4)
	// 826AC688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC694: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC69C: 38EBF420  addi r7, r11, -0xbe0
	ctx.r[7].s64 = ctx.r[11].s64 + -3040;
	// 826AC6A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AC6A4: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 826AC6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC6B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC6B8: 386AAE38  addi r3, r10, -0x51c8
	ctx.r[3].s64 = ctx.r[10].s64 + -20936;
	// 826AC6BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC6E0: 4BDBA741  bl 0x82466e20
	ctx.lr = 0x826AC6E4;
	sub_82466E20(ctx, base);
	// 826AC6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC6F8 size=108
    let mut pc: u32 = 0x826AC6F8;
    'dispatch: loop {
        match pc {
            0x826AC6F8 => {
    //   block [0x826AC6F8..0x826AC764)
	// 826AC6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC704: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC70C: 38EBF480  addi r7, r11, -0xb80
	ctx.r[7].s64 = ctx.r[11].s64 + -2944;
	// 826AC710: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC714: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 826AC718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC720: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC728: 386AAE68  addi r3, r10, -0x5198
	ctx.r[3].s64 = ctx.r[10].s64 + -20888;
	// 826AC72C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC74C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC750: 4BDBA6D1  bl 0x82466e20
	ctx.lr = 0x826AC754;
	sub_82466E20(ctx, base);
	// 826AC754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC768 size=116
    let mut pc: u32 = 0x826AC768;
    'dispatch: loop {
        match pc {
            0x826AC768 => {
    //   block [0x826AC768..0x826AC7DC)
	// 826AC768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC774: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AC778: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC77C: 392BD538  addi r9, r11, -0x2ac8
	ctx.r[9].s64 = ctx.r[11].s64 + -10952;
	// 826AC780: 38AAB378  addi r5, r10, -0x4c88
	ctx.r[5].s64 = ctx.r[10].s64 + -19592;
	// 826AC784: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC788: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826AC78C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826AC790: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC794: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 826AC798: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC79C: 396BF4B0  addi r11, r11, -0xb50
	ctx.r[11].s64 = ctx.r[11].s64 + -2896;
	// 826AC7A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AC7A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC7A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AC7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC7B0: 386AAE98  addi r3, r10, -0x5168
	ctx.r[3].s64 = ctx.r[10].s64 + -20840;
	// 826AC7B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC7B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AC7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC7C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AC7C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC7C8: 4BDBA659  bl 0x82466e20
	ctx.lr = 0x826AC7CC;
	sub_82466E20(ctx, base);
	// 826AC7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC7E0 size=100
    let mut pc: u32 = 0x826AC7E0;
    'dispatch: loop {
        match pc {
            0x826AC7E0 => {
    //   block [0x826AC7E0..0x826AC844)
	// 826AC7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC7EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC7F4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AC7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC800: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826AC804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC814: 386AAEC8  addi r3, r10, -0x5138
	ctx.r[3].s64 = ctx.r[10].s64 + -20792;
	// 826AC818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC81C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC830: 4BDBA5F1  bl 0x82466e20
	ctx.lr = 0x826AC834;
	sub_82466E20(ctx, base);
	// 826AC834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC848 size=100
    let mut pc: u32 = 0x826AC848;
    'dispatch: loop {
        match pc {
            0x826AC848 => {
    //   block [0x826AC848..0x826AC8AC)
	// 826AC848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC85C: 38AAAF58  addi r5, r10, -0x50a8
	ctx.r[5].s64 = ctx.r[10].s64 + -20648;
	// 826AC860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC868: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 826AC86C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC87C: 386AAEF8  addi r3, r10, -0x5108
	ctx.r[3].s64 = ctx.r[10].s64 + -20744;
	// 826AC880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC888: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC890: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC898: 4BDBA589  bl 0x82466e20
	ctx.lr = 0x826AC89C;
	sub_82466E20(ctx, base);
	// 826AC89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC8B0 size=100
    let mut pc: u32 = 0x826AC8B0;
    'dispatch: loop {
        match pc {
            0x826AC8B0 => {
    //   block [0x826AC8B0..0x826AC914)
	// 826AC8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC8BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC8C4: 38AAAE98  addi r5, r10, -0x5168
	ctx.r[5].s64 = ctx.r[10].s64 + -20840;
	// 826AC8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC8CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC8D0: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826AC8D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC8E4: 386AAF28  addi r3, r10, -0x50d8
	ctx.r[3].s64 = ctx.r[10].s64 + -20696;
	// 826AC8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC8EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC8F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC8F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC900: 4BDBA521  bl 0x82466e20
	ctx.lr = 0x826AC904;
	sub_82466E20(ctx, base);
	// 826AC904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC918 size=104
    let mut pc: u32 = 0x826AC918;
    'dispatch: loop {
        match pc {
            0x826AC918 => {
    //   block [0x826AC918..0x826AC980)
	// 826AC918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC92C: 392AD5B8  addi r9, r10, -0x2a48
	ctx.r[9].s64 = ctx.r[10].s64 + -10824;
	// 826AC930: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC938: 38AAAEC8  addi r5, r10, -0x5138
	ctx.r[5].s64 = ctx.r[10].s64 + -20792;
	// 826AC93C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC94C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 826AC950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC958: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC960: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC964: 386AAF58  addi r3, r10, -0x50a8
	ctx.r[3].s64 = ctx.r[10].s64 + -20648;
	// 826AC968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC96C: 4BDBA4B5  bl 0x82466e20
	ctx.lr = 0x826AC970;
	sub_82466E20(ctx, base);
	// 826AC970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC980 size=108
    let mut pc: u32 = 0x826AC980;
    'dispatch: loop {
        match pc {
            0x826AC980 => {
    //   block [0x826AC980..0x826AC9EC)
	// 826AC980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC98C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC994: 38EBF64C  addi r7, r11, -0x9b4
	ctx.r[7].s64 = ctx.r[11].s64 + -2484;
	// 826AC998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC99C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 826AC9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC9A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC9A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC9B0: 386AAF88  addi r3, r10, -0x5078
	ctx.r[3].s64 = ctx.r[10].s64 + -20600;
	// 826AC9B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC9CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC9D8: 4BDBA449  bl 0x82466e20
	ctx.lr = 0x826AC9DC;
	sub_82466E20(ctx, base);
	// 826AC9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC9F0 size=112
    let mut pc: u32 = 0x826AC9F0;
    'dispatch: loop {
        match pc {
            0x826AC9F0 => {
    //   block [0x826AC9F0..0x826ACA60)
	// 826AC9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC9FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA04: 38AAAF58  addi r5, r10, -0x50a8
	ctx.r[5].s64 = ctx.r[10].s64 + -20648;
	// 826ACA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACA0C: 390BF680  addi r8, r11, -0x980
	ctx.r[8].s64 = ctx.r[11].s64 + -2432;
	// 826ACA10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826ACA14: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 826ACA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACA1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACA28: 386AAFB8  addi r3, r10, -0x5048
	ctx.r[3].s64 = ctx.r[10].s64 + -20552;
	// 826ACA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACA4C: 4BDBA3D5  bl 0x82466e20
	ctx.lr = 0x826ACA50;
	sub_82466E20(ctx, base);
	// 826ACA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ACA60 size=24
    let mut pc: u32 = 0x826ACA60;
    'dispatch: loop {
        match pc {
            0x826ACA60 => {
    //   block [0x826ACA60..0x826ACA78)
	// 826ACA60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA64: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ACA68: 394A4450  addi r10, r10, 0x4450
	ctx.r[10].s64 = ctx.r[10].s64 + 17488;
	// 826ACA6C: 816BF67C  lwz r11, -0x984(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2436 as u32) ) } as u64;
	// 826ACA70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826ACA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACA78 size=116
    let mut pc: u32 = 0x826ACA78;
    'dispatch: loop {
        match pc {
            0x826ACA78 => {
    //   block [0x826ACA78..0x826ACAEC)
	// 826ACA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACA84: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ACA8C: 390B4450  addi r8, r11, 0x4450
	ctx.r[8].s64 = ctx.r[11].s64 + 17488;
	// 826ACA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACA94: 392AD628  addi r9, r10, -0x29d8
	ctx.r[9].s64 = ctx.r[10].s64 + -10712;
	// 826ACA98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA9C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826ACAA0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ACAA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACAAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACABC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826ACAC0: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826ACAC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ACAC8: 386BAFE8  addi r3, r11, -0x5018
	ctx.r[3].s64 = ctx.r[11].s64 + -20504;
	// 826ACACC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ACAD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACAD8: 4BDBA349  bl 0x82466e20
	ctx.lr = 0x826ACADC;
	sub_82466E20(ctx, base);
	// 826ACADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACAF0 size=100
    let mut pc: u32 = 0x826ACAF0;
    'dispatch: loop {
        match pc {
            0x826ACAF0 => {
    //   block [0x826ACAF0..0x826ACB54)
	// 826ACAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACAFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACB04: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACB08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACB10: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 826ACB14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACB24: 386AB018  addi r3, r10, -0x4fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -20456;
	// 826ACB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACB2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACB30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACB38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACB40: 4BDBA2E1  bl 0x82466e20
	ctx.lr = 0x826ACB44;
	sub_82466E20(ctx, base);
	// 826ACB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACB58 size=100
    let mut pc: u32 = 0x826ACB58;
    'dispatch: loop {
        match pc {
            0x826ACB58 => {
    //   block [0x826ACB58..0x826ACBBC)
	// 826ACB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACB64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACB6C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACB70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACB78: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 826ACB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACB8C: 386AB048  addi r3, r10, -0x4fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -20408;
	// 826ACB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACB94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACB98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACBA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACBA8: 4BDBA279  bl 0x82466e20
	ctx.lr = 0x826ACBAC;
	sub_82466E20(ctx, base);
	// 826ACBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACBC0 size=100
    let mut pc: u32 = 0x826ACBC0;
    'dispatch: loop {
        match pc {
            0x826ACBC0 => {
    //   block [0x826ACBC0..0x826ACC24)
	// 826ACBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACBCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACBD4: 38AAB0A8  addi r5, r10, -0x4f58
	ctx.r[5].s64 = ctx.r[10].s64 + -20312;
	// 826ACBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACBE0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826ACBE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACBF4: 386AB078  addi r3, r10, -0x4f88
	ctx.r[3].s64 = ctx.r[10].s64 + -20360;
	// 826ACBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACC10: 4BDBA211  bl 0x82466e20
	ctx.lr = 0x826ACC14;
	sub_82466E20(ctx, base);
	// 826ACC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACC28 size=100
    let mut pc: u32 = 0x826ACC28;
    'dispatch: loop {
        match pc {
            0x826ACC28 => {
    //   block [0x826ACC28..0x826ACC8C)
	// 826ACC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACC34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACC3C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACC48: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 826ACC4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACC5C: 386AB0A8  addi r3, r10, -0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + -20312;
	// 826ACC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACC78: 4BDBA1A9  bl 0x82466e20
	ctx.lr = 0x826ACC7C;
	sub_82466E20(ctx, base);
	// 826ACC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACC90 size=100
    let mut pc: u32 = 0x826ACC90;
    'dispatch: loop {
        match pc {
            0x826ACC90 => {
    //   block [0x826ACC90..0x826ACCF4)
	// 826ACC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACC9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACCA4: 38AAB0A8  addi r5, r10, -0x4f58
	ctx.r[5].s64 = ctx.r[10].s64 + -20312;
	// 826ACCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACCB0: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 826ACCB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACCC4: 386AB0D8  addi r3, r10, -0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + -20264;
	// 826ACCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACCE0: 4BDBA141  bl 0x82466e20
	ctx.lr = 0x826ACCE4;
	sub_82466E20(ctx, base);
	// 826ACCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACCF8 size=100
    let mut pc: u32 = 0x826ACCF8;
    'dispatch: loop {
        match pc {
            0x826ACCF8 => {
    //   block [0x826ACCF8..0x826ACD5C)
	// 826ACCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACD04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACD0C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACD10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACD18: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 826ACD1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACD2C: 386AB108  addi r3, r10, -0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -20216;
	// 826ACD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACD34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACD48: 4BDBA0D9  bl 0x82466e20
	ctx.lr = 0x826ACD4C;
	sub_82466E20(ctx, base);
	// 826ACD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACD60 size=100
    let mut pc: u32 = 0x826ACD60;
    'dispatch: loop {
        match pc {
            0x826ACD60 => {
    //   block [0x826ACD60..0x826ACDC4)
	// 826ACD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACD6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACD74: 38AAB018  addi r5, r10, -0x4fe8
	ctx.r[5].s64 = ctx.r[10].s64 + -20456;
	// 826ACD78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACD80: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 826ACD84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACD94: 386AB138  addi r3, r10, -0x4ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -20168;
	// 826ACD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACD9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACDA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACDA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACDB0: 4BDBA071  bl 0x82466e20
	ctx.lr = 0x826ACDB4;
	sub_82466E20(ctx, base);
	// 826ACDB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACDB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACDBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACDC8 size=100
    let mut pc: u32 = 0x826ACDC8;
    'dispatch: loop {
        match pc {
            0x826ACDC8 => {
    //   block [0x826ACDC8..0x826ACE2C)
	// 826ACDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACDD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACDDC: 38AAB108  addi r5, r10, -0x4ef8
	ctx.r[5].s64 = ctx.r[10].s64 + -20216;
	// 826ACDE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACDE8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 826ACDEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACDFC: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 826ACE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACE04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACE08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACE10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACE18: 4BDBA009  bl 0x82466e20
	ctx.lr = 0x826ACE1C;
	sub_82466E20(ctx, base);
	// 826ACE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACE30 size=100
    let mut pc: u32 = 0x826ACE30;
    'dispatch: loop {
        match pc {
            0x826ACE30 => {
    //   block [0x826ACE30..0x826ACE94)
	// 826ACE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACE44: 38AAB018  addi r5, r10, -0x4fe8
	ctx.r[5].s64 = ctx.r[10].s64 + -20456;
	// 826ACE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACE50: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 826ACE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACE64: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 826ACE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACE6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACE70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACE78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACE80: 4BDB9FA1  bl 0x82466e20
	ctx.lr = 0x826ACE84;
	sub_82466E20(ctx, base);
	// 826ACE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACE98 size=112
    let mut pc: u32 = 0x826ACE98;
    'dispatch: loop {
        match pc {
            0x826ACE98 => {
    //   block [0x826ACE98..0x826ACF08)
	// 826ACE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACEA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACEAC: 38AAB228  addi r5, r10, -0x4dd8
	ctx.r[5].s64 = ctx.r[10].s64 + -19928;
	// 826ACEB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACEB4: 390BF728  addi r8, r11, -0x8d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2264;
	// 826ACEB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ACEBC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 826ACEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACEC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACED0: 386AB1C8  addi r3, r10, -0x4e38
	ctx.r[3].s64 = ctx.r[10].s64 + -20024;
	// 826ACED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACEF4: 4BDB9F2D  bl 0x82466e20
	ctx.lr = 0x826ACEF8;
	sub_82466E20(ctx, base);
	// 826ACEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACF08 size=112
    let mut pc: u32 = 0x826ACF08;
    'dispatch: loop {
        match pc {
            0x826ACF08 => {
    //   block [0x826ACF08..0x826ACF78)
	// 826ACF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACF14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACF1C: 38AAB258  addi r5, r10, -0x4da8
	ctx.r[5].s64 = ctx.r[10].s64 + -19880;
	// 826ACF20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACF24: 390BF758  addi r8, r11, -0x8a8
	ctx.r[8].s64 = ctx.r[11].s64 + -2216;
	// 826ACF28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ACF2C: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 826ACF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACF34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACF40: 386AB1F8  addi r3, r10, -0x4e08
	ctx.r[3].s64 = ctx.r[10].s64 + -19976;
	// 826ACF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACF64: 4BDB9EBD  bl 0x82466e20
	ctx.lr = 0x826ACF68;
	sub_82466E20(ctx, base);
	// 826ACF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACF78 size=112
    let mut pc: u32 = 0x826ACF78;
    'dispatch: loop {
        match pc {
            0x826ACF78 => {
    //   block [0x826ACF78..0x826ACFE8)
	// 826ACF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACF84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACF88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACF8C: 38AAB378  addi r5, r10, -0x4c88
	ctx.r[5].s64 = ctx.r[10].s64 + -19592;
	// 826ACF90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACF94: 390BF770  addi r8, r11, -0x890
	ctx.r[8].s64 = ctx.r[11].s64 + -2192;
	// 826ACF98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ACF9C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 826ACFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACFA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACFB0: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 826ACFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACFD4: 4BDB9E4D  bl 0x82466e20
	ctx.lr = 0x826ACFD8;
	sub_82466E20(ctx, base);
	// 826ACFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACFE8 size=112
    let mut pc: u32 = 0x826ACFE8;
    'dispatch: loop {
        match pc {
            0x826ACFE8 => {
    //   block [0x826ACFE8..0x826AD058)
	// 826ACFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACFF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACFF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACFFC: 38AAB228  addi r5, r10, -0x4dd8
	ctx.r[5].s64 = ctx.r[10].s64 + -19928;
	// 826AD000: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD004: 390BF7A0  addi r8, r11, -0x860
	ctx.r[8].s64 = ctx.r[11].s64 + -2144;
	// 826AD008: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD00C: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 826AD010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD014: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD020: 386AB258  addi r3, r10, -0x4da8
	ctx.r[3].s64 = ctx.r[10].s64 + -19880;
	// 826AD024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD044: 4BDB9DDD  bl 0x82466e20
	ctx.lr = 0x826AD048;
	sub_82466E20(ctx, base);
	// 826AD048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD058 size=108
    let mut pc: u32 = 0x826AD058;
    'dispatch: loop {
        match pc {
            0x826AD058 => {
    //   block [0x826AD058..0x826AD0C4)
	// 826AD058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD064: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD06C: 38EBF7B8  addi r7, r11, -0x848
	ctx.r[7].s64 = ctx.r[11].s64 + -2120;
	// 826AD070: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD074: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 826AD078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD07C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD088: 386AB288  addi r3, r10, -0x4d78
	ctx.r[3].s64 = ctx.r[10].s64 + -19832;
	// 826AD08C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD0B0: 4BDB9D71  bl 0x82466e20
	ctx.lr = 0x826AD0B4;
	sub_82466E20(ctx, base);
	// 826AD0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD0C8 size=112
    let mut pc: u32 = 0x826AD0C8;
    'dispatch: loop {
        match pc {
            0x826AD0C8 => {
    //   block [0x826AD0C8..0x826AD138)
	// 826AD0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD0D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD0DC: 38AAB258  addi r5, r10, -0x4da8
	ctx.r[5].s64 = ctx.r[10].s64 + -19880;
	// 826AD0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD0E4: 390BF7D0  addi r8, r11, -0x830
	ctx.r[8].s64 = ctx.r[11].s64 + -2096;
	// 826AD0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD0EC: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 826AD0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD0F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD100: 386AB2B8  addi r3, r10, -0x4d48
	ctx.r[3].s64 = ctx.r[10].s64 + -19784;
	// 826AD104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD124: 4BDB9CFD  bl 0x82466e20
	ctx.lr = 0x826AD128;
	sub_82466E20(ctx, base);
	// 826AD128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD138 size=116
    let mut pc: u32 = 0x826AD138;
    'dispatch: loop {
        match pc {
            0x826AD138 => {
    //   block [0x826AD138..0x826AD1AC)
	// 826AD138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD144: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD148: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AD14C: 390AF7E8  addi r8, r10, -0x818
	ctx.r[8].s64 = ctx.r[10].s64 + -2072;
	// 826AD150: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD154: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AD158: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD15C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD160: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AD164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD16C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 826AD170: 396BD63C  addi r11, r11, -0x29c4
	ctx.r[11].s64 = ctx.r[11].s64 + -10692;
	// 826AD174: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD178: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD17C: 386AB2E8  addi r3, r10, -0x4d18
	ctx.r[3].s64 = ctx.r[10].s64 + -19736;
	// 826AD180: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AD184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD188: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AD18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD198: 4BDB9C89  bl 0x82466e20
	ctx.lr = 0x826AD19C;
	sub_82466E20(ctx, base);
	// 826AD19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD1B0 size=116
    let mut pc: u32 = 0x826AD1B0;
    'dispatch: loop {
        match pc {
            0x826AD1B0 => {
    //   block [0x826AD1B0..0x826AD224)
	// 826AD1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD1BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD1C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AD1C4: 392AD764  addi r9, r10, -0x289c
	ctx.r[9].s64 = ctx.r[10].s64 + -10396;
	// 826AD1C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD1CC: 38C00045  li r6, 0x45
	ctx.r[6].s64 = 69;
	// 826AD1D0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD1D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD1D8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826AD1DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD1E0: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 826AD1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD1E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD1EC: 396BF8B0  addi r11, r11, -0x750
	ctx.r[11].s64 = ctx.r[11].s64 + -1872;
	// 826AD1F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD1F8: 386AB318  addi r3, r10, -0x4ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -19688;
	// 826AD1FC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826AD200: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD208: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826AD20C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD210: 4BDB9C11  bl 0x82466e20
	ctx.lr = 0x826AD214;
	sub_82466E20(ctx, base);
	// 826AD214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AD228 size=48
    let mut pc: u32 = 0x826AD228;
    'dispatch: loop {
        match pc {
            0x826AD228 => {
    //   block [0x826AD228..0x826AD258)
	// 826AD228: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD22C: 814BFF30  lwz r10, -0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-208 as u32) ) } as u64;
	// 826AD230: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD234: 396B4558  addi r11, r11, 0x4558
	ctx.r[11].s64 = ctx.r[11].s64 + 17752;
	// 826AD238: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826AD23C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD240: 814AFF2C  lwz r10, -0xd4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-212 as u32) ) } as u64;
	// 826AD244: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826AD248: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD24C: 814AFF28  lwz r10, -0xd8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-216 as u32) ) } as u64;
	// 826AD250: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 826AD254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD258 size=116
    let mut pc: u32 = 0x826AD258;
    'dispatch: loop {
        match pc {
            0x826AD258 => {
    //   block [0x826AD258..0x826AD2CC)
	// 826AD258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD264: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AD268: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD26C: 392BD840  addi r9, r11, -0x27c0
	ctx.r[9].s64 = ctx.r[11].s64 + -10176;
	// 826AD270: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD274: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD278: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826AD27C: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 826AD280: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD284: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 826AD288: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD28C: 396B4558  addi r11, r11, 0x4558
	ctx.r[11].s64 = ctx.r[11].s64 + 17752;
	// 826AD290: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AD294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD298: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD2A0: 386AB348  addi r3, r10, -0x4cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -19640;
	// 826AD2A4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826AD2A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AD2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD2B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD2B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD2B8: 4BDB9B69  bl 0x82466e20
	ctx.lr = 0x826AD2BC;
	sub_82466E20(ctx, base);
	// 826AD2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD2D0 size=116
    let mut pc: u32 = 0x826AD2D0;
    'dispatch: loop {
        match pc {
            0x826AD2D0 => {
    //   block [0x826AD2D0..0x826AD344)
	// 826AD2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD2DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD2E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD2E4: 390BFF40  addi r8, r11, -0xc0
	ctx.r[8].s64 = ctx.r[11].s64 + -192;
	// 826AD2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD2EC: 392AD9F0  addi r9, r10, -0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + -9744;
	// 826AD2F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD2F4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AD2F8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD2FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD304: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD314: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AD318: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 826AD31C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AD320: 386BB378  addi r3, r11, -0x4c88
	ctx.r[3].s64 = ctx.r[11].s64 + -19592;
	// 826AD324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AD328: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD330: 4BDB9AF1  bl 0x82466e20
	ctx.lr = 0x826AD334;
	sub_82466E20(ctx, base);
	// 826AD334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD348 size=112
    let mut pc: u32 = 0x826AD348;
    'dispatch: loop {
        match pc {
            0x826AD348 => {
    //   block [0x826AD348..0x826AD3B8)
	// 826AD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD358: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD35C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD364: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 826AD368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD36C: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 826AD370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD380: 386AB3A8  addi r3, r10, -0x4c58
	ctx.r[3].s64 = ctx.r[10].s64 + -19544;
	// 826AD384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD3A4: 4BDB9A7D  bl 0x82466e20
	ctx.lr = 0x826AD3A8;
	sub_82466E20(ctx, base);
	// 826AD3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AD3B8 size=36
    let mut pc: u32 = 0x826AD3B8;
    'dispatch: loop {
        match pc {
            0x826AD3B8 => {
    //   block [0x826AD3B8..0x826AD3DC)
	// 826AD3B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD3BC: 814BFFEC  lwz r10, -0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) } as u64;
	// 826AD3C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD3C4: 396B4948  addi r11, r11, 0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + 18760;
	// 826AD3C8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826AD3CC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AD3D0: 814AFF3C  lwz r10, -0xc4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-196 as u32) ) } as u64;
	// 826AD3D4: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826AD3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD3E0 size=116
    let mut pc: u32 = 0x826AD3E0;
    'dispatch: loop {
        match pc {
            0x826AD3E0 => {
    //   block [0x826AD3E0..0x826AD454)
	// 826AD3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD3EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AD3F0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AD3F4: 392ADA58  addi r9, r10, -0x25a8
	ctx.r[9].s64 = ctx.r[10].s64 + -9640;
	// 826AD3F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD3FC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826AD400: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD404: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD408: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826AD40C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD410: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 826AD414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD418: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AD41C: 396B4948  addi r11, r11, 0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + 18760;
	// 826AD420: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD428: 386AB3D8  addi r3, r10, -0x4c28
	ctx.r[3].s64 = ctx.r[10].s64 + -19496;
	// 826AD42C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826AD430: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AD434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD438: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 826AD43C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AD440: 4BDB99E1  bl 0x82466e20
	ctx.lr = 0x826AD444;
	sub_82466E20(ctx, base);
	// 826AD444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD458 size=108
    let mut pc: u32 = 0x826AD458;
    'dispatch: loop {
        match pc {
            0x826AD458 => {
    //   block [0x826AD458..0x826AD4C4)
	// 826AD458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD464: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD46C: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 826AD470: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AD474: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 826AD478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD47C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD488: 386AB408  addi r3, r10, -0x4bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -19448;
	// 826AD48C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD4B0: 4BDB9971  bl 0x82466e20
	ctx.lr = 0x826AD4B4;
	sub_82466E20(ctx, base);
	// 826AD4B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD4C8 size=112
    let mut pc: u32 = 0x826AD4C8;
    'dispatch: loop {
        match pc {
            0x826AD4C8 => {
    //   block [0x826AD4C8..0x826AD538)
	// 826AD4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD4D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD4D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD4DC: 38AA9188  addi r5, r10, -0x6e78
	ctx.r[5].s64 = ctx.r[10].s64 + -28280;
	// 826AD4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD4E4: 390B0068  addi r8, r11, 0x68
	ctx.r[8].s64 = ctx.r[11].s64 + 104;
	// 826AD4E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AD4EC: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826AD4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD500: 386AB438  addi r3, r10, -0x4bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -19400;
	// 826AD504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD524: 4BDB98FD  bl 0x82466e20
	ctx.lr = 0x826AD528;
	sub_82466E20(ctx, base);
	// 826AD528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD538 size=108
    let mut pc: u32 = 0x826AD538;
    'dispatch: loop {
        match pc {
            0x826AD538 => {
    //   block [0x826AD538..0x826AD5A4)
	// 826AD538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD544: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD54C: 38EB0080  addi r7, r11, 0x80
	ctx.r[7].s64 = ctx.r[11].s64 + 128;
	// 826AD550: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD554: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 826AD558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD55C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD568: 386AB468  addi r3, r10, -0x4b98
	ctx.r[3].s64 = ctx.r[10].s64 + -19352;
	// 826AD56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD590: 4BDB9891  bl 0x82466e20
	ctx.lr = 0x826AD594;
	sub_82466E20(ctx, base);
	// 826AD594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD5A8 size=112
    let mut pc: u32 = 0x826AD5A8;
    'dispatch: loop {
        match pc {
            0x826AD5A8 => {
    //   block [0x826AD5A8..0x826AD618)
	// 826AD5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD5B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD5BC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD5C4: 390B0098  addi r8, r11, 0x98
	ctx.r[8].s64 = ctx.r[11].s64 + 152;
	// 826AD5C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AD5CC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826AD5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD5E0: 386AB498  addi r3, r10, -0x4b68
	ctx.r[3].s64 = ctx.r[10].s64 + -19304;
	// 826AD5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD604: 4BDB981D  bl 0x82466e20
	ctx.lr = 0x826AD608;
	sub_82466E20(ctx, base);
	// 826AD608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD618 size=108
    let mut pc: u32 = 0x826AD618;
    'dispatch: loop {
        match pc {
            0x826AD618 => {
    //   block [0x826AD618..0x826AD684)
	// 826AD618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD624: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD62C: 38EB00E0  addi r7, r11, 0xe0
	ctx.r[7].s64 = ctx.r[11].s64 + 224;
	// 826AD630: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AD634: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 826AD638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD648: 386AB4C8  addi r3, r10, -0x4b38
	ctx.r[3].s64 = ctx.r[10].s64 + -19256;
	// 826AD64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD670: 4BDB97B1  bl 0x82466e20
	ctx.lr = 0x826AD674;
	sub_82466E20(ctx, base);
	// 826AD674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD688 size=108
    let mut pc: u32 = 0x826AD688;
    'dispatch: loop {
        match pc {
            0x826AD688 => {
    //   block [0x826AD688..0x826AD6F4)
	// 826AD688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD694: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD69C: 38EB0110  addi r7, r11, 0x110
	ctx.r[7].s64 = ctx.r[11].s64 + 272;
	// 826AD6A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AD6A4: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 826AD6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD6B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AD6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD6B8: 386AB4F8  addi r3, r10, -0x4b08
	ctx.r[3].s64 = ctx.r[10].s64 + -19208;
	// 826AD6BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AD6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AD6E0: 4BDB9741  bl 0x82466e20
	ctx.lr = 0x826AD6E4;
	sub_82466E20(ctx, base);
	// 826AD6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD6F8 size=112
    let mut pc: u32 = 0x826AD6F8;
    'dispatch: loop {
        match pc {
            0x826AD6F8 => {
    //   block [0x826AD6F8..0x826AD768)
	// 826AD6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD708: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD70C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AD710: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD714: 390B0128  addi r8, r11, 0x128
	ctx.r[8].s64 = ctx.r[11].s64 + 296;
	// 826AD718: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AD71C: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 826AD720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD730: 386AB528  addi r3, r10, -0x4ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -19160;
	// 826AD734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD754: 4BDB96CD  bl 0x82466e20
	ctx.lr = 0x826AD758;
	sub_82466E20(ctx, base);
	// 826AD758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD768 size=112
    let mut pc: u32 = 0x826AD768;
    'dispatch: loop {
        match pc {
            0x826AD768 => {
    //   block [0x826AD768..0x826AD7D8)
	// 826AD768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD778: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD77C: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD780: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD784: 390B0158  addi r8, r11, 0x158
	ctx.r[8].s64 = ctx.r[11].s64 + 344;
	// 826AD788: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AD78C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 826AD790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD7A0: 386AB558  addi r3, r10, -0x4aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -19112;
	// 826AD7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD7C4: 4BDB965D  bl 0x82466e20
	ctx.lr = 0x826AD7C8;
	sub_82466E20(ctx, base);
	// 826AD7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD7D8 size=112
    let mut pc: u32 = 0x826AD7D8;
    'dispatch: loop {
        match pc {
            0x826AD7D8 => {
    //   block [0x826AD7D8..0x826AD848)
	// 826AD7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD7E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD7E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD7EC: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD7F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD7F4: 390B01A0  addi r8, r11, 0x1a0
	ctx.r[8].s64 = ctx.r[11].s64 + 416;
	// 826AD7F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD7FC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826AD800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD810: 386AB588  addi r3, r10, -0x4a78
	ctx.r[3].s64 = ctx.r[10].s64 + -19064;
	// 826AD814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD834: 4BDB95ED  bl 0x82466e20
	ctx.lr = 0x826AD838;
	sub_82466E20(ctx, base);
	// 826AD838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD848 size=112
    let mut pc: u32 = 0x826AD848;
    'dispatch: loop {
        match pc {
            0x826AD848 => {
    //   block [0x826AD848..0x826AD8B8)
	// 826AD848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD85C: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD864: 390B0200  addi r8, r11, 0x200
	ctx.r[8].s64 = ctx.r[11].s64 + 512;
	// 826AD868: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD86C: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 826AD870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD880: 386AB5B8  addi r3, r10, -0x4a48
	ctx.r[3].s64 = ctx.r[10].s64 + -19016;
	// 826AD884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD8A4: 4BDB957D  bl 0x82466e20
	ctx.lr = 0x826AD8A8;
	sub_82466E20(ctx, base);
	// 826AD8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD8B8 size=112
    let mut pc: u32 = 0x826AD8B8;
    'dispatch: loop {
        match pc {
            0x826AD8B8 => {
    //   block [0x826AD8B8..0x826AD928)
	// 826AD8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD8C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD8C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD8CC: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD8D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD8D4: 390B0260  addi r8, r11, 0x260
	ctx.r[8].s64 = ctx.r[11].s64 + 608;
	// 826AD8D8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AD8DC: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826AD8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD8F0: 386AB5E8  addi r3, r10, -0x4a18
	ctx.r[3].s64 = ctx.r[10].s64 + -18968;
	// 826AD8F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD914: 4BDB950D  bl 0x82466e20
	ctx.lr = 0x826AD918;
	sub_82466E20(ctx, base);
	// 826AD918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD928 size=112
    let mut pc: u32 = 0x826AD928;
    'dispatch: loop {
        match pc {
            0x826AD928 => {
    //   block [0x826AD928..0x826AD998)
	// 826AD928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD93C: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AD940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD944: 390B0320  addi r8, r11, 0x320
	ctx.r[8].s64 = ctx.r[11].s64 + 800;
	// 826AD948: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AD94C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 826AD950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD960: 386AB618  addi r3, r10, -0x49e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18920;
	// 826AD964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD984: 4BDB949D  bl 0x82466e20
	ctx.lr = 0x826AD988;
	sub_82466E20(ctx, base);
	// 826AD988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AD990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AD994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AD998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AD998 size=112
    let mut pc: u32 = 0x826AD998;
    'dispatch: loop {
        match pc {
            0x826AD998 => {
    //   block [0x826AD998..0x826ADA08)
	// 826AD998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AD99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AD9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AD9A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD9A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AD9AC: 38AAA3E8  addi r5, r10, -0x5c18
	ctx.r[5].s64 = ctx.r[10].s64 + -23576;
	// 826AD9B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AD9B4: 390B0380  addi r8, r11, 0x380
	ctx.r[8].s64 = ctx.r[11].s64 + 896;
	// 826AD9B8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AD9BC: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826AD9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AD9C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AD9C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AD9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AD9D0: 386AB648  addi r3, r10, -0x49b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18872;
	// 826AD9D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AD9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AD9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AD9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AD9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AD9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AD9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AD9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AD9F4: 4BDB942D  bl 0x82466e20
	ctx.lr = 0x826AD9F8;
	sub_82466E20(ctx, base);
	// 826AD9F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AD9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADA08 size=112
    let mut pc: u32 = 0x826ADA08;
    'dispatch: loop {
        match pc {
            0x826ADA08 => {
    //   block [0x826ADA08..0x826ADA78)
	// 826ADA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADA14: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ADA18: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826ADA1C: 38EA0440  addi r7, r10, 0x440
	ctx.r[7].s64 = ctx.r[10].s64 + 1088;
	// 826ADA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADA24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ADA28: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 826ADA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADA30: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADA34: 396BDA80  addi r11, r11, -0x2580
	ctx.r[11].s64 = ctx.r[11].s64 + -9600;
	// 826ADA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADA3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADA40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADA44: 386AB678  addi r3, r10, -0x4988
	ctx.r[3].s64 = ctx.r[10].s64 + -18824;
	// 826ADA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADA4C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826ADA50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADA54: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826ADA58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADA5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADA60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADA64: 4BDB93BD  bl 0x82466e20
	ctx.lr = 0x826ADA68;
	sub_82466E20(ctx, base);
	// 826ADA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADA78 size=112
    let mut pc: u32 = 0x826ADA78;
    'dispatch: loop {
        match pc {
            0x826ADA78 => {
    //   block [0x826ADA78..0x826ADAE8)
	// 826ADA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADA84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADA88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADA8C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADA90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADA94: 390B0608  addi r8, r11, 0x608
	ctx.r[8].s64 = ctx.r[11].s64 + 1544;
	// 826ADA98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADA9C: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 826ADAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADAA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADAB0: 386AB6A8  addi r3, r10, -0x4958
	ctx.r[3].s64 = ctx.r[10].s64 + -18776;
	// 826ADAB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADAC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADAD4: 4BDB934D  bl 0x82466e20
	ctx.lr = 0x826ADAD8;
	sub_82466E20(ctx, base);
	// 826ADAD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADAE8 size=108
    let mut pc: u32 = 0x826ADAE8;
    'dispatch: loop {
        match pc {
            0x826ADAE8 => {
    //   block [0x826ADAE8..0x826ADB54)
	// 826ADAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADAF4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADAF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADAFC: 38EB0620  addi r7, r11, 0x620
	ctx.r[7].s64 = ctx.r[11].s64 + 1568;
	// 826ADB00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADB04: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826ADB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADB18: 386AB6D8  addi r3, r10, -0x4928
	ctx.r[3].s64 = ctx.r[10].s64 + -18728;
	// 826ADB1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADB40: 4BDB92E1  bl 0x82466e20
	ctx.lr = 0x826ADB44;
	sub_82466E20(ctx, base);
	// 826ADB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADB58 size=112
    let mut pc: u32 = 0x826ADB58;
    'dispatch: loop {
        match pc {
            0x826ADB58 => {
    //   block [0x826ADB58..0x826ADBC8)
	// 826ADB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADB64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADB6C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADB70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADB74: 390B0650  addi r8, r11, 0x650
	ctx.r[8].s64 = ctx.r[11].s64 + 1616;
	// 826ADB78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADB7C: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826ADB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADB84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADB90: 386AB708  addi r3, r10, -0x48f8
	ctx.r[3].s64 = ctx.r[10].s64 + -18680;
	// 826ADB94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADB98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADBA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADBA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADBA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADBB4: 4BDB926D  bl 0x82466e20
	ctx.lr = 0x826ADBB8;
	sub_82466E20(ctx, base);
	// 826ADBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADBC8 size=108
    let mut pc: u32 = 0x826ADBC8;
    'dispatch: loop {
        match pc {
            0x826ADBC8 => {
    //   block [0x826ADBC8..0x826ADC34)
	// 826ADBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADBD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADBDC: 38EB0668  addi r7, r11, 0x668
	ctx.r[7].s64 = ctx.r[11].s64 + 1640;
	// 826ADBE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADBE4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826ADBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADBEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADBF8: 386AB738  addi r3, r10, -0x48c8
	ctx.r[3].s64 = ctx.r[10].s64 + -18632;
	// 826ADBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADC20: 4BDB9201  bl 0x82466e20
	ctx.lr = 0x826ADC24;
	sub_82466E20(ctx, base);
	// 826ADC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADC38 size=108
    let mut pc: u32 = 0x826ADC38;
    'dispatch: loop {
        match pc {
            0x826ADC38 => {
    //   block [0x826ADC38..0x826ADCA4)
	// 826ADC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADC44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADC48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADC4C: 38EB0698  addi r7, r11, 0x698
	ctx.r[7].s64 = ctx.r[11].s64 + 1688;
	// 826ADC50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ADC54: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826ADC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADC5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADC60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADC68: 386AB768  addi r3, r10, -0x4898
	ctx.r[3].s64 = ctx.r[10].s64 + -18584;
	// 826ADC6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADC8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADC90: 4BDB9191  bl 0x82466e20
	ctx.lr = 0x826ADC94;
	sub_82466E20(ctx, base);
	// 826ADC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADCA8 size=112
    let mut pc: u32 = 0x826ADCA8;
    'dispatch: loop {
        match pc {
            0x826ADCA8 => {
    //   block [0x826ADCA8..0x826ADD18)
	// 826ADCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADCB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADCB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADCBC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ADCC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADCC4: 390B06E0  addi r8, r11, 0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1760;
	// 826ADCC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ADCCC: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826ADCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADCD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADCD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADCDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADCE0: 386AB798  addi r3, r10, -0x4868
	ctx.r[3].s64 = ctx.r[10].s64 + -18536;
	// 826ADCE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADD04: 4BDB911D  bl 0x82466e20
	ctx.lr = 0x826ADD08;
	sub_82466E20(ctx, base);
	// 826ADD08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADD18 size=112
    let mut pc: u32 = 0x826ADD18;
    'dispatch: loop {
        match pc {
            0x826ADD18 => {
    //   block [0x826ADD18..0x826ADD88)
	// 826ADD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD28: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADD2C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADD30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADD34: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 826ADD38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADD3C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826ADD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADD44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADD4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADD50: 386AB7C8  addi r3, r10, -0x4838
	ctx.r[3].s64 = ctx.r[10].s64 + -18488;
	// 826ADD54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADD64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ADD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADD74: 4BDB90AD  bl 0x82466e20
	ctx.lr = 0x826ADD78;
	sub_82466E20(ctx, base);
	// 826ADD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADD88 size=112
    let mut pc: u32 = 0x826ADD88;
    'dispatch: loop {
        match pc {
            0x826ADD88 => {
    //   block [0x826ADD88..0x826ADDF8)
	// 826ADD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADD94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADD98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADD9C: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826ADDA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADDA4: 390B0740  addi r8, r11, 0x740
	ctx.r[8].s64 = ctx.r[11].s64 + 1856;
	// 826ADDA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ADDAC: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826ADDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADDB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADDC0: 386AB7F8  addi r3, r10, -0x4808
	ctx.r[3].s64 = ctx.r[10].s64 + -18440;
	// 826ADDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADDE4: 4BDB903D  bl 0x82466e20
	ctx.lr = 0x826ADDE8;
	sub_82466E20(ctx, base);
	// 826ADDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADDF8 size=112
    let mut pc: u32 = 0x826ADDF8;
    'dispatch: loop {
        match pc {
            0x826ADDF8 => {
    //   block [0x826ADDF8..0x826ADE68)
	// 826ADDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADE04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADE0C: 38AAB2E8  addi r5, r10, -0x4d18
	ctx.r[5].s64 = ctx.r[10].s64 + -19736;
	// 826ADE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADE14: 390B0770  addi r8, r11, 0x770
	ctx.r[8].s64 = ctx.r[11].s64 + 1904;
	// 826ADE18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ADE1C: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826ADE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADE24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADE30: 386AB828  addi r3, r10, -0x47d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18392;
	// 826ADE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADE54: 4BDB8FCD  bl 0x82466e20
	ctx.lr = 0x826ADE58;
	sub_82466E20(ctx, base);
	// 826ADE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADE68 size=108
    let mut pc: u32 = 0x826ADE68;
    'dispatch: loop {
        match pc {
            0x826ADE68 => {
    //   block [0x826ADE68..0x826ADED4)
	// 826ADE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADE74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADE78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADE7C: 38EB0788  addi r7, r11, 0x788
	ctx.r[7].s64 = ctx.r[11].s64 + 1928;
	// 826ADE80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ADE84: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826ADE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADE8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADE90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ADE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADE98: 386AB858  addi r3, r10, -0x47a8
	ctx.r[3].s64 = ctx.r[10].s64 + -18344;
	// 826ADE9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ADEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADEBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ADEC0: 4BDB8F61  bl 0x82466e20
	ctx.lr = 0x826ADEC4;
	sub_82466E20(ctx, base);
	// 826ADEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADED8 size=112
    let mut pc: u32 = 0x826ADED8;
    'dispatch: loop {
        match pc {
            0x826ADED8 => {
    //   block [0x826ADED8..0x826ADF48)
	// 826ADED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADEE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADEE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADEEC: 38AAB858  addi r5, r10, -0x47a8
	ctx.r[5].s64 = ctx.r[10].s64 + -18344;
	// 826ADEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADEF4: 390B07B8  addi r8, r11, 0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1976;
	// 826ADEF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ADEFC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826ADF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADF04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADF10: 386AB888  addi r3, r10, -0x4778
	ctx.r[3].s64 = ctx.r[10].s64 + -18296;
	// 826ADF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ADF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ADF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ADF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADF34: 4BDB8EED  bl 0x82466e20
	ctx.lr = 0x826ADF38;
	sub_82466E20(ctx, base);
	// 826ADF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ADF48 size=24
    let mut pc: u32 = 0x826ADF48;
    'dispatch: loop {
        match pc {
            0x826ADF48 => {
    //   block [0x826ADF48..0x826ADF60)
	// 826ADF48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADF4C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ADF50: 394A4A08  addi r10, r10, 0x4a08
	ctx.r[10].s64 = ctx.r[10].s64 + 18952;
	// 826ADF54: 816B07E8  lwz r11, 0x7e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2024 as u32) ) } as u64;
	// 826ADF58: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826ADF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADF60 size=116
    let mut pc: u32 = 0x826ADF60;
    'dispatch: loop {
        match pc {
            0x826ADF60 => {
    //   block [0x826ADF60..0x826ADFD4)
	// 826ADF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADF6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADF70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ADF74: 390B4A08  addi r8, r11, 0x4a08
	ctx.r[8].s64 = ctx.r[11].s64 + 18952;
	// 826ADF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADF7C: 392ADB20  addi r9, r10, -0x24e0
	ctx.r[9].s64 = ctx.r[10].s64 + -9440;
	// 826ADF80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ADF84: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826ADF88: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ADF8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ADF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ADF94: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ADF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ADFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ADFA4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826ADFA8: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826ADFAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ADFB0: 386BB8B8  addi r3, r11, -0x4748
	ctx.r[3].s64 = ctx.r[11].s64 + -18248;
	// 826ADFB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ADFB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ADFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ADFC0: 4BDB8E61  bl 0x82466e20
	ctx.lr = 0x826ADFC4;
	sub_82466E20(ctx, base);
	// 826ADFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ADFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ADFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ADFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ADFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ADFD8 size=108
    let mut pc: u32 = 0x826ADFD8;
    'dispatch: loop {
        match pc {
            0x826ADFD8 => {
    //   block [0x826ADFD8..0x826AE044)
	// 826ADFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ADFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ADFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ADFE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ADFE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ADFEC: 38EB07F0  addi r7, r11, 0x7f0
	ctx.r[7].s64 = ctx.r[11].s64 + 2032;
	// 826ADFF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ADFF4: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826ADFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ADFFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE008: 386AB8E8  addi r3, r10, -0x4718
	ctx.r[3].s64 = ctx.r[10].s64 + -18200;
	// 826AE00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE030: 4BDB8DF1  bl 0x82466e20
	ctx.lr = 0x826AE034;
	sub_82466E20(ctx, base);
	// 826AE034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE048 size=108
    let mut pc: u32 = 0x826AE048;
    'dispatch: loop {
        match pc {
            0x826AE048 => {
    //   block [0x826AE048..0x826AE0B4)
	// 826AE048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE054: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE058: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE05C: 38EB0838  addi r7, r11, 0x838
	ctx.r[7].s64 = ctx.r[11].s64 + 2104;
	// 826AE060: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE064: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826AE068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE078: 386AB918  addi r3, r10, -0x46e8
	ctx.r[3].s64 = ctx.r[10].s64 + -18152;
	// 826AE07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE0A0: 4BDB8D81  bl 0x82466e20
	ctx.lr = 0x826AE0A4;
	sub_82466E20(ctx, base);
	// 826AE0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE0B8 size=108
    let mut pc: u32 = 0x826AE0B8;
    'dispatch: loop {
        match pc {
            0x826AE0B8 => {
    //   block [0x826AE0B8..0x826AE124)
	// 826AE0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE0C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE0C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE0CC: 38EB0868  addi r7, r11, 0x868
	ctx.r[7].s64 = ctx.r[11].s64 + 2152;
	// 826AE0D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE0D4: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826AE0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE0E8: 386AB948  addi r3, r10, -0x46b8
	ctx.r[3].s64 = ctx.r[10].s64 + -18104;
	// 826AE0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE110: 4BDB8D11  bl 0x82466e20
	ctx.lr = 0x826AE114;
	sub_82466E20(ctx, base);
	// 826AE114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE128 size=112
    let mut pc: u32 = 0x826AE128;
    'dispatch: loop {
        match pc {
            0x826AE128 => {
    //   block [0x826AE128..0x826AE198)
	// 826AE128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE138: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE13C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE140: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE144: 390B0898  addi r8, r11, 0x898
	ctx.r[8].s64 = ctx.r[11].s64 + 2200;
	// 826AE148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AE14C: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826AE150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE160: 386AB978  addi r3, r10, -0x4688
	ctx.r[3].s64 = ctx.r[10].s64 + -18056;
	// 826AE164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE184: 4BDB8C9D  bl 0x82466e20
	ctx.lr = 0x826AE188;
	sub_82466E20(ctx, base);
	// 826AE188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE198 size=112
    let mut pc: u32 = 0x826AE198;
    'dispatch: loop {
        match pc {
            0x826AE198 => {
    //   block [0x826AE198..0x826AE208)
	// 826AE198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE1A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE1AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE1B4: 390B08C8  addi r8, r11, 0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + 2248;
	// 826AE1B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE1BC: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826AE1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE1D0: 386AB9A8  addi r3, r10, -0x4658
	ctx.r[3].s64 = ctx.r[10].s64 + -18008;
	// 826AE1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE1F4: 4BDB8C2D  bl 0x82466e20
	ctx.lr = 0x826AE1F8;
	sub_82466E20(ctx, base);
	// 826AE1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE208 size=112
    let mut pc: u32 = 0x826AE208;
    'dispatch: loop {
        match pc {
            0x826AE208 => {
    //   block [0x826AE208..0x826AE278)
	// 826AE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE218: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE21C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE224: 390B08E0  addi r8, r11, 0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2272;
	// 826AE228: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE22C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826AE230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE240: 386AB9D8  addi r3, r10, -0x4628
	ctx.r[3].s64 = ctx.r[10].s64 + -17960;
	// 826AE244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE264: 4BDB8BBD  bl 0x82466e20
	ctx.lr = 0x826AE268;
	sub_82466E20(ctx, base);
	// 826AE268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE278 size=108
    let mut pc: u32 = 0x826AE278;
    'dispatch: loop {
        match pc {
            0x826AE278 => {
    //   block [0x826AE278..0x826AE2E4)
	// 826AE278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE284: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE28C: 38EB08F8  addi r7, r11, 0x8f8
	ctx.r[7].s64 = ctx.r[11].s64 + 2296;
	// 826AE290: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AE294: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826AE298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE29C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE2A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE2A8: 386ABA08  addi r3, r10, -0x45f8
	ctx.r[3].s64 = ctx.r[10].s64 + -17912;
	// 826AE2AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE2D0: 4BDB8B51  bl 0x82466e20
	ctx.lr = 0x826AE2D4;
	sub_82466E20(ctx, base);
	// 826AE2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE2E8 size=112
    let mut pc: u32 = 0x826AE2E8;
    'dispatch: loop {
        match pc {
            0x826AE2E8 => {
    //   block [0x826AE2E8..0x826AE358)
	// 826AE2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE2FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE304: 390B0928  addi r8, r11, 0x928
	ctx.r[8].s64 = ctx.r[11].s64 + 2344;
	// 826AE308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AE30C: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826AE310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE320: 386ABA38  addi r3, r10, -0x45c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17864;
	// 826AE324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE344: 4BDB8ADD  bl 0x82466e20
	ctx.lr = 0x826AE348;
	sub_82466E20(ctx, base);
	// 826AE348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE358 size=108
    let mut pc: u32 = 0x826AE358;
    'dispatch: loop {
        match pc {
            0x826AE358 => {
    //   block [0x826AE358..0x826AE3C4)
	// 826AE358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE364: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE36C: 38EB0940  addi r7, r11, 0x940
	ctx.r[7].s64 = ctx.r[11].s64 + 2368;
	// 826AE370: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826AE374: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826AE378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE37C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE388: 386ABA68  addi r3, r10, -0x4598
	ctx.r[3].s64 = ctx.r[10].s64 + -17816;
	// 826AE38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE3B0: 4BDB8A71  bl 0x82466e20
	ctx.lr = 0x826AE3B4;
	sub_82466E20(ctx, base);
	// 826AE3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE3C8 size=116
    let mut pc: u32 = 0x826AE3C8;
    'dispatch: loop {
        match pc {
            0x826AE3C8 => {
    //   block [0x826AE3C8..0x826AE43C)
	// 826AE3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE3D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AE3D8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826AE3DC: 390A0A30  addi r8, r10, 0xa30
	ctx.r[8].s64 = ctx.r[10].s64 + 2608;
	// 826AE3E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE3E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AE3E8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE3EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE3F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AE3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE3FC: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826AE400: 396BDB38  addi r11, r11, -0x24c8
	ctx.r[11].s64 = ctx.r[11].s64 + -9416;
	// 826AE404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE40C: 386ABA98  addi r3, r10, -0x4568
	ctx.r[3].s64 = ctx.r[10].s64 + -17768;
	// 826AE410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AE414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AE41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE428: 4BDB89F9  bl 0x82466e20
	ctx.lr = 0x826AE42C;
	sub_82466E20(ctx, base);
	// 826AE42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE440 size=108
    let mut pc: u32 = 0x826AE440;
    'dispatch: loop {
        match pc {
            0x826AE440 => {
    //   block [0x826AE440..0x826AE4AC)
	// 826AE440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE44C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE454: 38EB0BF8  addi r7, r11, 0xbf8
	ctx.r[7].s64 = ctx.r[11].s64 + 3064;
	// 826AE458: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826AE45C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826AE460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE470: 386ABAC8  addi r3, r10, -0x4538
	ctx.r[3].s64 = ctx.r[10].s64 + -17720;
	// 826AE474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE498: 4BDB8989  bl 0x82466e20
	ctx.lr = 0x826AE49C;
	sub_82466E20(ctx, base);
	// 826AE49C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE4A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE4A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE4B0 size=112
    let mut pc: u32 = 0x826AE4B0;
    'dispatch: loop {
        match pc {
            0x826AE4B0 => {
    //   block [0x826AE4B0..0x826AE520)
	// 826AE4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE4BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE4C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE4C4: 38AAA418  addi r5, r10, -0x5be8
	ctx.r[5].s64 = ctx.r[10].s64 + -23528;
	// 826AE4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE4CC: 390B0D90  addi r8, r11, 0xd90
	ctx.r[8].s64 = ctx.r[11].s64 + 3472;
	// 826AE4D0: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826AE4D4: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826AE4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE4DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE4E8: 386ABAF8  addi r3, r10, -0x4508
	ctx.r[3].s64 = ctx.r[10].s64 + -17672;
	// 826AE4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE50C: 4BDB8915  bl 0x82466e20
	ctx.lr = 0x826AE510;
	sub_82466E20(ctx, base);
	// 826AE510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE520 size=100
    let mut pc: u32 = 0x826AE520;
    'dispatch: loop {
        match pc {
            0x826AE520 => {
    //   block [0x826AE520..0x826AE584)
	// 826AE520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE52C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE534: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE540: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826AE544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE554: 386ABB28  addi r3, r10, -0x44d8
	ctx.r[3].s64 = ctx.r[10].s64 + -17624;
	// 826AE558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE55C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE570: 4BDB88B1  bl 0x82466e20
	ctx.lr = 0x826AE574;
	sub_82466E20(ctx, base);
	// 826AE574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE588 size=112
    let mut pc: u32 = 0x826AE588;
    'dispatch: loop {
        match pc {
            0x826AE588 => {
    //   block [0x826AE588..0x826AE5F8)
	// 826AE588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE598: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE59C: 38AABB28  addi r5, r10, -0x44d8
	ctx.r[5].s64 = ctx.r[10].s64 + -17624;
	// 826AE5A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE5A4: 390B0FE8  addi r8, r11, 0xfe8
	ctx.r[8].s64 = ctx.r[11].s64 + 4072;
	// 826AE5A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AE5AC: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826AE5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE5C0: 386ABB58  addi r3, r10, -0x44a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17576;
	// 826AE5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE5E4: 4BDB883D  bl 0x82466e20
	ctx.lr = 0x826AE5E8;
	sub_82466E20(ctx, base);
	// 826AE5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE5F8 size=100
    let mut pc: u32 = 0x826AE5F8;
    'dispatch: loop {
        match pc {
            0x826AE5F8 => {
    //   block [0x826AE5F8..0x826AE65C)
	// 826AE5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE60C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE610: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE618: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826AE61C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE62C: 386ABB88  addi r3, r10, -0x4478
	ctx.r[3].s64 = ctx.r[10].s64 + -17528;
	// 826AE630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE634: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE638: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE640: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE648: 4BDB87D9  bl 0x82466e20
	ctx.lr = 0x826AE64C;
	sub_82466E20(ctx, base);
	// 826AE64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE660 size=108
    let mut pc: u32 = 0x826AE660;
    'dispatch: loop {
        match pc {
            0x826AE660 => {
    //   block [0x826AE660..0x826AE6CC)
	// 826AE660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE66C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE674: 38EB1060  addi r7, r11, 0x1060
	ctx.r[7].s64 = ctx.r[11].s64 + 4192;
	// 826AE678: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AE67C: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826AE680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE690: 386ABBB8  addi r3, r10, -0x4448
	ctx.r[3].s64 = ctx.r[10].s64 + -17480;
	// 826AE694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AE698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AE6B8: 4BDB8769  bl 0x82466e20
	ctx.lr = 0x826AE6BC;
	sub_82466E20(ctx, base);
	// 826AE6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE6D0 size=112
    let mut pc: u32 = 0x826AE6D0;
    'dispatch: loop {
        match pc {
            0x826AE6D0 => {
    //   block [0x826AE6D0..0x826AE740)
	// 826AE6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE6DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE6E0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE6E4: 38AABB88  addi r5, r10, -0x4478
	ctx.r[5].s64 = ctx.r[10].s64 + -17528;
	// 826AE6E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE6EC: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 826AE6F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AE6F4: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826AE6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE6FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE708: 386ABBE8  addi r3, r10, -0x4418
	ctx.r[3].s64 = ctx.r[10].s64 + -17432;
	// 826AE70C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE72C: 4BDB86F5  bl 0x82466e20
	ctx.lr = 0x826AE730;
	sub_82466E20(ctx, base);
	// 826AE730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE740 size=100
    let mut pc: u32 = 0x826AE740;
    'dispatch: loop {
        match pc {
            0x826AE740 => {
    //   block [0x826AE740..0x826AE7A4)
	// 826AE740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE74C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE754: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE758: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE760: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826AE764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE774: 386ABC18  addi r3, r10, -0x43e8
	ctx.r[3].s64 = ctx.r[10].s64 + -17384;
	// 826AE778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE780: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE788: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE790: 4BDB8691  bl 0x82466e20
	ctx.lr = 0x826AE794;
	sub_82466E20(ctx, base);
	// 826AE794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE7A8 size=100
    let mut pc: u32 = 0x826AE7A8;
    'dispatch: loop {
        match pc {
            0x826AE7A8 => {
    //   block [0x826AE7A8..0x826AE80C)
	// 826AE7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE7B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE7BC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE7C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE7C8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826AE7CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE7DC: 386ABC48  addi r3, r10, -0x43b8
	ctx.r[3].s64 = ctx.r[10].s64 + -17336;
	// 826AE7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE7E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE7F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE7F8: 4BDB8629  bl 0x82466e20
	ctx.lr = 0x826AE7FC;
	sub_82466E20(ctx, base);
	// 826AE7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE810 size=112
    let mut pc: u32 = 0x826AE810;
    'dispatch: loop {
        match pc {
            0x826AE810 => {
    //   block [0x826AE810..0x826AE880)
	// 826AE810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE81C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE820: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE824: 38AABC18  addi r5, r10, -0x43e8
	ctx.r[5].s64 = ctx.r[10].s64 + -17384;
	// 826AE828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE82C: 390B10D8  addi r8, r11, 0x10d8
	ctx.r[8].s64 = ctx.r[11].s64 + 4312;
	// 826AE830: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AE834: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826AE838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE83C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE848: 386ABC78  addi r3, r10, -0x4388
	ctx.r[3].s64 = ctx.r[10].s64 + -17288;
	// 826AE84C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE86C: 4BDB85B5  bl 0x82466e20
	ctx.lr = 0x826AE870;
	sub_82466E20(ctx, base);
	// 826AE870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE880 size=112
    let mut pc: u32 = 0x826AE880;
    'dispatch: loop {
        match pc {
            0x826AE880 => {
    //   block [0x826AE880..0x826AE8F0)
	// 826AE880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE88C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE890: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE894: 38AABC48  addi r5, r10, -0x43b8
	ctx.r[5].s64 = ctx.r[10].s64 + -17336;
	// 826AE898: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE89C: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 826AE8A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AE8A4: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826AE8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE8AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE8B8: 386ABCA8  addi r3, r10, -0x4358
	ctx.r[3].s64 = ctx.r[10].s64 + -17240;
	// 826AE8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE8DC: 4BDB8545  bl 0x82466e20
	ctx.lr = 0x826AE8E0;
	sub_82466E20(ctx, base);
	// 826AE8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE8F0 size=100
    let mut pc: u32 = 0x826AE8F0;
    'dispatch: loop {
        match pc {
            0x826AE8F0 => {
    //   block [0x826AE8F0..0x826AE954)
	// 826AE8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE8FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE904: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AE908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE910: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826AE914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE924: 386ABCD8  addi r3, r10, -0x4328
	ctx.r[3].s64 = ctx.r[10].s64 + -17192;
	// 826AE928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AE934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AE93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE940: 4BDB84E1  bl 0x82466e20
	ctx.lr = 0x826AE944;
	sub_82466E20(ctx, base);
	// 826AE944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE958 size=112
    let mut pc: u32 = 0x826AE958;
    'dispatch: loop {
        match pc {
            0x826AE958 => {
    //   block [0x826AE958..0x826AE9C8)
	// 826AE958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE968: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE96C: 38AABCD8  addi r5, r10, -0x4328
	ctx.r[5].s64 = ctx.r[10].s64 + -17192;
	// 826AE970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE974: 390B1198  addi r8, r11, 0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + 4504;
	// 826AE978: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826AE97C: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826AE980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AE98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AE990: 386ABD08  addi r3, r10, -0x42f8
	ctx.r[3].s64 = ctx.r[10].s64 + -17144;
	// 826AE994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AE998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AE99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AE9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AE9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AE9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AE9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AE9B4: 4BDB846D  bl 0x82466e20
	ctx.lr = 0x826AE9B8;
	sub_82466E20(ctx, base);
	// 826AE9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AE9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AE9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AE9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AE9C8 size=108
    let mut pc: u32 = 0x826AE9C8;
    'dispatch: loop {
        match pc {
            0x826AE9C8 => {
    //   block [0x826AE9C8..0x826AEA34)
	// 826AE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AE9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AE9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AE9D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AE9D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AE9DC: 38EB1288  addi r7, r11, 0x1288
	ctx.r[7].s64 = ctx.r[11].s64 + 4744;
	// 826AE9E0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826AE9E4: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826AE9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AE9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AE9F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AE9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AE9F8: 386ABD38  addi r3, r10, -0x42c8
	ctx.r[3].s64 = ctx.r[10].s64 + -17096;
	// 826AE9FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEA20: 4BDB8401  bl 0x82466e20
	ctx.lr = 0x826AEA24;
	sub_82466E20(ctx, base);
	// 826AEA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEA38 size=108
    let mut pc: u32 = 0x826AEA38;
    'dispatch: loop {
        match pc {
            0x826AEA38 => {
    //   block [0x826AEA38..0x826AEAA4)
	// 826AEA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEA44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEA48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEA4C: 38EB1378  addi r7, r11, 0x1378
	ctx.r[7].s64 = ctx.r[11].s64 + 4984;
	// 826AEA50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AEA54: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826AEA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEA5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEA68: 386ABD68  addi r3, r10, -0x4298
	ctx.r[3].s64 = ctx.r[10].s64 + -17048;
	// 826AEA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEA90: 4BDB8391  bl 0x82466e20
	ctx.lr = 0x826AEA94;
	sub_82466E20(ctx, base);
	// 826AEA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEAA8 size=108
    let mut pc: u32 = 0x826AEAA8;
    'dispatch: loop {
        match pc {
            0x826AEAA8 => {
    //   block [0x826AEAA8..0x826AEB14)
	// 826AEAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEAB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEAB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEABC: 38EB13C0  addi r7, r11, 0x13c0
	ctx.r[7].s64 = ctx.r[11].s64 + 5056;
	// 826AEAC0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826AEAC4: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826AEAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEACC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEAD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEAD8: 386ABD98  addi r3, r10, -0x4268
	ctx.r[3].s64 = ctx.r[10].s64 + -17000;
	// 826AEADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEAFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEB00: 4BDB8321  bl 0x82466e20
	ctx.lr = 0x826AEB04;
	sub_82466E20(ctx, base);
	// 826AEB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEB18 size=108
    let mut pc: u32 = 0x826AEB18;
    'dispatch: loop {
        match pc {
            0x826AEB18 => {
    //   block [0x826AEB18..0x826AEB84)
	// 826AEB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEB24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEB2C: 38EB1498  addi r7, r11, 0x1498
	ctx.r[7].s64 = ctx.r[11].s64 + 5272;
	// 826AEB30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AEB34: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826AEB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEB3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEB40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEB48: 386ABDC8  addi r3, r10, -0x4238
	ctx.r[3].s64 = ctx.r[10].s64 + -16952;
	// 826AEB4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEB70: 4BDB82B1  bl 0x82466e20
	ctx.lr = 0x826AEB74;
	sub_82466E20(ctx, base);
	// 826AEB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEB88 size=100
    let mut pc: u32 = 0x826AEB88;
    'dispatch: loop {
        match pc {
            0x826AEB88 => {
    //   block [0x826AEB88..0x826AEBEC)
	// 826AEB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEB94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEB9C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEBA8: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826AEBAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEBBC: 386ABDF8  addi r3, r10, -0x4208
	ctx.r[3].s64 = ctx.r[10].s64 + -16904;
	// 826AEBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEBC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEBC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEBD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AEBD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEBD8: 4BDB8249  bl 0x82466e20
	ctx.lr = 0x826AEBDC;
	sub_82466E20(ctx, base);
	// 826AEBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEBF0 size=112
    let mut pc: u32 = 0x826AEBF0;
    'dispatch: loop {
        match pc {
            0x826AEBF0 => {
    //   block [0x826AEBF0..0x826AEC60)
	// 826AEBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEBFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEC04: 38AABDF8  addi r5, r10, -0x4208
	ctx.r[5].s64 = ctx.r[10].s64 + -16904;
	// 826AEC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEC0C: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 826AEC10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEC14: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826AEC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEC1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEC28: 386ABE28  addi r3, r10, -0x41d8
	ctx.r[3].s64 = ctx.r[10].s64 + -16856;
	// 826AEC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEC4C: 4BDB81D5  bl 0x82466e20
	ctx.lr = 0x826AEC50;
	sub_82466E20(ctx, base);
	// 826AEC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEC60 size=108
    let mut pc: u32 = 0x826AEC60;
    'dispatch: loop {
        match pc {
            0x826AEC60 => {
    //   block [0x826AEC60..0x826AECCC)
	// 826AEC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEC6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEC70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEC74: 38EB14F8  addi r7, r11, 0x14f8
	ctx.r[7].s64 = ctx.r[11].s64 + 5368;
	// 826AEC78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AEC7C: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826AEC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEC84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEC90: 386ABE58  addi r3, r10, -0x41a8
	ctx.r[3].s64 = ctx.r[10].s64 + -16808;
	// 826AEC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AECA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AECA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AECA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AECAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AECB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AECB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AECB8: 4BDB8169  bl 0x82466e20
	ctx.lr = 0x826AECBC;
	sub_82466E20(ctx, base);
	// 826AECBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AECC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AECC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AECC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AECD0 size=112
    let mut pc: u32 = 0x826AECD0;
    'dispatch: loop {
        match pc {
            0x826AECD0 => {
    //   block [0x826AECD0..0x826AED40)
	// 826AECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AECD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AECDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AECE0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AECE4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AECE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AECEC: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 826AECF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AECF4: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826AECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AECFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AED00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AED04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AED08: 386ABE88  addi r3, r10, -0x4178
	ctx.r[3].s64 = ctx.r[10].s64 + -16760;
	// 826AED0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AED10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AED14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AED18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AED1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AED20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AED24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AED28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AED2C: 4BDB80F5  bl 0x82466e20
	ctx.lr = 0x826AED30;
	sub_82466E20(ctx, base);
	// 826AED30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AED34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AED38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AED3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AED40 size=108
    let mut pc: u32 = 0x826AED40;
    'dispatch: loop {
        match pc {
            0x826AED40 => {
    //   block [0x826AED40..0x826AEDAC)
	// 826AED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AED48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AED4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AED50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AED54: 38EB1558  addi r7, r11, 0x1558
	ctx.r[7].s64 = ctx.r[11].s64 + 5464;
	// 826AED58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AED5C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826AED60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AED64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AED68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AED70: 386ABEB8  addi r3, r10, -0x4148
	ctx.r[3].s64 = ctx.r[10].s64 + -16712;
	// 826AED74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AED78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AED7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AED80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AED84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AED88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AED8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AED90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AED94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AED98: 4BDB8089  bl 0x82466e20
	ctx.lr = 0x826AED9C;
	sub_82466E20(ctx, base);
	// 826AED9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEDA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEDA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEDB0 size=112
    let mut pc: u32 = 0x826AEDB0;
    'dispatch: loop {
        match pc {
            0x826AEDB0 => {
    //   block [0x826AEDB0..0x826AEE20)
	// 826AEDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEDBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEDC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEDC4: 38AABE88  addi r5, r10, -0x4178
	ctx.r[5].s64 = ctx.r[10].s64 + -16760;
	// 826AEDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEDCC: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826AEDD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AEDD4: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826AEDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEDDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEDE8: 386ABEE8  addi r3, r10, -0x4118
	ctx.r[3].s64 = ctx.r[10].s64 + -16664;
	// 826AEDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEE0C: 4BDB8015  bl 0x82466e20
	ctx.lr = 0x826AEE10;
	sub_82466E20(ctx, base);
	// 826AEE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEE20 size=100
    let mut pc: u32 = 0x826AEE20;
    'dispatch: loop {
        match pc {
            0x826AEE20 => {
    //   block [0x826AEE20..0x826AEE84)
	// 826AEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEE2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEE34: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEE40: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826AEE44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEE54: 386ABF18  addi r3, r10, -0x40e8
	ctx.r[3].s64 = ctx.r[10].s64 + -16616;
	// 826AEE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEE5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AEE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AEE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEE70: 4BDB7FB1  bl 0x82466e20
	ctx.lr = 0x826AEE74;
	sub_82466E20(ctx, base);
	// 826AEE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEE88 size=112
    let mut pc: u32 = 0x826AEE88;
    'dispatch: loop {
        match pc {
            0x826AEE88 => {
    //   block [0x826AEE88..0x826AEEF8)
	// 826AEE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEE94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEE98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEE9C: 38AABF18  addi r5, r10, -0x40e8
	ctx.r[5].s64 = ctx.r[10].s64 + -16616;
	// 826AEEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEEA4: 390B15B8  addi r8, r11, 0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5560;
	// 826AEEA8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AEEAC: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826AEEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEEB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEEC0: 386ABF48  addi r3, r10, -0x40b8
	ctx.r[3].s64 = ctx.r[10].s64 + -16568;
	// 826AEEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEEE4: 4BDB7F3D  bl 0x82466e20
	ctx.lr = 0x826AEEE8;
	sub_82466E20(ctx, base);
	// 826AEEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEEF8 size=108
    let mut pc: u32 = 0x826AEEF8;
    'dispatch: loop {
        match pc {
            0x826AEEF8 => {
    //   block [0x826AEEF8..0x826AEF64)
	// 826AEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEF04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEF08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEF0C: 38EB1660  addi r7, r11, 0x1660
	ctx.r[7].s64 = ctx.r[11].s64 + 5728;
	// 826AEF10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AEF14: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826AEF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEF1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AEF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEF28: 386ABF78  addi r3, r10, -0x4088
	ctx.r[3].s64 = ctx.r[10].s64 + -16520;
	// 826AEF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AEF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AEF50: 4BDB7ED1  bl 0x82466e20
	ctx.lr = 0x826AEF54;
	sub_82466E20(ctx, base);
	// 826AEF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEF68 size=112
    let mut pc: u32 = 0x826AEF68;
    'dispatch: loop {
        match pc {
            0x826AEF68 => {
    //   block [0x826AEF68..0x826AEFD8)
	// 826AEF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEF74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEF7C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEF80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AEF84: 390B1690  addi r8, r11, 0x1690
	ctx.r[8].s64 = ctx.r[11].s64 + 5776;
	// 826AEF88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEF8C: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826AEF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AEF94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEF98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AEF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AEFA0: 386ABFA8  addi r3, r10, -0x4058
	ctx.r[3].s64 = ctx.r[10].s64 + -16472;
	// 826AEFA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AEFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AEFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AEFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AEFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AEFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AEFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AEFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AEFC4: 4BDB7E5D  bl 0x82466e20
	ctx.lr = 0x826AEFC8;
	sub_82466E20(ctx, base);
	// 826AEFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AEFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AEFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AEFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AEFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AEFD8 size=112
    let mut pc: u32 = 0x826AEFD8;
    'dispatch: loop {
        match pc {
            0x826AEFD8 => {
    //   block [0x826AEFD8..0x826AF048)
	// 826AEFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AEFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AEFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AEFE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AEFE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AEFEC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AEFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AEFF4: 390B16D8  addi r8, r11, 0x16d8
	ctx.r[8].s64 = ctx.r[11].s64 + 5848;
	// 826AEFF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AEFFC: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826AF000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF010: 386ABFD8  addi r3, r10, -0x4028
	ctx.r[3].s64 = ctx.r[10].s64 + -16424;
	// 826AF014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF034: 4BDB7DED  bl 0x82466e20
	ctx.lr = 0x826AF038;
	sub_82466E20(ctx, base);
	// 826AF038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF048 size=100
    let mut pc: u32 = 0x826AF048;
    'dispatch: loop {
        match pc {
            0x826AF048 => {
    //   block [0x826AF048..0x826AF0AC)
	// 826AF048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF05C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF060: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF068: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826AF06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF07C: 386AC008  addi r3, r10, -0x3ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -16376;
	// 826AF080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AF08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AF094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF098: 4BDB7D89  bl 0x82466e20
	ctx.lr = 0x826AF09C;
	sub_82466E20(ctx, base);
	// 826AF09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF0B0 size=112
    let mut pc: u32 = 0x826AF0B0;
    'dispatch: loop {
        match pc {
            0x826AF0B0 => {
    //   block [0x826AF0B0..0x826AF120)
	// 826AF0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF0BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF0C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF0C4: 38AAC008  addi r5, r10, -0x3ff8
	ctx.r[5].s64 = ctx.r[10].s64 + -16376;
	// 826AF0C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF0CC: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 826AF0D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AF0D4: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826AF0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF0E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF0E8: 386AC038  addi r3, r10, -0x3fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -16328;
	// 826AF0EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF0F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF0FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF10C: 4BDB7D15  bl 0x82466e20
	ctx.lr = 0x826AF110;
	sub_82466E20(ctx, base);
	// 826AF110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF120 size=112
    let mut pc: u32 = 0x826AF120;
    'dispatch: loop {
        match pc {
            0x826AF120 => {
    //   block [0x826AF120..0x826AF190)
	// 826AF120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF130: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF134: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF13C: 390B1768  addi r8, r11, 0x1768
	ctx.r[8].s64 = ctx.r[11].s64 + 5992;
	// 826AF140: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF144: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826AF148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF158: 386AC068  addi r3, r10, -0x3f98
	ctx.r[3].s64 = ctx.r[10].s64 + -16280;
	// 826AF15C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF17C: 4BDB7CA5  bl 0x82466e20
	ctx.lr = 0x826AF180;
	sub_82466E20(ctx, base);
	// 826AF180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF190 size=112
    let mut pc: u32 = 0x826AF190;
    'dispatch: loop {
        match pc {
            0x826AF190 => {
    //   block [0x826AF190..0x826AF200)
	// 826AF190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF1A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF1A4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AF1A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF1AC: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 826AF1B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF1B4: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826AF1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF1BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF1C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF1C8: 386AC098  addi r3, r10, -0x3f68
	ctx.r[3].s64 = ctx.r[10].s64 + -16232;
	// 826AF1CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF1DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AF1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF1EC: 4BDB7C35  bl 0x82466e20
	ctx.lr = 0x826AF1F0;
	sub_82466E20(ctx, base);
	// 826AF1F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF200 size=112
    let mut pc: u32 = 0x826AF200;
    'dispatch: loop {
        match pc {
            0x826AF200 => {
    //   block [0x826AF200..0x826AF270)
	// 826AF200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF210: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF214: 38AAC068  addi r5, r10, -0x3f98
	ctx.r[5].s64 = ctx.r[10].s64 + -16280;
	// 826AF218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AF21C: 390B1798  addi r8, r11, 0x1798
	ctx.r[8].s64 = ctx.r[11].s64 + 6040;
	// 826AF220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AF224: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826AF228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF22C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF238: 386AC0C8  addi r3, r10, -0x3f38
	ctx.r[3].s64 = ctx.r[10].s64 + -16184;
	// 826AF23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF25C: 4BDB7BC5  bl 0x82466e20
	ctx.lr = 0x826AF260;
	sub_82466E20(ctx, base);
	// 826AF260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF270 size=72
    let mut pc: u32 = 0x826AF270;
    'dispatch: loop {
        match pc {
            0x826AF270 => {
    //   block [0x826AF270..0x826AF2B8)
	// 826AF270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF27C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF280: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826AF284: 38CBB410  addi r6, r11, -0x4bf0
	ctx.r[6].s64 = ctx.r[11].s64 + -19440;
	// 826AF288: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF28C: 388BDB88  addi r4, r11, -0x2478
	ctx.r[4].s64 = ctx.r[11].s64 + -9336;
	// 826AF290: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AF294: 386BC0F8  addi r3, r11, -0x3f08
	ctx.r[3].s64 = ctx.r[11].s64 + -16136;
	// 826AF298: 4BDCC7F1  bl 0x8247ba88
	ctx.lr = 0x826AF29C;
	sub_8247BA88(ctx, base);
	// 826AF29C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826AF2A0: 386BCEA0  addi r3, r11, -0x3160
	ctx.r[3].s64 = ctx.r[11].s64 + -12640;
	// 826AF2A4: 4BE83895  bl 0x82532b38
	ctx.lr = 0x826AF2A8;
	sub_82532B38(ctx, base);
	// 826AF2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826AF2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF2B8 size=108
    let mut pc: u32 = 0x826AF2B8;
    'dispatch: loop {
        match pc {
            0x826AF2B8 => {
    //   block [0x826AF2B8..0x826AF324)
	// 826AF2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF2C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF2CC: 38EB5498  addi r7, r11, 0x5498
	ctx.r[7].s64 = ctx.r[11].s64 + 21656;
	// 826AF2D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF2D4: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826AF2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF2DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF2E8: 386AC114  addi r3, r10, -0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + -16108;
	// 826AF2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF310: 4BDB7B11  bl 0x82466e20
	ctx.lr = 0x826AF314;
	sub_82466E20(ctx, base);
	// 826AF314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AF328 size=24
    let mut pc: u32 = 0x826AF328;
    'dispatch: loop {
        match pc {
            0x826AF328 => {
    //   block [0x826AF328..0x826AF340)
	// 826AF328: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF32C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826AF330: 394AB670  addi r10, r10, -0x4990
	ctx.r[10].s64 = ctx.r[10].s64 + -18832;
	// 826AF334: 816B5510  lwz r11, 0x5510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21776 as u32) ) } as u64;
	// 826AF338: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826AF33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF340 size=112
    let mut pc: u32 = 0x826AF340;
    'dispatch: loop {
        match pc {
            0x826AF340 => {
    //   block [0x826AF340..0x826AF3B0)
	// 826AF340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF34C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AF350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF354: 392BECA4  addi r9, r11, -0x135c
	ctx.r[9].s64 = ctx.r[11].s64 + -4956;
	// 826AF358: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826AF35C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826AF360: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826AF364: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826AF368: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF36C: 396BB670  addi r11, r11, -0x4990
	ctx.r[11].s64 = ctx.r[11].s64 + -18832;
	// 826AF370: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AF374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF378: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AF37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF380: 386AC144  addi r3, r10, -0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -16060;
	// 826AF384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AF388: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AF38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF390: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AF394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF398: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AF39C: 4BDB7A85  bl 0x82466e20
	ctx.lr = 0x826AF3A0;
	sub_82466E20(ctx, base);
	// 826AF3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF3B0 size=108
    let mut pc: u32 = 0x826AF3B0;
    'dispatch: loop {
        match pc {
            0x826AF3B0 => {
    //   block [0x826AF3B0..0x826AF41C)
	// 826AF3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF3BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF3C4: 38EB5514  addi r7, r11, 0x5514
	ctx.r[7].s64 = ctx.r[11].s64 + 21780;
	// 826AF3C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AF3CC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826AF3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF3E0: 386AC174  addi r3, r10, -0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16012;
	// 826AF3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF408: 4BDB7A19  bl 0x82466e20
	ctx.lr = 0x826AF40C;
	sub_82466E20(ctx, base);
	// 826AF40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF420 size=108
    let mut pc: u32 = 0x826AF420;
    'dispatch: loop {
        match pc {
            0x826AF420 => {
    //   block [0x826AF420..0x826AF48C)
	// 826AF420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF42C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF434: 38EB5544  addi r7, r11, 0x5544
	ctx.r[7].s64 = ctx.r[11].s64 + 21828;
	// 826AF438: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AF43C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826AF440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF450: 386AC1A4  addi r3, r10, -0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15964;
	// 826AF454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF478: 4BDB79A9  bl 0x82466e20
	ctx.lr = 0x826AF47C;
	sub_82466E20(ctx, base);
	// 826AF47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF490 size=112
    let mut pc: u32 = 0x826AF490;
    'dispatch: loop {
        match pc {
            0x826AF490 => {
    //   block [0x826AF490..0x826AF500)
	// 826AF490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF4A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF4A4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF4AC: 390B5578  addi r8, r11, 0x5578
	ctx.r[8].s64 = ctx.r[11].s64 + 21880;
	// 826AF4B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AF4B4: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826AF4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF4BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF4C8: 386AC1D4  addi r3, r10, -0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15916;
	// 826AF4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF4EC: 4BDB7935  bl 0x82466e20
	ctx.lr = 0x826AF4F0;
	sub_82466E20(ctx, base);
	// 826AF4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF500 size=108
    let mut pc: u32 = 0x826AF500;
    'dispatch: loop {
        match pc {
            0x826AF500 => {
    //   block [0x826AF500..0x826AF56C)
	// 826AF500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF50C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF514: 38EB55D8  addi r7, r11, 0x55d8
	ctx.r[7].s64 = ctx.r[11].s64 + 21976;
	// 826AF518: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF51C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826AF520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF530: 386AC204  addi r3, r10, -0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15868;
	// 826AF534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF558: 4BDB78C9  bl 0x82466e20
	ctx.lr = 0x826AF55C;
	sub_82466E20(ctx, base);
	// 826AF55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF570 size=112
    let mut pc: u32 = 0x826AF570;
    'dispatch: loop {
        match pc {
            0x826AF570 => {
    //   block [0x826AF570..0x826AF5E0)
	// 826AF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF57C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF580: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF584: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF58C: 390B5650  addi r8, r11, 0x5650
	ctx.r[8].s64 = ctx.r[11].s64 + 22096;
	// 826AF590: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AF594: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826AF598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF59C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF5A8: 386AC234  addi r3, r10, -0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -15820;
	// 826AF5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF5CC: 4BDB7855  bl 0x82466e20
	ctx.lr = 0x826AF5D0;
	sub_82466E20(ctx, base);
	// 826AF5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF5E0 size=112
    let mut pc: u32 = 0x826AF5E0;
    'dispatch: loop {
        match pc {
            0x826AF5E0 => {
    //   block [0x826AF5E0..0x826AF650)
	// 826AF5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF5EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF5F0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF5F4: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF5FC: 390B56F8  addi r8, r11, 0x56f8
	ctx.r[8].s64 = ctx.r[11].s64 + 22264;
	// 826AF600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AF604: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826AF608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF60C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF618: 386AC264  addi r3, r10, -0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15772;
	// 826AF61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF63C: 4BDB77E5  bl 0x82466e20
	ctx.lr = 0x826AF640;
	sub_82466E20(ctx, base);
	// 826AF640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF650 size=108
    let mut pc: u32 = 0x826AF650;
    'dispatch: loop {
        match pc {
            0x826AF650 => {
    //   block [0x826AF650..0x826AF6BC)
	// 826AF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF65C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF664: 38EB5710  addi r7, r11, 0x5710
	ctx.r[7].s64 = ctx.r[11].s64 + 22288;
	// 826AF668: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AF66C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826AF670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF680: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 826AF684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF6A8: 4BDB7779  bl 0x82466e20
	ctx.lr = 0x826AF6AC;
	sub_82466E20(ctx, base);
	// 826AF6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF6C0 size=112
    let mut pc: u32 = 0x826AF6C0;
    'dispatch: loop {
        match pc {
            0x826AF6C0 => {
    //   block [0x826AF6C0..0x826AF730)
	// 826AF6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF6CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF6D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF6D4: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 826AF6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF6DC: 390B5788  addi r8, r11, 0x5788
	ctx.r[8].s64 = ctx.r[11].s64 + 22408;
	// 826AF6E0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AF6E4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826AF6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF6EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF6F8: 386AC2C4  addi r3, r10, -0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15676;
	// 826AF6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AF700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF71C: 4BDB7705  bl 0x82466e20
	ctx.lr = 0x826AF720;
	sub_82466E20(ctx, base);
	// 826AF720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF730 size=108
    let mut pc: u32 = 0x826AF730;
    'dispatch: loop {
        match pc {
            0x826AF730 => {
    //   block [0x826AF730..0x826AF79C)
	// 826AF730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF73C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF744: 38EB5830  addi r7, r11, 0x5830
	ctx.r[7].s64 = ctx.r[11].s64 + 22576;
	// 826AF748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AF74C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826AF750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF760: 386AC2F4  addi r3, r10, -0x3d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -15628;
	// 826AF764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF788: 4BDB7699  bl 0x82466e20
	ctx.lr = 0x826AF78C;
	sub_82466E20(ctx, base);
	// 826AF78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF7A0 size=108
    let mut pc: u32 = 0x826AF7A0;
    'dispatch: loop {
        match pc {
            0x826AF7A0 => {
    //   block [0x826AF7A0..0x826AF80C)
	// 826AF7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF7AC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF7B4: 38EB5848  addi r7, r11, 0x5848
	ctx.r[7].s64 = ctx.r[11].s64 + 22600;
	// 826AF7B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AF7BC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826AF7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF7C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF7C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF7D0: 386AC324  addi r3, r10, -0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -15580;
	// 826AF7D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF7F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF7F8: 4BDB7629  bl 0x82466e20
	ctx.lr = 0x826AF7FC;
	sub_82466E20(ctx, base);
	// 826AF7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF810 size=116
    let mut pc: u32 = 0x826AF810;
    'dispatch: loop {
        match pc {
            0x826AF810 => {
    //   block [0x826AF810..0x826AF884)
	// 826AF810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF81C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF820: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AF824: 390B58A8  addi r8, r11, 0x58a8
	ctx.r[8].s64 = ctx.r[11].s64 + 22696;
	// 826AF828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF82C: 392AECE0  addi r9, r10, -0x1320
	ctx.r[9].s64 = ctx.r[10].s64 + -4896;
	// 826AF830: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AF838: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF83C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AF840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF844: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF854: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AF858: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826AF85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AF860: 386BC354  addi r3, r11, -0x3cac
	ctx.r[3].s64 = ctx.r[11].s64 + -15532;
	// 826AF864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AF868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF870: 4BDB75B1  bl 0x82466e20
	ctx.lr = 0x826AF874;
	sub_82466E20(ctx, base);
	// 826AF874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF888 size=108
    let mut pc: u32 = 0x826AF888;
    'dispatch: loop {
        match pc {
            0x826AF888 => {
    //   block [0x826AF888..0x826AF8F4)
	// 826AF888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF894: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF89C: 38EB58C0  addi r7, r11, 0x58c0
	ctx.r[7].s64 = ctx.r[11].s64 + 22720;
	// 826AF8A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AF8A4: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826AF8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF8AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF8B8: 386AC384  addi r3, r10, -0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15484;
	// 826AF8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF8E0: 4BDB7541  bl 0x82466e20
	ctx.lr = 0x826AF8E4;
	sub_82466E20(ctx, base);
	// 826AF8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF8F8 size=108
    let mut pc: u32 = 0x826AF8F8;
    'dispatch: loop {
        match pc {
            0x826AF8F8 => {
    //   block [0x826AF8F8..0x826AF964)
	// 826AF8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF904: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF90C: 38EB5908  addi r7, r11, 0x5908
	ctx.r[7].s64 = ctx.r[11].s64 + 22792;
	// 826AF910: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826AF914: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826AF918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF91C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF928: 386AC3B4  addi r3, r10, -0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15436;
	// 826AF92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF950: 4BDB74D1  bl 0x82466e20
	ctx.lr = 0x826AF954;
	sub_82466E20(ctx, base);
	// 826AF954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF968 size=108
    let mut pc: u32 = 0x826AF968;
    'dispatch: loop {
        match pc {
            0x826AF968 => {
    //   block [0x826AF968..0x826AF9D4)
	// 826AF968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF974: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AF978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF97C: 38EB5998  addi r7, r11, 0x5998
	ctx.r[7].s64 = ctx.r[11].s64 + 22936;
	// 826AF980: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826AF984: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826AF988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF98C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AF994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF998: 386AC3E4  addi r3, r10, -0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15388;
	// 826AF99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AF9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AF9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AF9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AF9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AF9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AF9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AF9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AF9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AF9C0: 4BDB7461  bl 0x82466e20
	ctx.lr = 0x826AF9C4;
	sub_82466E20(ctx, base);
	// 826AF9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AF9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AF9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AF9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AF9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AF9D8 size=100
    let mut pc: u32 = 0x826AF9D8;
    'dispatch: loop {
        match pc {
            0x826AF9D8 => {
    //   block [0x826AF9D8..0x826AFA3C)
	// 826AF9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AF9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AF9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AF9E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AF9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AF9EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AF9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AF9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AF9F8: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826AF9FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFA0C: 386AC414  addi r3, r10, -0x3bec
	ctx.r[3].s64 = ctx.r[10].s64 + -15340;
	// 826AFA10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFA14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFA18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFA20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFA28: 4BDB73F9  bl 0x82466e20
	ctx.lr = 0x826AFA2C;
	sub_82466E20(ctx, base);
	// 826AFA2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFA40 size=112
    let mut pc: u32 = 0x826AFA40;
    'dispatch: loop {
        match pc {
            0x826AFA40 => {
    //   block [0x826AFA40..0x826AFAB0)
	// 826AFA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFA4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA50: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFA54: 38AAC414  addi r5, r10, -0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + -15340;
	// 826AFA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFA5C: 390B5A28  addi r8, r11, 0x5a28
	ctx.r[8].s64 = ctx.r[11].s64 + 23080;
	// 826AFA60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AFA64: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826AFA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFA6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFA70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFA78: 386AC444  addi r3, r10, -0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15292;
	// 826AFA7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AFA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFA9C: 4BDB7385  bl 0x82466e20
	ctx.lr = 0x826AFAA0;
	sub_82466E20(ctx, base);
	// 826AFAA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFAB0 size=108
    let mut pc: u32 = 0x826AFAB0;
    'dispatch: loop {
        match pc {
            0x826AFAB0 => {
    //   block [0x826AFAB0..0x826AFB1C)
	// 826AFAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFABC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFAC4: 38EB5A88  addi r7, r11, 0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + 23176;
	// 826AFAC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AFACC: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826AFAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFAD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFAD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFAE0: 386AC474  addi r3, r10, -0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15244;
	// 826AFAE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFB08: 4BDB7319  bl 0x82466e20
	ctx.lr = 0x826AFB0C;
	sub_82466E20(ctx, base);
	// 826AFB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFB20 size=108
    let mut pc: u32 = 0x826AFB20;
    'dispatch: loop {
        match pc {
            0x826AFB20 => {
    //   block [0x826AFB20..0x826AFB8C)
	// 826AFB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFB2C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFB34: 38EB5AB8  addi r7, r11, 0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + 23224;
	// 826AFB38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AFB3C: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826AFB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFB44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFB50: 386AC4A4  addi r3, r10, -0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15196;
	// 826AFB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFB78: 4BDB72A9  bl 0x82466e20
	ctx.lr = 0x826AFB7C;
	sub_82466E20(ctx, base);
	// 826AFB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFB90 size=108
    let mut pc: u32 = 0x826AFB90;
    'dispatch: loop {
        match pc {
            0x826AFB90 => {
    //   block [0x826AFB90..0x826AFBFC)
	// 826AFB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFB9C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFBA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFBA4: 38EB5B18  addi r7, r11, 0x5b18
	ctx.r[7].s64 = ctx.r[11].s64 + 23320;
	// 826AFBA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AFBAC: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826AFBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFBB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFBC0: 386AC4D4  addi r3, r10, -0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15148;
	// 826AFBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFBE8: 4BDB7239  bl 0x82466e20
	ctx.lr = 0x826AFBEC;
	sub_82466E20(ctx, base);
	// 826AFBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFC00 size=112
    let mut pc: u32 = 0x826AFC00;
    'dispatch: loop {
        match pc {
            0x826AFC00 => {
    //   block [0x826AFC00..0x826AFC70)
	// 826AFC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFC0C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AFC10: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFC14: 392AED14  addi r9, r10, -0x12ec
	ctx.r[9].s64 = ctx.r[10].s64 + -4844;
	// 826AFC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFC1C: 390B5B80  addi r8, r11, 0x5b80
	ctx.r[8].s64 = ctx.r[11].s64 + 23424;
	// 826AFC20: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826AFC24: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826AFC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFC30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFC38: 386AC504  addi r3, r10, -0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + -15100;
	// 826AFC3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFC40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AFC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFC5C: 4BDB71C5  bl 0x82466e20
	ctx.lr = 0x826AFC60;
	sub_82466E20(ctx, base);
	// 826AFC60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFC70 size=108
    let mut pc: u32 = 0x826AFC70;
    'dispatch: loop {
        match pc {
            0x826AFC70 => {
    //   block [0x826AFC70..0x826AFCDC)
	// 826AFC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFC7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFC84: 38EB5C88  addi r7, r11, 0x5c88
	ctx.r[7].s64 = ctx.r[11].s64 + 23688;
	// 826AFC88: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826AFC8C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826AFC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFC94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFC98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFCA0: 386AC534  addi r3, r10, -0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + -15052;
	// 826AFCA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFCC8: 4BDB7159  bl 0x82466e20
	ctx.lr = 0x826AFCCC;
	sub_82466E20(ctx, base);
	// 826AFCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFCE0 size=112
    let mut pc: u32 = 0x826AFCE0;
    'dispatch: loop {
        match pc {
            0x826AFCE0 => {
    //   block [0x826AFCE0..0x826AFD50)
	// 826AFCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFCEC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AFCF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFCF4: 392AED30  addi r9, r10, -0x12d0
	ctx.r[9].s64 = ctx.r[10].s64 + -4816;
	// 826AFCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFCFC: 390B5DA8  addi r8, r11, 0x5da8
	ctx.r[8].s64 = ctx.r[11].s64 + 23976;
	// 826AFD00: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AFD04: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826AFD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFD0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFD18: 386AC564  addi r3, r10, -0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15004;
	// 826AFD1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFD20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AFD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFD3C: 4BDB70E5  bl 0x82466e20
	ctx.lr = 0x826AFD40;
	sub_82466E20(ctx, base);
	// 826AFD40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFD50 size=100
    let mut pc: u32 = 0x826AFD50;
    'dispatch: loop {
        match pc {
            0x826AFD50 => {
    //   block [0x826AFD50..0x826AFDB4)
	// 826AFD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFD5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFD64: 38AACAA4  addi r5, r10, -0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + -13660;
	// 826AFD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFD70: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826AFD74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFD84: 386AC594  addi r3, r10, -0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -14956;
	// 826AFD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFDA0: 4BDB7081  bl 0x82466e20
	ctx.lr = 0x826AFDA4;
	sub_82466E20(ctx, base);
	// 826AFDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFDB8 size=116
    let mut pc: u32 = 0x826AFDB8;
    'dispatch: loop {
        match pc {
            0x826AFDB8 => {
    //   block [0x826AFDB8..0x826AFE2C)
	// 826AFDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFDC4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AFDC8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AFDCC: 390A5DC0  addi r8, r10, 0x5dc0
	ctx.r[8].s64 = ctx.r[10].s64 + 24000;
	// 826AFDD0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFDD4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AFDD8: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826AFDDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFDE0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFDEC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826AFDF0: 396BED44  addi r11, r11, -0x12bc
	ctx.r[11].s64 = ctx.r[11].s64 + -4796;
	// 826AFDF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFDF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFDFC: 386AC5C4  addi r3, r10, -0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -14908;
	// 826AFE00: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AFE04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFE08: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AFE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFE18: 4BDB7009  bl 0x82466e20
	ctx.lr = 0x826AFE1C;
	sub_82466E20(ctx, base);
	// 826AFE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFE30 size=100
    let mut pc: u32 = 0x826AFE30;
    'dispatch: loop {
        match pc {
            0x826AFE30 => {
    //   block [0x826AFE30..0x826AFE94)
	// 826AFE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFE44: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 826AFE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFE50: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826AFE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFE64: 386AC5F4  addi r3, r10, -0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -14860;
	// 826AFE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFE6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFE70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AFE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFE78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AFE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFE80: 4BDB6FA1  bl 0x82466e20
	ctx.lr = 0x826AFE84;
	sub_82466E20(ctx, base);
	// 826AFE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFE98 size=112
    let mut pc: u32 = 0x826AFE98;
    'dispatch: loop {
        match pc {
            0x826AFE98 => {
    //   block [0x826AFE98..0x826AFF08)
	// 826AFE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFEA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFEAC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826AFEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFEB4: 390B5E50  addi r8, r11, 0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + 24144;
	// 826AFEB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AFEBC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826AFEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFEC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFED0: 386AC624  addi r3, r10, -0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14812;
	// 826AFED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AFED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFEF4: 4BDB6F2D  bl 0x82466e20
	ctx.lr = 0x826AFEF8;
	sub_82466E20(ctx, base);
	// 826AFEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFF08 size=116
    let mut pc: u32 = 0x826AFF08;
    'dispatch: loop {
        match pc {
            0x826AFF08 => {
    //   block [0x826AFF08..0x826AFF7C)
	// 826AFF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFF14: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AFF18: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AFF1C: 390A5E98  addi r8, r10, 0x5e98
	ctx.r[8].s64 = ctx.r[10].s64 + 24216;
	// 826AFF20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFF24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AFF28: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826AFF2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFF30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AFF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AFF3C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826AFF40: 396BED64  addi r11, r11, -0x129c
	ctx.r[11].s64 = ctx.r[11].s64 + -4764;
	// 826AFF44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFF48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFF4C: 386AC654  addi r3, r10, -0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + -14764;
	// 826AFF50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AFF54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFF58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AFF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFF68: 4BDB6EB9  bl 0x82466e20
	ctx.lr = 0x826AFF6C;
	sub_82466E20(ctx, base);
	// 826AFF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AFF80 size=108
    let mut pc: u32 = 0x826AFF80;
    'dispatch: loop {
        match pc {
            0x826AFF80 => {
    //   block [0x826AFF80..0x826AFFEC)
	// 826AFF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AFF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AFF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AFF8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826AFF94: 38EB5F28  addi r7, r11, 0x5f28
	ctx.r[7].s64 = ctx.r[11].s64 + 24360;
	// 826AFF98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AFF9C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826AFFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AFFA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AFFA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AFFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AFFB0: 386AC684  addi r3, r10, -0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + -14716;
	// 826AFFB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AFFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AFFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AFFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AFFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AFFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AFFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AFFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AFFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AFFD8: 4BDB6E49  bl 0x82466e20
	ctx.lr = 0x826AFFDC;
	sub_82466E20(ctx, base);
	// 826AFFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AFFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AFFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AFFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AFFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AFFF0 size=24
    let mut pc: u32 = 0x826AFFF0;
    'dispatch: loop {
        match pc {
            0x826AFFF0 => {
    //   block [0x826AFFF0..0x826B0008)
	// 826AFFF0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AFFF4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826AFFF8: 394AB6B8  addi r10, r10, -0x4948
	ctx.r[10].s64 = ctx.r[10].s64 + -18760;
	// 826AFFFC: 816B5F70  lwz r11, 0x5f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24432 as u32) ) } as u64;
	// 826B0000: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B0004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0008 size=116
    let mut pc: u32 = 0x826B0008;
    'dispatch: loop {
        match pc {
            0x826B0008 => {
    //   block [0x826B0008..0x826B007C)
	// 826B0008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0014: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0018: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B001C: 392BED98  addi r9, r11, -0x1268
	ctx.r[9].s64 = ctx.r[11].s64 + -4712;
	// 826B0020: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0024: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0028: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B002C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 826B0030: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B0034: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826B0038: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B003C: 396BB6B8  addi r11, r11, -0x4948
	ctx.r[11].s64 = ctx.r[11].s64 + -18760;
	// 826B0040: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B0044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0048: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B004C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0050: 386AC6B4  addi r3, r10, -0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + -14668;
	// 826B0054: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0058: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0060: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B0064: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0068: 4BDB6DB9  bl 0x82466e20
	ctx.lr = 0x826B006C;
	sub_82466E20(ctx, base);
	// 826B006C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0080 size=112
    let mut pc: u32 = 0x826B0080;
    'dispatch: loop {
        match pc {
            0x826B0080 => {
    //   block [0x826B0080..0x826B00F0)
	// 826B0080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B008C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0090: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0094: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B009C: 390B5F74  addi r8, r11, 0x5f74
	ctx.r[8].s64 = ctx.r[11].s64 + 24436;
	// 826B00A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B00A4: 388A596C  addi r4, r10, 0x596c
	ctx.r[4].s64 = ctx.r[10].s64 + 22892;
	// 826B00A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B00AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B00B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B00B8: 386AC6E4  addi r3, r10, -0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + -14620;
	// 826B00BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B00C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B00C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B00C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B00CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B00D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B00D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B00D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B00DC: 4BDB6D45  bl 0x82466e20
	ctx.lr = 0x826B00E0;
	sub_82466E20(ctx, base);
	// 826B00E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B00E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B00E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B00EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B00F0 size=112
    let mut pc: u32 = 0x826B00F0;
    'dispatch: loop {
        match pc {
            0x826B00F0 => {
    //   block [0x826B00F0..0x826B0160)
	// 826B00F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B00F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B00F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B00FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0100: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0104: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B010C: 390B5FA4  addi r8, r11, 0x5fa4
	ctx.r[8].s64 = ctx.r[11].s64 + 24484;
	// 826B0110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B0114: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826B0118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B011C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0128: 386AC714  addi r3, r10, -0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14572;
	// 826B012C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B013C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B014C: 4BDB6CD5  bl 0x82466e20
	ctx.lr = 0x826B0150;
	sub_82466E20(ctx, base);
	// 826B0150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0160 size=100
    let mut pc: u32 = 0x826B0160;
    'dispatch: loop {
        match pc {
            0x826B0160 => {
    //   block [0x826B0160..0x826B01C4)
	// 826B0160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B016C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0174: 38AACAA4  addi r5, r10, -0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + -13660;
	// 826B0178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B017C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0180: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826B0184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B018C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0194: 386AC744  addi r3, r10, -0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14524;
	// 826B0198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B019C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B01A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B01A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B01A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B01AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B01B0: 4BDB6C71  bl 0x82466e20
	ctx.lr = 0x826B01B4;
	sub_82466E20(ctx, base);
	// 826B01B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B01B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B01BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B01C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B01C8 size=116
    let mut pc: u32 = 0x826B01C8;
    'dispatch: loop {
        match pc {
            0x826B01C8 => {
    //   block [0x826B01C8..0x826B023C)
	// 826B01C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B01CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B01D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B01D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B01D8: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826B01DC: 390A5FC0  addi r8, r10, 0x5fc0
	ctx.r[8].s64 = ctx.r[10].s64 + 24512;
	// 826B01E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B01E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B01E8: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B01EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B01F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B01F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B01F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B01FC: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826B0200: 396BEDE0  addi r11, r11, -0x1220
	ctx.r[11].s64 = ctx.r[11].s64 + -4640;
	// 826B0204: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0208: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B020C: 386AC774  addi r3, r10, -0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + -14476;
	// 826B0210: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0218: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B021C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0228: 4BDB6BF9  bl 0x82466e20
	ctx.lr = 0x826B022C;
	sub_82466E20(ctx, base);
	// 826B022C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0240 size=112
    let mut pc: u32 = 0x826B0240;
    'dispatch: loop {
        match pc {
            0x826B0240 => {
    //   block [0x826B0240..0x826B02B0)
	// 826B0240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B024C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0250: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0254: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B025C: 390B6158  addi r8, r11, 0x6158
	ctx.r[8].s64 = ctx.r[11].s64 + 24920;
	// 826B0260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B0264: 388A5998  addi r4, r10, 0x5998
	ctx.r[4].s64 = ctx.r[10].s64 + 22936;
	// 826B0268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B026C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0278: 386AC7A4  addi r3, r10, -0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + -14428;
	// 826B027C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B028C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B029C: 4BDB6B85  bl 0x82466e20
	ctx.lr = 0x826B02A0;
	sub_82466E20(ctx, base);
	// 826B02A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B02A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B02A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B02AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B02B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B02B0 size=112
    let mut pc: u32 = 0x826B02B0;
    'dispatch: loop {
        match pc {
            0x826B02B0 => {
    //   block [0x826B02B0..0x826B0320)
	// 826B02B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B02B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B02B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B02BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B02C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B02C4: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B02C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B02CC: 390B6170  addi r8, r11, 0x6170
	ctx.r[8].s64 = ctx.r[11].s64 + 24944;
	// 826B02D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B02D4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826B02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B02DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B02E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B02E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B02E8: 386AC7D4  addi r3, r10, -0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + -14380;
	// 826B02EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B02F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B02F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B02F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B02FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B030C: 4BDB6B15  bl 0x82466e20
	ctx.lr = 0x826B0310;
	sub_82466E20(ctx, base);
	// 826B0310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0320 size=112
    let mut pc: u32 = 0x826B0320;
    'dispatch: loop {
        match pc {
            0x826B0320 => {
    //   block [0x826B0320..0x826B0390)
	// 826B0320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B032C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0330: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0334: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B033C: 390B6218  addi r8, r11, 0x6218
	ctx.r[8].s64 = ctx.r[11].s64 + 25112;
	// 826B0340: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B0344: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826B0348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B034C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0358: 386AC804  addi r3, r10, -0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14332;
	// 826B035C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B037C: 4BDB6AA5  bl 0x82466e20
	ctx.lr = 0x826B0380;
	sub_82466E20(ctx, base);
	// 826B0380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0390 size=112
    let mut pc: u32 = 0x826B0390;
    'dispatch: loop {
        match pc {
            0x826B0390 => {
    //   block [0x826B0390..0x826B0400)
	// 826B0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B039C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B03A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B03A4: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B03A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B03AC: 390B62F0  addi r8, r11, 0x62f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25328;
	// 826B03B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826B03B4: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826B03B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B03BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B03C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B03C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B03C8: 386AC834  addi r3, r10, -0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14284;
	// 826B03CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B03D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B03D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B03D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B03DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B03E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B03E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B03E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B03EC: 4BDB6A35  bl 0x82466e20
	ctx.lr = 0x826B03F0;
	sub_82466E20(ctx, base);
	// 826B03F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B03F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B03F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B03FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0400 size=112
    let mut pc: u32 = 0x826B0400;
    'dispatch: loop {
        match pc {
            0x826B0400 => {
    //   block [0x826B0400..0x826B0470)
	// 826B0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B040C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0410: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0414: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B041C: 390B63F8  addi r8, r11, 0x63f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25592;
	// 826B0420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B0424: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826B0428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B042C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0438: 386AC864  addi r3, r10, -0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + -14236;
	// 826B043C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B044C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B045C: 4BDB69C5  bl 0x82466e20
	ctx.lr = 0x826B0460;
	sub_82466E20(ctx, base);
	// 826B0460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B0470 size=24
    let mut pc: u32 = 0x826B0470;
    'dispatch: loop {
        match pc {
            0x826B0470 => {
    //   block [0x826B0470..0x826B0488)
	// 826B0470: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0474: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B0478: 394AB7D8  addi r10, r10, -0x4828
	ctx.r[10].s64 = ctx.r[10].s64 + -18472;
	// 826B047C: 816B5FBC  lwz r11, 0x5fbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24508 as u32) ) } as u64;
	// 826B0480: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B0484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0488 size=116
    let mut pc: u32 = 0x826B0488;
    'dispatch: loop {
        match pc {
            0x826B0488 => {
    //   block [0x826B0488..0x826B04FC)
	// 826B0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0494: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0498: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B049C: 392BEE3C  addi r9, r11, -0x11c4
	ctx.r[9].s64 = ctx.r[11].s64 + -4548;
	// 826B04A0: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B04A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B04A8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B04AC: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826B04B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B04B4: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826B04B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B04BC: 396BB7D8  addi r11, r11, -0x4828
	ctx.r[11].s64 = ctx.r[11].s64 + -18472;
	// 826B04C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B04C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B04C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B04CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B04D0: 386AC894  addi r3, r10, -0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + -14188;
	// 826B04D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B04D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B04DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B04E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B04E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B04E8: 4BDB6939  bl 0x82466e20
	ctx.lr = 0x826B04EC;
	sub_82466E20(ctx, base);
	// 826B04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0500 size=116
    let mut pc: u32 = 0x826B0500;
    'dispatch: loop {
        match pc {
            0x826B0500 => {
    //   block [0x826B0500..0x826B0574)
	// 826B0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B050C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B0510: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B0514: 390A6428  addi r8, r10, 0x6428
	ctx.r[8].s64 = ctx.r[10].s64 + 25640;
	// 826B0518: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B051C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0520: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B0524: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0528: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B052C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0534: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826B0538: 396BEEAC  addi r11, r11, -0x1154
	ctx.r[11].s64 = ctx.r[11].s64 + -4436;
	// 826B053C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0544: 386AC8C4  addi r3, r10, -0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + -14140;
	// 826B0548: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B054C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0550: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B055C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0560: 4BDB68C1  bl 0x82466e20
	ctx.lr = 0x826B0564;
	sub_82466E20(ctx, base);
	// 826B0564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B056C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0578 size=108
    let mut pc: u32 = 0x826B0578;
    'dispatch: loop {
        match pc {
            0x826B0578 => {
    //   block [0x826B0578..0x826B05E4)
	// 826B0578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0584: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B058C: 38EB6458  addi r7, r11, 0x6458
	ctx.r[7].s64 = ctx.r[11].s64 + 25688;
	// 826B0590: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B0594: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826B0598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B059C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B05A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B05A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B05A8: 386AC8F4  addi r3, r10, -0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + -14092;
	// 826B05AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B05B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B05B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B05B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B05BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B05C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B05C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B05C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B05CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B05D0: 4BDB6851  bl 0x82466e20
	ctx.lr = 0x826B05D4;
	sub_82466E20(ctx, base);
	// 826B05D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B05D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B05DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B05E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B05E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B05E8 size=24
    let mut pc: u32 = 0x826B05E8;
    'dispatch: loop {
        match pc {
            0x826B05E8 => {
    //   block [0x826B05E8..0x826B0600)
	// 826B05E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B05EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B05F0: 394AB970  addi r10, r10, -0x4690
	ctx.r[10].s64 = ctx.r[10].s64 + -18064;
	// 826B05F4: 816B64E8  lwz r11, 0x64e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25832 as u32) ) } as u64;
	// 826B05F8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B05FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0600 size=116
    let mut pc: u32 = 0x826B0600;
    'dispatch: loop {
        match pc {
            0x826B0600 => {
    //   block [0x826B0600..0x826B0674)
	// 826B0600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B060C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0610: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0614: 392BEED0  addi r9, r11, -0x1130
	ctx.r[9].s64 = ctx.r[11].s64 + -4400;
	// 826B0618: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B061C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0620: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826B0624: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826B0628: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B062C: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826B0630: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0634: 396BB970  addi r11, r11, -0x4690
	ctx.r[11].s64 = ctx.r[11].s64 + -18064;
	// 826B0638: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B063C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0640: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B0644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0648: 386AC924  addi r3, r10, -0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14044;
	// 826B064C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B0650: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B0654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0658: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B065C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B0660: 4BDB67C1  bl 0x82466e20
	ctx.lr = 0x826B0664;
	sub_82466E20(ctx, base);
	// 826B0664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B066C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0678 size=112
    let mut pc: u32 = 0x826B0678;
    'dispatch: loop {
        match pc {
            0x826B0678 => {
    //   block [0x826B0678..0x826B06E8)
	// 826B0678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B067C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0688: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B068C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0694: 390B64EC  addi r8, r11, 0x64ec
	ctx.r[8].s64 = ctx.r[11].s64 + 25836;
	// 826B0698: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B069C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826B06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B06A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B06A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B06AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B06B0: 386AC954  addi r3, r10, -0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13996;
	// 826B06B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B06B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B06BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B06C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B06CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B06D4: 4BDB674D  bl 0x82466e20
	ctx.lr = 0x826B06D8;
	sub_82466E20(ctx, base);
	// 826B06D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B06DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B06E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B06E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B06E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B06E8 size=116
    let mut pc: u32 = 0x826B06E8;
    'dispatch: loop {
        match pc {
            0x826B06E8 => {
    //   block [0x826B06E8..0x826B075C)
	// 826B06E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B06EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B06F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B06F4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B06F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826B06FC: 390A6520  addi r8, r10, 0x6520
	ctx.r[8].s64 = ctx.r[10].s64 + 25888;
	// 826B0700: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0704: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0708: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B070C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0710: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B071C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826B0720: 396BEF28  addi r11, r11, -0x10d8
	ctx.r[11].s64 = ctx.r[11].s64 + -4312;
	// 826B0724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0728: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B072C: 386AC984  addi r3, r10, -0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + -13948;
	// 826B0730: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0738: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0748: 4BDB66D9  bl 0x82466e20
	ctx.lr = 0x826B074C;
	sub_82466E20(ctx, base);
	// 826B074C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0760 size=108
    let mut pc: u32 = 0x826B0760;
    'dispatch: loop {
        match pc {
            0x826B0760 => {
    //   block [0x826B0760..0x826B07CC)
	// 826B0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B076C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0774: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 826B0778: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826B077C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826B0780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B078C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0790: 386AC9B4  addi r3, r10, -0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + -13900;
	// 826B0794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B079C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B07A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B07A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B07A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B07AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B07B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B07B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B07B8: 4BDB6669  bl 0x82466e20
	ctx.lr = 0x826B07BC;
	sub_82466E20(ctx, base);
	// 826B07BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B07C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B07C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B07C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B07D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B07D0 size=116
    let mut pc: u32 = 0x826B07D0;
    'dispatch: loop {
        match pc {
            0x826B07D0 => {
    //   block [0x826B07D0..0x826B0844)
	// 826B07D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B07D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B07D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B07DC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B07E0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826B07E4: 390A66D0  addi r8, r10, 0x66d0
	ctx.r[8].s64 = ctx.r[10].s64 + 26320;
	// 826B07E8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B07EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B07F0: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B07F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B07F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B07FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0804: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826B0808: 396BEF40  addi r11, r11, -0x10c0
	ctx.r[11].s64 = ctx.r[11].s64 + -4288;
	// 826B080C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0814: 386AC9E4  addi r3, r10, -0x361c
	ctx.r[3].s64 = ctx.r[10].s64 + -13852;
	// 826B0818: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B081C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0820: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B082C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0830: 4BDB65F1  bl 0x82466e20
	ctx.lr = 0x826B0834;
	sub_82466E20(ctx, base);
	// 826B0834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B083C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0848 size=112
    let mut pc: u32 = 0x826B0848;
    'dispatch: loop {
        match pc {
            0x826B0848 => {
    //   block [0x826B0848..0x826B08B8)
	// 826B0848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B085C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0864: 390B67C0  addi r8, r11, 0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26560;
	// 826B0868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B086C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826B0870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0880: 386ACA14  addi r3, r10, -0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13804;
	// 826B0884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B088C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B089C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B08A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B08A4: 4BDB657D  bl 0x82466e20
	ctx.lr = 0x826B08A8;
	sub_82466E20(ctx, base);
	// 826B08A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B08AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B08B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B08B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B08B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B08B8 size=112
    let mut pc: u32 = 0x826B08B8;
    'dispatch: loop {
        match pc {
            0x826B08B8 => {
    //   block [0x826B08B8..0x826B0928)
	// 826B08B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B08BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B08C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B08C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B08C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B08CC: 38AAC804  addi r5, r10, -0x37fc
	ctx.r[5].s64 = ctx.r[10].s64 + -14332;
	// 826B08D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B08D4: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 826B08D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B08DC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826B08E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B08E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B08E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B08EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B08F0: 386ACA44  addi r3, r10, -0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13756;
	// 826B08F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B08F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B08FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B090C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0914: 4BDB650D  bl 0x82466e20
	ctx.lr = 0x826B0918;
	sub_82466E20(ctx, base);
	// 826B0918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B091C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0928 size=112
    let mut pc: u32 = 0x826B0928;
    'dispatch: loop {
        match pc {
            0x826B0928 => {
    //   block [0x826B0928..0x826B0998)
	// 826B0928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B093C: 38AAC744  addi r5, r10, -0x38bc
	ctx.r[5].s64 = ctx.r[10].s64 + -14524;
	// 826B0940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0944: 390B6868  addi r8, r11, 0x6868
	ctx.r[8].s64 = ctx.r[11].s64 + 26728;
	// 826B0948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B094C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826B0950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B095C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0960: 386ACA74  addi r3, r10, -0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + -13708;
	// 826B0964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B0968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B096C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B097C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0984: 4BDB649D  bl 0x82466e20
	ctx.lr = 0x826B0988;
	sub_82466E20(ctx, base);
	// 826B0988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B098C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0998 size=116
    let mut pc: u32 = 0x826B0998;
    'dispatch: loop {
        match pc {
            0x826B0998 => {
    //   block [0x826B0998..0x826B0A0C)
	// 826B0998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B099C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B09A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B09A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B09A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B09AC: 390B6880  addi r8, r11, 0x6880
	ctx.r[8].s64 = ctx.r[11].s64 + 26752;
	// 826B09B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B09B4: 392AEF8C  addi r9, r10, -0x1074
	ctx.r[9].s64 = ctx.r[10].s64 + -4212;
	// 826B09B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B09BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B09C0: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B09C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B09C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B09CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B09D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B09D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B09D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B09DC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B09E0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826B09E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B09E8: 386BCAA4  addi r3, r11, -0x355c
	ctx.r[3].s64 = ctx.r[11].s64 + -13660;
	// 826B09EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B09F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B09F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B09F8: 4BDB6429  bl 0x82466e20
	ctx.lr = 0x826B09FC;
	sub_82466E20(ctx, base);
	// 826B09FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0A10 size=108
    let mut pc: u32 = 0x826B0A10;
    'dispatch: loop {
        match pc {
            0x826B0A10 => {
    //   block [0x826B0A10..0x826B0A7C)
	// 826B0A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0A1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0A24: 38EB6898  addi r7, r11, 0x6898
	ctx.r[7].s64 = ctx.r[11].s64 + 26776;
	// 826B0A28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B0A2C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826B0A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0A40: 386ACAD4  addi r3, r10, -0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + -13612;
	// 826B0A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0A68: 4BDB63B9  bl 0x82466e20
	ctx.lr = 0x826B0A6C;
	sub_82466E20(ctx, base);
	// 826B0A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0A80 size=108
    let mut pc: u32 = 0x826B0A80;
    'dispatch: loop {
        match pc {
            0x826B0A80 => {
    //   block [0x826B0A80..0x826B0AEC)
	// 826B0A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0A8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826B0A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0A94: 38EB68E0  addi r7, r11, 0x68e0
	ctx.r[7].s64 = ctx.r[11].s64 + 26848;
	// 826B0A98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B0A9C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826B0AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B0AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B0AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0AB0: 386ACB04  addi r3, r10, -0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13564;
	// 826B0AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B0AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B0AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B0ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B0AD8: 4BDB6349  bl 0x82466e20
	ctx.lr = 0x826B0ADC;
	sub_82466E20(ctx, base);
	// 826B0ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B0AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B0AF0 size=116
    let mut pc: u32 = 0x826B0AF0;
    'dispatch: loop {
        match pc {
            0x826B0AF0 => {
    //   block [0x826B0AF0..0x826B0B64)
	// 826B0AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B0AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B0AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B0AFC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826B0B00: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826B0B04: 390A6928  addi r8, r10, 0x6928
	ctx.r[8].s64 = ctx.r[10].s64 + 26920;
	// 826B0B08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0B0C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B0B10: 38AAC594  addi r5, r10, -0x3a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -14956;
	// 826B0B14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B0B18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B0B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B0B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B0B24: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826B0B28: 396BEFA0  addi r11, r11, -0x1060
	ctx.r[11].s64 = ctx.r[11].s64 + -4192;
	// 826B0B2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B0B30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B0B34: 386ACB34  addi r3, r10, -0x34cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13516;
	// 826B0B38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B0B3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B0B40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B0B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B0B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B0B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B0B50: 4BDB62D1  bl 0x82466e20
	ctx.lr = 0x826B0B54;
	sub_82466E20(ctx, base);
	// 826B0B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B0B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B0B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B0B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


