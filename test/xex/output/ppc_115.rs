pub fn sub_826ED1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED1D8 size=108
    let mut pc: u32 = 0x826ED1D8;
    'dispatch: loop {
        match pc {
            0x826ED1D8 => {
    //   block [0x826ED1D8..0x826ED244)
	// 826ED1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED1E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED1E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED1EC: 38EB55E0  addi r7, r11, 0x55e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21984;
	// 826ED1F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ED1F4: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826ED1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED1FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED208: 386A61FC  addi r3, r10, 0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + 25084;
	// 826ED20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED230: 4BD79BF1  bl 0x82466e20
	ctx.lr = 0x826ED234;
	sub_82466E20(ctx, base);
	// 826ED234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED248 size=112
    let mut pc: u32 = 0x826ED248;
    'dispatch: loop {
        match pc {
            0x826ED248 => {
    //   block [0x826ED248..0x826ED2B8)
	// 826ED248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED258: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED25C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED264: 390B5628  addi r8, r11, 0x5628
	ctx.r[8].s64 = ctx.r[11].s64 + 22056;
	// 826ED268: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED26C: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826ED270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED280: 386A622C  addi r3, r10, 0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + 25132;
	// 826ED284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED2A4: 4BD79B7D  bl 0x82466e20
	ctx.lr = 0x826ED2A8;
	sub_82466E20(ctx, base);
	// 826ED2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED2B8 size=108
    let mut pc: u32 = 0x826ED2B8;
    'dispatch: loop {
        match pc {
            0x826ED2B8 => {
    //   block [0x826ED2B8..0x826ED324)
	// 826ED2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED2C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED2C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED2CC: 38EB5640  addi r7, r11, 0x5640
	ctx.r[7].s64 = ctx.r[11].s64 + 22080;
	// 826ED2D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ED2D4: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826ED2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED2E8: 386A625C  addi r3, r10, 0x625c
	ctx.r[3].s64 = ctx.r[10].s64 + 25180;
	// 826ED2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED310: 4BD79B11  bl 0x82466e20
	ctx.lr = 0x826ED314;
	sub_82466E20(ctx, base);
	// 826ED314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED328 size=112
    let mut pc: u32 = 0x826ED328;
    'dispatch: loop {
        match pc {
            0x826ED328 => {
    //   block [0x826ED328..0x826ED398)
	// 826ED328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED338: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED33C: 38AA622C  addi r5, r10, 0x622c
	ctx.r[5].s64 = ctx.r[10].s64 + 25132;
	// 826ED340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED344: 390B5688  addi r8, r11, 0x5688
	ctx.r[8].s64 = ctx.r[11].s64 + 22152;
	// 826ED348: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED34C: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826ED350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED360: 386A628C  addi r3, r10, 0x628c
	ctx.r[3].s64 = ctx.r[10].s64 + 25228;
	// 826ED364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED384: 4BD79A9D  bl 0x82466e20
	ctx.lr = 0x826ED388;
	sub_82466E20(ctx, base);
	// 826ED388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED398 size=100
    let mut pc: u32 = 0x826ED398;
    'dispatch: loop {
        match pc {
            0x826ED398 => {
    //   block [0x826ED398..0x826ED3FC)
	// 826ED398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED3AC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED3B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED3B8: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826ED3BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED3CC: 386A62BC  addi r3, r10, 0x62bc
	ctx.r[3].s64 = ctx.r[10].s64 + 25276;
	// 826ED3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ED3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ED3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED3E8: 4BD79A39  bl 0x82466e20
	ctx.lr = 0x826ED3EC;
	sub_82466E20(ctx, base);
	// 826ED3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED400 size=112
    let mut pc: u32 = 0x826ED400;
    'dispatch: loop {
        match pc {
            0x826ED400 => {
    //   block [0x826ED400..0x826ED470)
	// 826ED400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED40C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED410: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED414: 38AA62BC  addi r5, r10, 0x62bc
	ctx.r[5].s64 = ctx.r[10].s64 + 25276;
	// 826ED418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED41C: 390B56A0  addi r8, r11, 0x56a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22176;
	// 826ED420: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826ED424: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826ED428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED42C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED438: 386A62EC  addi r3, r10, 0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + 25324;
	// 826ED43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED45C: 4BD799C5  bl 0x82466e20
	ctx.lr = 0x826ED460;
	sub_82466E20(ctx, base);
	// 826ED460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED470 size=108
    let mut pc: u32 = 0x826ED470;
    'dispatch: loop {
        match pc {
            0x826ED470 => {
    //   block [0x826ED470..0x826ED4DC)
	// 826ED470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED47C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED484: 38EB5748  addi r7, r11, 0x5748
	ctx.r[7].s64 = ctx.r[11].s64 + 22344;
	// 826ED488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED48C: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826ED490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED4A0: 386A631C  addi r3, r10, 0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + 25372;
	// 826ED4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED4C8: 4BD79959  bl 0x82466e20
	ctx.lr = 0x826ED4CC;
	sub_82466E20(ctx, base);
	// 826ED4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED4E0 size=112
    let mut pc: u32 = 0x826ED4E0;
    'dispatch: loop {
        match pc {
            0x826ED4E0 => {
    //   block [0x826ED4E0..0x826ED550)
	// 826ED4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED4EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED4F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED4F4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED4FC: 390B5778  addi r8, r11, 0x5778
	ctx.r[8].s64 = ctx.r[11].s64 + 22392;
	// 826ED500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED504: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826ED508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED50C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED518: 386A634C  addi r3, r10, 0x634c
	ctx.r[3].s64 = ctx.r[10].s64 + 25420;
	// 826ED51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED53C: 4BD798E5  bl 0x82466e20
	ctx.lr = 0x826ED540;
	sub_82466E20(ctx, base);
	// 826ED540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED550 size=112
    let mut pc: u32 = 0x826ED550;
    'dispatch: loop {
        match pc {
            0x826ED550 => {
    //   block [0x826ED550..0x826ED5C0)
	// 826ED550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED55C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED560: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED564: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED56C: 390B57C0  addi r8, r11, 0x57c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22464;
	// 826ED570: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED574: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826ED578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED57C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED588: 386A637C  addi r3, r10, 0x637c
	ctx.r[3].s64 = ctx.r[10].s64 + 25468;
	// 826ED58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED5AC: 4BD79875  bl 0x82466e20
	ctx.lr = 0x826ED5B0;
	sub_82466E20(ctx, base);
	// 826ED5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED5C0 size=100
    let mut pc: u32 = 0x826ED5C0;
    'dispatch: loop {
        match pc {
            0x826ED5C0 => {
    //   block [0x826ED5C0..0x826ED624)
	// 826ED5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED5CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED5D4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED5D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED5E0: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826ED5E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED5F4: 386A63AC  addi r3, r10, 0x63ac
	ctx.r[3].s64 = ctx.r[10].s64 + 25516;
	// 826ED5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED5FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ED604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ED60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED610: 4BD79811  bl 0x82466e20
	ctx.lr = 0x826ED614;
	sub_82466E20(ctx, base);
	// 826ED614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED628 size=112
    let mut pc: u32 = 0x826ED628;
    'dispatch: loop {
        match pc {
            0x826ED628 => {
    //   block [0x826ED628..0x826ED698)
	// 826ED628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED638: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED63C: 38AA63AC  addi r5, r10, 0x63ac
	ctx.r[5].s64 = ctx.r[10].s64 + 25516;
	// 826ED640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED644: 390B5808  addi r8, r11, 0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + 22536;
	// 826ED648: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED64C: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826ED650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED654: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED660: 386A63DC  addi r3, r10, 0x63dc
	ctx.r[3].s64 = ctx.r[10].s64 + 25564;
	// 826ED664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED684: 4BD7979D  bl 0x82466e20
	ctx.lr = 0x826ED688;
	sub_82466E20(ctx, base);
	// 826ED688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED698 size=112
    let mut pc: u32 = 0x826ED698;
    'dispatch: loop {
        match pc {
            0x826ED698 => {
    //   block [0x826ED698..0x826ED708)
	// 826ED698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED6A8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED6AC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED6B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED6B4: 390B5850  addi r8, r11, 0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + 22608;
	// 826ED6B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED6BC: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826ED6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED6C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED6D0: 386A640C  addi r3, r10, 0x640c
	ctx.r[3].s64 = ctx.r[10].s64 + 25612;
	// 826ED6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED6F4: 4BD7972D  bl 0x82466e20
	ctx.lr = 0x826ED6F8;
	sub_82466E20(ctx, base);
	// 826ED6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED708 size=112
    let mut pc: u32 = 0x826ED708;
    'dispatch: loop {
        match pc {
            0x826ED708 => {
    //   block [0x826ED708..0x826ED778)
	// 826ED708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED718: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED71C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826ED720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED724: 390B5868  addi r8, r11, 0x5868
	ctx.r[8].s64 = ctx.r[11].s64 + 22632;
	// 826ED728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ED72C: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826ED730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED740: 386A643C  addi r3, r10, 0x643c
	ctx.r[3].s64 = ctx.r[10].s64 + 25660;
	// 826ED744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED754: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826ED758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED764: 4BD796BD  bl 0x82466e20
	ctx.lr = 0x826ED768;
	sub_82466E20(ctx, base);
	// 826ED768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED778 size=112
    let mut pc: u32 = 0x826ED778;
    'dispatch: loop {
        match pc {
            0x826ED778 => {
    //   block [0x826ED778..0x826ED7E8)
	// 826ED778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED788: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826ED78C: 38AA640C  addi r5, r10, 0x640c
	ctx.r[5].s64 = ctx.r[10].s64 + 25612;
	// 826ED790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ED794: 390B5880  addi r8, r11, 0x5880
	ctx.r[8].s64 = ctx.r[11].s64 + 22656;
	// 826ED798: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826ED79C: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826ED7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED7A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED7B0: 386A646C  addi r3, r10, 0x646c
	ctx.r[3].s64 = ctx.r[10].s64 + 25708;
	// 826ED7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ED7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED7D4: 4BD7964D  bl 0x82466e20
	ctx.lr = 0x826ED7D8;
	sub_82466E20(ctx, base);
	// 826ED7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED7E8 size=72
    let mut pc: u32 = 0x826ED7E8;
    'dispatch: loop {
        match pc {
            0x826ED7E8 => {
    //   block [0x826ED7E8..0x826ED830)
	// 826ED7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED7F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ED7F8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826ED7FC: 38CB68C8  addi r6, r11, 0x68c8
	ctx.r[6].s64 = ctx.r[11].s64 + 26824;
	// 826ED800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826ED804: 388B8C78  addi r4, r11, -0x7388
	ctx.r[4].s64 = ctx.r[11].s64 + -29576;
	// 826ED808: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826ED80C: 386B649C  addi r3, r11, 0x649c
	ctx.r[3].s64 = ctx.r[11].s64 + 25756;
	// 826ED810: 4BD8E279  bl 0x8247ba88
	ctx.lr = 0x826ED814;
	sub_8247BA88(ctx, base);
	// 826ED814: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826ED818: 386BCF30  addi r3, r11, -0x30d0
	ctx.r[3].s64 = ctx.r[11].s64 + -12496;
	// 826ED81C: 4BE4531D  bl 0x82532b38
	ctx.lr = 0x826ED820;
	sub_82532B38(ctx, base);
	// 826ED820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826ED824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED830 size=108
    let mut pc: u32 = 0x826ED830;
    'dispatch: loop {
        match pc {
            0x826ED830 => {
    //   block [0x826ED830..0x826ED89C)
	// 826ED830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED83C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED844: 38EB8820  addi r7, r11, -0x77e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30688;
	// 826ED848: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826ED84C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826ED850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED858: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED85C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED860: 386A64B4  addi r3, r10, 0x64b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25780;
	// 826ED864: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED888: 4BD79599  bl 0x82466e20
	ctx.lr = 0x826ED88C;
	sub_82466E20(ctx, base);
	// 826ED88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ED8A0 size=24
    let mut pc: u32 = 0x826ED8A0;
    'dispatch: loop {
        match pc {
            0x826ED8A0 => {
    //   block [0x826ED8A0..0x826ED8B8)
	// 826ED8A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED8A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826ED8A8: 394A0E60  addi r10, r10, 0xe60
	ctx.r[10].s64 = ctx.r[10].s64 + 3680;
	// 826ED8AC: 816B8898  lwz r11, -0x7768(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30568 as u32) ) } as u64;
	// 826ED8B0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826ED8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED8B8 size=112
    let mut pc: u32 = 0x826ED8B8;
    'dispatch: loop {
        match pc {
            0x826ED8B8 => {
    //   block [0x826ED8B8..0x826ED928)
	// 826ED8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED8C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826ED8C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED8CC: 392A9414  addi r9, r10, -0x6bec
	ctx.r[9].s64 = ctx.r[10].s64 + -27628;
	// 826ED8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED8D4: 390B0E60  addi r8, r11, 0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + 3680;
	// 826ED8D8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826ED8DC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826ED8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED8E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ED8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED8F0: 386A64E4  addi r3, r10, 0x64e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25828;
	// 826ED8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ED8F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ED8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED914: 4BD7950D  bl 0x82466e20
	ctx.lr = 0x826ED918;
	sub_82466E20(ctx, base);
	// 826ED918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED928 size=108
    let mut pc: u32 = 0x826ED928;
    'dispatch: loop {
        match pc {
            0x826ED928 => {
    //   block [0x826ED928..0x826ED994)
	// 826ED928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED934: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED93C: 38EB889C  addi r7, r11, -0x7764
	ctx.r[7].s64 = ctx.r[11].s64 + -30564;
	// 826ED940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED944: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826ED948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED958: 386A6514  addi r3, r10, 0x6514
	ctx.r[3].s64 = ctx.r[10].s64 + 25876;
	// 826ED95C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED97C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED980: 4BD794A1  bl 0x82466e20
	ctx.lr = 0x826ED984;
	sub_82466E20(ctx, base);
	// 826ED984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ED990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ED998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ED998 size=108
    let mut pc: u32 = 0x826ED998;
    'dispatch: loop {
        match pc {
            0x826ED998 => {
    //   block [0x826ED998..0x826EDA04)
	// 826ED998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ED99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ED9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ED9A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826ED9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826ED9AC: 38EB88CC  addi r7, r11, -0x7734
	ctx.r[7].s64 = ctx.r[11].s64 + -30516;
	// 826ED9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ED9B4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826ED9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ED9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826ED9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ED9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ED9C8: 386A6544  addi r3, r10, 0x6544
	ctx.r[3].s64 = ctx.r[10].s64 + 25924;
	// 826ED9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ED9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ED9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ED9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ED9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ED9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ED9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ED9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ED9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ED9F0: 4BD79431  bl 0x82466e20
	ctx.lr = 0x826ED9F4;
	sub_82466E20(ctx, base);
	// 826ED9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ED9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ED9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EDA08 size=24
    let mut pc: u32 = 0x826EDA08;
    'dispatch: loop {
        match pc {
            0x826EDA08 => {
    //   block [0x826EDA08..0x826EDA20)
	// 826EDA08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDA0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EDA10: 394A0EA8  addi r10, r10, 0xea8
	ctx.r[10].s64 = ctx.r[10].s64 + 3752;
	// 826EDA14: 816B88FC  lwz r11, -0x7704(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30468 as u32) ) } as u64;
	// 826EDA18: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EDA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDA20 size=116
    let mut pc: u32 = 0x826EDA20;
    'dispatch: loop {
        match pc {
            0x826EDA20 => {
    //   block [0x826EDA20..0x826EDA94)
	// 826EDA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDA2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDA30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EDA34: 390B0EA8  addi r8, r11, 0xea8
	ctx.r[8].s64 = ctx.r[11].s64 + 3752;
	// 826EDA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDA3C: 392A9448  addi r9, r10, -0x6bb8
	ctx.r[9].s64 = ctx.r[10].s64 + -27576;
	// 826EDA40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDA44: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826EDA48: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDA4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDA54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDA64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EDA68: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826EDA6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EDA70: 386B6574  addi r3, r11, 0x6574
	ctx.r[3].s64 = ctx.r[11].s64 + 25972;
	// 826EDA74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EDA78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDA80: 4BD793A1  bl 0x82466e20
	ctx.lr = 0x826EDA84;
	sub_82466E20(ctx, base);
	// 826EDA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDA98 size=108
    let mut pc: u32 = 0x826EDA98;
    'dispatch: loop {
        match pc {
            0x826EDA98 => {
    //   block [0x826EDA98..0x826EDB04)
	// 826EDA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDAA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDAAC: 38EB8900  addi r7, r11, -0x7700
	ctx.r[7].s64 = ctx.r[11].s64 + -30464;
	// 826EDAB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDAB4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826EDAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDAC8: 386A65A4  addi r3, r10, 0x65a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26020;
	// 826EDACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDAF0: 4BD79331  bl 0x82466e20
	ctx.lr = 0x826EDAF4;
	sub_82466E20(ctx, base);
	// 826EDAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDB08 size=112
    let mut pc: u32 = 0x826EDB08;
    'dispatch: loop {
        match pc {
            0x826EDB08 => {
    //   block [0x826EDB08..0x826EDB78)
	// 826EDB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDB14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDB1C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDB20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDB24: 390B8990  addi r8, r11, -0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + -30320;
	// 826EDB28: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826EDB2C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826EDB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDB34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDB40: 386A65D4  addi r3, r10, 0x65d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26068;
	// 826EDB44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDB64: 4BD792BD  bl 0x82466e20
	ctx.lr = 0x826EDB68;
	sub_82466E20(ctx, base);
	// 826EDB68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDB78 size=112
    let mut pc: u32 = 0x826EDB78;
    'dispatch: loop {
        match pc {
            0x826EDB78 => {
    //   block [0x826EDB78..0x826EDBE8)
	// 826EDB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDB84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDB88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDB8C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDB90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDB94: 390B8AB0  addi r8, r11, -0x7550
	ctx.r[8].s64 = ctx.r[11].s64 + -30032;
	// 826EDB98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EDB9C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826EDBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDBA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDBB0: 386A6604  addi r3, r10, 0x6604
	ctx.r[3].s64 = ctx.r[10].s64 + 26116;
	// 826EDBB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDBD4: 4BD7924D  bl 0x82466e20
	ctx.lr = 0x826EDBD8;
	sub_82466E20(ctx, base);
	// 826EDBD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDBE8 size=108
    let mut pc: u32 = 0x826EDBE8;
    'dispatch: loop {
        match pc {
            0x826EDBE8 => {
    //   block [0x826EDBE8..0x826EDC54)
	// 826EDBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDBF4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDBFC: 38EB8AC8  addi r7, r11, -0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + -30008;
	// 826EDC00: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDC04: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826EDC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDC0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDC18: 386A6634  addi r3, r10, 0x6634
	ctx.r[3].s64 = ctx.r[10].s64 + 26164;
	// 826EDC1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDC3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDC40: 4BD791E1  bl 0x82466e20
	ctx.lr = 0x826EDC44;
	sub_82466E20(ctx, base);
	// 826EDC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDC58 size=112
    let mut pc: u32 = 0x826EDC58;
    'dispatch: loop {
        match pc {
            0x826EDC58 => {
    //   block [0x826EDC58..0x826EDCC8)
	// 826EDC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDC64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDC6C: 38AA6574  addi r5, r10, 0x6574
	ctx.r[5].s64 = ctx.r[10].s64 + 25972;
	// 826EDC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDC74: 390B8B58  addi r8, r11, -0x74a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29864;
	// 826EDC78: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826EDC7C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826EDC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDC84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDC88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDC90: 386A6664  addi r3, r10, 0x6664
	ctx.r[3].s64 = ctx.r[10].s64 + 26212;
	// 826EDC94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EDC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDCB4: 4BD7916D  bl 0x82466e20
	ctx.lr = 0x826EDCB8;
	sub_82466E20(ctx, base);
	// 826EDCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDCC8 size=108
    let mut pc: u32 = 0x826EDCC8;
    'dispatch: loop {
        match pc {
            0x826EDCC8 => {
    //   block [0x826EDCC8..0x826EDD34)
	// 826EDCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDCD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDCDC: 38EB8C48  addi r7, r11, -0x73b8
	ctx.r[7].s64 = ctx.r[11].s64 + -29624;
	// 826EDCE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EDCE4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826EDCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDCEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDCF8: 386A6694  addi r3, r10, 0x6694
	ctx.r[3].s64 = ctx.r[10].s64 + 26260;
	// 826EDCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDD20: 4BD79101  bl 0x82466e20
	ctx.lr = 0x826EDD24;
	sub_82466E20(ctx, base);
	// 826EDD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDD38 size=108
    let mut pc: u32 = 0x826EDD38;
    'dispatch: loop {
        match pc {
            0x826EDD38 => {
    //   block [0x826EDD38..0x826EDDA4)
	// 826EDD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDD44: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDD48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDD4C: 38EB8C60  addi r7, r11, -0x73a0
	ctx.r[7].s64 = ctx.r[11].s64 + -29600;
	// 826EDD50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EDD54: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826EDD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDD60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDD68: 386A66C4  addi r3, r10, 0x66c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26308;
	// 826EDD6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDD74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDD8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDD90: 4BD79091  bl 0x82466e20
	ctx.lr = 0x826EDD94;
	sub_82466E20(ctx, base);
	// 826EDD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDDA8 size=116
    let mut pc: u32 = 0x826EDDA8;
    'dispatch: loop {
        match pc {
            0x826EDDA8 => {
    //   block [0x826EDDA8..0x826EDE1C)
	// 826EDDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDDB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDDB8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EDDBC: 390B8CC4  addi r8, r11, -0x733c
	ctx.r[8].s64 = ctx.r[11].s64 + -29500;
	// 826EDDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDDC4: 392A9474  addi r9, r10, -0x6b8c
	ctx.r[9].s64 = ctx.r[10].s64 + -27532;
	// 826EDDC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDDCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826EDDD0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDDD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EDDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDDDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDDEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EDDF0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826EDDF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EDDF8: 386B66F4  addi r3, r11, 0x66f4
	ctx.r[3].s64 = ctx.r[11].s64 + 26356;
	// 826EDDFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EDE00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDE08: 4BD79019  bl 0x82466e20
	ctx.lr = 0x826EDE0C;
	sub_82466E20(ctx, base);
	// 826EDE0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDE20 size=108
    let mut pc: u32 = 0x826EDE20;
    'dispatch: loop {
        match pc {
            0x826EDE20 => {
    //   block [0x826EDE20..0x826EDE8C)
	// 826EDE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDE2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDE34: 38EB8CE0  addi r7, r11, -0x7320
	ctx.r[7].s64 = ctx.r[11].s64 + -29472;
	// 826EDE38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EDE3C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826EDE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDE48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDE50: 386A6724  addi r3, r10, 0x6724
	ctx.r[3].s64 = ctx.r[10].s64 + 26404;
	// 826EDE54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDE74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDE78: 4BD78FA9  bl 0x82466e20
	ctx.lr = 0x826EDE7C;
	sub_82466E20(ctx, base);
	// 826EDE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDE90 size=108
    let mut pc: u32 = 0x826EDE90;
    'dispatch: loop {
        match pc {
            0x826EDE90 => {
    //   block [0x826EDE90..0x826EDEFC)
	// 826EDE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDE9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDEA4: 38EB8D28  addi r7, r11, -0x72d8
	ctx.r[7].s64 = ctx.r[11].s64 + -29400;
	// 826EDEA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDEAC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826EDEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDEB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDEC0: 386A6754  addi r3, r10, 0x6754
	ctx.r[3].s64 = ctx.r[10].s64 + 26452;
	// 826EDEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDEE8: 4BD78F39  bl 0x82466e20
	ctx.lr = 0x826EDEEC;
	sub_82466E20(ctx, base);
	// 826EDEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDF00 size=108
    let mut pc: u32 = 0x826EDF00;
    'dispatch: loop {
        match pc {
            0x826EDF00 => {
    //   block [0x826EDF00..0x826EDF6C)
	// 826EDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDF0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDF14: 38EB8DB8  addi r7, r11, -0x7248
	ctx.r[7].s64 = ctx.r[11].s64 + -29256;
	// 826EDF18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EDF1C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826EDF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDF24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EDF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDF30: 386A6784  addi r3, r10, 0x6784
	ctx.r[3].s64 = ctx.r[10].s64 + 26500;
	// 826EDF34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EDF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EDF58: 4BD78EC9  bl 0x82466e20
	ctx.lr = 0x826EDF5C;
	sub_82466E20(ctx, base);
	// 826EDF5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDF70 size=100
    let mut pc: u32 = 0x826EDF70;
    'dispatch: loop {
        match pc {
            0x826EDF70 => {
    //   block [0x826EDF70..0x826EDFD4)
	// 826EDF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDF7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EDF84: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EDF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EDF90: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826EDF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EDF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EDFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EDFA4: 386A67B4  addi r3, r10, 0x67b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26548;
	// 826EDFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EDFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EDFB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EDFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EDFB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EDFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EDFC0: 4BD78E61  bl 0x82466e20
	ctx.lr = 0x826EDFC4;
	sub_82466E20(ctx, base);
	// 826EDFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EDFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EDFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EDFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EDFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EDFD8 size=112
    let mut pc: u32 = 0x826EDFD8;
    'dispatch: loop {
        match pc {
            0x826EDFD8 => {
    //   block [0x826EDFD8..0x826EE048)
	// 826EDFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EDFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EDFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EDFE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EDFE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EDFEC: 38AA67B4  addi r5, r10, 0x67b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26548;
	// 826EDFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EDFF4: 390B8E48  addi r8, r11, -0x71b8
	ctx.r[8].s64 = ctx.r[11].s64 + -29112;
	// 826EDFF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826EDFFC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826EE000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE010: 386A67E4  addi r3, r10, 0x67e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26596;
	// 826EE014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE034: 4BD78DED  bl 0x82466e20
	ctx.lr = 0x826EE038;
	sub_82466E20(ctx, base);
	// 826EE038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE048 size=108
    let mut pc: u32 = 0x826EE048;
    'dispatch: loop {
        match pc {
            0x826EE048 => {
    //   block [0x826EE048..0x826EE0B4)
	// 826EE048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE054: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE05C: 38EB8EA8  addi r7, r11, -0x7158
	ctx.r[7].s64 = ctx.r[11].s64 + -29016;
	// 826EE060: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EE064: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826EE068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE06C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE078: 386A6814  addi r3, r10, 0x6814
	ctx.r[3].s64 = ctx.r[10].s64 + 26644;
	// 826EE07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE0A0: 4BD78D81  bl 0x82466e20
	ctx.lr = 0x826EE0A4;
	sub_82466E20(ctx, base);
	// 826EE0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE0B8 size=108
    let mut pc: u32 = 0x826EE0B8;
    'dispatch: loop {
        match pc {
            0x826EE0B8 => {
    //   block [0x826EE0B8..0x826EE124)
	// 826EE0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE0C4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE0CC: 38EB8ED8  addi r7, r11, -0x7128
	ctx.r[7].s64 = ctx.r[11].s64 + -28968;
	// 826EE0D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE0D4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826EE0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE0DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE0E8: 386A6844  addi r3, r10, 0x6844
	ctx.r[3].s64 = ctx.r[10].s64 + 26692;
	// 826EE0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE110: 4BD78D11  bl 0x82466e20
	ctx.lr = 0x826EE114;
	sub_82466E20(ctx, base);
	// 826EE114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE128 size=108
    let mut pc: u32 = 0x826EE128;
    'dispatch: loop {
        match pc {
            0x826EE128 => {
    //   block [0x826EE128..0x826EE194)
	// 826EE128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE134: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE13C: 38EB8F38  addi r7, r11, -0x70c8
	ctx.r[7].s64 = ctx.r[11].s64 + -28872;
	// 826EE140: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE144: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826EE148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE14C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE158: 386A6874  addi r3, r10, 0x6874
	ctx.r[3].s64 = ctx.r[10].s64 + 26740;
	// 826EE15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE180: 4BD78CA1  bl 0x82466e20
	ctx.lr = 0x826EE184;
	sub_82466E20(ctx, base);
	// 826EE184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EE198 size=24
    let mut pc: u32 = 0x826EE198;
    'dispatch: loop {
        match pc {
            0x826EE198 => {
    //   block [0x826EE198..0x826EE1B0)
	// 826EE198: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE19C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE1A0: 394A0F20  addi r10, r10, 0xf20
	ctx.r[10].s64 = ctx.r[10].s64 + 3872;
	// 826EE1A4: 816B8CDC  lwz r11, -0x7324(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29476 as u32) ) } as u64;
	// 826EE1A8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 826EE1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE1B0 size=116
    let mut pc: u32 = 0x826EE1B0;
    'dispatch: loop {
        match pc {
            0x826EE1B0 => {
    //   block [0x826EE1B0..0x826EE224)
	// 826EE1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE1BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE1C0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE1C4: 390B0F20  addi r8, r11, 0xf20
	ctx.r[8].s64 = ctx.r[11].s64 + 3872;
	// 826EE1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE1CC: 392A94A8  addi r9, r10, -0x6b58
	ctx.r[9].s64 = ctx.r[10].s64 + -27480;
	// 826EE1D0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE1D4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826EE1D8: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE1DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE1E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE1F4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EE1F8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826EE1FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE200: 386B68A4  addi r3, r11, 0x68a4
	ctx.r[3].s64 = ctx.r[11].s64 + 26788;
	// 826EE204: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE208: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE210: 4BD78C11  bl 0x82466e20
	ctx.lr = 0x826EE214;
	sub_82466E20(ctx, base);
	// 826EE214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE228 size=112
    let mut pc: u32 = 0x826EE228;
    'dispatch: loop {
        match pc {
            0x826EE228 => {
    //   block [0x826EE228..0x826EE298)
	// 826EE228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE234: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE23C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE244: 390B8F98  addi r8, r11, -0x7068
	ctx.r[8].s64 = ctx.r[11].s64 + -28776;
	// 826EE248: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EE24C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 826EE250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE260: 386A68D4  addi r3, r10, 0x68d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26836;
	// 826EE264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE284: 4BD78B9D  bl 0x82466e20
	ctx.lr = 0x826EE288;
	sub_82466E20(ctx, base);
	// 826EE288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE298 size=112
    let mut pc: u32 = 0x826EE298;
    'dispatch: loop {
        match pc {
            0x826EE298 => {
    //   block [0x826EE298..0x826EE308)
	// 826EE298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE2A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE2A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE2AC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE2B4: 390B8FE0  addi r8, r11, -0x7020
	ctx.r[8].s64 = ctx.r[11].s64 + -28704;
	// 826EE2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EE2BC: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826EE2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE2C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE2D0: 386A6904  addi r3, r10, 0x6904
	ctx.r[3].s64 = ctx.r[10].s64 + 26884;
	// 826EE2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE2F4: 4BD78B2D  bl 0x82466e20
	ctx.lr = 0x826EE2F8;
	sub_82466E20(ctx, base);
	// 826EE2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE308 size=112
    let mut pc: u32 = 0x826EE308;
    'dispatch: loop {
        match pc {
            0x826EE308 => {
    //   block [0x826EE308..0x826EE378)
	// 826EE308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE318: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE31C: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE324: 390B9028  addi r8, r11, -0x6fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -28632;
	// 826EE328: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826EE32C: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826EE330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE340: 386A6934  addi r3, r10, 0x6934
	ctx.r[3].s64 = ctx.r[10].s64 + 26932;
	// 826EE344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE364: 4BD78ABD  bl 0x82466e20
	ctx.lr = 0x826EE368;
	sub_82466E20(ctx, base);
	// 826EE368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE378 size=108
    let mut pc: u32 = 0x826EE378;
    'dispatch: loop {
        match pc {
            0x826EE378 => {
    //   block [0x826EE378..0x826EE3E4)
	// 826EE378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE384: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE38C: 38EB9118  addi r7, r11, -0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -28392;
	// 826EE390: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EE394: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826EE398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE3A8: 386A6964  addi r3, r10, 0x6964
	ctx.r[3].s64 = ctx.r[10].s64 + 26980;
	// 826EE3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE3D0: 4BD78A51  bl 0x82466e20
	ctx.lr = 0x826EE3D4;
	sub_82466E20(ctx, base);
	// 826EE3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE3E8 size=108
    let mut pc: u32 = 0x826EE3E8;
    'dispatch: loop {
        match pc {
            0x826EE3E8 => {
    //   block [0x826EE3E8..0x826EE454)
	// 826EE3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE3F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE3FC: 38EB9178  addi r7, r11, -0x6e88
	ctx.r[7].s64 = ctx.r[11].s64 + -28296;
	// 826EE400: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EE404: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826EE408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE40C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE418: 386A6994  addi r3, r10, 0x6994
	ctx.r[3].s64 = ctx.r[10].s64 + 27028;
	// 826EE41C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE43C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE440: 4BD789E1  bl 0x82466e20
	ctx.lr = 0x826EE444;
	sub_82466E20(ctx, base);
	// 826EE444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE458 size=112
    let mut pc: u32 = 0x826EE458;
    'dispatch: loop {
        match pc {
            0x826EE458 => {
    //   block [0x826EE458..0x826EE4C8)
	// 826EE458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE464: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE46C: 392B94DC  addi r9, r11, -0x6b24
	ctx.r[9].s64 = ctx.r[11].s64 + -27428;
	// 826EE470: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826EE474: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EE478: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE47C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826EE480: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE484: 396B9228  addi r11, r11, -0x6dd8
	ctx.r[11].s64 = ctx.r[11].s64 + -28120;
	// 826EE488: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EE48C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE490: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EE494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE498: 386A69C4  addi r3, r10, 0x69c4
	ctx.r[3].s64 = ctx.r[10].s64 + 27076;
	// 826EE49C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE4A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EE4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE4A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EE4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE4B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE4B4: 4BD7896D  bl 0x82466e20
	ctx.lr = 0x826EE4B8;
	sub_82466E20(ctx, base);
	// 826EE4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE4C8 size=112
    let mut pc: u32 = 0x826EE4C8;
    'dispatch: loop {
        match pc {
            0x826EE4C8 => {
    //   block [0x826EE4C8..0x826EE538)
	// 826EE4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE4D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE4D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE4DC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE4E4: 390B9378  addi r8, r11, -0x6c88
	ctx.r[8].s64 = ctx.r[11].s64 + -27784;
	// 826EE4E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826EE4EC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826EE4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE500: 386A69F4  addi r3, r10, 0x69f4
	ctx.r[3].s64 = ctx.r[10].s64 + 27124;
	// 826EE504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE524: 4BD788FD  bl 0x82466e20
	ctx.lr = 0x826EE528;
	sub_82466E20(ctx, base);
	// 826EE528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE538 size=112
    let mut pc: u32 = 0x826EE538;
    'dispatch: loop {
        match pc {
            0x826EE538 => {
    //   block [0x826EE538..0x826EE5A8)
	// 826EE538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE548: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE54C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE554: 390B9420  addi r8, r11, -0x6be0
	ctx.r[8].s64 = ctx.r[11].s64 + -27616;
	// 826EE558: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826EE55C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826EE560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE570: 386A6A24  addi r3, r10, 0x6a24
	ctx.r[3].s64 = ctx.r[10].s64 + 27172;
	// 826EE574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE594: 4BD7888D  bl 0x82466e20
	ctx.lr = 0x826EE598;
	sub_82466E20(ctx, base);
	// 826EE598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE5A8 size=112
    let mut pc: u32 = 0x826EE5A8;
    'dispatch: loop {
        match pc {
            0x826EE5A8 => {
    //   block [0x826EE5A8..0x826EE618)
	// 826EE5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE5B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE5B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE5BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE5C4: 390B94B0  addi r8, r11, -0x6b50
	ctx.r[8].s64 = ctx.r[11].s64 + -27472;
	// 826EE5C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826EE5CC: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826EE5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE5D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE5E0: 386A6A54  addi r3, r10, 0x6a54
	ctx.r[3].s64 = ctx.r[10].s64 + 27220;
	// 826EE5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE604: 4BD7881D  bl 0x82466e20
	ctx.lr = 0x826EE608;
	sub_82466E20(ctx, base);
	// 826EE608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE618 size=108
    let mut pc: u32 = 0x826EE618;
    'dispatch: loop {
        match pc {
            0x826EE618 => {
    //   block [0x826EE618..0x826EE684)
	// 826EE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE624: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE62C: 38EB9528  addi r7, r11, -0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -27352;
	// 826EE630: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EE634: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826EE638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE63C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE648: 386A6A84  addi r3, r10, 0x6a84
	ctx.r[3].s64 = ctx.r[10].s64 + 27268;
	// 826EE64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE670: 4BD787B1  bl 0x82466e20
	ctx.lr = 0x826EE674;
	sub_82466E20(ctx, base);
	// 826EE674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE688 size=112
    let mut pc: u32 = 0x826EE688;
    'dispatch: loop {
        match pc {
            0x826EE688 => {
    //   block [0x826EE688..0x826EE6F8)
	// 826EE688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE694: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE698: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE69C: 392A9538  addi r9, r10, -0x6ac8
	ctx.r[9].s64 = ctx.r[10].s64 + -27336;
	// 826EE6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE6A4: 390B95D0  addi r8, r11, -0x6a30
	ctx.r[8].s64 = ctx.r[11].s64 + -27184;
	// 826EE6A8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826EE6AC: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826EE6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE6B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE6BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE6C0: 386A6AB4  addi r3, r10, 0x6ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 27316;
	// 826EE6C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE6C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE6E4: 4BD7873D  bl 0x82466e20
	ctx.lr = 0x826EE6E8;
	sub_82466E20(ctx, base);
	// 826EE6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE6F8 size=100
    let mut pc: u32 = 0x826EE6F8;
    'dispatch: loop {
        match pc {
            0x826EE6F8 => {
    //   block [0x826EE6F8..0x826EE75C)
	// 826EE6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE704: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE70C: 38AA7294  addi r5, r10, 0x7294
	ctx.r[5].s64 = ctx.r[10].s64 + 29332;
	// 826EE710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE718: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826EE71C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE72C: 386A6AE4  addi r3, r10, 0x6ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 27364;
	// 826EE730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EE73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE748: 4BD786D9  bl 0x82466e20
	ctx.lr = 0x826EE74C;
	sub_82466E20(ctx, base);
	// 826EE74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE760 size=108
    let mut pc: u32 = 0x826EE760;
    'dispatch: loop {
        match pc {
            0x826EE760 => {
    //   block [0x826EE760..0x826EE7CC)
	// 826EE760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE76C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE774: 38EB9604  addi r7, r11, -0x69fc
	ctx.r[7].s64 = ctx.r[11].s64 + -27132;
	// 826EE778: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EE77C: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826EE780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EE78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE790: 386A6B14  addi r3, r10, 0x6b14
	ctx.r[3].s64 = ctx.r[10].s64 + 27412;
	// 826EE794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EE798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE7B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE7B8: 4BD78669  bl 0x82466e20
	ctx.lr = 0x826EE7BC;
	sub_82466E20(ctx, base);
	// 826EE7BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE7D0 size=112
    let mut pc: u32 = 0x826EE7D0;
    'dispatch: loop {
        match pc {
            0x826EE7D0 => {
    //   block [0x826EE7D0..0x826EE840)
	// 826EE7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE7DC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EE7E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE7E4: 392A9598  addi r9, r10, -0x6a68
	ctx.r[9].s64 = ctx.r[10].s64 + -27240;
	// 826EE7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE7EC: 390B9638  addi r8, r11, -0x69c8
	ctx.r[8].s64 = ctx.r[11].s64 + -27080;
	// 826EE7F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826EE7F4: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826EE7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE808: 386A6B44  addi r3, r10, 0x6b44
	ctx.r[3].s64 = ctx.r[10].s64 + 27460;
	// 826EE80C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE810: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EE828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE82C: 4BD785F5  bl 0x82466e20
	ctx.lr = 0x826EE830;
	sub_82466E20(ctx, base);
	// 826EE830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE840 size=112
    let mut pc: u32 = 0x826EE840;
    'dispatch: loop {
        match pc {
            0x826EE840 => {
    //   block [0x826EE840..0x826EE8B0)
	// 826EE840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE84C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE850: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE854: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE85C: 390B96B0  addi r8, r11, -0x6950
	ctx.r[8].s64 = ctx.r[11].s64 + -26960;
	// 826EE860: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EE864: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826EE868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE878: 386A6B74  addi r3, r10, 0x6b74
	ctx.r[3].s64 = ctx.r[10].s64 + 27508;
	// 826EE87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EE880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE89C: 4BD78585  bl 0x82466e20
	ctx.lr = 0x826EE8A0;
	sub_82466E20(ctx, base);
	// 826EE8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE8B0 size=116
    let mut pc: u32 = 0x826EE8B0;
    'dispatch: loop {
        match pc {
            0x826EE8B0 => {
    //   block [0x826EE8B0..0x826EE924)
	// 826EE8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE8BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE8C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826EE8C4: 390A96E0  addi r8, r10, -0x6920
	ctx.r[8].s64 = ctx.r[10].s64 + -26912;
	// 826EE8C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE8CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE8D0: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EE8D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE8D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EE8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EE8E4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826EE8E8: 396B95AC  addi r11, r11, -0x6a54
	ctx.r[11].s64 = ctx.r[11].s64 + -27220;
	// 826EE8EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE8F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE8F4: 386A6BA4  addi r3, r10, 0x6ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 27556;
	// 826EE8F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EE8FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE900: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EE904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE910: 4BD78511  bl 0x82466e20
	ctx.lr = 0x826EE914;
	sub_82466E20(ctx, base);
	// 826EE914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE928 size=100
    let mut pc: u32 = 0x826EE928;
    'dispatch: loop {
        match pc {
            0x826EE928 => {
    //   block [0x826EE928..0x826EE98C)
	// 826EE928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EE93C: 38AA6BA4  addi r5, r10, 0x6ba4
	ctx.r[5].s64 = ctx.r[10].s64 + 27556;
	// 826EE940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EE948: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826EE94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EE954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EE95C: 386A6BD4  addi r3, r10, 0x6bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27604;
	// 826EE960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EE964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EE968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EE96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EE974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EE978: 4BD784A9  bl 0x82466e20
	ctx.lr = 0x826EE97C;
	sub_82466E20(ctx, base);
	// 826EE97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EE980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EE984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EE988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EE990 size=24
    let mut pc: u32 = 0x826EE990;
    'dispatch: loop {
        match pc {
            0x826EE990 => {
    //   block [0x826EE990..0x826EE9A8)
	// 826EE990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE994: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EE998: 394A1058  addi r10, r10, 0x1058
	ctx.r[10].s64 = ctx.r[10].s64 + 4184;
	// 826EE99C: 816B9634  lwz r11, -0x69cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27084 as u32) ) } as u64;
	// 826EE9A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826EE9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EE9A8 size=116
    let mut pc: u32 = 0x826EE9A8;
    'dispatch: loop {
        match pc {
            0x826EE9A8 => {
    //   block [0x826EE9A8..0x826EEA1C)
	// 826EE9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EE9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EE9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EE9B4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EE9B8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE9BC: 392B95E8  addi r9, r11, -0x6a18
	ctx.r[9].s64 = ctx.r[11].s64 + -27160;
	// 826EE9C0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EE9C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EE9C8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EE9CC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826EE9D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EE9D4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826EE9D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EE9DC: 396B1058  addi r11, r11, 0x1058
	ctx.r[11].s64 = ctx.r[11].s64 + 4184;
	// 826EE9E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EE9E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EE9E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EE9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EE9F0: 386A6C04  addi r3, r10, 0x6c04
	ctx.r[3].s64 = ctx.r[10].s64 + 27652;
	// 826EE9F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EE9F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EE9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEA00: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EEA04: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EEA08: 4BD78419  bl 0x82466e20
	ctx.lr = 0x826EEA0C;
	sub_82466E20(ctx, base);
	// 826EEA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEA20 size=116
    let mut pc: u32 = 0x826EEA20;
    'dispatch: loop {
        match pc {
            0x826EEA20 => {
    //   block [0x826EEA20..0x826EEA94)
	// 826EEA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEA2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EEA30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEA34: 392B963C  addi r9, r11, -0x69c4
	ctx.r[9].s64 = ctx.r[11].s64 + -27076;
	// 826EEA38: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEA3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEA40: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826EEA44: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826EEA48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEA4C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826EEA50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEA54: 396B9790  addi r11, r11, -0x6870
	ctx.r[11].s64 = ctx.r[11].s64 + -26736;
	// 826EEA58: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826EEA5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEA60: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826EEA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEA68: 386A6C34  addi r3, r10, 0x6c34
	ctx.r[3].s64 = ctx.r[10].s64 + 27700;
	// 826EEA6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EEA70: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826EEA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEA78: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826EEA7C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EEA80: 4BD783A1  bl 0x82466e20
	ctx.lr = 0x826EEA84;
	sub_82466E20(ctx, base);
	// 826EEA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEA98 size=108
    let mut pc: u32 = 0x826EEA98;
    'dispatch: loop {
        match pc {
            0x826EEA98 => {
    //   block [0x826EEA98..0x826EEB04)
	// 826EEA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEAA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEAAC: 38EB9868  addi r7, r11, -0x6798
	ctx.r[7].s64 = ctx.r[11].s64 + -26520;
	// 826EEAB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EEAB4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826EEAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEAC8: 386A6C64  addi r3, r10, 0x6c64
	ctx.r[3].s64 = ctx.r[10].s64 + 27748;
	// 826EEACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEAF0: 4BD78331  bl 0x82466e20
	ctx.lr = 0x826EEAF4;
	sub_82466E20(ctx, base);
	// 826EEAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EEB08 size=24
    let mut pc: u32 = 0x826EEB08;
    'dispatch: loop {
        match pc {
            0x826EEB08 => {
    //   block [0x826EEB08..0x826EEB20)
	// 826EEB08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEB0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEB10: 394A1100  addi r10, r10, 0x1100
	ctx.r[10].s64 = ctx.r[10].s64 + 4352;
	// 826EEB14: 816B978C  lwz r11, -0x6874(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26740 as u32) ) } as u64;
	// 826EEB18: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EEB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEB20 size=116
    let mut pc: u32 = 0x826EEB20;
    'dispatch: loop {
        match pc {
            0x826EEB20 => {
    //   block [0x826EEB20..0x826EEB94)
	// 826EEB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEB2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEB30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EEB34: 390B1100  addi r8, r11, 0x1100
	ctx.r[8].s64 = ctx.r[11].s64 + 4352;
	// 826EEB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEB3C: 392A96B0  addi r9, r10, -0x6950
	ctx.r[9].s64 = ctx.r[10].s64 + -26960;
	// 826EEB40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEB44: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826EEB48: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEB4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEB54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEB64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EEB68: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826EEB6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EEB70: 386B6C94  addi r3, r11, 0x6c94
	ctx.r[3].s64 = ctx.r[11].s64 + 27796;
	// 826EEB74: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826EEB78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEB80: 4BD782A1  bl 0x82466e20
	ctx.lr = 0x826EEB84;
	sub_82466E20(ctx, base);
	// 826EEB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEB98 size=112
    let mut pc: u32 = 0x826EEB98;
    'dispatch: loop {
        match pc {
            0x826EEB98 => {
    //   block [0x826EEB98..0x826EEC08)
	// 826EEB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEBA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEBAC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EEBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEBB4: 390B98CC  addi r8, r11, -0x6734
	ctx.r[8].s64 = ctx.r[11].s64 + -26420;
	// 826EEBB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EEBBC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826EEBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEBD0: 386A6CC4  addi r3, r10, 0x6cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 27844;
	// 826EEBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEBF4: 4BD7822D  bl 0x82466e20
	ctx.lr = 0x826EEBF8;
	sub_82466E20(ctx, base);
	// 826EEBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EEC08 size=24
    let mut pc: u32 = 0x826EEC08;
    'dispatch: loop {
        match pc {
            0x826EEC08 => {
    //   block [0x826EEC08..0x826EEC20)
	// 826EEC08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEC0C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEC10: 394A1298  addi r10, r10, 0x1298
	ctx.r[10].s64 = ctx.r[10].s64 + 4760;
	// 826EEC14: 816B98FC  lwz r11, -0x6704(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26372 as u32) ) } as u64;
	// 826EEC18: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826EEC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEC20 size=116
    let mut pc: u32 = 0x826EEC20;
    'dispatch: loop {
        match pc {
            0x826EEC20 => {
    //   block [0x826EEC20..0x826EEC94)
	// 826EEC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEC2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEC30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EEC34: 390B1298  addi r8, r11, 0x1298
	ctx.r[8].s64 = ctx.r[11].s64 + 4760;
	// 826EEC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEC3C: 392A96E8  addi r9, r10, -0x6918
	ctx.r[9].s64 = ctx.r[10].s64 + -26904;
	// 826EEC40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEC44: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826EEC48: 38AA6C34  addi r5, r10, 0x6c34
	ctx.r[5].s64 = ctx.r[10].s64 + 27700;
	// 826EEC4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEC54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEC64: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EEC68: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826EEC6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EEC70: 386B6CF4  addi r3, r11, 0x6cf4
	ctx.r[3].s64 = ctx.r[11].s64 + 27892;
	// 826EEC74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EEC78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEC80: 4BD781A1  bl 0x82466e20
	ctx.lr = 0x826EEC84;
	sub_82466E20(ctx, base);
	// 826EEC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEC98 size=112
    let mut pc: u32 = 0x826EEC98;
    'dispatch: loop {
        match pc {
            0x826EEC98 => {
    //   block [0x826EEC98..0x826EED08)
	// 826EEC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EECA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EECA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EECA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EECAC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826EECB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EECB4: 390B9900  addi r8, r11, -0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + -26368;
	// 826EECB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EECBC: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826EECC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EECC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EECC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EECCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EECD0: 386A6D24  addi r3, r10, 0x6d24
	ctx.r[3].s64 = ctx.r[10].s64 + 27940;
	// 826EECD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EECD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EECDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EECE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EECE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EECEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EECF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EECF4: 4BD7812D  bl 0x82466e20
	ctx.lr = 0x826EECF8;
	sub_82466E20(ctx, base);
	// 826EECF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EECFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EED00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EED08 size=100
    let mut pc: u32 = 0x826EED08;
    'dispatch: loop {
        match pc {
            0x826EED08 => {
    //   block [0x826EED08..0x826EED6C)
	// 826EED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EED10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EED14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EED1C: 38AA7294  addi r5, r10, 0x7294
	ctx.r[5].s64 = ctx.r[10].s64 + 29332;
	// 826EED20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EED24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EED28: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826EED2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EED34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EED38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EED3C: 386A6D54  addi r3, r10, 0x6d54
	ctx.r[3].s64 = ctx.r[10].s64 + 27988;
	// 826EED40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EED44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EED48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EED4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EED50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EED54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EED58: 4BD780C9  bl 0x82466e20
	ctx.lr = 0x826EED5C;
	sub_82466E20(ctx, base);
	// 826EED5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EED60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EED64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EED68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EED70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EED70 size=108
    let mut pc: u32 = 0x826EED70;
    'dispatch: loop {
        match pc {
            0x826EED70 => {
    //   block [0x826EED70..0x826EEDDC)
	// 826EED70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EED74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EED78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EED7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EED80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EED84: 38EB9918  addi r7, r11, -0x66e8
	ctx.r[7].s64 = ctx.r[11].s64 + -26344;
	// 826EED88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826EED8C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826EED90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EED94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EED98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EED9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEDA0: 386A6D84  addi r3, r10, 0x6d84
	ctx.r[3].s64 = ctx.r[10].s64 + 28036;
	// 826EEDA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEDC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEDC8: 4BD78059  bl 0x82466e20
	ctx.lr = 0x826EEDCC;
	sub_82466E20(ctx, base);
	// 826EEDCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEDE0 size=112
    let mut pc: u32 = 0x826EEDE0;
    'dispatch: loop {
        match pc {
            0x826EEDE0 => {
    //   block [0x826EEDE0..0x826EEE50)
	// 826EEDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEDEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEDF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEDF4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EEDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEDFC: 390B99F0  addi r8, r11, -0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + -26128;
	// 826EEE00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EEE04: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826EEE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEE0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEE18: 386A6DB4  addi r3, r10, 0x6db4
	ctx.r[3].s64 = ctx.r[10].s64 + 28084;
	// 826EEE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEE3C: 4BD77FE5  bl 0x82466e20
	ctx.lr = 0x826EEE40;
	sub_82466E20(ctx, base);
	// 826EEE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEE50 size=108
    let mut pc: u32 = 0x826EEE50;
    'dispatch: loop {
        match pc {
            0x826EEE50 => {
    //   block [0x826EEE50..0x826EEEBC)
	// 826EEE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEE5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEE64: 38EB9A20  addi r7, r11, -0x65e0
	ctx.r[7].s64 = ctx.r[11].s64 + -26080;
	// 826EEE68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EEE6C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826EEE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEE74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEE80: 386A6DE4  addi r3, r10, 0x6de4
	ctx.r[3].s64 = ctx.r[10].s64 + 28132;
	// 826EEE84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEEA8: 4BD77F79  bl 0x82466e20
	ctx.lr = 0x826EEEAC;
	sub_82466E20(ctx, base);
	// 826EEEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEEC0 size=112
    let mut pc: u32 = 0x826EEEC0;
    'dispatch: loop {
        match pc {
            0x826EEEC0 => {
    //   block [0x826EEEC0..0x826EEF30)
	// 826EEEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEED0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEED4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EEED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEEDC: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 826EEEE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EEEE4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826EEEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEEEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEEF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EEEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEEF8: 386A6E14  addi r3, r10, 0x6e14
	ctx.r[3].s64 = ctx.r[10].s64 + 28180;
	// 826EEEFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EEF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEF1C: 4BD77F05  bl 0x82466e20
	ctx.lr = 0x826EEF20;
	sub_82466E20(ctx, base);
	// 826EEF20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEF30 size=112
    let mut pc: u32 = 0x826EEF30;
    'dispatch: loop {
        match pc {
            0x826EEF30 => {
    //   block [0x826EEF30..0x826EEFA0)
	// 826EEF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEF3C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EEF40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826EEF44: 38EA9A68  addi r7, r10, -0x6598
	ctx.r[7].s64 = ctx.r[10].s64 + -26008;
	// 826EEF48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEF4C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EEF50: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826EEF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEF58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEF5C: 396B96FC  addi r11, r11, -0x6904
	ctx.r[11].s64 = ctx.r[11].s64 + -26884;
	// 826EEF60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEF68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEF6C: 386A6E44  addi r3, r10, 0x6e44
	ctx.r[3].s64 = ctx.r[10].s64 + 28228;
	// 826EEF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEF74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EEF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEF7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EEF80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEF84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEF88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEF8C: 4BD77E95  bl 0x82466e20
	ctx.lr = 0x826EEF90;
	sub_82466E20(ctx, base);
	// 826EEF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EEF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EEF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EEF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EEFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EEFA0 size=108
    let mut pc: u32 = 0x826EEFA0;
    'dispatch: loop {
        match pc {
            0x826EEFA0 => {
    //   block [0x826EEFA0..0x826EF00C)
	// 826EEFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EEFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EEFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EEFAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EEFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EEFB4: 38EB9B40  addi r7, r11, -0x64c0
	ctx.r[7].s64 = ctx.r[11].s64 + -25792;
	// 826EEFB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826EEFBC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826EEFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EEFC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EEFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EEFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EEFD0: 386A6E74  addi r3, r10, 0x6e74
	ctx.r[3].s64 = ctx.r[10].s64 + 28276;
	// 826EEFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EEFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EEFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EEFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EEFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EEFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EEFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EEFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EEFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EEFF8: 4BD77E29  bl 0x82466e20
	ctx.lr = 0x826EEFFC;
	sub_82466E20(ctx, base);
	// 826EEFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF010 size=108
    let mut pc: u32 = 0x826EF010;
    'dispatch: loop {
        match pc {
            0x826EF010 => {
    //   block [0x826EF010..0x826EF07C)
	// 826EF010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF01C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF024: 38EB9B58  addi r7, r11, -0x64a8
	ctx.r[7].s64 = ctx.r[11].s64 + -25768;
	// 826EF028: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826EF02C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826EF030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF034: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF040: 386A6EA4  addi r3, r10, 0x6ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 28324;
	// 826EF044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF068: 4BD77DB9  bl 0x82466e20
	ctx.lr = 0x826EF06C;
	sub_82466E20(ctx, base);
	// 826EF06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF080 size=108
    let mut pc: u32 = 0x826EF080;
    'dispatch: loop {
        match pc {
            0x826EF080 => {
    //   block [0x826EF080..0x826EF0EC)
	// 826EF080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF08C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF094: 38EB9C60  addi r7, r11, -0x63a0
	ctx.r[7].s64 = ctx.r[11].s64 + -25504;
	// 826EF098: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EF09C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826EF0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF0A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF0B0: 386A6ED4  addi r3, r10, 0x6ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 28372;
	// 826EF0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF0D8: 4BD77D49  bl 0x82466e20
	ctx.lr = 0x826EF0DC;
	sub_82466E20(ctx, base);
	// 826EF0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF0F0 size=112
    let mut pc: u32 = 0x826EF0F0;
    'dispatch: loop {
        match pc {
            0x826EF0F0 => {
    //   block [0x826EF0F0..0x826EF160)
	// 826EF0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF0FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF100: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF104: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF10C: 390B9CC0  addi r8, r11, -0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + -25408;
	// 826EF110: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826EF114: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826EF118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF128: 386A6F04  addi r3, r10, 0x6f04
	ctx.r[3].s64 = ctx.r[10].s64 + 28420;
	// 826EF12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF14C: 4BD77CD5  bl 0x82466e20
	ctx.lr = 0x826EF150;
	sub_82466E20(ctx, base);
	// 826EF150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF160 size=112
    let mut pc: u32 = 0x826EF160;
    'dispatch: loop {
        match pc {
            0x826EF160 => {
    //   block [0x826EF160..0x826EF1D0)
	// 826EF160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF16C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF170: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF174: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF17C: 390B9DE0  addi r8, r11, -0x6220
	ctx.r[8].s64 = ctx.r[11].s64 + -25120;
	// 826EF180: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF184: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826EF188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF190: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF198: 386A6F34  addi r3, r10, 0x6f34
	ctx.r[3].s64 = ctx.r[10].s64 + 28468;
	// 826EF19C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF1A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF1B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF1BC: 4BD77C65  bl 0x82466e20
	ctx.lr = 0x826EF1C0;
	sub_82466E20(ctx, base);
	// 826EF1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF1D0 size=116
    let mut pc: u32 = 0x826EF1D0;
    'dispatch: loop {
        match pc {
            0x826EF1D0 => {
    //   block [0x826EF1D0..0x826EF244)
	// 826EF1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF1DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF1E0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826EF1E4: 390A9DF8  addi r8, r10, -0x6208
	ctx.r[8].s64 = ctx.r[10].s64 + -25096;
	// 826EF1E8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF1EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF1F0: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF1F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF204: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826EF208: 396B972C  addi r11, r11, -0x68d4
	ctx.r[11].s64 = ctx.r[11].s64 + -26836;
	// 826EF20C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF210: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF214: 386A6F64  addi r3, r10, 0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + 28516;
	// 826EF218: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF21C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF220: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF230: 4BD77BF1  bl 0x82466e20
	ctx.lr = 0x826EF234;
	sub_82466E20(ctx, base);
	// 826EF234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF248 size=108
    let mut pc: u32 = 0x826EF248;
    'dispatch: loop {
        match pc {
            0x826EF248 => {
    //   block [0x826EF248..0x826EF2B4)
	// 826EF248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF254: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EF25C: 38EB9E58  addi r7, r11, -0x61a8
	ctx.r[7].s64 = ctx.r[11].s64 + -25000;
	// 826EF260: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826EF264: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826EF268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF26C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF278: 386A6F94  addi r3, r10, 0x6f94
	ctx.r[3].s64 = ctx.r[10].s64 + 28564;
	// 826EF27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF2A0: 4BD77B81  bl 0x82466e20
	ctx.lr = 0x826EF2A4;
	sub_82466E20(ctx, base);
	// 826EF2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF2B8 size=112
    let mut pc: u32 = 0x826EF2B8;
    'dispatch: loop {
        match pc {
            0x826EF2B8 => {
    //   block [0x826EF2B8..0x826EF328)
	// 826EF2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF2C4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF2C8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EF2CC: 38EA9F00  addi r7, r10, -0x6100
	ctx.r[7].s64 = ctx.r[10].s64 + -24832;
	// 826EF2D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EF2D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF2D8: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826EF2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF2E0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF2E4: 396B9740  addi r11, r11, -0x68c0
	ctx.r[11].s64 = ctx.r[11].s64 + -26816;
	// 826EF2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF2EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF2F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF2F4: 386A6FC4  addi r3, r10, 0x6fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28612;
	// 826EF2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF2FC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF304: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF308: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF30C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF314: 4BD77B0D  bl 0x82466e20
	ctx.lr = 0x826EF318;
	sub_82466E20(ctx, base);
	// 826EF318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF328 size=112
    let mut pc: u32 = 0x826EF328;
    'dispatch: loop {
        match pc {
            0x826EF328 => {
    //   block [0x826EF328..0x826EF398)
	// 826EF328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF338: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF33C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF344: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826EF348: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EF34C: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826EF350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF360: 386A6FF4  addi r3, r10, 0x6ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 28660;
	// 826EF364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF384: 4BD77A9D  bl 0x82466e20
	ctx.lr = 0x826EF388;
	sub_82466E20(ctx, base);
	// 826EF388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF398 size=112
    let mut pc: u32 = 0x826EF398;
    'dispatch: loop {
        match pc {
            0x826EF398 => {
    //   block [0x826EF398..0x826EF408)
	// 826EF398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF3A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF3AC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF3B4: 390B9FC0  addi r8, r11, -0x6040
	ctx.r[8].s64 = ctx.r[11].s64 + -24640;
	// 826EF3B8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826EF3BC: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826EF3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF3C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF3D0: 386A7024  addi r3, r10, 0x7024
	ctx.r[3].s64 = ctx.r[10].s64 + 28708;
	// 826EF3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF3F4: 4BD77A2D  bl 0x82466e20
	ctx.lr = 0x826EF3F8;
	sub_82466E20(ctx, base);
	// 826EF3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF408 size=112
    let mut pc: u32 = 0x826EF408;
    'dispatch: loop {
        match pc {
            0x826EF408 => {
    //   block [0x826EF408..0x826EF478)
	// 826EF408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF418: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF41C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF424: 390BA0C8  addi r8, r11, -0x5f38
	ctx.r[8].s64 = ctx.r[11].s64 + -24376;
	// 826EF428: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF42C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826EF430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF440: 386A7054  addi r3, r10, 0x7054
	ctx.r[3].s64 = ctx.r[10].s64 + 28756;
	// 826EF444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF464: 4BD779BD  bl 0x82466e20
	ctx.lr = 0x826EF468;
	sub_82466E20(ctx, base);
	// 826EF468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF478 size=112
    let mut pc: u32 = 0x826EF478;
    'dispatch: loop {
        match pc {
            0x826EF478 => {
    //   block [0x826EF478..0x826EF4E8)
	// 826EF478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF488: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF48C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EF490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF494: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 826EF498: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EF49C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826EF4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF4B0: 386A7084  addi r3, r10, 0x7084
	ctx.r[3].s64 = ctx.r[10].s64 + 28804;
	// 826EF4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF4D4: 4BD7794D  bl 0x82466e20
	ctx.lr = 0x826EF4D8;
	sub_82466E20(ctx, base);
	// 826EF4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF4E8 size=108
    let mut pc: u32 = 0x826EF4E8;
    'dispatch: loop {
        match pc {
            0x826EF4E8 => {
    //   block [0x826EF4E8..0x826EF554)
	// 826EF4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF4F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF4FC: 38EBA110  addi r7, r11, -0x5ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -24304;
	// 826EF500: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EF504: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826EF508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF50C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF518: 386A70B4  addi r3, r10, 0x70b4
	ctx.r[3].s64 = ctx.r[10].s64 + 28852;
	// 826EF51C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF53C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF540: 4BD778E1  bl 0x82466e20
	ctx.lr = 0x826EF544;
	sub_82466E20(ctx, base);
	// 826EF544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EF558 size=24
    let mut pc: u32 = 0x826EF558;
    'dispatch: loop {
        match pc {
            0x826EF558 => {
    //   block [0x826EF558..0x826EF570)
	// 826EF558: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF55C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF560: 394A1388  addi r10, r10, 0x1388
	ctx.r[10].s64 = ctx.r[10].s64 + 5000;
	// 826EF564: 816BA188  lwz r11, -0x5e78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24184 as u32) ) } as u64;
	// 826EF568: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826EF56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF570 size=116
    let mut pc: u32 = 0x826EF570;
    'dispatch: loop {
        match pc {
            0x826EF570 => {
    //   block [0x826EF570..0x826EF5E4)
	// 826EF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF57C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF580: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EF584: 390B1388  addi r8, r11, 0x1388
	ctx.r[8].s64 = ctx.r[11].s64 + 5000;
	// 826EF588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF58C: 392A9778  addi r9, r10, -0x6888
	ctx.r[9].s64 = ctx.r[10].s64 + -26760;
	// 826EF590: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF594: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826EF598: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF59C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF5A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF5B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EF5B8: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826EF5BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF5C0: 386B70E4  addi r3, r11, 0x70e4
	ctx.r[3].s64 = ctx.r[11].s64 + 28900;
	// 826EF5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EF5C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF5D0: 4BD77851  bl 0x82466e20
	ctx.lr = 0x826EF5D4;
	sub_82466E20(ctx, base);
	// 826EF5D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF5E8 size=112
    let mut pc: u32 = 0x826EF5E8;
    'dispatch: loop {
        match pc {
            0x826EF5E8 => {
    //   block [0x826EF5E8..0x826EF658)
	// 826EF5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF5F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF5F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF5FC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF604: 390BA18C  addi r8, r11, -0x5e74
	ctx.r[8].s64 = ctx.r[11].s64 + -24180;
	// 826EF608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EF60C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826EF610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF620: 386A7114  addi r3, r10, 0x7114
	ctx.r[3].s64 = ctx.r[10].s64 + 28948;
	// 826EF624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF644: 4BD777DD  bl 0x82466e20
	ctx.lr = 0x826EF648;
	sub_82466E20(ctx, base);
	// 826EF648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF658 size=116
    let mut pc: u32 = 0x826EF658;
    'dispatch: loop {
        match pc {
            0x826EF658 => {
    //   block [0x826EF658..0x826EF6CC)
	// 826EF658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF664: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF668: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826EF66C: 390AA1C0  addi r8, r10, -0x5e40
	ctx.r[8].s64 = ctx.r[10].s64 + -24128;
	// 826EF670: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF674: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF678: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF67C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF680: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF68C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826EF690: 396B978C  addi r11, r11, -0x6874
	ctx.r[11].s64 = ctx.r[11].s64 + -26740;
	// 826EF694: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF698: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF69C: 386A7144  addi r3, r10, 0x7144
	ctx.r[3].s64 = ctx.r[10].s64 + 28996;
	// 826EF6A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF6A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF6A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF6B8: 4BD77769  bl 0x82466e20
	ctx.lr = 0x826EF6BC;
	sub_82466E20(ctx, base);
	// 826EF6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF6D0 size=112
    let mut pc: u32 = 0x826EF6D0;
    'dispatch: loop {
        match pc {
            0x826EF6D0 => {
    //   block [0x826EF6D0..0x826EF740)
	// 826EF6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF6DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF6E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF6E4: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF6E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF6EC: 390BA280  addi r8, r11, -0x5d80
	ctx.r[8].s64 = ctx.r[11].s64 + -23936;
	// 826EF6F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF6F4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826EF6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF6FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF708: 386A7174  addi r3, r10, 0x7174
	ctx.r[3].s64 = ctx.r[10].s64 + 29044;
	// 826EF70C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF72C: 4BD776F5  bl 0x82466e20
	ctx.lr = 0x826EF730;
	sub_82466E20(ctx, base);
	// 826EF730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF740 size=108
    let mut pc: u32 = 0x826EF740;
    'dispatch: loop {
        match pc {
            0x826EF740 => {
    //   block [0x826EF740..0x826EF7AC)
	// 826EF740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF74C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF754: 38EBA298  addi r7, r11, -0x5d68
	ctx.r[7].s64 = ctx.r[11].s64 + -23912;
	// 826EF758: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826EF75C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826EF760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EF76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF770: 386A71A4  addi r3, r10, 0x71a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29092;
	// 826EF774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EF778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EF798: 4BD77689  bl 0x82466e20
	ctx.lr = 0x826EF79C;
	sub_82466E20(ctx, base);
	// 826EF79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF7B0 size=116
    let mut pc: u32 = 0x826EF7B0;
    'dispatch: loop {
        match pc {
            0x826EF7B0 => {
    //   block [0x826EF7B0..0x826EF824)
	// 826EF7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF7BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EF7C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826EF7C4: 390AA3D0  addi r8, r10, -0x5c30
	ctx.r[8].s64 = ctx.r[10].s64 + -23600;
	// 826EF7C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF7CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EF7D0: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF7D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF7D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF7E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF7E4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826EF7E8: 396B97B0  addi r11, r11, -0x6850
	ctx.r[11].s64 = ctx.r[11].s64 + -26704;
	// 826EF7EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF7F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF7F4: 386A71D4  addi r3, r10, 0x71d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29140;
	// 826EF7F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EF7FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF800: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EF804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF810: 4BD77611  bl 0x82466e20
	ctx.lr = 0x826EF814;
	sub_82466E20(ctx, base);
	// 826EF814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF828 size=112
    let mut pc: u32 = 0x826EF828;
    'dispatch: loop {
        match pc {
            0x826EF828 => {
    //   block [0x826EF828..0x826EF898)
	// 826EF828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF834: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF838: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF83C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF844: 390BA478  addi r8, r11, -0x5b88
	ctx.r[8].s64 = ctx.r[11].s64 + -23432;
	// 826EF848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF84C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826EF850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF854: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF860: 386A7204  addi r3, r10, 0x7204
	ctx.r[3].s64 = ctx.r[10].s64 + 29188;
	// 826EF864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF884: 4BD7759D  bl 0x82466e20
	ctx.lr = 0x826EF888;
	sub_82466E20(ctx, base);
	// 826EF888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF898 size=112
    let mut pc: u32 = 0x826EF898;
    'dispatch: loop {
        match pc {
            0x826EF898 => {
    //   block [0x826EF898..0x826EF908)
	// 826EF898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF8A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF8A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF8AC: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF8B4: 390BA490  addi r8, r11, -0x5b70
	ctx.r[8].s64 = ctx.r[11].s64 + -23408;
	// 826EF8B8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826EF8BC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826EF8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF8C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF8D0: 386A7234  addi r3, r10, 0x7234
	ctx.r[3].s64 = ctx.r[10].s64 + 29236;
	// 826EF8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF8F4: 4BD7752D  bl 0x82466e20
	ctx.lr = 0x826EF8F8;
	sub_82466E20(ctx, base);
	// 826EF8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF908 size=112
    let mut pc: u32 = 0x826EF908;
    'dispatch: loop {
        match pc {
            0x826EF908 => {
    //   block [0x826EF908..0x826EF978)
	// 826EF908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF914: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF918: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF91C: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EF920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF924: 390BA5C8  addi r8, r11, -0x5a38
	ctx.r[8].s64 = ctx.r[11].s64 + -23096;
	// 826EF928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EF92C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826EF930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF940: 386A7264  addi r3, r10, 0x7264
	ctx.r[3].s64 = ctx.r[10].s64 + 29284;
	// 826EF944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EF948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EF94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EF950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF964: 4BD774BD  bl 0x82466e20
	ctx.lr = 0x826EF968;
	sub_82466E20(ctx, base);
	// 826EF968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF978 size=116
    let mut pc: u32 = 0x826EF978;
    'dispatch: loop {
        match pc {
            0x826EF978 => {
    //   block [0x826EF978..0x826EF9EC)
	// 826EF978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF984: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EF988: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EF98C: 390BA5E0  addi r8, r11, -0x5a20
	ctx.r[8].s64 = ctx.r[11].s64 + -23072;
	// 826EF990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EF994: 392A97E8  addi r9, r10, -0x6818
	ctx.r[9].s64 = ctx.r[10].s64 + -26648;
	// 826EF998: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EF99C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826EF9A0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EF9A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EF9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EF9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EF9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EF9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EF9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EF9BC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EF9C0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826EF9C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EF9C8: 386B7294  addi r3, r11, 0x7294
	ctx.r[3].s64 = ctx.r[11].s64 + 29332;
	// 826EF9CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EF9D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EF9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EF9D8: 4BD77449  bl 0x82466e20
	ctx.lr = 0x826EF9DC;
	sub_82466E20(ctx, base);
	// 826EF9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EF9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EF9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EF9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EF9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EF9F0 size=100
    let mut pc: u32 = 0x826EF9F0;
    'dispatch: loop {
        match pc {
            0x826EF9F0 => {
    //   block [0x826EF9F0..0x826EFA54)
	// 826EF9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EF9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EF9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EF9FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFA04: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFA10: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826EFA14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFA24: 386A72C4  addi r3, r10, 0x72c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29380;
	// 826EFA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFA30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EFA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFA38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EFA3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFA40: 4BD773E1  bl 0x82466e20
	ctx.lr = 0x826EFA44;
	sub_82466E20(ctx, base);
	// 826EFA44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFA48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFA4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFA58 size=112
    let mut pc: u32 = 0x826EFA58;
    'dispatch: loop {
        match pc {
            0x826EFA58 => {
    //   block [0x826EFA58..0x826EFAC8)
	// 826EFA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFA64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFA6C: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826EFA70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFA74: 390BA610  addi r8, r11, -0x59f0
	ctx.r[8].s64 = ctx.r[11].s64 + -23024;
	// 826EFA78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EFA7C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826EFA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFA84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFA88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFA90: 386A72F4  addi r3, r10, 0x72f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29428;
	// 826EFA94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFAAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFAB4: 4BD7736D  bl 0x82466e20
	ctx.lr = 0x826EFAB8;
	sub_82466E20(ctx, base);
	// 826EFAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFAC8 size=112
    let mut pc: u32 = 0x826EFAC8;
    'dispatch: loop {
        match pc {
            0x826EFAC8 => {
    //   block [0x826EFAC8..0x826EFB38)
	// 826EFAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFAD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFAD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFADC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFAE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFAE4: 390BA628  addi r8, r11, -0x59d8
	ctx.r[8].s64 = ctx.r[11].s64 + -23000;
	// 826EFAE8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826EFAEC: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 826EFAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFAF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFAF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFB00: 386A7324  addi r3, r10, 0x7324
	ctx.r[3].s64 = ctx.r[10].s64 + 29476;
	// 826EFB04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFB08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFB24: 4BD772FD  bl 0x82466e20
	ctx.lr = 0x826EFB28;
	sub_82466E20(ctx, base);
	// 826EFB28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFB38 size=112
    let mut pc: u32 = 0x826EFB38;
    'dispatch: loop {
        match pc {
            0x826EFB38 => {
    //   block [0x826EFB38..0x826EFBA8)
	// 826EFB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFB44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFB48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFB4C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFB50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFB54: 390BA6D0  addi r8, r11, -0x5930
	ctx.r[8].s64 = ctx.r[11].s64 + -22832;
	// 826EFB58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFB5C: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826EFB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFB64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFB68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFB70: 386A7354  addi r3, r10, 0x7354
	ctx.r[3].s64 = ctx.r[10].s64 + 29524;
	// 826EFB74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFB78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFB8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFB94: 4BD7728D  bl 0x82466e20
	ctx.lr = 0x826EFB98;
	sub_82466E20(ctx, base);
	// 826EFB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFBA8 size=112
    let mut pc: u32 = 0x826EFBA8;
    'dispatch: loop {
        match pc {
            0x826EFBA8 => {
    //   block [0x826EFBA8..0x826EFC18)
	// 826EFBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFBB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFBB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFBBC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFBC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFBC4: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 826EFBC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EFBCC: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 826EFBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFBD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFBD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFBE0: 386A7384  addi r3, r10, 0x7384
	ctx.r[3].s64 = ctx.r[10].s64 + 29572;
	// 826EFBE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFC04: 4BD7721D  bl 0x82466e20
	ctx.lr = 0x826EFC08;
	sub_82466E20(ctx, base);
	// 826EFC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFC18 size=116
    let mut pc: u32 = 0x826EFC18;
    'dispatch: loop {
        match pc {
            0x826EFC18 => {
    //   block [0x826EFC18..0x826EFC8C)
	// 826EFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFC24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EFC28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826EFC2C: 390AA748  addi r8, r10, -0x58b8
	ctx.r[8].s64 = ctx.r[10].s64 + -22712;
	// 826EFC30: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFC34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EFC38: 38AA6D54  addi r5, r10, 0x6d54
	ctx.r[5].s64 = ctx.r[10].s64 + 27988;
	// 826EFC3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFC40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EFC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFC48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFC4C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826EFC50: 396B97FC  addi r11, r11, -0x6804
	ctx.r[11].s64 = ctx.r[11].s64 + -26628;
	// 826EFC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFC58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFC5C: 386A73B4  addi r3, r10, 0x73b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29620;
	// 826EFC60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EFC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFC68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EFC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFC70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFC78: 4BD771A9  bl 0x82466e20
	ctx.lr = 0x826EFC7C;
	sub_82466E20(ctx, base);
	// 826EFC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFC90 size=100
    let mut pc: u32 = 0x826EFC90;
    'dispatch: loop {
        match pc {
            0x826EFC90 => {
    //   block [0x826EFC90..0x826EFCF4)
	// 826EFC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFCA4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826EFCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFCB0: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 826EFCB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFCC4: 386A73E4  addi r3, r10, 0x73e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29668;
	// 826EFCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EFCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EFCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFCE0: 4BD77141  bl 0x82466e20
	ctx.lr = 0x826EFCE4;
	sub_82466E20(ctx, base);
	// 826EFCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFCF8 size=108
    let mut pc: u32 = 0x826EFCF8;
    'dispatch: loop {
        match pc {
            0x826EFCF8 => {
    //   block [0x826EFCF8..0x826EFD64)
	// 826EFCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFD04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFD0C: 38EBA808  addi r7, r11, -0x57f8
	ctx.r[7].s64 = ctx.r[11].s64 + -22520;
	// 826EFD10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFD14: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 826EFD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFD1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFD24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFD28: 386A7414  addi r3, r10, 0x7414
	ctx.r[3].s64 = ctx.r[10].s64 + 29716;
	// 826EFD2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFD4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFD50: 4BD770D1  bl 0x82466e20
	ctx.lr = 0x826EFD54;
	sub_82466E20(ctx, base);
	// 826EFD54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFD68 size=112
    let mut pc: u32 = 0x826EFD68;
    'dispatch: loop {
        match pc {
            0x826EFD68 => {
    //   block [0x826EFD68..0x826EFDD8)
	// 826EFD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFD74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFD7C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFD80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFD84: 390BA838  addi r8, r11, -0x57c8
	ctx.r[8].s64 = ctx.r[11].s64 + -22472;
	// 826EFD88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EFD8C: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826EFD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFD94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFDA0: 386A7444  addi r3, r10, 0x7444
	ctx.r[3].s64 = ctx.r[10].s64 + 29764;
	// 826EFDA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFDC4: 4BD7705D  bl 0x82466e20
	ctx.lr = 0x826EFDC8;
	sub_82466E20(ctx, base);
	// 826EFDC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFDD8 size=108
    let mut pc: u32 = 0x826EFDD8;
    'dispatch: loop {
        match pc {
            0x826EFDD8 => {
    //   block [0x826EFDD8..0x826EFE44)
	// 826EFDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFDE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFDEC: 38EBA868  addi r7, r11, -0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + -22424;
	// 826EFDF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFDF4: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 826EFDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFDFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFE08: 386A7474  addi r3, r10, 0x7474
	ctx.r[3].s64 = ctx.r[10].s64 + 29812;
	// 826EFE0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFE2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFE30: 4BD76FF1  bl 0x82466e20
	ctx.lr = 0x826EFE34;
	sub_82466E20(ctx, base);
	// 826EFE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFE48 size=112
    let mut pc: u32 = 0x826EFE48;
    'dispatch: loop {
        match pc {
            0x826EFE48 => {
    //   block [0x826EFE48..0x826EFEB8)
	// 826EFE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFE54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFE5C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFE64: 390BA898  addi r8, r11, -0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + -22376;
	// 826EFE68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFE6C: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826EFE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFE74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFE78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFE80: 386A74A4  addi r3, r10, 0x74a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29860;
	// 826EFE84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFEA4: 4BD76F7D  bl 0x82466e20
	ctx.lr = 0x826EFEA8;
	sub_82466E20(ctx, base);
	// 826EFEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFEB8 size=108
    let mut pc: u32 = 0x826EFEB8;
    'dispatch: loop {
        match pc {
            0x826EFEB8 => {
    //   block [0x826EFEB8..0x826EFF24)
	// 826EFEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFEC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFECC: 38EBA8E0  addi r7, r11, -0x5720
	ctx.r[7].s64 = ctx.r[11].s64 + -22304;
	// 826EFED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFED4: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 826EFED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFEDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFEE8: 386A74D4  addi r3, r10, 0x74d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29908;
	// 826EFEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFF10: 4BD76F11  bl 0x82466e20
	ctx.lr = 0x826EFF14;
	sub_82466E20(ctx, base);
	// 826EFF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFF28 size=112
    let mut pc: u32 = 0x826EFF28;
    'dispatch: loop {
        match pc {
            0x826EFF28 => {
    //   block [0x826EFF28..0x826EFF98)
	// 826EFF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFF38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFF3C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826EFF40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFF44: 390BA910  addi r8, r11, -0x56f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22256;
	// 826EFF48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EFF4C: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826EFF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFF54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFF58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EFF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFF60: 386A7504  addi r3, r10, 0x7504
	ctx.r[3].s64 = ctx.r[10].s64 + 29956;
	// 826EFF64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EFF68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFF84: 4BD76E9D  bl 0x82466e20
	ctx.lr = 0x826EFF88;
	sub_82466E20(ctx, base);
	// 826EFF88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EFF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EFF98 size=108
    let mut pc: u32 = 0x826EFF98;
    'dispatch: loop {
        match pc {
            0x826EFF98 => {
    //   block [0x826EFF98..0x826F0004)
	// 826EFF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EFF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EFFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EFFA4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EFFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826EFFAC: 38EBA958  addi r7, r11, -0x56a8
	ctx.r[7].s64 = ctx.r[11].s64 + -22184;
	// 826EFFB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EFFB4: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 826EFFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EFFBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EFFC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EFFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EFFC8: 386A7534  addi r3, r10, 0x7534
	ctx.r[3].s64 = ctx.r[10].s64 + 30004;
	// 826EFFCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EFFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EFFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EFFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EFFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EFFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EFFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EFFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EFFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EFFF0: 4BD76E31  bl 0x82466e20
	ctx.lr = 0x826EFFF4;
	sub_82466E20(ctx, base);
	// 826EFFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EFFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EFFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0008 size=112
    let mut pc: u32 = 0x826F0008;
    'dispatch: loop {
        match pc {
            0x826F0008 => {
    //   block [0x826F0008..0x826F0078)
	// 826F0008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0014: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0018: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F001C: 38AA73E4  addi r5, r10, 0x73e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29668;
	// 826F0020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0024: 390BA988  addi r8, r11, -0x5678
	ctx.r[8].s64 = ctx.r[11].s64 + -22136;
	// 826F0028: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F002C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 826F0030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0034: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F003C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0040: 386A7564  addi r3, r10, 0x7564
	ctx.r[3].s64 = ctx.r[10].s64 + 30052;
	// 826F0044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F004C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0064: 4BD76DBD  bl 0x82466e20
	ctx.lr = 0x826F0068;
	sub_82466E20(ctx, base);
	// 826F0068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F006C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0078 size=108
    let mut pc: u32 = 0x826F0078;
    'dispatch: loop {
        match pc {
            0x826F0078 => {
    //   block [0x826F0078..0x826F00E4)
	// 826F0078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F007C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0084: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F008C: 38EBA9D8  addi r7, r11, -0x5628
	ctx.r[7].s64 = ctx.r[11].s64 + -22056;
	// 826F0090: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F0094: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826F0098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F009C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F00A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F00A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F00A8: 386A7594  addi r3, r10, 0x7594
	ctx.r[3].s64 = ctx.r[10].s64 + 30100;
	// 826F00AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F00B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F00B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F00B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F00BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F00C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F00C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F00C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F00CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F00D0: 4BD76D51  bl 0x82466e20
	ctx.lr = 0x826F00D4;
	sub_82466E20(ctx, base);
	// 826F00D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F00D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F00DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F00E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F00E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F00E8 size=112
    let mut pc: u32 = 0x826F00E8;
    'dispatch: loop {
        match pc {
            0x826F00E8 => {
    //   block [0x826F00E8..0x826F0158)
	// 826F00E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F00EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F00F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F00F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F00F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F00FC: 392A98B8  addi r9, r10, -0x6748
	ctx.r[9].s64 = ctx.r[10].s64 + -26440;
	// 826F0100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0104: 390BAA38  addi r8, r11, -0x55c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21960;
	// 826F0108: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F010C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826F0110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0114: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F011C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0120: 386A75C4  addi r3, r10, 0x75c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30148;
	// 826F0124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F013C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0144: 4BD76CDD  bl 0x82466e20
	ctx.lr = 0x826F0148;
	sub_82466E20(ctx, base);
	// 826F0148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0158 size=108
    let mut pc: u32 = 0x826F0158;
    'dispatch: loop {
        match pc {
            0x826F0158 => {
    //   block [0x826F0158..0x826F01C4)
	// 826F0158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0164: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F016C: 38EBAAF8  addi r7, r11, -0x5508
	ctx.r[7].s64 = ctx.r[11].s64 + -21768;
	// 826F0170: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F0174: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826F0178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F017C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0188: 386A75F4  addi r3, r10, 0x75f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30196;
	// 826F018C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F019C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F01A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F01A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F01A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F01AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F01B0: 4BD76C71  bl 0x82466e20
	ctx.lr = 0x826F01B4;
	sub_82466E20(ctx, base);
	// 826F01B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F01B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F01BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F01C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F01C8 size=108
    let mut pc: u32 = 0x826F01C8;
    'dispatch: loop {
        match pc {
            0x826F01C8 => {
    //   block [0x826F01C8..0x826F0234)
	// 826F01C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F01CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F01D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F01D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F01D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F01DC: 38EBAB40  addi r7, r11, -0x54c0
	ctx.r[7].s64 = ctx.r[11].s64 + -21696;
	// 826F01E0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F01E4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826F01E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F01EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F01F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F01F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F01F8: 386A7624  addi r3, r10, 0x7624
	ctx.r[3].s64 = ctx.r[10].s64 + 30244;
	// 826F01FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F020C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F021C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0220: 4BD76C01  bl 0x82466e20
	ctx.lr = 0x826F0224;
	sub_82466E20(ctx, base);
	// 826F0224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F022C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0238 size=24
    let mut pc: u32 = 0x826F0238;
    'dispatch: loop {
        match pc {
            0x826F0238 => {
    //   block [0x826F0238..0x826F0250)
	// 826F0238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F023C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0240: 394A1490  addi r10, r10, 0x1490
	ctx.r[10].s64 = ctx.r[10].s64 + 5264;
	// 826F0244: 816BA9D0  lwz r11, -0x5630(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22064 as u32) ) } as u64;
	// 826F0248: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 826F024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0250 size=116
    let mut pc: u32 = 0x826F0250;
    'dispatch: loop {
        match pc {
            0x826F0250 => {
    //   block [0x826F0250..0x826F02C4)
	// 826F0250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F025C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F0260: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0264: 392B9844  addi r9, r11, -0x67bc
	ctx.r[9].s64 = ctx.r[11].s64 + -26556;
	// 826F0268: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826F026C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0270: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 826F0274: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826F0278: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F027C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826F0280: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0284: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 826F0288: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F028C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0290: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F0294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0298: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 826F029C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F02A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F02A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F02A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F02AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F02B0: 4BD76B71  bl 0x82466e20
	ctx.lr = 0x826F02B4;
	sub_82466E20(ctx, base);
	// 826F02B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F02B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F02BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F02C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F02C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F02C8 size=100
    let mut pc: u32 = 0x826F02C8;
    'dispatch: loop {
        match pc {
            0x826F02C8 => {
    //   block [0x826F02C8..0x826F032C)
	// 826F02C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F02CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F02D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F02D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F02DC: 38AA6AE4  addi r5, r10, 0x6ae4
	ctx.r[5].s64 = ctx.r[10].s64 + 27364;
	// 826F02E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F02E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F02E8: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826F02EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F02F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F02F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F02F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F02FC: 386A7684  addi r3, r10, 0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + 30340;
	// 826F0300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0304: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F0314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0318: 4BD76B09  bl 0x82466e20
	ctx.lr = 0x826F031C;
	sub_82466E20(ctx, base);
	// 826F031C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0330 size=24
    let mut pc: u32 = 0x826F0330;
    'dispatch: loop {
        match pc {
            0x826F0330 => {
    //   block [0x826F0330..0x826F0348)
	// 826F0330: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0334: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0338: 394A1640  addi r10, r10, 0x1640
	ctx.r[10].s64 = ctx.r[10].s64 + 5696;
	// 826F033C: 816BABEC  lwz r11, -0x5414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21524 as u32) ) } as u64;
	// 826F0340: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0348 size=116
    let mut pc: u32 = 0x826F0348;
    'dispatch: loop {
        match pc {
            0x826F0348 => {
    //   block [0x826F0348..0x826F03BC)
	// 826F0348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0354: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0358: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F035C: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 826F0360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0364: 392A9950  addi r9, r10, -0x66b0
	ctx.r[9].s64 = ctx.r[10].s64 + -26288;
	// 826F0368: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F036C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826F0370: 38AA7684  addi r5, r10, 0x7684
	ctx.r[5].s64 = ctx.r[10].s64 + 30340;
	// 826F0374: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F037C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F038C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F0390: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826F0394: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0398: 386B76B4  addi r3, r11, 0x76b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30388;
	// 826F039C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F03A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F03A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F03A8: 4BD76A79  bl 0x82466e20
	ctx.lr = 0x826F03AC;
	sub_82466E20(ctx, base);
	// 826F03AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F03B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F03B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F03B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F03C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F03C0 size=112
    let mut pc: u32 = 0x826F03C0;
    'dispatch: loop {
        match pc {
            0x826F03C0 => {
    //   block [0x826F03C0..0x826F0430)
	// 826F03C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F03C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F03C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F03CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F03D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F03D4: 38AA7684  addi r5, r10, 0x7684
	ctx.r[5].s64 = ctx.r[10].s64 + 30340;
	// 826F03D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F03DC: 390BABF0  addi r8, r11, -0x5410
	ctx.r[8].s64 = ctx.r[11].s64 + -21520;
	// 826F03E0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826F03E4: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 826F03E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F03EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F03F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F03F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F03F8: 386A76E4  addi r3, r10, 0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30436;
	// 826F03FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F040C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F041C: 4BD76A05  bl 0x82466e20
	ctx.lr = 0x826F0420;
	sub_82466E20(ctx, base);
	// 826F0420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F042C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0430 size=112
    let mut pc: u32 = 0x826F0430;
    'dispatch: loop {
        match pc {
            0x826F0430 => {
    //   block [0x826F0430..0x826F04A0)
	// 826F0430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F043C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0440: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0444: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F044C: 390BACC8  addi r8, r11, -0x5338
	ctx.r[8].s64 = ctx.r[11].s64 + -21304;
	// 826F0450: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826F0454: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 826F0458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F045C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0468: 386A7714  addi r3, r10, 0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + 30484;
	// 826F046C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F047C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F048C: 4BD76995  bl 0x82466e20
	ctx.lr = 0x826F0490;
	sub_82466E20(ctx, base);
	// 826F0490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F049C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F04A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F04A0 size=108
    let mut pc: u32 = 0x826F04A0;
    'dispatch: loop {
        match pc {
            0x826F04A0 => {
    //   block [0x826F04A0..0x826F050C)
	// 826F04A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F04A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F04A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F04AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F04B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F04B4: 38EBADA0  addi r7, r11, -0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + -21088;
	// 826F04B8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F04BC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826F04C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F04C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F04C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F04CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F04D0: 386A7744  addi r3, r10, 0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + 30532;
	// 826F04D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F04D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F04DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F04E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F04E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F04E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F04EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F04F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F04F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F04F8: 4BD76929  bl 0x82466e20
	ctx.lr = 0x826F04FC;
	sub_82466E20(ctx, base);
	// 826F04FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0510 size=108
    let mut pc: u32 = 0x826F0510;
    'dispatch: loop {
        match pc {
            0x826F0510 => {
    //   block [0x826F0510..0x826F057C)
	// 826F0510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F051C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0524: 38EBAE18  addi r7, r11, -0x51e8
	ctx.r[7].s64 = ctx.r[11].s64 + -20968;
	// 826F0528: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F052C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826F0530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0534: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F053C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0540: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 826F0544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F055C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0568: 4BD768B9  bl 0x82466e20
	ctx.lr = 0x826F056C;
	sub_82466E20(ctx, base);
	// 826F056C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0580 size=112
    let mut pc: u32 = 0x826F0580;
    'dispatch: loop {
        match pc {
            0x826F0580 => {
    //   block [0x826F0580..0x826F05F0)
	// 826F0580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F058C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0590: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0594: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F059C: 390BAE60  addi r8, r11, -0x51a0
	ctx.r[8].s64 = ctx.r[11].s64 + -20896;
	// 826F05A0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 826F05A4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826F05A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F05AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F05B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F05B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F05B8: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 826F05BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F05C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F05C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F05C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F05CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F05D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F05D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F05D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F05DC: 4BD76845  bl 0x82466e20
	ctx.lr = 0x826F05E0;
	sub_82466E20(ctx, base);
	// 826F05E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F05E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F05E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F05EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F05F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F05F0 size=108
    let mut pc: u32 = 0x826F05F0;
    'dispatch: loop {
        match pc {
            0x826F05F0 => {
    //   block [0x826F05F0..0x826F065C)
	// 826F05F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F05F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F05F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F05FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0604: 38EBB040  addi r7, r11, -0x4fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -20416;
	// 826F0608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F060C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826F0610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F061C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0620: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 826F0624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F062C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F063C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0648: 4BD767D9  bl 0x82466e20
	ctx.lr = 0x826F064C;
	sub_82466E20(ctx, base);
	// 826F064C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0660 size=24
    let mut pc: u32 = 0x826F0660;
    'dispatch: loop {
        match pc {
            0x826F0660 => {
    //   block [0x826F0660..0x826F0678)
	// 826F0660: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0664: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0668: 394A1760  addi r10, r10, 0x1760
	ctx.r[10].s64 = ctx.r[10].s64 + 5984;
	// 826F066C: 816BB058  lwz r11, -0x4fa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20392 as u32) ) } as u64;
	// 826F0670: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F0674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0678 size=112
    let mut pc: u32 = 0x826F0678;
    'dispatch: loop {
        match pc {
            0x826F0678 => {
    //   block [0x826F0678..0x826F06E8)
	// 826F0678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F067C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0684: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0688: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F068C: 392A99A8  addi r9, r10, -0x6658
	ctx.r[9].s64 = ctx.r[10].s64 + -26200;
	// 826F0690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0694: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 826F0698: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F069C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 826F06A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F06A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F06A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F06AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F06B0: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 826F06B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F06B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F06BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F06C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F06C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F06C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F06CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F06D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F06D4: 4BD7674D  bl 0x82466e20
	ctx.lr = 0x826F06D8;
	sub_82466E20(ctx, base);
	// 826F06D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F06DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F06E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F06E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F06E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F06E8 size=112
    let mut pc: u32 = 0x826F06E8;
    'dispatch: loop {
        match pc {
            0x826F06E8 => {
    //   block [0x826F06E8..0x826F0758)
	// 826F06E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F06EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F06F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F06F4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F06F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F06FC: 38EAB060  addi r7, r10, -0x4fa0
	ctx.r[7].s64 = ctx.r[10].s64 + -20384;
	// 826F0700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0704: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F0708: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826F070C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0710: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0714: 396B99BC  addi r11, r11, -0x6644
	ctx.r[11].s64 = ctx.r[11].s64 + -26180;
	// 826F0718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F071C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0724: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 826F0728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F072C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F0730: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0734: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F0738: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F073C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0740: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0744: 4BD766DD  bl 0x82466e20
	ctx.lr = 0x826F0748;
	sub_82466E20(ctx, base);
	// 826F0748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F074C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0758 size=112
    let mut pc: u32 = 0x826F0758;
    'dispatch: loop {
        match pc {
            0x826F0758 => {
    //   block [0x826F0758..0x826F07C8)
	// 826F0758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0768: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F076C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F0770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0774: 390BB0F0  addi r8, r11, -0x4f10
	ctx.r[8].s64 = ctx.r[11].s64 + -20240;
	// 826F0778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F077C: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826F0780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F078C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0790: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 826F0794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F0798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F079C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F07A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F07A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F07A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F07AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F07B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F07B4: 4BD7666D  bl 0x82466e20
	ctx.lr = 0x826F07B8;
	sub_82466E20(ctx, base);
	// 826F07B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F07BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F07C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F07C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F07C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F07C8 size=108
    let mut pc: u32 = 0x826F07C8;
    'dispatch: loop {
        match pc {
            0x826F07C8 => {
    //   block [0x826F07C8..0x826F0834)
	// 826F07C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F07CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F07D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F07D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F07D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F07DC: 38EBB108  addi r7, r11, -0x4ef8
	ctx.r[7].s64 = ctx.r[11].s64 + -20216;
	// 826F07E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F07E4: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826F07E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F07EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F07F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F07F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F07F8: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 826F07FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F080C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F081C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0820: 4BD76601  bl 0x82466e20
	ctx.lr = 0x826F0824;
	sub_82466E20(ctx, base);
	// 826F0824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F082C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0838 size=108
    let mut pc: u32 = 0x826F0838;
    'dispatch: loop {
        match pc {
            0x826F0838 => {
    //   block [0x826F0838..0x826F08A4)
	// 826F0838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0844: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F084C: 38EBB168  addi r7, r11, -0x4e98
	ctx.r[7].s64 = ctx.r[11].s64 + -20120;
	// 826F0850: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0854: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826F0858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F085C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0868: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 826F086C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F087C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F088C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0890: 4BD76591  bl 0x82466e20
	ctx.lr = 0x826F0894;
	sub_82466E20(ctx, base);
	// 826F0894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F089C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F08A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F08A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F08A8 size=116
    let mut pc: u32 = 0x826F08A8;
    'dispatch: loop {
        match pc {
            0x826F08A8 => {
    //   block [0x826F08A8..0x826F091C)
	// 826F08A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F08AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F08B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F08B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F08B8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F08BC: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 826F08C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F08C4: 392A99F0  addi r9, r10, -0x6610
	ctx.r[9].s64 = ctx.r[10].s64 + -26128;
	// 826F08C8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F08CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F08D0: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F08D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F08D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F08DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F08E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F08E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F08E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F08EC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F08F0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826F08F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F08F8: 386B78F4  addi r3, r11, 0x78f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30964;
	// 826F08FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0908: 4BD76519  bl 0x82466e20
	ctx.lr = 0x826F090C;
	sub_82466E20(ctx, base);
	// 826F090C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0920 size=96
    let mut pc: u32 = 0x826F0920;
    'dispatch: loop {
        match pc {
            0x826F0920 => {
    //   block [0x826F0920..0x826F0980)
	// 826F0920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F092C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0934: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826F0938: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F093C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0940: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 826F0944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F094C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F095C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0960: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F0964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0968: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F096C: 4BD764B5  bl 0x82466e20
	ctx.lr = 0x826F0970;
	sub_82466E20(ctx, base);
	// 826F0970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F097C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0980 size=112
    let mut pc: u32 = 0x826F0980;
    'dispatch: loop {
        match pc {
            0x826F0980 => {
    //   block [0x826F0980..0x826F09F0)
	// 826F0980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F098C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0990: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0994: 38AA7924  addi r5, r10, 0x7924
	ctx.r[5].s64 = ctx.r[10].s64 + 31012;
	// 826F0998: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F099C: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 826F09A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F09A4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826F09A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F09AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F09B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F09B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F09B8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 826F09BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F09C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F09C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F09C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F09CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F09D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F09D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F09D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F09DC: 4BD76445  bl 0x82466e20
	ctx.lr = 0x826F09E0;
	sub_82466E20(ctx, base);
	// 826F09E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F09E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F09E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F09EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F09F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F09F0 size=112
    let mut pc: u32 = 0x826F09F0;
    'dispatch: loop {
        match pc {
            0x826F09F0 => {
    //   block [0x826F09F0..0x826F0A60)
	// 826F09F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F09F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F09F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F09FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0A00: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0A04: 392A9A0C  addi r9, r10, -0x65f4
	ctx.r[9].s64 = ctx.r[10].s64 + -26100;
	// 826F0A08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0A0C: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 826F0A10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F0A14: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826F0A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0A1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0A20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0A28: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 826F0A2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0A30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0A4C: 4BD763D5  bl 0x82466e20
	ctx.lr = 0x826F0A50;
	sub_82466E20(ctx, base);
	// 826F0A50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0A60 size=108
    let mut pc: u32 = 0x826F0A60;
    'dispatch: loop {
        match pc {
            0x826F0A60 => {
    //   block [0x826F0A60..0x826F0ACC)
	// 826F0A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0A6C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0A70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0A74: 38EBB290  addi r7, r11, -0x4d70
	ctx.r[7].s64 = ctx.r[11].s64 + -19824;
	// 826F0A78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0A7C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826F0A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0A88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0A90: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 826F0A94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0AA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0AB8: 4BD76369  bl 0x82466e20
	ctx.lr = 0x826F0ABC;
	sub_82466E20(ctx, base);
	// 826F0ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0AD0 size=108
    let mut pc: u32 = 0x826F0AD0;
    'dispatch: loop {
        match pc {
            0x826F0AD0 => {
    //   block [0x826F0AD0..0x826F0B3C)
	// 826F0AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0ADC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0AE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0AE4: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 826F0AE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0AEC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826F0AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0AF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0B00: 386A79E4  addi r3, r10, 0x79e4
	ctx.r[3].s64 = ctx.r[10].s64 + 31204;
	// 826F0B04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0B24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0B28: 4BD762F9  bl 0x82466e20
	ctx.lr = 0x826F0B2C;
	sub_82466E20(ctx, base);
	// 826F0B2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0B40 size=28
    let mut pc: u32 = 0x826F0B40;
    'dispatch: loop {
        match pc {
            0x826F0B40 => {
    //   block [0x826F0B40..0x826F0B5C)
	// 826F0B40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0B44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0B48: 394A1790  addi r10, r10, 0x1790
	ctx.r[10].s64 = ctx.r[10].s64 + 6032;
	// 826F0B4C: 816BB1E4  lwz r11, -0x4e1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19996 as u32) ) } as u64;
	// 826F0B50: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0B54: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F0B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0B60 size=112
    let mut pc: u32 = 0x826F0B60;
    'dispatch: loop {
        match pc {
            0x826F0B60 => {
    //   block [0x826F0B60..0x826F0BD0)
	// 826F0B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0B6C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0B70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0B74: 392A9BB8  addi r9, r10, -0x6448
	ctx.r[9].s64 = ctx.r[10].s64 + -25672;
	// 826F0B78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0B7C: 390B1790  addi r8, r11, 0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + 6032;
	// 826F0B80: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F0B84: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826F0B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0B8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0B98: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 826F0B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0BA0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F0BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0BBC: 4BD76265  bl 0x82466e20
	ctx.lr = 0x826F0BC0;
	sub_82466E20(ctx, base);
	// 826F0BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0BD0 size=108
    let mut pc: u32 = 0x826F0BD0;
    'dispatch: loop {
        match pc {
            0x826F0BD0 => {
    //   block [0x826F0BD0..0x826F0C3C)
	// 826F0BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0BDC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0BE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0BE4: 38EBB2F8  addi r7, r11, -0x4d08
	ctx.r[7].s64 = ctx.r[11].s64 + -19720;
	// 826F0BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0BEC: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 826F0BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0BF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0C00: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 826F0C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0C28: 4BD761F9  bl 0x82466e20
	ctx.lr = 0x826F0C2C;
	sub_82466E20(ctx, base);
	// 826F0C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0C40 size=108
    let mut pc: u32 = 0x826F0C40;
    'dispatch: loop {
        match pc {
            0x826F0C40 => {
    //   block [0x826F0C40..0x826F0CAC)
	// 826F0C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0C4C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0C50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0C54: 38EBB328  addi r7, r11, -0x4cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -19672;
	// 826F0C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0C5C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826F0C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0C70: 386A7A74  addi r3, r10, 0x7a74
	ctx.r[3].s64 = ctx.r[10].s64 + 31348;
	// 826F0C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0C98: 4BD76189  bl 0x82466e20
	ctx.lr = 0x826F0C9C;
	sub_82466E20(ctx, base);
	// 826F0C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0CB0 size=108
    let mut pc: u32 = 0x826F0CB0;
    'dispatch: loop {
        match pc {
            0x826F0CB0 => {
    //   block [0x826F0CB0..0x826F0D1C)
	// 826F0CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0CBC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0CC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0CC4: 38EBB358  addi r7, r11, -0x4ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -19624;
	// 826F0CC8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0CCC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826F0CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0CD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0CE0: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 826F0CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0D08: 4BD76119  bl 0x82466e20
	ctx.lr = 0x826F0D0C;
	sub_82466E20(ctx, base);
	// 826F0D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F0D20 size=24
    let mut pc: u32 = 0x826F0D20;
    'dispatch: loop {
        match pc {
            0x826F0D20 => {
    //   block [0x826F0D20..0x826F0D38)
	// 826F0D20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0D24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F0D28: 394A1850  addi r10, r10, 0x1850
	ctx.r[10].s64 = ctx.r[10].s64 + 6224;
	// 826F0D2C: 816BB370  lwz r11, -0x4c90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19600 as u32) ) } as u64;
	// 826F0D30: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F0D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0D38 size=112
    let mut pc: u32 = 0x826F0D38;
    'dispatch: loop {
        match pc {
            0x826F0D38 => {
    //   block [0x826F0D38..0x826F0DA8)
	// 826F0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0D44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0D48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0D4C: 392A9C0C  addi r9, r10, -0x63f4
	ctx.r[9].s64 = ctx.r[10].s64 + -25588;
	// 826F0D50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0D54: 390B1850  addi r8, r11, 0x1850
	ctx.r[8].s64 = ctx.r[11].s64 + 6224;
	// 826F0D58: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F0D5C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826F0D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0D70: 386A7AD4  addi r3, r10, 0x7ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 31444;
	// 826F0D74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0D78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0D94: 4BD7608D  bl 0x82466e20
	ctx.lr = 0x826F0D98;
	sub_82466E20(ctx, base);
	// 826F0D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0DA8 size=112
    let mut pc: u32 = 0x826F0DA8;
    'dispatch: loop {
        match pc {
            0x826F0DA8 => {
    //   block [0x826F0DA8..0x826F0E18)
	// 826F0DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0DB4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0DB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0DBC: 392A9C48  addi r9, r10, -0x63b8
	ctx.r[9].s64 = ctx.r[10].s64 + -25528;
	// 826F0DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0DC4: 390BB380  addi r8, r11, -0x4c80
	ctx.r[8].s64 = ctx.r[11].s64 + -19584;
	// 826F0DC8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F0DCC: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826F0DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0DD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0DE0: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 826F0DE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0DE8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F0DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0E04: 4BD7601D  bl 0x82466e20
	ctx.lr = 0x826F0E08;
	sub_82466E20(ctx, base);
	// 826F0E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0E18 size=108
    let mut pc: u32 = 0x826F0E18;
    'dispatch: loop {
        match pc {
            0x826F0E18 => {
    //   block [0x826F0E18..0x826F0E84)
	// 826F0E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0E24: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0E28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0E2C: 38EBB3C8  addi r7, r11, -0x4c38
	ctx.r[7].s64 = ctx.r[11].s64 + -19512;
	// 826F0E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0E34: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826F0E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0E48: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 826F0E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0E70: 4BD75FB1  bl 0x82466e20
	ctx.lr = 0x826F0E74;
	sub_82466E20(ctx, base);
	// 826F0E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0E88 size=108
    let mut pc: u32 = 0x826F0E88;
    'dispatch: loop {
        match pc {
            0x826F0E88 => {
    //   block [0x826F0E88..0x826F0EF4)
	// 826F0E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0E94: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0E9C: 38EBB3F8  addi r7, r11, -0x4c08
	ctx.r[7].s64 = ctx.r[11].s64 + -19464;
	// 826F0EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F0EA4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826F0EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0EB8: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 826F0EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0EE0: 4BD75F41  bl 0x82466e20
	ctx.lr = 0x826F0EE4;
	sub_82466E20(ctx, base);
	// 826F0EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0EF8 size=112
    let mut pc: u32 = 0x826F0EF8;
    'dispatch: loop {
        match pc {
            0x826F0EF8 => {
    //   block [0x826F0EF8..0x826F0F68)
	// 826F0EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0F04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F0F08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0F0C: 392A9C88  addi r9, r10, -0x6378
	ctx.r[9].s64 = ctx.r[10].s64 + -25464;
	// 826F0F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F0F14: 390BB428  addi r8, r11, -0x4bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -19416;
	// 826F0F18: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F0F1C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826F0F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F0F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0F30: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 826F0F34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F0F38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F0F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0F54: 4BD75ECD  bl 0x82466e20
	ctx.lr = 0x826F0F58;
	sub_82466E20(ctx, base);
	// 826F0F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0F68 size=108
    let mut pc: u32 = 0x826F0F68;
    'dispatch: loop {
        match pc {
            0x826F0F68 => {
    //   block [0x826F0F68..0x826F0FD4)
	// 826F0F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0F74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0F78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0F7C: 38EBB4A0  addi r7, r11, -0x4b60
	ctx.r[7].s64 = ctx.r[11].s64 + -19296;
	// 826F0F80: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826F0F84: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826F0F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0F8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F0F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F0F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F0F98: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 826F0F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F0FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F0FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F0FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F0FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F0FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F0FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F0FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F0FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F0FC0: 4BD75E61  bl 0x82466e20
	ctx.lr = 0x826F0FC4;
	sub_82466E20(ctx, base);
	// 826F0FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F0FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F0FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F0FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F0FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0FD8 size=108
    let mut pc: u32 = 0x826F0FD8;
    'dispatch: loop {
        match pc {
            0x826F0FD8 => {
    //   block [0x826F0FD8..0x826F1044)
	// 826F0FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0FE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0FE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0FEC: 38EBB5A8  addi r7, r11, -0x4a58
	ctx.r[7].s64 = ctx.r[11].s64 + -19032;
	// 826F0FF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0FF4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826F0FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1008: 386A7BF4  addi r3, r10, 0x7bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31732;
	// 826F100C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F101C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F102C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1030: 4BD75DF1  bl 0x82466e20
	ctx.lr = 0x826F1034;
	sub_82466E20(ctx, base);
	// 826F1034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F103C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1048 size=108
    let mut pc: u32 = 0x826F1048;
    'dispatch: loop {
        match pc {
            0x826F1048 => {
    //   block [0x826F1048..0x826F10B4)
	// 826F1048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1054: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F105C: 38EBB5C0  addi r7, r11, -0x4a40
	ctx.r[7].s64 = ctx.r[11].s64 + -19008;
	// 826F1060: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F1064: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826F1068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F106C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1078: 386A7C24  addi r3, r10, 0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + 31780;
	// 826F107C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F108C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F109C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F10A0: 4BD75D81  bl 0x82466e20
	ctx.lr = 0x826F10A4;
	sub_82466E20(ctx, base);
	// 826F10A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F10A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F10AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F10B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F10B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F10B8 size=24
    let mut pc: u32 = 0x826F10B8;
    'dispatch: loop {
        match pc {
            0x826F10B8 => {
    //   block [0x826F10B8..0x826F10D0)
	// 826F10B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F10BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F10C0: 394A1928  addi r10, r10, 0x1928
	ctx.r[10].s64 = ctx.r[10].s64 + 6440;
	// 826F10C4: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F10C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F10CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F10D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F10D0 size=108
    let mut pc: u32 = 0x826F10D0;
    'dispatch: loop {
        match pc {
            0x826F10D0 => {
    //   block [0x826F10D0..0x826F113C)
	// 826F10D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F10D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F10D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F10DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F10E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F10E4: 38EB1928  addi r7, r11, 0x1928
	ctx.r[7].s64 = ctx.r[11].s64 + 6440;
	// 826F10E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F10EC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826F10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F10F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F10F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F10FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1100: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 826F1104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F110C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F111C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1128: 4BD75CF9  bl 0x82466e20
	ctx.lr = 0x826F112C;
	sub_82466E20(ctx, base);
	// 826F112C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1140 size=24
    let mut pc: u32 = 0x826F1140;
    'dispatch: loop {
        match pc {
            0x826F1140 => {
    //   block [0x826F1140..0x826F1158)
	// 826F1140: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1144: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1148: 394A1958  addi r10, r10, 0x1958
	ctx.r[10].s64 = ctx.r[10].s64 + 6488;
	// 826F114C: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F1150: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F1154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1158 size=108
    let mut pc: u32 = 0x826F1158;
    'dispatch: loop {
        match pc {
            0x826F1158 => {
    //   block [0x826F1158..0x826F11C4)
	// 826F1158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1164: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F116C: 38EB1958  addi r7, r11, 0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + 6488;
	// 826F1170: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1174: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826F1178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F117C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1188: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 826F118C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F119C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F11A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F11A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F11A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F11AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F11B0: 4BD75C71  bl 0x82466e20
	ctx.lr = 0x826F11B4;
	sub_82466E20(ctx, base);
	// 826F11B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F11B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F11BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F11C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F11C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F11C8 size=108
    let mut pc: u32 = 0x826F11C8;
    'dispatch: loop {
        match pc {
            0x826F11C8 => {
    //   block [0x826F11C8..0x826F1234)
	// 826F11C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F11CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F11D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F11D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F11D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F11DC: 38EBB638  addi r7, r11, -0x49c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18888;
	// 826F11E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F11E4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826F11E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F11EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F11F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F11F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F11F8: 386A7CB4  addi r3, r10, 0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31924;
	// 826F11FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F120C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F121C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1220: 4BD75C01  bl 0x82466e20
	ctx.lr = 0x826F1224;
	sub_82466E20(ctx, base);
	// 826F1224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F122C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1238 size=24
    let mut pc: u32 = 0x826F1238;
    'dispatch: loop {
        match pc {
            0x826F1238 => {
    //   block [0x826F1238..0x826F1250)
	// 826F1238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F123C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1240: 394A1988  addi r10, r10, 0x1988
	ctx.r[10].s64 = ctx.r[10].s64 + 6536;
	// 826F1244: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F1248: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1250 size=108
    let mut pc: u32 = 0x826F1250;
    'dispatch: loop {
        match pc {
            0x826F1250 => {
    //   block [0x826F1250..0x826F12BC)
	// 826F1250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F125C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1264: 38EB1988  addi r7, r11, 0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + 6536;
	// 826F1268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F126C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826F1270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F127C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1280: 386A7CE4  addi r3, r10, 0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 31972;
	// 826F1284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F128C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F129C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F12A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F12A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F12A8: 4BD75B79  bl 0x82466e20
	ctx.lr = 0x826F12AC;
	sub_82466E20(ctx, base);
	// 826F12AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F12B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F12B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F12B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F12C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F12C0 size=112
    let mut pc: u32 = 0x826F12C0;
    'dispatch: loop {
        match pc {
            0x826F12C0 => {
    //   block [0x826F12C0..0x826F1330)
	// 826F12C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F12C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F12C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F12CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F12D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F12D4: 392A9CCC  addi r9, r10, -0x6334
	ctx.r[9].s64 = ctx.r[10].s64 + -25396;
	// 826F12D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F12DC: 390BB654  addi r8, r11, -0x49ac
	ctx.r[8].s64 = ctx.r[11].s64 + -18860;
	// 826F12E0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F12E4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826F12E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F12EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F12F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F12F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F12F8: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 826F12FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1300: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F130C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F131C: 4BD75B05  bl 0x82466e20
	ctx.lr = 0x826F1320;
	sub_82466E20(ctx, base);
	// 826F1320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F132C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1330 size=108
    let mut pc: u32 = 0x826F1330;
    'dispatch: loop {
        match pc {
            0x826F1330 => {
    //   block [0x826F1330..0x826F139C)
	// 826F1330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F133C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1344: 38EBB684  addi r7, r11, -0x497c
	ctx.r[7].s64 = ctx.r[11].s64 + -18812;
	// 826F1348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F134C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826F1350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F135C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1360: 386A7D44  addi r3, r10, 0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + 32068;
	// 826F1364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F136C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F137C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1388: 4BD75A99  bl 0x82466e20
	ctx.lr = 0x826F138C;
	sub_82466E20(ctx, base);
	// 826F138C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F13A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F13A0 size=108
    let mut pc: u32 = 0x826F13A0;
    'dispatch: loop {
        match pc {
            0x826F13A0 => {
    //   block [0x826F13A0..0x826F140C)
	// 826F13A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F13A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F13A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F13AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F13B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F13B4: 38EBB6B4  addi r7, r11, -0x494c
	ctx.r[7].s64 = ctx.r[11].s64 + -18764;
	// 826F13B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F13BC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826F13C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F13C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F13C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F13CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F13D0: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 826F13D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F13D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F13DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F13E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F13E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F13EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F13F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F13F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F13F8: 4BD75A29  bl 0x82466e20
	ctx.lr = 0x826F13FC;
	sub_82466E20(ctx, base);
	// 826F13FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1410 size=108
    let mut pc: u32 = 0x826F1410;
    'dispatch: loop {
        match pc {
            0x826F1410 => {
    //   block [0x826F1410..0x826F147C)
	// 826F1410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F141C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1424: 38EBB6CC  addi r7, r11, -0x4934
	ctx.r[7].s64 = ctx.r[11].s64 + -18740;
	// 826F1428: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F142C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826F1430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1440: 386A7DA4  addi r3, r10, 0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + 32164;
	// 826F1444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F144C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1468: 4BD759B9  bl 0x82466e20
	ctx.lr = 0x826F146C;
	sub_82466E20(ctx, base);
	// 826F146C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1480 size=112
    let mut pc: u32 = 0x826F1480;
    'dispatch: loop {
        match pc {
            0x826F1480 => {
    //   block [0x826F1480..0x826F14F0)
	// 826F1480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F148C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1490: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1494: 38AA7E04  addi r5, r10, 0x7e04
	ctx.r[5].s64 = ctx.r[10].s64 + 32260;
	// 826F1498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F149C: 390BB6FC  addi r8, r11, -0x4904
	ctx.r[8].s64 = ctx.r[11].s64 + -18692;
	// 826F14A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F14A4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826F14A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F14AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F14B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F14B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F14B8: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 826F14BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F14C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F14C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F14C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F14CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F14D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F14D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F14D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F14DC: 4BD75945  bl 0x82466e20
	ctx.lr = 0x826F14E0;
	sub_82466E20(ctx, base);
	// 826F14E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F14E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F14E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F14EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F14F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F14F0 size=108
    let mut pc: u32 = 0x826F14F0;
    'dispatch: loop {
        match pc {
            0x826F14F0 => {
    //   block [0x826F14F0..0x826F155C)
	// 826F14F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F14F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F14F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F14FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1504: 38EBB714  addi r7, r11, -0x48ec
	ctx.r[7].s64 = ctx.r[11].s64 + -18668;
	// 826F1508: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F150C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826F1510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1520: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 826F1524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F152C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F153C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1548: 4BD758D9  bl 0x82466e20
	ctx.lr = 0x826F154C;
	sub_82466E20(ctx, base);
	// 826F154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1560 size=108
    let mut pc: u32 = 0x826F1560;
    'dispatch: loop {
        match pc {
            0x826F1560 => {
    //   block [0x826F1560..0x826F15CC)
	// 826F1560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F156C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1574: 38EBB744  addi r7, r11, -0x48bc
	ctx.r[7].s64 = ctx.r[11].s64 + -18620;
	// 826F1578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F157C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826F1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1590: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 826F1594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F159C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F15A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F15A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F15A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F15AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F15B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F15B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F15B8: 4BD75869  bl 0x82466e20
	ctx.lr = 0x826F15BC;
	sub_82466E20(ctx, base);
	// 826F15BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F15C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F15C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F15C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F15D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F15D0 size=108
    let mut pc: u32 = 0x826F15D0;
    'dispatch: loop {
        match pc {
            0x826F15D0 => {
    //   block [0x826F15D0..0x826F163C)
	// 826F15D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F15D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F15D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F15DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F15E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F15E4: 38EBB75C  addi r7, r11, -0x48a4
	ctx.r[7].s64 = ctx.r[11].s64 + -18596;
	// 826F15E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F15EC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826F15F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F15F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F15F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F15FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1600: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 826F1604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F160C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1628: 4BD757F9  bl 0x82466e20
	ctx.lr = 0x826F162C;
	sub_82466E20(ctx, base);
	// 826F162C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1640 size=108
    let mut pc: u32 = 0x826F1640;
    'dispatch: loop {
        match pc {
            0x826F1640 => {
    //   block [0x826F1640..0x826F16AC)
	// 826F1640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F164C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1654: 38EBB790  addi r7, r11, -0x4870
	ctx.r[7].s64 = ctx.r[11].s64 + -18544;
	// 826F1658: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F165C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826F1660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1670: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 826F1674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F167C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1698: 4BD75789  bl 0x82466e20
	ctx.lr = 0x826F169C;
	sub_82466E20(ctx, base);
	// 826F169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F16A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F16A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F16A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F16B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F16B0 size=108
    let mut pc: u32 = 0x826F16B0;
    'dispatch: loop {
        match pc {
            0x826F16B0 => {
    //   block [0x826F16B0..0x826F171C)
	// 826F16B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F16B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F16B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F16BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F16C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F16C4: 38EBB838  addi r7, r11, -0x47c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18376;
	// 826F16C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F16CC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826F16D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F16D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F16D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F16DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F16E0: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 826F16E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F16E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F16EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F16F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F16F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F16F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F16FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1708: 4BD75719  bl 0x82466e20
	ctx.lr = 0x826F170C;
	sub_82466E20(ctx, base);
	// 826F170C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1720 size=108
    let mut pc: u32 = 0x826F1720;
    'dispatch: loop {
        match pc {
            0x826F1720 => {
    //   block [0x826F1720..0x826F178C)
	// 826F1720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F172C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1734: 38EBB868  addi r7, r11, -0x4798
	ctx.r[7].s64 = ctx.r[11].s64 + -18328;
	// 826F1738: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F173C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826F1740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F174C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1750: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 826F1754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F175C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1778: 4BD756A9  bl 0x82466e20
	ctx.lr = 0x826F177C;
	sub_82466E20(ctx, base);
	// 826F177C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1790 size=108
    let mut pc: u32 = 0x826F1790;
    'dispatch: loop {
        match pc {
            0x826F1790 => {
    //   block [0x826F1790..0x826F17FC)
	// 826F1790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F179C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F17A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F17A4: 38EBB880  addi r7, r11, -0x4780
	ctx.r[7].s64 = ctx.r[11].s64 + -18304;
	// 826F17A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F17AC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826F17B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F17B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F17B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F17BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F17C0: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 826F17C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F17C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F17CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F17D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F17D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F17D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F17DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F17E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F17E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F17E8: 4BD75639  bl 0x82466e20
	ctx.lr = 0x826F17EC;
	sub_82466E20(ctx, base);
	// 826F17EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F17F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F17F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F17F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1800 size=112
    let mut pc: u32 = 0x826F1800;
    'dispatch: loop {
        match pc {
            0x826F1800 => {
    //   block [0x826F1800..0x826F1870)
	// 826F1800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F180C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1810: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1814: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F181C: 390BB8B0  addi r8, r11, -0x4750
	ctx.r[8].s64 = ctx.r[11].s64 + -18256;
	// 826F1820: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F1824: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826F1828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F182C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1838: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 826F183C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F1840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F184C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F185C: 4BD755C5  bl 0x82466e20
	ctx.lr = 0x826F1860;
	sub_82466E20(ctx, base);
	// 826F1860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1870 size=24
    let mut pc: u32 = 0x826F1870;
    'dispatch: loop {
        match pc {
            0x826F1870 => {
    //   block [0x826F1870..0x826F1888)
	// 826F1870: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1874: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1878: 394A19B8  addi r10, r10, 0x19b8
	ctx.r[10].s64 = ctx.r[10].s64 + 6584;
	// 826F187C: 816BB78C  lwz r11, -0x4874(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18548 as u32) ) } as u64;
	// 826F1880: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F1884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1888 size=112
    let mut pc: u32 = 0x826F1888;
    'dispatch: loop {
        match pc {
            0x826F1888 => {
    //   block [0x826F1888..0x826F18F8)
	// 826F1888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F188C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1894: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1898: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F189C: 392A9CF8  addi r9, r10, -0x6308
	ctx.r[9].s64 = ctx.r[10].s64 + -25352;
	// 826F18A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F18A4: 390B19B8  addi r8, r11, 0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6584;
	// 826F18A8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F18AC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826F18B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F18B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F18B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F18BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F18C0: 386A7F84  addi r3, r10, 0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + 32644;
	// 826F18C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F18C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F18CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F18D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F18D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F18D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F18DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F18E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F18E4: 4BD7553D  bl 0x82466e20
	ctx.lr = 0x826F18E8;
	sub_82466E20(ctx, base);
	// 826F18E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F18EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F18F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F18F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F18F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F18F8 size=108
    let mut pc: u32 = 0x826F18F8;
    'dispatch: loop {
        match pc {
            0x826F18F8 => {
    //   block [0x826F18F8..0x826F1964)
	// 826F18F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F18FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1904: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F190C: 38EBB95C  addi r7, r11, -0x46a4
	ctx.r[7].s64 = ctx.r[11].s64 + -18084;
	// 826F1910: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1914: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826F1918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F191C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1928: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 826F192C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F194C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1950: 4BD754D1  bl 0x82466e20
	ctx.lr = 0x826F1954;
	sub_82466E20(ctx, base);
	// 826F1954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F195C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1968 size=116
    let mut pc: u32 = 0x826F1968;
    'dispatch: loop {
        match pc {
            0x826F1968 => {
    //   block [0x826F1968..0x826F19DC)
	// 826F1968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F196C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1974: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1978: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F197C: 390BB990  addi r8, r11, -0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + -18032;
	// 826F1980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1984: 392A9D3C  addi r9, r10, -0x62c4
	ctx.r[9].s64 = ctx.r[10].s64 + -25284;
	// 826F1988: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F198C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F1990: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1994: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F199C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F19A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F19A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F19A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F19AC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F19B0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826F19B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F19B8: 386B7FE4  addi r3, r11, 0x7fe4
	ctx.r[3].s64 = ctx.r[11].s64 + 32740;
	// 826F19BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F19C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F19C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F19C8: 4BD75459  bl 0x82466e20
	ctx.lr = 0x826F19CC;
	sub_82466E20(ctx, base);
	// 826F19CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F19D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F19D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F19D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F19E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F19E0 size=24
    let mut pc: u32 = 0x826F19E0;
    'dispatch: loop {
        match pc {
            0x826F19E0 => {
    //   block [0x826F19E0..0x826F19F8)
	// 826F19E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F19E4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F19E8: 394A1A30  addi r10, r10, 0x1a30
	ctx.r[10].s64 = ctx.r[10].s64 + 6704;
	// 826F19EC: 816BB98C  lwz r11, -0x4674(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 826F19F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F19F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F19F8 size=112
    let mut pc: u32 = 0x826F19F8;
    'dispatch: loop {
        match pc {
            0x826F19F8 => {
    //   block [0x826F19F8..0x826F1A68)
	// 826F19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F19FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1A04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1A08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1A0C: 392A9D78  addi r9, r10, -0x6288
	ctx.r[9].s64 = ctx.r[10].s64 + -25224;
	// 826F1A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1A14: 390B1A30  addi r8, r11, 0x1a30
	ctx.r[8].s64 = ctx.r[11].s64 + 6704;
	// 826F1A18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F1A1C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826F1A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1A24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1A30: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 826F1A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1A38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1A54: 4BD753CD  bl 0x82466e20
	ctx.lr = 0x826F1A58;
	sub_82466E20(ctx, base);
	// 826F1A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1A68 size=108
    let mut pc: u32 = 0x826F1A68;
    'dispatch: loop {
        match pc {
            0x826F1A68 => {
    //   block [0x826F1A68..0x826F1AD4)
	// 826F1A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1A74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1A7C: 38EBBA50  addi r7, r11, -0x45b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17840;
	// 826F1A80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1A84: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826F1A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1A8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1A98: 386A8044  addi r3, r10, -0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -32700;
	// 826F1A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1AC0: 4BD75361  bl 0x82466e20
	ctx.lr = 0x826F1AC4;
	sub_82466E20(ctx, base);
	// 826F1AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1AD8 size=108
    let mut pc: u32 = 0x826F1AD8;
    'dispatch: loop {
        match pc {
            0x826F1AD8 => {
    //   block [0x826F1AD8..0x826F1B44)
	// 826F1AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1AE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1AEC: 38EBBA68  addi r7, r11, -0x4598
	ctx.r[7].s64 = ctx.r[11].s64 + -17816;
	// 826F1AF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1AF4: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826F1AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1AFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1B08: 386A8074  addi r3, r10, -0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32652;
	// 826F1B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1B30: 4BD752F1  bl 0x82466e20
	ctx.lr = 0x826F1B34;
	sub_82466E20(ctx, base);
	// 826F1B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1B48 size=24
    let mut pc: u32 = 0x826F1B48;
    'dispatch: loop {
        match pc {
            0x826F1B48 => {
    //   block [0x826F1B48..0x826F1B60)
	// 826F1B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1B4C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1B50: 394A1A78  addi r10, r10, 0x1a78
	ctx.r[10].s64 = ctx.r[10].s64 + 6776;
	// 826F1B54: 816BBA98  lwz r11, -0x4568(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17768 as u32) ) } as u64;
	// 826F1B58: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F1B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1B60 size=112
    let mut pc: u32 = 0x826F1B60;
    'dispatch: loop {
        match pc {
            0x826F1B60 => {
    //   block [0x826F1B60..0x826F1BD0)
	// 826F1B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1B6C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1B70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1B74: 392A9DB4  addi r9, r10, -0x624c
	ctx.r[9].s64 = ctx.r[10].s64 + -25164;
	// 826F1B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1B7C: 390B1A78  addi r8, r11, 0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + 6776;
	// 826F1B80: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F1B84: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826F1B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1B8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1B98: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 826F1B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1BA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1BBC: 4BD75265  bl 0x82466e20
	ctx.lr = 0x826F1BC0;
	sub_82466E20(ctx, base);
	// 826F1BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1BD0 size=112
    let mut pc: u32 = 0x826F1BD0;
    'dispatch: loop {
        match pc {
            0x826F1BD0 => {
    //   block [0x826F1BD0..0x826F1C40)
	// 826F1BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1BDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1BE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1BE4: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1BEC: 390BBA9C  addi r8, r11, -0x4564
	ctx.r[8].s64 = ctx.r[11].s64 + -17764;
	// 826F1BF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F1BF4: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826F1BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1BFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1C00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1C08: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 826F1C0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F1C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1C2C: 4BD751F5  bl 0x82466e20
	ctx.lr = 0x826F1C30;
	sub_82466E20(ctx, base);
	// 826F1C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1C40 size=108
    let mut pc: u32 = 0x826F1C40;
    'dispatch: loop {
        match pc {
            0x826F1C40 => {
    //   block [0x826F1C40..0x826F1CAC)
	// 826F1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1C4C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1C54: 38EBBACC  addi r7, r11, -0x4534
	ctx.r[7].s64 = ctx.r[11].s64 + -17716;
	// 826F1C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1C5C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826F1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1C64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1C70: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 826F1C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1C98: 4BD75189  bl 0x82466e20
	ctx.lr = 0x826F1C9C;
	sub_82466E20(ctx, base);
	// 826F1C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1CB0 size=108
    let mut pc: u32 = 0x826F1CB0;
    'dispatch: loop {
        match pc {
            0x826F1CB0 => {
    //   block [0x826F1CB0..0x826F1D1C)
	// 826F1CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1CBC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1CC4: 38EBBB00  addi r7, r11, -0x4500
	ctx.r[7].s64 = ctx.r[11].s64 + -17664;
	// 826F1CC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F1CCC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826F1CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1CE0: 386A8134  addi r3, r10, -0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -32460;
	// 826F1CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1D08: 4BD75119  bl 0x82466e20
	ctx.lr = 0x826F1D0C;
	sub_82466E20(ctx, base);
	// 826F1D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1D20 size=108
    let mut pc: u32 = 0x826F1D20;
    'dispatch: loop {
        match pc {
            0x826F1D20 => {
    //   block [0x826F1D20..0x826F1D8C)
	// 826F1D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1D2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1D34: 38EBBB60  addi r7, r11, -0x44a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17568;
	// 826F1D38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1D3C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826F1D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1D50: 386A8164  addi r3, r10, -0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32412;
	// 826F1D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1D78: 4BD750A9  bl 0x82466e20
	ctx.lr = 0x826F1D7C;
	sub_82466E20(ctx, base);
	// 826F1D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1D90 size=108
    let mut pc: u32 = 0x826F1D90;
    'dispatch: loop {
        match pc {
            0x826F1D90 => {
    //   block [0x826F1D90..0x826F1DFC)
	// 826F1D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1D9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1DA4: 38EBBB90  addi r7, r11, -0x4470
	ctx.r[7].s64 = ctx.r[11].s64 + -17520;
	// 826F1DA8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826F1DAC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826F1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1DC0: 386A8194  addi r3, r10, -0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32364;
	// 826F1DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1DE8: 4BD75039  bl 0x82466e20
	ctx.lr = 0x826F1DEC;
	sub_82466E20(ctx, base);
	// 826F1DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1E00 size=108
    let mut pc: u32 = 0x826F1E00;
    'dispatch: loop {
        match pc {
            0x826F1E00 => {
    //   block [0x826F1E00..0x826F1E6C)
	// 826F1E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1E0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1E14: 38EBBCB0  addi r7, r11, -0x4350
	ctx.r[7].s64 = ctx.r[11].s64 + -17232;
	// 826F1E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1E1C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826F1E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1E30: 386A81C4  addi r3, r10, -0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32316;
	// 826F1E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1E58: 4BD74FC9  bl 0x82466e20
	ctx.lr = 0x826F1E5C;
	sub_82466E20(ctx, base);
	// 826F1E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1E70 size=108
    let mut pc: u32 = 0x826F1E70;
    'dispatch: loop {
        match pc {
            0x826F1E70 => {
    //   block [0x826F1E70..0x826F1EDC)
	// 826F1E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1E7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1E84: 38EBBCC8  addi r7, r11, -0x4338
	ctx.r[7].s64 = ctx.r[11].s64 + -17208;
	// 826F1E88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1E8C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826F1E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1E94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1EA0: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 826F1EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1EC8: 4BD74F59  bl 0x82466e20
	ctx.lr = 0x826F1ECC;
	sub_82466E20(ctx, base);
	// 826F1ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1EE0 size=108
    let mut pc: u32 = 0x826F1EE0;
    'dispatch: loop {
        match pc {
            0x826F1EE0 => {
    //   block [0x826F1EE0..0x826F1F4C)
	// 826F1EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1EEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1EF4: 38EBBCE0  addi r7, r11, -0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + -17184;
	// 826F1EF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1EFC: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826F1F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1F10: 386A8224  addi r3, r10, -0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -32220;
	// 826F1F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1F38: 4BD74EE9  bl 0x82466e20
	ctx.lr = 0x826F1F3C;
	sub_82466E20(ctx, base);
	// 826F1F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1F50 size=108
    let mut pc: u32 = 0x826F1F50;
    'dispatch: loop {
        match pc {
            0x826F1F50 => {
    //   block [0x826F1F50..0x826F1FBC)
	// 826F1F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1F5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1F64: 38EBBCF8  addi r7, r11, -0x4308
	ctx.r[7].s64 = ctx.r[11].s64 + -17160;
	// 826F1F68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1F6C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826F1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1F74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1F78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1F80: 386A8254  addi r3, r10, -0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + -32172;
	// 826F1F84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1FA8: 4BD74E79  bl 0x82466e20
	ctx.lr = 0x826F1FAC;
	sub_82466E20(ctx, base);
	// 826F1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1FC0 size=108
    let mut pc: u32 = 0x826F1FC0;
    'dispatch: loop {
        match pc {
            0x826F1FC0 => {
    //   block [0x826F1FC0..0x826F202C)
	// 826F1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1FCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1FD4: 38EBBD10  addi r7, r11, -0x42f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17136;
	// 826F1FD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1FDC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826F1FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1FE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1FF0: 386A8284  addi r3, r10, -0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -32124;
	// 826F1FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2018: 4BD74E09  bl 0x82466e20
	ctx.lr = 0x826F201C;
	sub_82466E20(ctx, base);
	// 826F201C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2030 size=108
    let mut pc: u32 = 0x826F2030;
    'dispatch: loop {
        match pc {
            0x826F2030 => {
    //   block [0x826F2030..0x826F209C)
	// 826F2030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F203C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2044: 38EBBD28  addi r7, r11, -0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + -17112;
	// 826F2048: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F204C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826F2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2054: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F205C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2060: 386A82B4  addi r3, r10, -0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32076;
	// 826F2064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F206C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F207C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2088: 4BD74D99  bl 0x82466e20
	ctx.lr = 0x826F208C;
	sub_82466E20(ctx, base);
	// 826F208C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F20A0 size=108
    let mut pc: u32 = 0x826F20A0;
    'dispatch: loop {
        match pc {
            0x826F20A0 => {
    //   block [0x826F20A0..0x826F210C)
	// 826F20A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F20A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F20A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F20AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F20B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F20B4: 38EBBD40  addi r7, r11, -0x42c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17088;
	// 826F20B8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F20BC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826F20C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F20C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F20C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F20CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F20D0: 386A82E4  addi r3, r10, -0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32028;
	// 826F20D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F20D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F20DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F20E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F20E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F20E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F20EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F20F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F20F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F20F8: 4BD74D29  bl 0x82466e20
	ctx.lr = 0x826F20FC;
	sub_82466E20(ctx, base);
	// 826F20FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2110 size=108
    let mut pc: u32 = 0x826F2110;
    'dispatch: loop {
        match pc {
            0x826F2110 => {
    //   block [0x826F2110..0x826F217C)
	// 826F2110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F211C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2124: 38EBBDD0  addi r7, r11, -0x4230
	ctx.r[7].s64 = ctx.r[11].s64 + -16944;
	// 826F2128: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F212C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826F2130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2134: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F213C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2140: 386A8314  addi r3, r10, -0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + -31980;
	// 826F2144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F214C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2168: 4BD74CB9  bl 0x82466e20
	ctx.lr = 0x826F216C;
	sub_82466E20(ctx, base);
	// 826F216C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2180 size=108
    let mut pc: u32 = 0x826F2180;
    'dispatch: loop {
        match pc {
            0x826F2180 => {
    //   block [0x826F2180..0x826F21EC)
	// 826F2180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F218C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2194: 38EBBE90  addi r7, r11, -0x4170
	ctx.r[7].s64 = ctx.r[11].s64 + -16752;
	// 826F2198: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F219C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826F21A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F21A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F21A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F21AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F21B0: 386A8344  addi r3, r10, -0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31932;
	// 826F21B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F21B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F21BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F21C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F21C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F21C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F21CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F21D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F21D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F21D8: 4BD74C49  bl 0x82466e20
	ctx.lr = 0x826F21DC;
	sub_82466E20(ctx, base);
	// 826F21DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F21E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F21E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F21E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F21F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F21F0 size=108
    let mut pc: u32 = 0x826F21F0;
    'dispatch: loop {
        match pc {
            0x826F21F0 => {
    //   block [0x826F21F0..0x826F225C)
	// 826F21F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F21F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F21F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F21FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2204: 38EBBF68  addi r7, r11, -0x4098
	ctx.r[7].s64 = ctx.r[11].s64 + -16536;
	// 826F2208: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F220C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826F2210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2214: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2220: 386A8374  addi r3, r10, -0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31884;
	// 826F2224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F222C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2248: 4BD74BD9  bl 0x82466e20
	ctx.lr = 0x826F224C;
	sub_82466E20(ctx, base);
	// 826F224C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2260 size=108
    let mut pc: u32 = 0x826F2260;
    'dispatch: loop {
        match pc {
            0x826F2260 => {
    //   block [0x826F2260..0x826F22CC)
	// 826F2260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F226C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2274: 38EBC028  addi r7, r11, -0x3fd8
	ctx.r[7].s64 = ctx.r[11].s64 + -16344;
	// 826F2278: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F227C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826F2280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F228C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2290: 386A83A4  addi r3, r10, -0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31836;
	// 826F2294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F229C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F22A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F22A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F22A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F22AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F22B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F22B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F22B8: 4BD74B69  bl 0x82466e20
	ctx.lr = 0x826F22BC;
	sub_82466E20(ctx, base);
	// 826F22BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F22C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F22C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F22C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F22D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F22D0 size=112
    let mut pc: u32 = 0x826F22D0;
    'dispatch: loop {
        match pc {
            0x826F22D0 => {
    //   block [0x826F22D0..0x826F2340)
	// 826F22D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F22D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F22D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F22DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F22E0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826F22E4: 38EAC0D0  addi r7, r10, -0x3f30
	ctx.r[7].s64 = ctx.r[10].s64 + -16176;
	// 826F22E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F22EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F22F0: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826F22F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F22F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F22FC: 396B9DC8  addi r11, r11, -0x6238
	ctx.r[11].s64 = ctx.r[11].s64 + -25144;
	// 826F2300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F230C: 386A83D4  addi r3, r10, -0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31788;
	// 826F2310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2314: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F2318: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F231C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F2320: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2324: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F232C: 4BD74AF5  bl 0x82466e20
	ctx.lr = 0x826F2330;
	sub_82466E20(ctx, base);
	// 826F2330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F233C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2340 size=108
    let mut pc: u32 = 0x826F2340;
    'dispatch: loop {
        match pc {
            0x826F2340 => {
    //   block [0x826F2340..0x826F23AC)
	// 826F2340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F234C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2354: 38EBC1F0  addi r7, r11, -0x3e10
	ctx.r[7].s64 = ctx.r[11].s64 + -15888;
	// 826F2358: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F235C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826F2360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F236C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2370: 386A8404  addi r3, r10, -0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -31740;
	// 826F2374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F237C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2398: 4BD74A89  bl 0x82466e20
	ctx.lr = 0x826F239C;
	sub_82466E20(ctx, base);
	// 826F239C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F23A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F23A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F23A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F23B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F23B0 size=108
    let mut pc: u32 = 0x826F23B0;
    'dispatch: loop {
        match pc {
            0x826F23B0 => {
    //   block [0x826F23B0..0x826F241C)
	// 826F23B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F23B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F23B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F23BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F23C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F23C4: 38EBC250  addi r7, r11, -0x3db0
	ctx.r[7].s64 = ctx.r[11].s64 + -15792;
	// 826F23C8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826F23CC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826F23D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F23D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F23D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F23DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F23E0: 386A8434  addi r3, r10, -0x7bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -31692;
	// 826F23E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F23E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F23EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F23F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F23F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F23F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F23FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2408: 4BD74A19  bl 0x82466e20
	ctx.lr = 0x826F240C;
	sub_82466E20(ctx, base);
	// 826F240C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2420 size=108
    let mut pc: u32 = 0x826F2420;
    'dispatch: loop {
        match pc {
            0x826F2420 => {
    //   block [0x826F2420..0x826F248C)
	// 826F2420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F242C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2434: 38EBC358  addi r7, r11, -0x3ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -15528;
	// 826F2438: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F243C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826F2440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2450: 386A8464  addi r3, r10, -0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -31644;
	// 826F2454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F245C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F246C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2478: 4BD749A9  bl 0x82466e20
	ctx.lr = 0x826F247C;
	sub_82466E20(ctx, base);
	// 826F247C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2490 size=108
    let mut pc: u32 = 0x826F2490;
    'dispatch: loop {
        match pc {
            0x826F2490 => {
    //   block [0x826F2490..0x826F24FC)
	// 826F2490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F249C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F24A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F24A4: 38EBC430  addi r7, r11, -0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -15312;
	// 826F24A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F24AC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826F24B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F24B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F24B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F24BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F24C0: 386A8494  addi r3, r10, -0x7b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31596;
	// 826F24C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F24C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F24CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F24D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F24D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F24DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F24E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F24E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F24E8: 4BD74939  bl 0x82466e20
	ctx.lr = 0x826F24EC;
	sub_82466E20(ctx, base);
	// 826F24EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F24F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F24F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F24F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2500 size=108
    let mut pc: u32 = 0x826F2500;
    'dispatch: loop {
        match pc {
            0x826F2500 => {
    //   block [0x826F2500..0x826F256C)
	// 826F2500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F250C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2514: 38EBC478  addi r7, r11, -0x3b88
	ctx.r[7].s64 = ctx.r[11].s64 + -15240;
	// 826F2518: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F251C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826F2520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F252C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2530: 386A84C4  addi r3, r10, -0x7b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31548;
	// 826F2534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F254C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2558: 4BD748C9  bl 0x82466e20
	ctx.lr = 0x826F255C;
	sub_82466E20(ctx, base);
	// 826F255C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2570 size=112
    let mut pc: u32 = 0x826F2570;
    'dispatch: loop {
        match pc {
            0x826F2570 => {
    //   block [0x826F2570..0x826F25E0)
	// 826F2570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F257C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2580: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2584: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F258C: 390BC490  addi r8, r11, -0x3b70
	ctx.r[8].s64 = ctx.r[11].s64 + -15216;
	// 826F2590: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F2594: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826F2598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F259C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F25A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F25A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F25A8: 386A84F4  addi r3, r10, -0x7b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31500;
	// 826F25AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F25B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F25B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F25B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F25BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F25C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F25C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F25C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F25CC: 4BD74855  bl 0x82466e20
	ctx.lr = 0x826F25D0;
	sub_82466E20(ctx, base);
	// 826F25D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F25D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F25D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F25DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F25E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F25E0 size=112
    let mut pc: u32 = 0x826F25E0;
    'dispatch: loop {
        match pc {
            0x826F25E0 => {
    //   block [0x826F25E0..0x826F2650)
	// 826F25E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F25E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F25E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F25EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F25F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F25F4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F25F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F25FC: 390BC4F0  addi r8, r11, -0x3b10
	ctx.r[8].s64 = ctx.r[11].s64 + -15120;
	// 826F2600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F2604: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 826F2608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F260C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2618: 386A8524  addi r3, r10, -0x7adc
	ctx.r[3].s64 = ctx.r[10].s64 + -31452;
	// 826F261C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F262C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F263C: 4BD747E5  bl 0x82466e20
	ctx.lr = 0x826F2640;
	sub_82466E20(ctx, base);
	// 826F2640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F264C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2650 size=108
    let mut pc: u32 = 0x826F2650;
    'dispatch: loop {
        match pc {
            0x826F2650 => {
    //   block [0x826F2650..0x826F26BC)
	// 826F2650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F265C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2664: 38EBC538  addi r7, r11, -0x3ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -15048;
	// 826F2668: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F266C: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826F2670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2674: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2680: 386A8554  addi r3, r10, -0x7aac
	ctx.r[3].s64 = ctx.r[10].s64 + -31404;
	// 826F2684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F269C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F26A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F26A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F26A8: 4BD74779  bl 0x82466e20
	ctx.lr = 0x826F26AC;
	sub_82466E20(ctx, base);
	// 826F26AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F26B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F26B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F26B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F26C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F26C0 size=24
    let mut pc: u32 = 0x826F26C0;
    'dispatch: loop {
        match pc {
            0x826F26C0 => {
    //   block [0x826F26C0..0x826F26D8)
	// 826F26C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F26C4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F26C8: 394A1AF0  addi r10, r10, 0x1af0
	ctx.r[10].s64 = ctx.r[10].s64 + 6896;
	// 826F26CC: 816BBAFC  lwz r11, -0x4504(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17668 as u32) ) } as u64;
	// 826F26D0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F26D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F26D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F26D8 size=112
    let mut pc: u32 = 0x826F26D8;
    'dispatch: loop {
        match pc {
            0x826F26D8 => {
    //   block [0x826F26D8..0x826F2748)
	// 826F26D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F26DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F26E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F26E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F26E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F26EC: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826F26F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F26F4: 390B1AF0  addi r8, r11, 0x1af0
	ctx.r[8].s64 = ctx.r[11].s64 + 6896;
	// 826F26F8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F26FC: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 826F2700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2704: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2710: 386A8584  addi r3, r10, -0x7a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -31356;
	// 826F2714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F272C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2734: 4BD746ED  bl 0x82466e20
	ctx.lr = 0x826F2738;
	sub_82466E20(ctx, base);
	// 826F2738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2748 size=108
    let mut pc: u32 = 0x826F2748;
    'dispatch: loop {
        match pc {
            0x826F2748 => {
    //   block [0x826F2748..0x826F27B4)
	// 826F2748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2754: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F275C: 38EBC550  addi r7, r11, -0x3ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -15024;
	// 826F2760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2764: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826F2768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F276C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2778: 386A85B4  addi r3, r10, -0x7a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31308;
	// 826F277C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F279C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F27A0: 4BD74681  bl 0x82466e20
	ctx.lr = 0x826F27A4;
	sub_82466E20(ctx, base);
	// 826F27A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F27A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F27AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F27B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F27B8 size=112
    let mut pc: u32 = 0x826F27B8;
    'dispatch: loop {
        match pc {
            0x826F27B8 => {
    //   block [0x826F27B8..0x826F2828)
	// 826F27B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F27BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F27C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F27C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F27C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F27CC: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826F27D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F27D4: 390BC5B0  addi r8, r11, -0x3a50
	ctx.r[8].s64 = ctx.r[11].s64 + -14928;
	// 826F27D8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F27DC: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 826F27E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F27E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F27E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F27F0: 386A85E4  addi r3, r10, -0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31260;
	// 826F27F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F27F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F27FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F280C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2814: 4BD7460D  bl 0x82466e20
	ctx.lr = 0x826F2818;
	sub_82466E20(ctx, base);
	// 826F2818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F281C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2828 size=108
    let mut pc: u32 = 0x826F2828;
    'dispatch: loop {
        match pc {
            0x826F2828 => {
    //   block [0x826F2828..0x826F2894)
	// 826F2828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2834: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F283C: 38EBC670  addi r7, r11, -0x3990
	ctx.r[7].s64 = ctx.r[11].s64 + -14736;
	// 826F2840: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F2844: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826F2848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F284C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2858: 386A8614  addi r3, r10, -0x79ec
	ctx.r[3].s64 = ctx.r[10].s64 + -31212;
	// 826F285C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F286C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F287C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2880: 4BD745A1  bl 0x82466e20
	ctx.lr = 0x826F2884;
	sub_82466E20(ctx, base);
	// 826F2884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F288C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2898 size=108
    let mut pc: u32 = 0x826F2898;
    'dispatch: loop {
        match pc {
            0x826F2898 => {
    //   block [0x826F2898..0x826F2904)
	// 826F2898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F28A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F28A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F28A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F28AC: 38EBC6E8  addi r7, r11, -0x3918
	ctx.r[7].s64 = ctx.r[11].s64 + -14616;
	// 826F28B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F28B4: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 826F28B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F28BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F28C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F28C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F28C8: 386A8644  addi r3, r10, -0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + -31164;
	// 826F28CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F28D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F28D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F28D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F28DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F28E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F28E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F28E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F28EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F28F0: 4BD74531  bl 0x82466e20
	ctx.lr = 0x826F28F4;
	sub_82466E20(ctx, base);
	// 826F28F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F28F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F28FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2908 size=108
    let mut pc: u32 = 0x826F2908;
    'dispatch: loop {
        match pc {
            0x826F2908 => {
    //   block [0x826F2908..0x826F2974)
	// 826F2908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2914: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F291C: 38EBC730  addi r7, r11, -0x38d0
	ctx.r[7].s64 = ctx.r[11].s64 + -14544;
	// 826F2920: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2924: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 826F2928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F292C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2938: 386A8674  addi r3, r10, -0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + -31116;
	// 826F293C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F294C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F295C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2960: 4BD744C1  bl 0x82466e20
	ctx.lr = 0x826F2964;
	sub_82466E20(ctx, base);
	// 826F2964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F296C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2978 size=112
    let mut pc: u32 = 0x826F2978;
    'dispatch: loop {
        match pc {
            0x826F2978 => {
    //   block [0x826F2978..0x826F29E8)
	// 826F2978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2988: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F298C: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 826F2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2994: 390BC778  addi r8, r11, -0x3888
	ctx.r[8].s64 = ctx.r[11].s64 + -14472;
	// 826F2998: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F299C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 826F29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F29A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F29A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F29AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F29B0: 386A86A4  addi r3, r10, -0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + -31068;
	// 826F29B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F29C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F29D4: 4BD7444D  bl 0x82466e20
	ctx.lr = 0x826F29D8;
	sub_82466E20(ctx, base);
	// 826F29D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F29E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F29E8 size=108
    let mut pc: u32 = 0x826F29E8;
    'dispatch: loop {
        match pc {
            0x826F29E8 => {
    //   block [0x826F29E8..0x826F2A54)
	// 826F29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F29F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F29F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F29F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F29FC: 38EBC7F0  addi r7, r11, -0x3810
	ctx.r[7].s64 = ctx.r[11].s64 + -14352;
	// 826F2A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2A04: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 826F2A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2A18: 386A86D4  addi r3, r10, -0x792c
	ctx.r[3].s64 = ctx.r[10].s64 + -31020;
	// 826F2A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2A40: 4BD743E1  bl 0x82466e20
	ctx.lr = 0x826F2A44;
	sub_82466E20(ctx, base);
	// 826F2A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2A58 size=108
    let mut pc: u32 = 0x826F2A58;
    'dispatch: loop {
        match pc {
            0x826F2A58 => {
    //   block [0x826F2A58..0x826F2AC4)
	// 826F2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2A64: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2A6C: 38EBC838  addi r7, r11, -0x37c8
	ctx.r[7].s64 = ctx.r[11].s64 + -14280;
	// 826F2A70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F2A74: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826F2A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2A7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2A88: 386A8704  addi r3, r10, -0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30972;
	// 826F2A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2AB0: 4BD74371  bl 0x82466e20
	ctx.lr = 0x826F2AB4;
	sub_82466E20(ctx, base);
	// 826F2AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2AC8 size=108
    let mut pc: u32 = 0x826F2AC8;
    'dispatch: loop {
        match pc {
            0x826F2AC8 => {
    //   block [0x826F2AC8..0x826F2B34)
	// 826F2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2AD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2ADC: 38EBC8F8  addi r7, r11, -0x3708
	ctx.r[7].s64 = ctx.r[11].s64 + -14088;
	// 826F2AE0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 826F2AE4: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 826F2AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2AF8: 386A8734  addi r3, r10, -0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30924;
	// 826F2AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2B20: 4BD74301  bl 0x82466e20
	ctx.lr = 0x826F2B24;
	sub_82466E20(ctx, base);
	// 826F2B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2B38 size=112
    let mut pc: u32 = 0x826F2B38;
    'dispatch: loop {
        match pc {
            0x826F2B38 => {
    //   block [0x826F2B38..0x826F2BA8)
	// 826F2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2B4C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2B54: 390BCA78  addi r8, r11, -0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + -13704;
	// 826F2B58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F2B5C: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826F2B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2B70: 386A8764  addi r3, r10, -0x789c
	ctx.r[3].s64 = ctx.r[10].s64 + -30876;
	// 826F2B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2B94: 4BD7428D  bl 0x82466e20
	ctx.lr = 0x826F2B98;
	sub_82466E20(ctx, base);
	// 826F2B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2BA8 size=108
    let mut pc: u32 = 0x826F2BA8;
    'dispatch: loop {
        match pc {
            0x826F2BA8 => {
    //   block [0x826F2BA8..0x826F2C14)
	// 826F2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2BB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2BBC: 38EBCAD8  addi r7, r11, -0x3528
	ctx.r[7].s64 = ctx.r[11].s64 + -13608;
	// 826F2BC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F2BC4: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 826F2BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2BCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2BD8: 386A8794  addi r3, r10, -0x786c
	ctx.r[3].s64 = ctx.r[10].s64 + -30828;
	// 826F2BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2C00: 4BD74221  bl 0x82466e20
	ctx.lr = 0x826F2C04;
	sub_82466E20(ctx, base);
	// 826F2C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2C18 size=112
    let mut pc: u32 = 0x826F2C18;
    'dispatch: loop {
        match pc {
            0x826F2C18 => {
    //   block [0x826F2C18..0x826F2C88)
	// 826F2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2C24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2C28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2C2C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2C34: 390BCB50  addi r8, r11, -0x34b0
	ctx.r[8].s64 = ctx.r[11].s64 + -13488;
	// 826F2C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F2C3C: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826F2C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2C44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2C50: 386A87C4  addi r3, r10, -0x783c
	ctx.r[3].s64 = ctx.r[10].s64 + -30780;
	// 826F2C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2C74: 4BD741AD  bl 0x82466e20
	ctx.lr = 0x826F2C78;
	sub_82466E20(ctx, base);
	// 826F2C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2C88 size=108
    let mut pc: u32 = 0x826F2C88;
    'dispatch: loop {
        match pc {
            0x826F2C88 => {
    //   block [0x826F2C88..0x826F2CF4)
	// 826F2C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2C94: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2C9C: 38EBCB98  addi r7, r11, -0x3468
	ctx.r[7].s64 = ctx.r[11].s64 + -13416;
	// 826F2CA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2CA4: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 826F2CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2CB8: 386A87F4  addi r3, r10, -0x780c
	ctx.r[3].s64 = ctx.r[10].s64 + -30732;
	// 826F2CBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2CE0: 4BD74141  bl 0x82466e20
	ctx.lr = 0x826F2CE4;
	sub_82466E20(ctx, base);
	// 826F2CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2CF8 size=108
    let mut pc: u32 = 0x826F2CF8;
    'dispatch: loop {
        match pc {
            0x826F2CF8 => {
    //   block [0x826F2CF8..0x826F2D64)
	// 826F2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2D04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2D08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F2D0C: 38EBCBF8  addi r7, r11, -0x3408
	ctx.r[7].s64 = ctx.r[11].s64 + -13320;
	// 826F2D10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2D14: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 826F2D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2D1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2D28: 386A8824  addi r3, r10, -0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30684;
	// 826F2D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2D50: 4BD740D1  bl 0x82466e20
	ctx.lr = 0x826F2D54;
	sub_82466E20(ctx, base);
	// 826F2D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2D68 size=108
    let mut pc: u32 = 0x826F2D68;
    'dispatch: loop {
        match pc {
            0x826F2D68 => {
    //   block [0x826F2D68..0x826F2DD4)
	// 826F2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2D74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2D7C: 38EBCC40  addi r7, r11, -0x33c0
	ctx.r[7].s64 = ctx.r[11].s64 + -13248;
	// 826F2D80: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F2D84: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826F2D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2D8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2D98: 386A8854  addi r3, r10, -0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30636;
	// 826F2D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2DC0: 4BD74061  bl 0x82466e20
	ctx.lr = 0x826F2DC4;
	sub_82466E20(ctx, base);
	// 826F2DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2DD8 size=108
    let mut pc: u32 = 0x826F2DD8;
    'dispatch: loop {
        match pc {
            0x826F2DD8 => {
    //   block [0x826F2DD8..0x826F2E44)
	// 826F2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2DE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2DEC: 38EBCD00  addi r7, r11, -0x3300
	ctx.r[7].s64 = ctx.r[11].s64 + -13056;
	// 826F2DF0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F2DF4: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 826F2DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2DFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2E08: 386A8884  addi r3, r10, -0x777c
	ctx.r[3].s64 = ctx.r[10].s64 + -30588;
	// 826F2E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2E30: 4BD73FF1  bl 0x82466e20
	ctx.lr = 0x826F2E34;
	sub_82466E20(ctx, base);
	// 826F2E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2E48 size=108
    let mut pc: u32 = 0x826F2E48;
    'dispatch: loop {
        match pc {
            0x826F2E48 => {
    //   block [0x826F2E48..0x826F2EB4)
	// 826F2E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2E54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2E5C: 38EBCD90  addi r7, r11, -0x3270
	ctx.r[7].s64 = ctx.r[11].s64 + -12912;
	// 826F2E60: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826F2E64: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 826F2E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2E6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2E78: 386A88B4  addi r3, r10, -0x774c
	ctx.r[3].s64 = ctx.r[10].s64 + -30540;
	// 826F2E7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2EA0: 4BD73F81  bl 0x82466e20
	ctx.lr = 0x826F2EA4;
	sub_82466E20(ctx, base);
	// 826F2EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2EB8 size=108
    let mut pc: u32 = 0x826F2EB8;
    'dispatch: loop {
        match pc {
            0x826F2EB8 => {
    //   block [0x826F2EB8..0x826F2F24)
	// 826F2EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2EC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2ECC: 38EBCEC8  addi r7, r11, -0x3138
	ctx.r[7].s64 = ctx.r[11].s64 + -12600;
	// 826F2ED0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2ED4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826F2ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2EDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2EE8: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 826F2EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2F10: 4BD73F11  bl 0x82466e20
	ctx.lr = 0x826F2F14;
	sub_82466E20(ctx, base);
	// 826F2F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2F28 size=116
    let mut pc: u32 = 0x826F2F28;
    'dispatch: loop {
        match pc {
            0x826F2F28 => {
    //   block [0x826F2F28..0x826F2F9C)
	// 826F2F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2F34: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2F38: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F2F3C: 390BCF30  addi r8, r11, -0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12496;
	// 826F2F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2F44: 392A9E7C  addi r9, r10, -0x6184
	ctx.r[9].s64 = ctx.r[10].s64 + -24964;
	// 826F2F48: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2F4C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F2F50: 38AA88E4  addi r5, r10, -0x771c
	ctx.r[5].s64 = ctx.r[10].s64 + -30492;
	// 826F2F54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2F6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F2F70: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826F2F74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F2F78: 386B8914  addi r3, r11, -0x76ec
	ctx.r[3].s64 = ctx.r[11].s64 + -30444;
	// 826F2F7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F2F80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2F88: 4BD73E99  bl 0x82466e20
	ctx.lr = 0x826F2F8C;
	sub_82466E20(ctx, base);
	// 826F2F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2FA0 size=96
    let mut pc: u32 = 0x826F2FA0;
    'dispatch: loop {
        match pc {
            0x826F2FA0 => {
    //   block [0x826F2FA0..0x826F3000)
	// 826F2FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2FAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2FB4: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826F2FB8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2FC0: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 826F2FC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2FCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F2FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2FE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F2FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2FE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F2FEC: 4BD73E35  bl 0x82466e20
	ctx.lr = 0x826F2FF0;
	sub_82466E20(ctx, base);
	// 826F2FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3000 size=112
    let mut pc: u32 = 0x826F3000;
    'dispatch: loop {
        match pc {
            0x826F3000 => {
    //   block [0x826F3000..0x826F3070)
	// 826F3000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F300C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3010: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3014: 38AAAA14  addi r5, r10, -0x55ec
	ctx.r[5].s64 = ctx.r[10].s64 + -21996;
	// 826F3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F301C: 390BCFA8  addi r8, r11, -0x3058
	ctx.r[8].s64 = ctx.r[11].s64 + -12376;
	// 826F3020: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F3024: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826F3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F302C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3038: 386A8974  addi r3, r10, -0x768c
	ctx.r[3].s64 = ctx.r[10].s64 + -30348;
	// 826F303C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F305C: 4BD73DC5  bl 0x82466e20
	ctx.lr = 0x826F3060;
	sub_82466E20(ctx, base);
	// 826F3060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3070 size=96
    let mut pc: u32 = 0x826F3070;
    'dispatch: loop {
        match pc {
            0x826F3070 => {
    //   block [0x826F3070..0x826F30D0)
	// 826F3070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F307C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3084: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826F3088: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F308C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3090: 386A89A4  addi r3, r10, -0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + -30300;
	// 826F3094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F309C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F30A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F30A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F30A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F30AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F30B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F30B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F30B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F30BC: 4BD73D65  bl 0x82466e20
	ctx.lr = 0x826F30C0;
	sub_82466E20(ctx, base);
	// 826F30C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F30C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F30C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F30CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F30D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F30D0 size=24
    let mut pc: u32 = 0x826F30D0;
    'dispatch: loop {
        match pc {
            0x826F30D0 => {
    //   block [0x826F30D0..0x826F30E8)
	// 826F30D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F30D4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F30D8: 394A1BB0  addi r10, r10, 0x1bb0
	ctx.r[10].s64 = ctx.r[10].s64 + 7088;
	// 826F30DC: 816BCF2C  lwz r11, -0x30d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12500 as u32) ) } as u64;
	// 826F30E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F30E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F30E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F30E8 size=116
    let mut pc: u32 = 0x826F30E8;
    'dispatch: loop {
        match pc {
            0x826F30E8 => {
    //   block [0x826F30E8..0x826F315C)
	// 826F30E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F30EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F30F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F30F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F30F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F30FC: 390B1BB0  addi r8, r11, 0x1bb0
	ctx.r[8].s64 = ctx.r[11].s64 + 7088;
	// 826F3100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3104: 392A9EB8  addi r9, r10, -0x6148
	ctx.r[9].s64 = ctx.r[10].s64 + -24904;
	// 826F3108: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F310C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F3110: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3114: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F311C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3124: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F312C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3130: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826F3134: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3138: 386B89D4  addi r3, r11, -0x762c
	ctx.r[3].s64 = ctx.r[11].s64 + -30252;
	// 826F313C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3148: 4BD73CD9  bl 0x82466e20
	ctx.lr = 0x826F314C;
	sub_82466E20(ctx, base);
	// 826F314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3160 size=104
    let mut pc: u32 = 0x826F3160;
    'dispatch: loop {
        match pc {
            0x826F3160 => {
    //   block [0x826F3160..0x826F31C8)
	// 826F3160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F316C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F3170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3174: 392A9EE4  addi r9, r10, -0x611c
	ctx.r[9].s64 = ctx.r[10].s64 + -24860;
	// 826F3178: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3180: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3184: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F318C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3194: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 826F3198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F319C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F31A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F31A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F31A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F31AC: 386A8A04  addi r3, r10, -0x75fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30204;
	// 826F31B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F31B4: 4BD73C6D  bl 0x82466e20
	ctx.lr = 0x826F31B8;
	sub_82466E20(ctx, base);
	// 826F31B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F31BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F31C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F31C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F31C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F31C8 size=96
    let mut pc: u32 = 0x826F31C8;
    'dispatch: loop {
        match pc {
            0x826F31C8 => {
    //   block [0x826F31C8..0x826F3228)
	// 826F31C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F31CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F31D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F31D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F31D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F31DC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826F31E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F31E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F31E8: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 826F31EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F31F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F31F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F31F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F31FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F320C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3214: 4BD73C0D  bl 0x82466e20
	ctx.lr = 0x826F3218;
	sub_82466E20(ctx, base);
	// 826F3218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3228 size=100
    let mut pc: u32 = 0x826F3228;
    'dispatch: loop {
        match pc {
            0x826F3228 => {
    //   block [0x826F3228..0x826F328C)
	// 826F3228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F323C: 38AA8A04  addi r5, r10, -0x75fc
	ctx.r[5].s64 = ctx.r[10].s64 + -30204;
	// 826F3240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3248: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 826F324C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F325C: 386A8A64  addi r3, r10, -0x759c
	ctx.r[3].s64 = ctx.r[10].s64 + -30108;
	// 826F3260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F326C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3278: 4BD73BA9  bl 0x82466e20
	ctx.lr = 0x826F327C;
	sub_82466E20(ctx, base);
	// 826F327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3290 size=112
    let mut pc: u32 = 0x826F3290;
    'dispatch: loop {
        match pc {
            0x826F3290 => {
    //   block [0x826F3290..0x826F3300)
	// 826F3290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F329C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F32A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F32A4: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F32A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F32AC: 390BD010  addi r8, r11, -0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -12272;
	// 826F32B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F32B4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826F32B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F32BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F32C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F32C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F32C8: 386A8A94  addi r3, r10, -0x756c
	ctx.r[3].s64 = ctx.r[10].s64 + -30060;
	// 826F32CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F32D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F32D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F32D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F32DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F32E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F32E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F32E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F32EC: 4BD73B35  bl 0x82466e20
	ctx.lr = 0x826F32F0;
	sub_82466E20(ctx, base);
	// 826F32F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F32F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F32F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F32FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3300 size=112
    let mut pc: u32 = 0x826F3300;
    'dispatch: loop {
        match pc {
            0x826F3300 => {
    //   block [0x826F3300..0x826F3370)
	// 826F3300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F330C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3310: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3314: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F3318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F331C: 390BD058  addi r8, r11, -0x2fa8
	ctx.r[8].s64 = ctx.r[11].s64 + -12200;
	// 826F3320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F3324: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826F3328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F332C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3338: 386A8AC4  addi r3, r10, -0x753c
	ctx.r[3].s64 = ctx.r[10].s64 + -30012;
	// 826F333C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F334C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F335C: 4BD73AC5  bl 0x82466e20
	ctx.lr = 0x826F3360;
	sub_82466E20(ctx, base);
	// 826F3360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3370 size=100
    let mut pc: u32 = 0x826F3370;
    'dispatch: loop {
        match pc {
            0x826F3370 => {
    //   block [0x826F3370..0x826F33D4)
	// 826F3370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F337C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3384: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F3388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F338C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3390: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826F3394: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F339C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F33A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F33A4: 386A8AF4  addi r3, r10, -0x750c
	ctx.r[3].s64 = ctx.r[10].s64 + -29964;
	// 826F33A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F33AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F33B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F33B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F33B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F33BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F33C0: 4BD73A61  bl 0x82466e20
	ctx.lr = 0x826F33C4;
	sub_82466E20(ctx, base);
	// 826F33C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F33C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F33CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F33D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F33D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F33D8 size=96
    let mut pc: u32 = 0x826F33D8;
    'dispatch: loop {
        match pc {
            0x826F33D8 => {
    //   block [0x826F33D8..0x826F3438)
	// 826F33D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F33DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F33E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F33E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F33E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F33EC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826F33F0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F33F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F33F8: 386A8B24  addi r3, r10, -0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29916;
	// 826F33FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3404: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F3408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F340C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F341C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3424: 4BD739FD  bl 0x82466e20
	ctx.lr = 0x826F3428;
	sub_82466E20(ctx, base);
	// 826F3428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3438 size=112
    let mut pc: u32 = 0x826F3438;
    'dispatch: loop {
        match pc {
            0x826F3438 => {
    //   block [0x826F3438..0x826F34A8)
	// 826F3438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F3448: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F344C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3454: 390BD070  addi r8, r11, -0x2f90
	ctx.r[8].s64 = ctx.r[11].s64 + -12176;
	// 826F3458: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F345C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826F3460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F346C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3470: 386A8B54  addi r3, r10, -0x74ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29868;
	// 826F3474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F347C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F348C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3494: 4BD7398D  bl 0x82466e20
	ctx.lr = 0x826F3498;
	sub_82466E20(ctx, base);
	// 826F3498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F34A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F34A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F34A8 size=96
    let mut pc: u32 = 0x826F34A8;
    'dispatch: loop {
        match pc {
            0x826F34A8 => {
    //   block [0x826F34A8..0x826F3508)
	// 826F34A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F34AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F34B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F34B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F34B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F34BC: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826F34C0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F34C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F34C8: 386A8B84  addi r3, r10, -0x747c
	ctx.r[3].s64 = ctx.r[10].s64 + -29820;
	// 826F34CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F34D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F34D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F34D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F34DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F34E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F34E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F34E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F34EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F34F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F34F4: 4BD7392D  bl 0x82466e20
	ctx.lr = 0x826F34F8;
	sub_82466E20(ctx, base);
	// 826F34F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F34FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3508 size=112
    let mut pc: u32 = 0x826F3508;
    'dispatch: loop {
        match pc {
            0x826F3508 => {
    //   block [0x826F3508..0x826F3578)
	// 826F3508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3518: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F351C: 38AA8B84  addi r5, r10, -0x747c
	ctx.r[5].s64 = ctx.r[10].s64 + -29820;
	// 826F3520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3524: 390BD0A0  addi r8, r11, -0x2f60
	ctx.r[8].s64 = ctx.r[11].s64 + -12128;
	// 826F3528: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F352C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826F3530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3534: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F353C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3540: 386A8BB4  addi r3, r10, -0x744c
	ctx.r[3].s64 = ctx.r[10].s64 + -29772;
	// 826F3544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F354C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F355C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3564: 4BD738BD  bl 0x82466e20
	ctx.lr = 0x826F3568;
	sub_82466E20(ctx, base);
	// 826F3568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F356C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3578 size=108
    let mut pc: u32 = 0x826F3578;
    'dispatch: loop {
        match pc {
            0x826F3578 => {
    //   block [0x826F3578..0x826F35E4)
	// 826F3578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3584: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F358C: 38EBD0B8  addi r7, r11, -0x2f48
	ctx.r[7].s64 = ctx.r[11].s64 + -12104;
	// 826F3590: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3594: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826F3598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F359C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F35A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F35A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F35A8: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 826F35AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F35B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F35B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F35B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F35BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F35C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F35C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F35C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F35CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F35D0: 4BD73851  bl 0x82466e20
	ctx.lr = 0x826F35D4;
	sub_82466E20(ctx, base);
	// 826F35D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F35D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F35DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F35E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F35E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F35E8 size=112
    let mut pc: u32 = 0x826F35E8;
    'dispatch: loop {
        match pc {
            0x826F35E8 => {
    //   block [0x826F35E8..0x826F3658)
	// 826F35E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F35EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F35F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F35F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F35F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F35FC: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3604: 390BD118  addi r8, r11, -0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -12008;
	// 826F3608: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F360C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826F3610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3614: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3620: 386A8C14  addi r3, r10, -0x73ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29676;
	// 826F3624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F362C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3644: 4BD737DD  bl 0x82466e20
	ctx.lr = 0x826F3648;
	sub_82466E20(ctx, base);
	// 826F3648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3658 size=112
    let mut pc: u32 = 0x826F3658;
    'dispatch: loop {
        match pc {
            0x826F3658 => {
    //   block [0x826F3658..0x826F36C8)
	// 826F3658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3664: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3668: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F366C: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F3670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3674: 390BD130  addi r8, r11, -0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -11984;
	// 826F3678: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F367C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826F3680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3690: 386A8C44  addi r3, r10, -0x73bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29628;
	// 826F3694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F369C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F36A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F36A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F36A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F36AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F36B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F36B4: 4BD7376D  bl 0x82466e20
	ctx.lr = 0x826F36B8;
	sub_82466E20(ctx, base);
	// 826F36B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F36BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F36C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F36C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F36C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F36C8 size=100
    let mut pc: u32 = 0x826F36C8;
    'dispatch: loop {
        match pc {
            0x826F36C8 => {
    //   block [0x826F36C8..0x826F372C)
	// 826F36C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F36CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F36D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F36D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F36D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F36DC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F36E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F36E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F36E8: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826F36EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F36F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F36F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F36F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F36FC: 386A8C74  addi r3, r10, -0x738c
	ctx.r[3].s64 = ctx.r[10].s64 + -29580;
	// 826F3700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3708: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3710: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3718: 4BD73709  bl 0x82466e20
	ctx.lr = 0x826F371C;
	sub_82466E20(ctx, base);
	// 826F371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3730 size=116
    let mut pc: u32 = 0x826F3730;
    'dispatch: loop {
        match pc {
            0x826F3730 => {
    //   block [0x826F3730..0x826F37A4)
	// 826F3730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F373C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3740: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F3744: 390BD160  addi r8, r11, -0x2ea0
	ctx.r[8].s64 = ctx.r[11].s64 + -11936;
	// 826F3748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F374C: 392A9F10  addi r9, r10, -0x60f0
	ctx.r[9].s64 = ctx.r[10].s64 + -24816;
	// 826F3750: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3754: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F3758: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F375C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3764: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F376C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3774: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3778: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826F377C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3780: 386B8CA4  addi r3, r11, -0x735c
	ctx.r[3].s64 = ctx.r[11].s64 + -29532;
	// 826F3784: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3788: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3790: 4BD73691  bl 0x82466e20
	ctx.lr = 0x826F3794;
	sub_82466E20(ctx, base);
	// 826F3794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F379C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F37A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F37A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F37A8 size=112
    let mut pc: u32 = 0x826F37A8;
    'dispatch: loop {
        match pc {
            0x826F37A8 => {
    //   block [0x826F37A8..0x826F3818)
	// 826F37A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F37AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F37B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F37B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F37B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F37BC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F37C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F37C4: 390BD190  addi r8, r11, -0x2e70
	ctx.r[8].s64 = ctx.r[11].s64 + -11888;
	// 826F37C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F37CC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826F37D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F37D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F37D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F37DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F37E0: 386A8CD4  addi r3, r10, -0x732c
	ctx.r[3].s64 = ctx.r[10].s64 + -29484;
	// 826F37E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F37E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F37EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F37F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F37F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F37F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F37FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3804: 4BD7361D  bl 0x82466e20
	ctx.lr = 0x826F3808;
	sub_82466E20(ctx, base);
	// 826F3808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F380C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


