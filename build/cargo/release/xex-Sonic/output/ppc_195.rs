pub fn sub_82E4E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E2F0 size=112
    let mut pc: u32 = 0x82E4E2F0;
    'dispatch: loop {
        match pc {
            0x82E4E2F0 => {
    //   block [0x82E4E2F0..0x82E4E360)
	// 82E4E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E2F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E2FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E304: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E4E308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4E310: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E4E314: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E4E318: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82E4E31C: 4BFA40CD  bl 0x82df23e8
	ctx.lr = 0x82E4E320;
	sub_82DF23E8(ctx, base);
	// 82E4E320: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4E324: 41820010  beq 0x82e4e334
	if ctx.cr[0].eq {
	pc = 0x82E4E334; continue 'dispatch;
	}
	// 82E4E328: 4BFFFDC1  bl 0x82e4e0e8
	ctx.lr = 0x82E4E32C;
	sub_82E4E0E8(ctx, base);
	// 82E4E32C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E330: 48000008  b 0x82e4e338
	pc = 0x82E4E338; continue 'dispatch;
	// 82E4E334: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4E338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E33C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E4E340: 4BFB0539  bl 0x82dfe878
	ctx.lr = 0x82E4E344;
	sub_82DFE878(ctx, base);
	// 82E4E344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E348: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E354: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E360 size=108
    let mut pc: u32 = 0x82E4E360;
    'dispatch: loop {
        match pc {
            0x82E4E360 => {
    //   block [0x82E4E360..0x82E4E3CC)
	// 82E4E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E364: 48359E05  bl 0x831a8168
	ctx.lr = 0x82E4E368;
	sub_831A8130(ctx, base);
	// 82E4E368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E36C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E374: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E37C: 388BAF88  addi r4, r11, -0x5078
	ctx.r[4].s64 = ctx.r[11].s64 + -20600;
	// 82E4E380: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4E384: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4E388: 4BFA5681  bl 0x82df3a08
	ctx.lr = 0x82E4E38C;
	sub_82DF3A08(ctx, base);
	// 82E4E38C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4E390: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E394: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E398: 48000409  bl 0x82e4e7a0
	ctx.lr = 0x82E4E39C;
	sub_82E4E7A0(ctx, base);
	// 82E4E39C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E4E3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E3A4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E3A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4E3AC: 4BFFFE9D  bl 0x82e4e248
	ctx.lr = 0x82E4E3B0;
	sub_82E4E248(ctx, base);
	// 82E4E3B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E3B4: 4BFA5075  bl 0x82df3428
	ctx.lr = 0x82E4E3B8;
	sub_82DF3428(ctx, base);
	// 82E4E3B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E3BC: 4BFA506D  bl 0x82df3428
	ctx.lr = 0x82E4E3C0;
	sub_82DF3428(ctx, base);
	// 82E4E3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4E3C8: 48359DF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E3D0 size=164
    let mut pc: u32 = 0x82E4E3D0;
    'dispatch: loop {
        match pc {
            0x82E4E3D0 => {
    //   block [0x82E4E3D0..0x82E4E474)
	// 82E4E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E3D4: 48359D99  bl 0x831a816c
	ctx.lr = 0x82E4E3D8;
	sub_831A8130(ctx, base);
	// 82E4E3D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E3DC: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E4E3E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E3E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E4E3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E4E3EC: 396BE2F0  addi r11, r11, -0x1d10
	ctx.r[11].s64 = ctx.r[11].s64 + -7440;
	// 82E4E3F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E4E3F4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E4E3F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4E3FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4E400: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E4E404: 4BFB04F5  bl 0x82dfe8f8
	ctx.lr = 0x82E4E408;
	sub_82DFE8F8(ctx, base);
	// 82E4E408: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E4E40C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E4E410: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E4E414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4E418: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4E41C: 4BFB347D  bl 0x82e01898
	ctx.lr = 0x82E4E420;
	sub_82E01898(ctx, base);
	// 82E4E420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E424: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4E428: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E42C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4E430: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4E434: 419A0024  beq cr6, 0x82e4e458
	if ctx.cr[6].eq {
	pc = 0x82E4E458; continue 'dispatch;
	}
	// 82E4E438: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E4E43C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E4E440: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E4E444: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E4E448: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E4E44C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E4E450: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E4E454: 4082FFE8  bne 0x82e4e43c
	if !ctx.cr[0].eq {
	pc = 0x82E4E43C; continue 'dispatch;
	}
	// 82E4E458: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4E45C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E460: 419A0008  beq cr6, 0x82e4e468
	if ctx.cr[6].eq {
	pc = 0x82E4E468; continue 'dispatch;
	}
	// 82E4E464: 4B47242D  bl 0x822c0890
	ctx.lr = 0x82E4E468;
	sub_822C0890(ctx, base);
	// 82E4E468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E46C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4E470: 48359D4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E478 size=108
    let mut pc: u32 = 0x82E4E478;
    'dispatch: loop {
        match pc {
            0x82E4E478 => {
    //   block [0x82E4E478..0x82E4E4E4)
	// 82E4E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E47C: 48359CED  bl 0x831a8168
	ctx.lr = 0x82E4E480;
	sub_831A8130(ctx, base);
	// 82E4E480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E484: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E48C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E494: 388BAF78  addi r4, r11, -0x5088
	ctx.r[4].s64 = ctx.r[11].s64 + -20616;
	// 82E4E498: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4E49C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E4E4A0: 4BFA5569  bl 0x82df3a08
	ctx.lr = 0x82E4E4A4;
	sub_82DF3A08(ctx, base);
	// 82E4E4A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4E4A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E4AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E4B0: 480002F1  bl 0x82e4e7a0
	ctx.lr = 0x82E4E4B4;
	sub_82E4E7A0(ctx, base);
	// 82E4E4B4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E4E4B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E4BC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E4C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E4E4C4: 4BFFFF0D  bl 0x82e4e3d0
	ctx.lr = 0x82E4E4C8;
	sub_82E4E3D0(ctx, base);
	// 82E4E4C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E4CC: 4BFA4F5D  bl 0x82df3428
	ctx.lr = 0x82E4E4D0;
	sub_82DF3428(ctx, base);
	// 82E4E4D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E4D4: 4BFA4F55  bl 0x82df3428
	ctx.lr = 0x82E4E4D8;
	sub_82DF3428(ctx, base);
	// 82E4E4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4E4E0: 48359CD8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E4E8 size=112
    let mut pc: u32 = 0x82E4E4E8;
    'dispatch: loop {
        match pc {
            0x82E4E4E8 => {
    //   block [0x82E4E4E8..0x82E4E558)
	// 82E4E4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E4F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E4F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E4F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E4FC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E504: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E50C: 388BAF78  addi r4, r11, -0x5088
	ctx.r[4].s64 = ctx.r[11].s64 + -20616;
	// 82E4E510: 4BFA54F9  bl 0x82df3a08
	ctx.lr = 0x82E4E514;
	sub_82DF3A08(ctx, base);
	// 82E4E514: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4E518: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E51C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E520: 48000281  bl 0x82e4e7a0
	ctx.lr = 0x82E4E524;
	sub_82E4E7A0(ctx, base);
	// 82E4E524: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4E528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E52C: 4BFB3F0D  bl 0x82e02438
	ctx.lr = 0x82E4E530;
	sub_82E02438(ctx, base);
	// 82E4E530: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E534: 4BFA4EF5  bl 0x82df3428
	ctx.lr = 0x82E4E538;
	sub_82DF3428(ctx, base);
	// 82E4E538: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E53C: 4BFA4EED  bl 0x82df3428
	ctx.lr = 0x82E4E540;
	sub_82DF3428(ctx, base);
	// 82E4E540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E54C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E558 size=112
    let mut pc: u32 = 0x82E4E558;
    'dispatch: loop {
        match pc {
            0x82E4E558 => {
    //   block [0x82E4E558..0x82E4E5C8)
	// 82E4E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E56C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E574: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E578: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E57C: 388BAF88  addi r4, r11, -0x5078
	ctx.r[4].s64 = ctx.r[11].s64 + -20600;
	// 82E4E580: 4BFA5489  bl 0x82df3a08
	ctx.lr = 0x82E4E584;
	sub_82DF3A08(ctx, base);
	// 82E4E584: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4E588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E58C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E590: 48000211  bl 0x82e4e7a0
	ctx.lr = 0x82E4E594;
	sub_82E4E7A0(ctx, base);
	// 82E4E594: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4E598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E59C: 4BFB3E9D  bl 0x82e02438
	ctx.lr = 0x82E4E5A0;
	sub_82E02438(ctx, base);
	// 82E4E5A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E5A4: 4BFA4E85  bl 0x82df3428
	ctx.lr = 0x82E4E5A8;
	sub_82DF3428(ctx, base);
	// 82E4E5A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E5AC: 4BFA4E7D  bl 0x82df3428
	ctx.lr = 0x82E4E5B0;
	sub_82DF3428(ctx, base);
	// 82E4E5B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E5BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E5C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E5C8 size=120
    let mut pc: u32 = 0x82E4E5C8;
    'dispatch: loop {
        match pc {
            0x82E4E5C8 => {
    //   block [0x82E4E5C8..0x82E4E640)
	// 82E4E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E5CC: 48359BA1  bl 0x831a816c
	ctx.lr = 0x82E4E5D0;
	sub_831A8130(ctx, base);
	// 82E4E5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E5D4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E4E5D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4E5DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E5E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E5E4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E5E8: 4BFF1921  bl 0x82e3ff08
	ctx.lr = 0x82E4E5EC;
	sub_82E3FF08(ctx, base);
	// 82E4E5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4E5F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4E5F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E5F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4E5FC: 4BFFFE7D  bl 0x82e4e478
	ctx.lr = 0x82E4E600;
	sub_82E4E478(ctx, base);
	// 82E4E600: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4E604: 419A0014  beq cr6, 0x82e4e618
	if ctx.cr[6].eq {
	pc = 0x82E4E618; continue 'dispatch;
	}
	// 82E4E608: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E4E60C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4E610: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4E614: 480004D5  bl 0x82e4eae8
	ctx.lr = 0x82E4E618;
	sub_82E4EAE8(ctx, base);
	// 82E4E618: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4E61C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E620: 419A0008  beq cr6, 0x82e4e628
	if ctx.cr[6].eq {
	pc = 0x82E4E628; continue 'dispatch;
	}
	// 82E4E624: 4B47226D  bl 0x822c0890
	ctx.lr = 0x82E4E628;
	sub_822C0890(ctx, base);
	// 82E4E628: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E62C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E630: 419A0008  beq cr6, 0x82e4e638
	if ctx.cr[6].eq {
	pc = 0x82E4E638; continue 'dispatch;
	}
	// 82E4E634: 4B47225D  bl 0x822c0890
	ctx.lr = 0x82E4E638;
	sub_822C0890(ctx, base);
	// 82E4E638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4E63C: 48359B80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E640 size=120
    let mut pc: u32 = 0x82E4E640;
    'dispatch: loop {
        match pc {
            0x82E4E640 => {
    //   block [0x82E4E640..0x82E4E6B8)
	// 82E4E640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E644: 48359B29  bl 0x831a816c
	ctx.lr = 0x82E4E648;
	sub_831A8130(ctx, base);
	// 82E4E648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E64C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E4E650: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4E654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E65C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4E660: 4BFF18A9  bl 0x82e3ff08
	ctx.lr = 0x82E4E664;
	sub_82E3FF08(ctx, base);
	// 82E4E664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4E668: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E4E66C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E670: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4E674: 4BFFFCED  bl 0x82e4e360
	ctx.lr = 0x82E4E678;
	sub_82E4E360(ctx, base);
	// 82E4E678: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4E67C: 419A0014  beq cr6, 0x82e4e690
	if ctx.cr[6].eq {
	pc = 0x82E4E690; continue 'dispatch;
	}
	// 82E4E680: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E4E684: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4E688: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4E68C: 4800134D  bl 0x82e4f9d8
	ctx.lr = 0x82E4E690;
	sub_82E4F9D8(ctx, base);
	// 82E4E690: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4E694: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E698: 419A0008  beq cr6, 0x82e4e6a0
	if ctx.cr[6].eq {
	pc = 0x82E4E6A0; continue 'dispatch;
	}
	// 82E4E69C: 4B4721F5  bl 0x822c0890
	ctx.lr = 0x82E4E6A0;
	sub_822C0890(ctx, base);
	// 82E4E6A0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E6A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E6A8: 419A0008  beq cr6, 0x82e4e6b0
	if ctx.cr[6].eq {
	pc = 0x82E4E6B0; continue 'dispatch;
	}
	// 82E4E6AC: 4B4721E5  bl 0x822c0890
	ctx.lr = 0x82E4E6B0;
	sub_822C0890(ctx, base);
	// 82E4E6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4E6B4: 48359B08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E6B8 size=228
    let mut pc: u32 = 0x82E4E6B8;
    'dispatch: loop {
        match pc {
            0x82E4E6B8 => {
    //   block [0x82E4E6B8..0x82E4E79C)
	// 82E4E6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E6C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E6C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E6C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E6CC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E6D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E6D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E6D8: 388BAF6C  addi r4, r11, -0x5094
	ctx.r[4].s64 = ctx.r[11].s64 + -20628;
	// 82E4E6DC: 4BFA532D  bl 0x82df3a08
	ctx.lr = 0x82E4E6E0;
	sub_82DF3A08(ctx, base);
	// 82E4E6E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4E6E4: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E4E6E8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82E4E6EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4E6F0: 388BE4E8  addi r4, r11, -0x1b18
	ctx.r[4].s64 = ctx.r[11].s64 + -6936;
	// 82E4E6F4: 4B483A2D  bl 0x822d2120
	ctx.lr = 0x82E4E6F8;
	sub_822D2120(ctx, base);
	// 82E4E6F8: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82E4E6FC: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E4E700: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4E704: 388BE5C8  addi r4, r11, -0x1a38
	ctx.r[4].s64 = ctx.r[11].s64 + -6712;
	// 82E4E708: 4B74BD01  bl 0x8259a408
	ctx.lr = 0x82E4E70C;
	sub_8259A408(ctx, base);
	// 82E4E70C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4E710: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E4E714: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E4E718: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82E4E71C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E720: 4BFB4AE9  bl 0x82e03208
	ctx.lr = 0x82E4E724;
	sub_82E03208(ctx, base);
	// 82E4E724: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E728: 4BFA4D01  bl 0x82df3428
	ctx.lr = 0x82E4E72C;
	sub_82DF3428(ctx, base);
	// 82E4E72C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E730: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E734: 388BAF70  addi r4, r11, -0x5090
	ctx.r[4].s64 = ctx.r[11].s64 + -20624;
	// 82E4E738: 4BFA52D1  bl 0x82df3a08
	ctx.lr = 0x82E4E73C;
	sub_82DF3A08(ctx, base);
	// 82E4E73C: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82E4E740: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E4E744: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E4E748: 388BE558  addi r4, r11, -0x1aa8
	ctx.r[4].s64 = ctx.r[11].s64 + -6824;
	// 82E4E74C: 4B4839D5  bl 0x822d2120
	ctx.lr = 0x82E4E750;
	sub_822D2120(ctx, base);
	// 82E4E750: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82E4E754: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E4E758: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4E75C: 388BE640  addi r4, r11, -0x19c0
	ctx.r[4].s64 = ctx.r[11].s64 + -6592;
	// 82E4E760: 4B74BCA9  bl 0x8259a408
	ctx.lr = 0x82E4E764;
	sub_8259A408(ctx, base);
	// 82E4E764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E4E768: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82E4E76C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E4E770: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4E774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4E778: 4BFB4A91  bl 0x82e03208
	ctx.lr = 0x82E4E77C;
	sub_82E03208(ctx, base);
	// 82E4E77C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E780: 4BFA4CA9  bl 0x82df3428
	ctx.lr = 0x82E4E784;
	sub_82DF3428(ctx, base);
	// 82E4E784: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E4E788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E7A0 size=132
    let mut pc: u32 = 0x82E4E7A0;
    'dispatch: loop {
        match pc {
            0x82E4E7A0 => {
    //   block [0x82E4E7A0..0x82E4E824)
	// 82E4E7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E7A4: 483599C5  bl 0x831a8168
	ctx.lr = 0x82E4E7A8;
	sub_831A8130(ctx, base);
	// 82E4E7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E7B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E7B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E4E7B8: 4BFA4939  bl 0x82df30f0
	ctx.lr = 0x82E4E7BC;
	sub_82DF30F0(ctx, base);
	// 82E4E7BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4E7C0: 4BFA4C71  bl 0x82df3430
	ctx.lr = 0x82E4E7C4;
	sub_82DF3430(ctx, base);
	// 82E4E7C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4E7C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E4E7CC: 4BFA4C65  bl 0x82df3430
	ctx.lr = 0x82E4E7D0;
	sub_82DF3430(ctx, base);
	// 82E4E7D0: 7D7C1A14  add r11, r28, r3
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[3].u64;
	// 82E4E7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E7D8: 388B0007  addi r4, r11, 7
	ctx.r[4].s64 = ctx.r[11].s64 + 7;
	// 82E4E7DC: 48279305  bl 0x830c7ae0
	ctx.lr = 0x82E4E7E0;
	sub_830C7AE0(ctx, base);
	// 82E4E7E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4E7E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E7E8: 388BC714  addi r4, r11, -0x38ec
	ctx.r[4].s64 = ctx.r[11].s64 + -14572;
	// 82E4E7EC: 4BFA4D8D  bl 0x82df3578
	ctx.lr = 0x82E4E7F0;
	sub_82DF3578(ctx, base);
	// 82E4E7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E7F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4E7F8: 4BFA4EE1  bl 0x82df36d8
	ctx.lr = 0x82E4E7FC;
	sub_82DF36D8(ctx, base);
	// 82E4E7FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E4E800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E804: 388B9FD8  addi r4, r11, -0x6028
	ctx.r[4].s64 = ctx.r[11].s64 + -24616;
	// 82E4E808: 4BFA4D71  bl 0x82df3578
	ctx.lr = 0x82E4E80C;
	sub_82DF3578(ctx, base);
	// 82E4E80C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E810: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4E814: 4BFA4EC5  bl 0x82df36d8
	ctx.lr = 0x82E4E818;
	sub_82DF36D8(ctx, base);
	// 82E4E818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4E820: 48359998  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E828 size=172
    let mut pc: u32 = 0x82E4E828;
    'dispatch: loop {
        match pc {
            0x82E4E828 => {
    //   block [0x82E4E828..0x82E4E8D4)
	// 82E4E828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E83C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E844: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4E848: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E84C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4E850: 4B4720E9  bl 0x822c0938
	ctx.lr = 0x82E4E854;
	sub_822C0938(ctx, base);
	// 82E4E854: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4E858: 41820028  beq 0x82e4e880
	if ctx.cr[0].eq {
	pc = 0x82E4E880; continue 'dispatch;
	}
	// 82E4E85C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4E860: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4E864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4E868: 392BC73C  addi r9, r11, -0x38c4
	ctx.r[9].s64 = ctx.r[11].s64 + -14532;
	// 82E4E86C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4E870: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4E874: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4E878: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4E87C: 48000008  b 0x82e4e884
	pc = 0x82E4E884; continue 'dispatch;
	// 82E4E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E884: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4E888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4E88C: 409A002C  bne cr6, 0x82e4e8b8
	if !ctx.cr[6].eq {
	pc = 0x82E4E8B8; continue 'dispatch;
	}
	// 82E4E890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E894: 4B4719D5  bl 0x822c0268
	ctx.lr = 0x82E4E898;
	sub_822C0268(ctx, base);
	// 82E4E898: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4E89C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4E8A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4E8A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4E8A8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4E8AC: 816BAF9C  lwz r11, -0x5064(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20580 as u32) ) } as u64;
	// 82E4E8B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4E8B4: 4B47174D  bl 0x822c0000
	ctx.lr = 0x82E4E8B8;
	sub_822C0000(ctx, base);
	// 82E4E8B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4E8BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E8C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E8CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E8D8 size=96
    let mut pc: u32 = 0x82E4E8D8;
    'dispatch: loop {
        match pc {
            0x82E4E8D8 => {
    //   block [0x82E4E8D8..0x82E4E938)
	// 82E4E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E8E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E8E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E8EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E8F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4E8F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E8FC: 419A0008  beq cr6, 0x82e4e904
	if ctx.cr[6].eq {
	pc = 0x82E4E904; continue 'dispatch;
	}
	// 82E4E900: 4B471F91  bl 0x822c0890
	ctx.lr = 0x82E4E904;
	sub_822C0890(ctx, base);
	// 82E4E904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E908: 4BFA4B21  bl 0x82df3428
	ctx.lr = 0x82E4E90C;
	sub_82DF3428(ctx, base);
	// 82E4E90C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4E910: 4182000C  beq 0x82e4e91c
	if ctx.cr[0].eq {
	pc = 0x82E4E91C; continue 'dispatch;
	}
	// 82E4E914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E918: 4B471951  bl 0x822c0268
	ctx.lr = 0x82E4E91C;
	sub_822C0268(ctx, base);
	// 82E4E91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4E920: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E92C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E938 size=112
    let mut pc: u32 = 0x82E4E938;
    'dispatch: loop {
        match pc {
            0x82E4E938 => {
    //   block [0x82E4E938..0x82E4E9A8)
	// 82E4E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E94C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4E950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4E954: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4E958: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E95C: 4BFFFECD  bl 0x82e4e828
	ctx.lr = 0x82E4E960;
	sub_82E4E828(ctx, base);
	// 82E4E960: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4E964: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4E968: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4E96C: 4B471695  bl 0x822c0000
	ctx.lr = 0x82E4E970;
	sub_822C0000(ctx, base);
	// 82E4E970: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4E974: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4E978: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4E97C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4E980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4E984: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4E988: 419A0008  beq cr6, 0x82e4e990
	if ctx.cr[6].eq {
	pc = 0x82E4E990; continue 'dispatch;
	}
	// 82E4E98C: 4B471F05  bl 0x822c0890
	ctx.lr = 0x82E4E990;
	sub_822C0890(ctx, base);
	// 82E4E990: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4E994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4E998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4E99C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4E9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4E9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4E9A8 size=184
    let mut pc: u32 = 0x82E4E9A8;
    'dispatch: loop {
        match pc {
            0x82E4E9A8 => {
    //   block [0x82E4E9A8..0x82E4EA60)
	// 82E4E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4E9B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4E9B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4E9B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4E9BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4E9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4E9C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4E9C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4E9CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4E9D0: 4B471F69  bl 0x822c0938
	ctx.lr = 0x82E4E9D4;
	sub_822C0938(ctx, base);
	// 82E4E9D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4E9D8: 41820028  beq 0x82e4ea00
	if ctx.cr[0].eq {
	pc = 0x82E4EA00; continue 'dispatch;
	}
	// 82E4E9DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4E9E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4E9E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4E9E8: 392BC71C  addi r9, r11, -0x38e4
	ctx.r[9].s64 = ctx.r[11].s64 + -14564;
	// 82E4E9EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4E9F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4E9F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4E9F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4E9FC: 48000008  b 0x82e4ea04
	pc = 0x82E4EA04; continue 'dispatch;
	// 82E4EA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4EA04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4EA08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4EA0C: 409A0038  bne cr6, 0x82e4ea44
	if !ctx.cr[6].eq {
	pc = 0x82E4EA44; continue 'dispatch;
	}
	// 82E4EA10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4EA14: 419A0010  beq cr6, 0x82e4ea24
	if ctx.cr[6].eq {
	pc = 0x82E4EA24; continue 'dispatch;
	}
	// 82E4EA18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4EA1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EA20: 4BFFFEB9  bl 0x82e4e8d8
	ctx.lr = 0x82E4EA24;
	sub_82E4E8D8(ctx, base);
	// 82E4EA24: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4EA28: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4EA2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EA30: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4EA34: 816BAF9C  lwz r11, -0x5064(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20580 as u32) ) } as u64;
	// 82E4EA38: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4EA3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4EA40: 4B4715C1  bl 0x822c0000
	ctx.lr = 0x82E4EA44;
	sub_822C0000(ctx, base);
	// 82E4EA44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4EA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4EA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4EA54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4EA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4EA60 size=12
    let mut pc: u32 = 0x82E4EA60;
    'dispatch: loop {
        match pc {
            0x82E4EA60 => {
    //   block [0x82E4EA60..0x82E4EA6C)
	// 82E4EA60: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4EA64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EA68: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EA6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4EA6C size=8
    let mut pc: u32 = 0x82E4EA6C;
    'dispatch: loop {
        match pc {
            0x82E4EA6C => {
    //   block [0x82E4EA6C..0x82E4EA74)
	// 82E4EA6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E4EA70: 4BFFFE68  b 0x82e4e8d8
	sub_82E4E8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EA74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4EA74 size=4
    let mut pc: u32 = 0x82E4EA74;
    'dispatch: loop {
        match pc {
            0x82E4EA74 => {
    //   block [0x82E4EA74..0x82E4EA78)
	// 82E4EA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EA78 size=112
    let mut pc: u32 = 0x82E4EA78;
    'dispatch: loop {
        match pc {
            0x82E4EA78 => {
    //   block [0x82E4EA78..0x82E4EAE8)
	// 82E4EA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4EA80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4EA84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4EA88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EA8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EA90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EA94: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4EA98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EA9C: 4BFFFF0D  bl 0x82e4e9a8
	ctx.lr = 0x82E4EAA0;
	sub_82E4E9A8(ctx, base);
	// 82E4EAA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4EAA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4EAA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EAAC: 4B471555  bl 0x822c0000
	ctx.lr = 0x82E4EAB0;
	sub_822C0000(ctx, base);
	// 82E4EAB0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EAB4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4EAB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4EABC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4EAC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EAC4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4EAC8: 419A0008  beq cr6, 0x82e4ead0
	if ctx.cr[6].eq {
	pc = 0x82E4EAD0; continue 'dispatch;
	}
	// 82E4EACC: 4B471DC5  bl 0x822c0890
	ctx.lr = 0x82E4EAD0;
	sub_822C0890(ctx, base);
	// 82E4EAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4EAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4EADC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4EAE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4EAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4EAE8 size=824
    let mut pc: u32 = 0x82E4EAE8;
    'dispatch: loop {
        match pc {
            0x82E4EAE8 => {
    //   block [0x82E4EAE8..0x82E4EE20)
	// 82E4EAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EAEC: 4835966D  bl 0x831a8158
	ctx.lr = 0x82E4EAF0;
	sub_831A8130(ctx, base);
	// 82E4EAF0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EAF4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E4EAF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4EAFC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E4EB00: 4BFAFD69  bl 0x82dfe868
	ctx.lr = 0x82E4EB04;
	sub_82DFE868(ctx, base);
	// 82E4EB04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4EB08: 40820310  bne 0x82e4ee18
	if !ctx.cr[0].eq {
	pc = 0x82E4EE18; continue 'dispatch;
	}
	// 82E4EB0C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E4EB10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EB14: 48000F75  bl 0x82e4fa88
	ctx.lr = 0x82E4EB18;
	sub_82E4FA88(ctx, base);
	// 82E4EB18: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EB1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4EB20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EB24: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4EB28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4EB2C: 48000F5D  bl 0x82e4fa88
	ctx.lr = 0x82E4EB30;
	sub_82E4FA88(ctx, base);
	// 82E4EB30: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4EB34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4EB38: 41820224  beq 0x82e4ed5c
	if ctx.cr[0].eq {
	pc = 0x82E4ED5C; continue 'dispatch;
	}
	// 82E4EB3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82E4EB40: 3B78000C  addi r27, r24, 0xc
	ctx.r[27].s64 = ctx.r[24].s64 + 12;
	// 82E4EB44: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E4EB48: 3BABF018  addi r29, r11, -0xfe8
	ctx.r[29].s64 = ctx.r[11].s64 + -4072;
	// 82E4EB4C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E4EB50: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82E4EB54: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4EB58: 4B471DE1  bl 0x822c0938
	ctx.lr = 0x82E4EB5C;
	sub_822C0938(ctx, base);
	// 82E4EB5C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E4EB60: 4182001C  beq 0x82e4eb7c
	if ctx.cr[0].eq {
	pc = 0x82E4EB7C; continue 'dispatch;
	}
	// 82E4EB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EB68: 4BFA4589  bl 0x82df30f0
	ctx.lr = 0x82E4EB6C;
	sub_82DF30F0(ctx, base);
	// 82E4EB6C: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E4EB70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4EB74: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82E4EB78: 48000008  b 0x82e4eb80
	pc = 0x82E4EB80; continue 'dispatch;
	// 82E4EB7C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4EB80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4EB84: 4BFFFEF5  bl 0x82e4ea78
	ctx.lr = 0x82E4EB88;
	sub_82E4EA78(ctx, base);
	// 82E4EB88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EB8C: 48000EFD  bl 0x82e4fa88
	ctx.lr = 0x82E4EB90;
	sub_82E4FA88(ctx, base);
	// 82E4EB90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EB94: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EB98: 4BFA4559  bl 0x82df30f0
	ctx.lr = 0x82E4EB9C;
	sub_82DF30F0(ctx, base);
	// 82E4EB9C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E4EBA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4EBA4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EBA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EBAC: 4BFA5B05  bl 0x82df46b0
	ctx.lr = 0x82E4EBB0;
	sub_82DF46B0(ctx, base);
	// 82E4EBB0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EBB4: 57EB07BF  clrlwi. r11, r31, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4EBB8: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E4EBBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4EBC0: 41820018  beq 0x82e4ebd8
	if ctx.cr[0].eq {
	pc = 0x82E4EBD8; continue 'dispatch;
	}
	// 82E4EBC4: 216B0004  subfic r11, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[11].s64 = (4 as i64) - ctx.r[11].s64;
	// 82E4EBC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4EBCC: 419A000C  beq cr6, 0x82e4ebd8
	if ctx.cr[6].eq {
	pc = 0x82E4EBD8; continue 'dispatch;
	}
	// 82E4EBD0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4EBD4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4EBD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EBDC: 48000EAD  bl 0x82e4fa88
	ctx.lr = 0x82E4EBE0;
	sub_82E4FA88(ctx, base);
	// 82E4EBE0: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4EBE4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4EBE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EBEC: 48000E9D  bl 0x82e4fa88
	ctx.lr = 0x82E4EBF0;
	sub_82E4FA88(ctx, base);
	// 82E4EBF0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4EBF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4EBF8: 4BFA44F9  bl 0x82df30f0
	ctx.lr = 0x82E4EBFC;
	sub_82DF30F0(ctx, base);
	// 82E4EBFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4EC00: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EC04: 4BFA53C5  bl 0x82df3fc8
	ctx.lr = 0x82E4EC08;
	sub_82DF3FC8(ctx, base);
	// 82E4EC08: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82E4EC0C: 409A000C  bne cr6, 0x82e4ec18
	if !ctx.cr[6].eq {
	pc = 0x82E4EC18; continue 'dispatch;
	}
	// 82E4EC10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EC14: 4BFA4E7D  bl 0x82df3a90
	ctx.lr = 0x82E4EC18;
	sub_82DF3A90(ctx, base);
	// 82E4EC18: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E4EC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E4EC20: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E4EC24: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E4EC28: 4BFA5941  bl 0x82df4568
	ctx.lr = 0x82E4EC2C;
	sub_82DF4568(ctx, base);
	// 82E4EC2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4EC30: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4EC34: 4BFA4F9D  bl 0x82df3bd0
	ctx.lr = 0x82E4EC38;
	sub_82DF3BD0(ctx, base);
	// 82E4EC38: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E4EC3C: 4BFA47ED  bl 0x82df3428
	ctx.lr = 0x82E4EC40;
	sub_82DF3428(ctx, base);
	// 82E4EC40: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E4EC44: 4801F3B5  bl 0x82e6dff8
	ctx.lr = 0x82E4EC48;
	sub_82E6DFF8(ctx, base);
	// 82E4EC48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4EC4C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E4EC50: 4BFDBE99  bl 0x82e2aae8
	ctx.lr = 0x82E4EC54;
	sub_82E2AAE8(ctx, base);
	// 82E4EC54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E4EC58: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E4EC5C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E4EC60: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E4EC64: 4BFDD8A5  bl 0x82e2c508
	ctx.lr = 0x82E4EC68;
	sub_82E2C508(ctx, base);
	// 82E4EC68: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E4EC6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EC70: 419A0008  beq cr6, 0x82e4ec78
	if ctx.cr[6].eq {
	pc = 0x82E4EC78; continue 'dispatch;
	}
	// 82E4EC74: 4B471C1D  bl 0x822c0890
	ctx.lr = 0x82E4EC78;
	sub_822C0890(ctx, base);
	// 82E4EC78: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82E4EC7C: 4BFA3745  bl 0x82df23c0
	ctx.lr = 0x82E4EC80;
	sub_82DF23C0(ctx, base);
	// 82E4EC80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4EC84: 41820010  beq 0x82e4ec94
	if ctx.cr[0].eq {
	pc = 0x82E4EC94; continue 'dispatch;
	}
	// 82E4EC88: 4B4968C1  bl 0x822e5548
	ctx.lr = 0x82E4EC8C;
	sub_822E5548(ctx, base);
	// 82E4EC8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4EC90: 48000008  b 0x82e4ec98
	pc = 0x82E4EC98; continue 'dispatch;
	// 82E4EC94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4EC98: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82E4EC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4ECA0: 4BBA10F1  bl 0x829efd90
	ctx.lr = 0x82E4ECA4;
	sub_829EFD90(ctx, base);
	// 82E4ECA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E4ECA8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E4ECAC: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E4ECB0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E4ECB4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ECB8: 4BFDCF29  bl 0x82e2bbe0
	ctx.lr = 0x82E4ECBC;
	sub_82E2BBE0(ctx, base);
	// 82E4ECBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4ECC0: 395E000C  addi r10, r30, 0xc
	ctx.r[10].s64 = ctx.r[30].s64 + 12;
	// 82E4ECC4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E4ECC8: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E4ECCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ECD0: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4ECD4: 4B47578D  bl 0x822c4460
	ctx.lr = 0x82E4ECD8;
	sub_822C4460(ctx, base);
	// 82E4ECD8: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E4ECDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4ECE0: 419A0008  beq cr6, 0x82e4ece8
	if ctx.cr[6].eq {
	pc = 0x82E4ECE8; continue 'dispatch;
	}
	// 82E4ECE4: 4B471BAD  bl 0x822c0890
	ctx.lr = 0x82E4ECE8;
	sub_822C0890(ctx, base);
	// 82E4ECE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ECEC: 938B0018  stw r28, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82E4ECF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ECF4: 938B001C  stw r28, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82E4ECF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ECFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4ED00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4ED04: 419A0014  beq cr6, 0x82e4ed18
	if ctx.cr[6].eq {
	pc = 0x82E4ED18; continue 'dispatch;
	}
	// 82E4ED08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ED0C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82E4ED10: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4ED14: 48278DCD  bl 0x830c7ae0
	ctx.lr = 0x82E4ED18;
	sub_830C7AE0(ctx, base);
	// 82E4ED18: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4ED1C: 4BFAFB3D  bl 0x82dfe858
	ctx.lr = 0x82E4ED20;
	sub_82DFE858(ctx, base);
	// 82E4ED20: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E4ED24: 4BFDBDDD  bl 0x82e2ab00
	ctx.lr = 0x82E4ED28;
	sub_82E2AB00(ctx, base);
	// 82E4ED28: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4ED2C: 4BFA46FD  bl 0x82df3428
	ctx.lr = 0x82E4ED30;
	sub_82DF3428(ctx, base);
	// 82E4ED30: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E4ED34: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E4ED38: 4BFD97F1  bl 0x82e28528
	ctx.lr = 0x82E4ED3C;
	sub_82E28528(ctx, base);
	// 82E4ED3C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4ED40: 4BFA46E9  bl 0x82df3428
	ctx.lr = 0x82E4ED44;
	sub_82DF3428(ctx, base);
	// 82E4ED44: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E4ED48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4ED4C: 419A0008  beq cr6, 0x82e4ed54
	if ctx.cr[6].eq {
	pc = 0x82E4ED54; continue 'dispatch;
	}
	// 82E4ED50: 4B471B41  bl 0x822c0890
	ctx.lr = 0x82E4ED54;
	sub_822C0890(ctx, base);
	// 82E4ED54: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82E4ED58: 4082FDF4  bne 0x82e4eb4c
	if !ctx.cr[0].eq {
	pc = 0x82E4EB4C; continue 'dispatch;
	}
	// 82E4ED5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4ED60: 48000D29  bl 0x82e4fa88
	ctx.lr = 0x82E4ED64;
	sub_82E4FA88(ctx, base);
	// 82E4ED64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4ED68: 418200A8  beq 0x82e4ee10
	if ctx.cr[0].eq {
	pc = 0x82E4EE10; continue 'dispatch;
	}
	// 82E4ED6C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4ED70: 3BD8001C  addi r30, r24, 0x1c
	ctx.r[30].s64 = ctx.r[24].s64 + 28;
	// 82E4ED74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4ED78: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82E4ED7C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E4ED80: 93810060  stw r28, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82E4ED84: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82E4ED88: 4B471BB1  bl 0x822c0938
	ctx.lr = 0x82E4ED8C;
	sub_822C0938(ctx, base);
	// 82E4ED8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4ED90: 41820020  beq 0x82e4edb0
	if ctx.cr[0].eq {
	pc = 0x82E4EDB0; continue 'dispatch;
	}
	// 82E4ED94: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E4ED98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4ED9C: 93830004  stw r28, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E4EDA0: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E4EDA4: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E4EDA8: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82E4EDAC: 48000008  b 0x82e4edb4
	pc = 0x82E4EDB4; continue 'dispatch;
	// 82E4EDB0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E4EDB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4EDB8: 4BFFFB81  bl 0x82e4e938
	ctx.lr = 0x82E4EDBC;
	sub_82E4E938(ctx, base);
	// 82E4EDBC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4EDC0: 815FFFF8  lwz r10, -8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EDC4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E4EDC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4EDCC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4EDD0: C01FFFFC  lfs f0, -4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EDD4: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E4EDD8: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EDDC: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4EDE0: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EDE4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4EDE8: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4EDEC: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E4EDF0: 4BFD9739  bl 0x82e28528
	ctx.lr = 0x82E4EDF4;
	sub_82E28528(ctx, base);
	// 82E4EDF4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E4EDF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EDFC: 419A0008  beq cr6, 0x82e4ee04
	if ctx.cr[6].eq {
	pc = 0x82E4EE04; continue 'dispatch;
	}
	// 82E4EE00: 4B471A91  bl 0x822c0890
	ctx.lr = 0x82E4EE04;
	sub_822C0890(ctx, base);
	// 82E4EE04: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E4EE08: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 82E4EE0C: 4082FF70  bne 0x82e4ed7c
	if !ctx.cr[0].eq {
	pc = 0x82E4ED7C; continue 'dispatch;
	}
	// 82E4EE10: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E4EE14: 4BFAFA45  bl 0x82dfe858
	ctx.lr = 0x82E4EE18;
	sub_82DFE858(ctx, base);
	// 82E4EE18: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E4EE1C: 4835938C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EE20 size=172
    let mut pc: u32 = 0x82E4EE20;
    'dispatch: loop {
        match pc {
            0x82E4EE20 => {
    //   block [0x82E4EE20..0x82E4EECC)
	// 82E4EE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4EE28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4EE2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4EE30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EE34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4EE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4EE3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4EE40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4EE44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4EE48: 4B471AF1  bl 0x822c0938
	ctx.lr = 0x82E4EE4C;
	sub_822C0938(ctx, base);
	// 82E4EE4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4EE50: 41820028  beq 0x82e4ee78
	if ctx.cr[0].eq {
	pc = 0x82E4EE78; continue 'dispatch;
	}
	// 82E4EE54: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4EE58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4EE5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4EE60: 392BC73C  addi r9, r11, -0x38c4
	ctx.r[9].s64 = ctx.r[11].s64 + -14532;
	// 82E4EE64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4EE68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4EE6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4EE70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4EE74: 48000008  b 0x82e4ee7c
	pc = 0x82E4EE7C; continue 'dispatch;
	// 82E4EE78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4EE7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4EE80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4EE84: 409A002C  bne cr6, 0x82e4eeb0
	if !ctx.cr[6].eq {
	pc = 0x82E4EEB0; continue 'dispatch;
	}
	// 82E4EE88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EE8C: 4B4713DD  bl 0x822c0268
	ctx.lr = 0x82E4EE90;
	sub_822C0268(ctx, base);
	// 82E4EE90: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4EE94: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4EE98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4EE9C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4EEA0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4EEA4: 816BAFA0  lwz r11, -0x5060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20576 as u32) ) } as u64;
	// 82E4EEA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4EEAC: 4B471155  bl 0x822c0000
	ctx.lr = 0x82E4EEB0;
	sub_822C0000(ctx, base);
	// 82E4EEB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4EEB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4EEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4EEC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4EEC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4EEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EED0 size=88
    let mut pc: u32 = 0x82E4EED0;
    'dispatch: loop {
        match pc {
            0x82E4EED0 => {
    //   block [0x82E4EED0..0x82E4EF28)
	// 82E4EED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4EED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4EEDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4EEE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EEE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EEE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EEEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4EEF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EEF4: 419A0008  beq cr6, 0x82e4eefc
	if ctx.cr[6].eq {
	pc = 0x82E4EEFC; continue 'dispatch;
	}
	// 82E4EEF8: 4B471999  bl 0x822c0890
	ctx.lr = 0x82E4EEFC;
	sub_822C0890(ctx, base);
	// 82E4EEFC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4EF00: 4182000C  beq 0x82e4ef0c
	if ctx.cr[0].eq {
	pc = 0x82E4EF0C; continue 'dispatch;
	}
	// 82E4EF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EF08: 4B471361  bl 0x822c0268
	ctx.lr = 0x82E4EF0C;
	sub_822C0268(ctx, base);
	// 82E4EF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EF10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4EF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4EF1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4EF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4EF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EF28 size=112
    let mut pc: u32 = 0x82E4EF28;
    'dispatch: loop {
        match pc {
            0x82E4EF28 => {
    //   block [0x82E4EF28..0x82E4EF98)
	// 82E4EF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4EF30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4EF34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4EF38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EF3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EF40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4EF44: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4EF48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EF4C: 4BFFFED5  bl 0x82e4ee20
	ctx.lr = 0x82E4EF50;
	sub_82E4EE20(ctx, base);
	// 82E4EF50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4EF54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4EF58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4EF5C: 4B4710A5  bl 0x822c0000
	ctx.lr = 0x82E4EF60;
	sub_822C0000(ctx, base);
	// 82E4EF60: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4EF64: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4EF68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4EF6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4EF70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4EF74: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4EF78: 419A0008  beq cr6, 0x82e4ef80
	if ctx.cr[6].eq {
	pc = 0x82E4EF80; continue 'dispatch;
	}
	// 82E4EF7C: 4B471915  bl 0x822c0890
	ctx.lr = 0x82E4EF80;
	sub_822C0890(ctx, base);
	// 82E4EF80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4EF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4EF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4EF8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4EF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4EF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EF98 size=88
    let mut pc: u32 = 0x82E4EF98;
    'dispatch: loop {
        match pc {
            0x82E4EF98 => {
    //   block [0x82E4EF98..0x82E4EFF0)
	// 82E4EF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EF9C: 483591CD  bl 0x831a8168
	ctx.lr = 0x82E4EFA0;
	sub_831A8130(ctx, base);
	// 82E4EFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4EFA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E4EFA8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E4EFAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4EFB0: 7F1D2040  cmplw cr6, r29, r4
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E4EFB4: 419A0030  beq cr6, 0x82e4efe4
	if ctx.cr[6].eq {
	pc = 0x82E4EFE4; continue 'dispatch;
	}
	// 82E4EFB8: 7F9F2050  subf r28, r31, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[31].s64;
	// 82E4EFBC: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 82E4EFC0: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82E4EFC4: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82E4EFC8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E4EFCC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4EFD0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E4EFD4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4EFD8: 4B475489  bl 0x822c4460
	ctx.lr = 0x82E4EFDC;
	sub_822C4460(ctx, base);
	// 82E4EFDC: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E4EFE0: 409AFFDC  bne cr6, 0x82e4efbc
	if !ctx.cr[6].eq {
	pc = 0x82E4EFBC; continue 'dispatch;
	}
	// 82E4EFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4EFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4EFEC: 483591CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4EFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4EFF0 size=84
    let mut pc: u32 = 0x82E4EFF0;
    'dispatch: loop {
        match pc {
            0x82E4EFF0 => {
    //   block [0x82E4EFF0..0x82E4F044)
	// 82E4EFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4EFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4EFF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4EFFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F004: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F00C: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4F010: 419A001C  beq cr6, 0x82e4f02c
	if ctx.cr[6].eq {
	pc = 0x82E4F02C; continue 'dispatch;
	}
	// 82E4F014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4F018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F01C: 4BFFFEB5  bl 0x82e4eed0
	ctx.lr = 0x82E4F020;
	sub_82E4EED0(ctx, base);
	// 82E4F020: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E4F024: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E4F028: 409AFFEC  bne cr6, 0x82e4f014
	if !ctx.cr[6].eq {
	pc = 0x82E4F014; continue 'dispatch;
	}
	// 82E4F02C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F038: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F03C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F048 size=100
    let mut pc: u32 = 0x82E4F048;
    'dispatch: loop {
        match pc {
            0x82E4F048 => {
    //   block [0x82E4F048..0x82E4F0AC)
	// 82E4F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F05C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F060: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E4F064: 419A0030  beq cr6, 0x82e4f094
	if ctx.cr[6].eq {
	pc = 0x82E4F094; continue 'dispatch;
	}
	// 82E4F068: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4F06C: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82E4F070: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F074: 419A0008  beq cr6, 0x82e4f07c
	if ctx.cr[6].eq {
	pc = 0x82E4F07C; continue 'dispatch;
	}
	// 82E4F078: 4B4711F1  bl 0x822c0268
	ctx.lr = 0x82E4F07C;
	sub_822C0268(ctx, base);
	// 82E4F07C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F080: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4F084: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4F088: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4F08C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4F090: 4B4711D9  bl 0x822c0268
	ctx.lr = 0x82E4F094;
	sub_822C0268(ctx, base);
	// 82E4F094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F0A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F0A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F0B0 size=172
    let mut pc: u32 = 0x82E4F0B0;
    'dispatch: loop {
        match pc {
            0x82E4F0B0 => {
    //   block [0x82E4F0B0..0x82E4F15C)
	// 82E4F0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F0B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F0BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F0C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F0C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F0CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4F0D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4F0D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F0D8: 4B471861  bl 0x822c0938
	ctx.lr = 0x82E4F0DC;
	sub_822C0938(ctx, base);
	// 82E4F0DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F0E0: 41820028  beq 0x82e4f108
	if ctx.cr[0].eq {
	pc = 0x82E4F108; continue 'dispatch;
	}
	// 82E4F0E4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4F0E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4F0EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4F0F0: 392BC74C  addi r9, r11, -0x38b4
	ctx.r[9].s64 = ctx.r[11].s64 + -14516;
	// 82E4F0F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F0F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4F0FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4F100: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4F104: 48000008  b 0x82e4f10c
	pc = 0x82E4F10C; continue 'dispatch;
	// 82E4F108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F10C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F114: 409A002C  bne cr6, 0x82e4f140
	if !ctx.cr[6].eq {
	pc = 0x82E4F140; continue 'dispatch;
	}
	// 82E4F118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F11C: 4BFFFF2D  bl 0x82e4f048
	ctx.lr = 0x82E4F120;
	sub_82E4F048(ctx, base);
	// 82E4F120: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4F124: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4F128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F12C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4F130: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4F134: 816BAFA0  lwz r11, -0x5060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20576 as u32) ) } as u64;
	// 82E4F138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4F13C: 4B470EC5  bl 0x822c0000
	ctx.lr = 0x82E4F140;
	sub_822C0000(ctx, base);
	// 82E4F140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4F144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4F160 size=8
    let mut pc: u32 = 0x82E4F160;
    'dispatch: loop {
        match pc {
            0x82E4F160 => {
    //   block [0x82E4F160..0x82E4F168)
	// 82E4F160: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4F164: 4BFFFEE4  b 0x82e4f048
	sub_82E4F048(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F168 size=112
    let mut pc: u32 = 0x82E4F168;
    'dispatch: loop {
        match pc {
            0x82E4F168 => {
    //   block [0x82E4F168..0x82E4F1D8)
	// 82E4F168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F184: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4F188: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F18C: 4BFFFF25  bl 0x82e4f0b0
	ctx.lr = 0x82E4F190;
	sub_82E4F0B0(ctx, base);
	// 82E4F190: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4F194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4F198: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F19C: 4B470E65  bl 0x822c0000
	ctx.lr = 0x82E4F1A0;
	sub_822C0000(ctx, base);
	// 82E4F1A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F1A4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4F1A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F1AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F1B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F1B4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F1B8: 419A0008  beq cr6, 0x82e4f1c0
	if ctx.cr[6].eq {
	pc = 0x82E4F1C0; continue 'dispatch;
	}
	// 82E4F1BC: 4B4716D5  bl 0x822c0890
	ctx.lr = 0x82E4F1C0;
	sub_822C0890(ctx, base);
	// 82E4F1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F1CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F1D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F1D8 size=104
    let mut pc: u32 = 0x82E4F1D8;
    'dispatch: loop {
        match pc {
            0x82E4F1D8 => {
    //   block [0x82E4F1D8..0x82E4F240)
	// 82E4F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F1E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F1E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F1EC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E4F1F0: 4BFA4239  bl 0x82df3428
	ctx.lr = 0x82E4F1F4;
	sub_82DF3428(ctx, base);
	// 82E4F1F4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E4F1F8: 4BFDDBC9  bl 0x82e2cdc0
	ctx.lr = 0x82E4F1FC;
	sub_82E2CDC0(ctx, base);
	// 82E4F1FC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E4F200: 4BFDDBC1  bl 0x82e2cdc0
	ctx.lr = 0x82E4F204;
	sub_82E2CDC0(ctx, base);
	// 82E4F204: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E4F208: 4BFDDBB9  bl 0x82e2cdc0
	ctx.lr = 0x82E4F20C;
	sub_82E2CDC0(ctx, base);
	// 82E4F20C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F214: 419A0008  beq cr6, 0x82e4f21c
	if ctx.cr[6].eq {
	pc = 0x82E4F21C; continue 'dispatch;
	}
	// 82E4F218: 4B471051  bl 0x822c0268
	ctx.lr = 0x82E4F21C;
	sub_822C0268(ctx, base);
	// 82E4F21C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F220: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4F224: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4F228: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4F22C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4F230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F240 size=104
    let mut pc: u32 = 0x82E4F240;
    'dispatch: loop {
        match pc {
            0x82E4F240 => {
    //   block [0x82E4F240..0x82E4F2A8)
	// 82E4F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F24C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F258: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E4F25C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E4F260: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E4F264: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E4F268: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E4F26C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E4F270: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E4F274: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E4F278: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E4F27C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E4F280: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E4F284: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E4F288: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E4F28C: 4BFA3E65  bl 0x82df30f0
	ctx.lr = 0x82E4F290;
	sub_82DF30F0(ctx, base);
	// 82E4F290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4F298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F2A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F2A8 size=196
    let mut pc: u32 = 0x82E4F2A8;
    'dispatch: loop {
        match pc {
            0x82E4F2A8 => {
    //   block [0x82E4F2A8..0x82E4F36C)
	// 82E4F2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F2B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F2B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F2B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F2BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F2C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4F2C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4F2CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F2D0: 4B471669  bl 0x822c0938
	ctx.lr = 0x82E4F2D4;
	sub_822C0938(ctx, base);
	// 82E4F2D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F2D8: 41820028  beq 0x82e4f300
	if ctx.cr[0].eq {
	pc = 0x82E4F300; continue 'dispatch;
	}
	// 82E4F2DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4F2E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4F2E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4F2E8: 392BC75C  addi r9, r11, -0x38a4
	ctx.r[9].s64 = ctx.r[11].s64 + -14500;
	// 82E4F2EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F2F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4F2F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4F2F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4F2FC: 48000008  b 0x82e4f304
	pc = 0x82E4F304; continue 'dispatch;
	// 82E4F300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F304: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F30C: 409A0044  bne cr6, 0x82e4f350
	if !ctx.cr[6].eq {
	pc = 0x82E4F350; continue 'dispatch;
	}
	// 82E4F310: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4F314: 419A001C  beq cr6, 0x82e4f330
	if ctx.cr[6].eq {
	pc = 0x82E4F330; continue 'dispatch;
	}
	// 82E4F318: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E4F31C: 4BFA410D  bl 0x82df3428
	ctx.lr = 0x82E4F320;
	sub_82DF3428(ctx, base);
	// 82E4F320: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E4F324: 4BFDDA9D  bl 0x82e2cdc0
	ctx.lr = 0x82E4F328;
	sub_82E2CDC0(ctx, base);
	// 82E4F328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F32C: 4B470F3D  bl 0x822c0268
	ctx.lr = 0x82E4F330;
	sub_822C0268(ctx, base);
	// 82E4F330: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4F334: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4F338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F33C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4F340: 816BAFA0  lwz r11, -0x5060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20576 as u32) ) } as u64;
	// 82E4F344: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4F348: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4F34C: 4B470CB5  bl 0x822c0000
	ctx.lr = 0x82E4F350;
	sub_822C0000(ctx, base);
	// 82E4F350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4F354: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F370 size=72
    let mut pc: u32 = 0x82E4F370;
    'dispatch: loop {
        match pc {
            0x82E4F370 => {
    //   block [0x82E4F370..0x82E4F3B8)
	// 82E4F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F380: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4F384: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4F388: 419A001C  beq cr6, 0x82e4f3a4
	if ctx.cr[6].eq {
	pc = 0x82E4F3A4; continue 'dispatch;
	}
	// 82E4F38C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E4F390: 4BFA4099  bl 0x82df3428
	ctx.lr = 0x82E4F394;
	sub_82DF3428(ctx, base);
	// 82E4F394: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E4F398: 4BFDDA29  bl 0x82e2cdc0
	ctx.lr = 0x82E4F39C;
	sub_82E2CDC0(ctx, base);
	// 82E4F39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F3A0: 4B470EC9  bl 0x822c0268
	ctx.lr = 0x82E4F3A4;
	sub_822C0268(ctx, base);
	// 82E4F3A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4F3A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F3AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F3B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F3B8 size=188
    let mut pc: u32 = 0x82E4F3B8;
    'dispatch: loop {
        match pc {
            0x82E4F3B8 => {
    //   block [0x82E4F3B8..0x82E4F474)
	// 82E4F3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F3C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F3C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F3C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F3CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F3D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4F3D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E4F3DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F3E0: 4B471559  bl 0x822c0938
	ctx.lr = 0x82E4F3E4;
	sub_822C0938(ctx, base);
	// 82E4F3E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F3E8: 41820028  beq 0x82e4f410
	if ctx.cr[0].eq {
	pc = 0x82E4F410; continue 'dispatch;
	}
	// 82E4F3EC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4F3F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E4F3F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E4F3F8: 392BC72C  addi r9, r11, -0x38d4
	ctx.r[9].s64 = ctx.r[11].s64 + -14548;
	// 82E4F3FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F400: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4F404: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E4F408: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E4F40C: 48000008  b 0x82e4f414
	pc = 0x82E4F414; continue 'dispatch;
	// 82E4F410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4F414: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F41C: 409A003C  bne cr6, 0x82e4f458
	if !ctx.cr[6].eq {
	pc = 0x82E4F458; continue 'dispatch;
	}
	// 82E4F420: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4F424: 419A0014  beq cr6, 0x82e4f438
	if ctx.cr[6].eq {
	pc = 0x82E4F438; continue 'dispatch;
	}
	// 82E4F428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F42C: 4BFFFDAD  bl 0x82e4f1d8
	ctx.lr = 0x82E4F430;
	sub_82E4F1D8(ctx, base);
	// 82E4F430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F434: 4B470E35  bl 0x822c0268
	ctx.lr = 0x82E4F438;
	sub_822C0268(ctx, base);
	// 82E4F438: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E4F43C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E4F440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F444: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E4F448: 816BAFA0  lwz r11, -0x5060(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20576 as u32) ) } as u64;
	// 82E4F44C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4F450: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E4F454: 4B470BAD  bl 0x822c0000
	ctx.lr = 0x82E4F458;
	sub_822C0000(ctx, base);
	// 82E4F458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4F45C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F46C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F478 size=64
    let mut pc: u32 = 0x82E4F478;
    'dispatch: loop {
        match pc {
            0x82E4F478 => {
    //   block [0x82E4F478..0x82E4F4B8)
	// 82E4F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F488: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4F48C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E4F490: 419A0014  beq cr6, 0x82e4f4a4
	if ctx.cr[6].eq {
	pc = 0x82E4F4A4; continue 'dispatch;
	}
	// 82E4F494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F498: 4BFFFD41  bl 0x82e4f1d8
	ctx.lr = 0x82E4F49C;
	sub_82E4F1D8(ctx, base);
	// 82E4F49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F4A0: 4B470DC9  bl 0x822c0268
	ctx.lr = 0x82E4F4A4;
	sub_822C0268(ctx, base);
	// 82E4F4A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4F4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F4B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F4B8 size=112
    let mut pc: u32 = 0x82E4F4B8;
    'dispatch: loop {
        match pc {
            0x82E4F4B8 => {
    //   block [0x82E4F4B8..0x82E4F528)
	// 82E4F4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F4C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F4C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F4CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F4D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F4D4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4F4D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F4DC: 4BFFFDCD  bl 0x82e4f2a8
	ctx.lr = 0x82E4F4E0;
	sub_82E4F2A8(ctx, base);
	// 82E4F4E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4F4E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4F4E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F4EC: 4B470B15  bl 0x822c0000
	ctx.lr = 0x82E4F4F0;
	sub_822C0000(ctx, base);
	// 82E4F4F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F4F4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4F4F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F4FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F500: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F504: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F508: 419A0008  beq cr6, 0x82e4f510
	if ctx.cr[6].eq {
	pc = 0x82E4F510; continue 'dispatch;
	}
	// 82E4F50C: 4B471385  bl 0x822c0890
	ctx.lr = 0x82E4F510;
	sub_822C0890(ctx, base);
	// 82E4F510: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F51C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F528 size=112
    let mut pc: u32 = 0x82E4F528;
    'dispatch: loop {
        match pc {
            0x82E4F528 => {
    //   block [0x82E4F528..0x82E4F598)
	// 82E4F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4F530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4F534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4F538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F53C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F544: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4F548: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F54C: 4BFFFE6D  bl 0x82e4f3b8
	ctx.lr = 0x82E4F550;
	sub_82E4F3B8(ctx, base);
	// 82E4F550: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E4F554: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4F558: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E4F55C: 4B470AA5  bl 0x822c0000
	ctx.lr = 0x82E4F560;
	sub_822C0000(ctx, base);
	// 82E4F560: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F564: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E4F568: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F56C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4F570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F574: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E4F578: 419A0008  beq cr6, 0x82e4f580
	if ctx.cr[6].eq {
	pc = 0x82E4F580; continue 'dispatch;
	}
	// 82E4F57C: 4B471315  bl 0x822c0890
	ctx.lr = 0x82E4F580;
	sub_822C0890(ctx, base);
	// 82E4F580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4F584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4F588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4F58C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4F590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4F594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F598 size=772
    let mut pc: u32 = 0x82E4F598;
    'dispatch: loop {
        match pc {
            0x82E4F598 => {
    //   block [0x82E4F598..0x82E4F89C)
	// 82E4F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F59C: 48358BC5  bl 0x831a8160
	ctx.lr = 0x82E4F5A0;
	sub_831A8130(ctx, base);
	// 82E4F5A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F5A4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E4F5A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4F5AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4F5B0: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82E4F5B4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F5B8: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E4F5BC: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E4F5C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4F5C4: 4B471375  bl 0x822c0938
	ctx.lr = 0x82E4F5C8;
	sub_822C0938(ctx, base);
	// 82E4F5C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F5CC: 41820010  beq 0x82e4f5dc
	if ctx.cr[0].eq {
	pc = 0x82E4F5DC; continue 'dispatch;
	}
	// 82E4F5D0: 4BFFFC71  bl 0x82e4f240
	ctx.lr = 0x82E4F5D4;
	sub_82E4F240(ctx, base);
	// 82E4F5D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4F5D8: 48000008  b 0x82e4f5e0
	pc = 0x82E4F5E0; continue 'dispatch;
	// 82E4F5DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4F5E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4F5E4: 4BFFFF45  bl 0x82e4f528
	ctx.lr = 0x82E4F5E8;
	sub_82E4F528(ctx, base);
	// 82E4F5E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F5EC: 4800049D  bl 0x82e4fa88
	ctx.lr = 0x82E4F5F0;
	sub_82E4FA88(ctx, base);
	// 82E4F5F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F5F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F5F8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F5FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4F600: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E4F604: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82E4F608: 4BFA50A9  bl 0x82df46b0
	ctx.lr = 0x82E4F60C;
	sub_82DF46B0(ctx, base);
	// 82E4F60C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F610: 57EB07BF  clrlwi. r11, r31, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4F614: 7D5F5214  add r10, r31, r10
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[10].u64;
	// 82E4F618: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4F61C: 41820018  beq 0x82e4f634
	if ctx.cr[0].eq {
	pc = 0x82E4F634; continue 'dispatch;
	}
	// 82E4F620: 216B0004  subfic r11, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[11].s64 = (4 as i64) - ctx.r[11].s64;
	// 82E4F624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F628: 419A000C  beq cr6, 0x82e4f634
	if ctx.cr[6].eq {
	pc = 0x82E4F634; continue 'dispatch;
	}
	// 82E4F62C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4F630: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4F634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F638: 48000451  bl 0x82e4fa88
	ctx.lr = 0x82E4F63C;
	sub_82E4FA88(ctx, base);
	// 82E4F63C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F640: 41820028  beq 0x82e4f668
	if ctx.cr[0].eq {
	pc = 0x82E4F668; continue 'dispatch;
	}
	// 82E4F644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F64C: 4800043D  bl 0x82e4fa88
	ctx.lr = 0x82E4F650;
	sub_82E4FA88(ctx, base);
	// 82E4F650: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E4F654: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4F658: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F65C: 4BFE4935  bl 0x82e33f90
	ctx.lr = 0x82E4F660;
	sub_82E33F90(ctx, base);
	// 82E4F660: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E4F664: 4082FFE4  bne 0x82e4f648
	if !ctx.cr[0].eq {
	pc = 0x82E4F648; continue 'dispatch;
	}
	// 82E4F668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F66C: 4800041D  bl 0x82e4fa88
	ctx.lr = 0x82E4F670;
	sub_82E4FA88(ctx, base);
	// 82E4F670: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F674: 418200A0  beq 0x82e4f714
	if ctx.cr[0].eq {
	pc = 0x82E4F714; continue 'dispatch;
	}
	// 82E4F678: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F67C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4F680: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82E4F684: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E4F688: 4B4712B1  bl 0x822c0938
	ctx.lr = 0x82E4F68C;
	sub_822C0938(ctx, base);
	// 82E4F68C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F690: 4182001C  beq 0x82e4f6ac
	if ctx.cr[0].eq {
	pc = 0x82E4F6AC; continue 'dispatch;
	}
	// 82E4F694: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E4F698: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4F69C: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E4F6A0: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E4F6A4: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E4F6A8: 48000008  b 0x82e4f6b0
	pc = 0x82E4F6B0; continue 'dispatch;
	// 82E4F6AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4F6B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4F6B4: 4BFFF875  bl 0x82e4ef28
	ctx.lr = 0x82E4F6B8;
	sub_82E4EF28(ctx, base);
	// 82E4F6B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F6BC: 480003CD  bl 0x82e4fa88
	ctx.lr = 0x82E4F6C0;
	sub_82E4FA88(ctx, base);
	// 82E4F6C0: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4F6C4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E4F6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F6CC: 480003BD  bl 0x82e4fa88
	ctx.lr = 0x82E4F6D0;
	sub_82E4FA88(ctx, base);
	// 82E4F6D0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4F6D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F6D8: 480003B1  bl 0x82e4fa88
	ctx.lr = 0x82E4F6DC;
	sub_82E4FA88(ctx, base);
	// 82E4F6DC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4F6E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F6E4: 480003A5  bl 0x82e4fa88
	ctx.lr = 0x82E4F6E8;
	sub_82E4FA88(ctx, base);
	// 82E4F6E8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82E4F6EC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F6F0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4F6F4: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82E4F6F8: 4BFE9BB1  bl 0x82e392a8
	ctx.lr = 0x82E4F6FC;
	sub_82E392A8(ctx, base);
	// 82E4F6FC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4F700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F704: 419A0008  beq cr6, 0x82e4f70c
	if ctx.cr[6].eq {
	pc = 0x82E4F70C; continue 'dispatch;
	}
	// 82E4F708: 4B471189  bl 0x822c0890
	ctx.lr = 0x82E4F70C;
	sub_822C0890(ctx, base);
	// 82E4F70C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4F710: 4082FF6C  bne 0x82e4f67c
	if !ctx.cr[0].eq {
	pc = 0x82E4F67C; continue 'dispatch;
	}
	// 82E4F714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F718: 48000371  bl 0x82e4fa88
	ctx.lr = 0x82E4F71C;
	sub_82E4FA88(ctx, base);
	// 82E4F71C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F720: 418200A0  beq 0x82e4f7c0
	if ctx.cr[0].eq {
	pc = 0x82E4F7C0; continue 'dispatch;
	}
	// 82E4F724: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F728: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E4F72C: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82E4F730: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E4F734: 4B471205  bl 0x822c0938
	ctx.lr = 0x82E4F738;
	sub_822C0938(ctx, base);
	// 82E4F738: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F73C: 4182001C  beq 0x82e4f758
	if ctx.cr[0].eq {
	pc = 0x82E4F758; continue 'dispatch;
	}
	// 82E4F740: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E4F744: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4F748: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E4F74C: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E4F750: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E4F754: 48000008  b 0x82e4f75c
	pc = 0x82E4F75C; continue 'dispatch;
	// 82E4F758: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4F75C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4F760: 4BFFF7C9  bl 0x82e4ef28
	ctx.lr = 0x82E4F764;
	sub_82E4EF28(ctx, base);
	// 82E4F764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F768: 48000321  bl 0x82e4fa88
	ctx.lr = 0x82E4F76C;
	sub_82E4FA88(ctx, base);
	// 82E4F76C: 83E10058  lwz r31, 0x58(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E4F770: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E4F774: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F778: 48000311  bl 0x82e4fa88
	ctx.lr = 0x82E4F77C;
	sub_82E4FA88(ctx, base);
	// 82E4F77C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4F780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F784: 48000305  bl 0x82e4fa88
	ctx.lr = 0x82E4F788;
	sub_82E4FA88(ctx, base);
	// 82E4F788: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E4F78C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F790: 480002F9  bl 0x82e4fa88
	ctx.lr = 0x82E4F794;
	sub_82E4FA88(ctx, base);
	// 82E4F794: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82E4F798: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F79C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4F7A0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E4F7A4: 4BFE9B05  bl 0x82e392a8
	ctx.lr = 0x82E4F7A8;
	sub_82E392A8(ctx, base);
	// 82E4F7A8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4F7AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F7B0: 419A0008  beq cr6, 0x82e4f7b8
	if ctx.cr[6].eq {
	pc = 0x82E4F7B8; continue 'dispatch;
	}
	// 82E4F7B4: 4B4710DD  bl 0x822c0890
	ctx.lr = 0x82E4F7B8;
	sub_822C0890(ctx, base);
	// 82E4F7B8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4F7BC: 4082FF6C  bne 0x82e4f728
	if !ctx.cr[0].eq {
	pc = 0x82E4F728; continue 'dispatch;
	}
	// 82E4F7C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F7C4: 480002C5  bl 0x82e4fa88
	ctx.lr = 0x82E4F7C8;
	sub_82E4FA88(ctx, base);
	// 82E4F7C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F7CC: 418200BC  beq 0x82e4f888
	if ctx.cr[0].eq {
	pc = 0x82E4F888; continue 'dispatch;
	}
	// 82E4F7D0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E4F7D4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E4F7D8: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82E4F7DC: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82E4F7E0: 4B471159  bl 0x822c0938
	ctx.lr = 0x82E4F7E4;
	sub_822C0938(ctx, base);
	// 82E4F7E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F7E8: 41820018  beq 0x82e4f800
	if ctx.cr[0].eq {
	pc = 0x82E4F800; continue 'dispatch;
	}
	// 82E4F7EC: 93A3000C  stw r29, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E4F7F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4F7F4: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E4F7F8: 93A30014  stw r29, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E4F7FC: 48000008  b 0x82e4f804
	pc = 0x82E4F804; continue 'dispatch;
	// 82E4F800: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4F804: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E4F808: 4BFFF961  bl 0x82e4f168
	ctx.lr = 0x82E4F80C;
	sub_82E4F168(ctx, base);
	// 82E4F80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F810: 48000279  bl 0x82e4fa88
	ctx.lr = 0x82E4F814;
	sub_82E4FA88(ctx, base);
	// 82E4F814: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E4F818: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E4F81C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F820: 48000269  bl 0x82e4fa88
	ctx.lr = 0x82E4F824;
	sub_82E4FA88(ctx, base);
	// 82E4F824: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E4F828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F82C: 4800025D  bl 0x82e4fa88
	ctx.lr = 0x82E4F830;
	sub_82E4FA88(ctx, base);
	// 82E4F830: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E4F834: 4182002C  beq 0x82e4f860
	if ctx.cr[0].eq {
	pc = 0x82E4F860; continue 'dispatch;
	}
	// 82E4F838: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E4F83C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F840: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F844: 48000245  bl 0x82e4fa88
	ctx.lr = 0x82E4F848;
	sub_82E4FA88(ctx, base);
	// 82E4F848: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82E4F84C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4F850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4F854: 4BFE473D  bl 0x82e33f90
	ctx.lr = 0x82E4F858;
	sub_82E33F90(ctx, base);
	// 82E4F858: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E4F85C: 4082FFE4  bne 0x82e4f840
	if !ctx.cr[0].eq {
	pc = 0x82E4F840; continue 'dispatch;
	}
	// 82E4F860: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F864: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E4F868: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E4F86C: 4BFE9A3D  bl 0x82e392a8
	ctx.lr = 0x82E4F870;
	sub_82E392A8(ctx, base);
	// 82E4F870: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E4F874: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F878: 419A0008  beq cr6, 0x82e4f880
	if ctx.cr[6].eq {
	pc = 0x82E4F880; continue 'dispatch;
	}
	// 82E4F87C: 4B471015  bl 0x822c0890
	ctx.lr = 0x82E4F880;
	sub_822C0890(ctx, base);
	// 82E4F880: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E4F884: 4082FF50  bne 0x82e4f7d4
	if !ctx.cr[0].eq {
	pc = 0x82E4F7D4; continue 'dispatch;
	}
	// 82E4F888: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F88C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E4F890: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F894: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E4F898: 48358918  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F8A0 size=308
    let mut pc: u32 = 0x82E4F8A0;
    'dispatch: loop {
        match pc {
            0x82E4F8A0 => {
    //   block [0x82E4F8A0..0x82E4F9D4)
	// 82E4F8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F8A4: 483588C5  bl 0x831a8168
	ctx.lr = 0x82E4F8A8;
	sub_831A8130(ctx, base);
	// 82E4F8A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F8AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E4F8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F8B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4F8B8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E4F8BC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F8C0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E4F8C4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E4F8C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4F8CC: 4B47106D  bl 0x822c0938
	ctx.lr = 0x82E4F8D0;
	sub_822C0938(ctx, base);
	// 82E4F8D0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E4F8D4: 41820020  beq 0x82e4f8f4
	if ctx.cr[0].eq {
	pc = 0x82E4F8F4; continue 'dispatch;
	}
	// 82E4F8D8: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E4F8DC: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 82E4F8E0: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E4F8E4: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E4F8E8: 4BFA3809  bl 0x82df30f0
	ctx.lr = 0x82E4F8EC;
	sub_82DF30F0(ctx, base);
	// 82E4F8EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E4F8F0: 48000008  b 0x82e4f8f8
	pc = 0x82E4F8F8; continue 'dispatch;
	// 82E4F8F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4F8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F8FC: 4BFFFBBD  bl 0x82e4f4b8
	ctx.lr = 0x82E4F900;
	sub_82E4F4B8(ctx, base);
	// 82E4F900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F904: 48000185  bl 0x82e4fa88
	ctx.lr = 0x82E4F908;
	sub_82E4FA88(ctx, base);
	// 82E4F908: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F90C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E4F910: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F914: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E4F918: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E4F91C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82E4F920: 4BFA4D91  bl 0x82df46b0
	ctx.lr = 0x82E4F924;
	sub_82DF46B0(ctx, base);
	// 82E4F924: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F928: 57CB07BF  clrlwi. r11, r30, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4F92C: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 82E4F930: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E4F934: 41820018  beq 0x82e4f94c
	if ctx.cr[0].eq {
	pc = 0x82E4F94C; continue 'dispatch;
	}
	// 82E4F938: 216B0004  subfic r11, r11, 4
	ctx.xer.ca = ctx.r[11].u32 <= 4 as u32;
	ctx.r[11].s64 = (4 as i64) - ctx.r[11].s64;
	// 82E4F93C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F940: 419A000C  beq cr6, 0x82e4f94c
	if ctx.cr[6].eq {
	pc = 0x82E4F94C; continue 'dispatch;
	}
	// 82E4F944: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4F948: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4F94C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4F950: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F954: 48000135  bl 0x82e4fa88
	ctx.lr = 0x82E4F958;
	sub_82E4FA88(ctx, base);
	// 82E4F958: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E4F95C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F960: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E4F964: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F968: 4BFEB7D1  bl 0x82e3b138
	ctx.lr = 0x82E4F96C;
	sub_82E3B138(ctx, base);
	// 82E4F96C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F970: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E4F974: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4F97C: 40990044  ble cr6, 0x82e4f9c0
	if !ctx.cr[6].gt {
	pc = 0x82E4F9C0; continue 'dispatch;
	}
	// 82E4F980: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4F984: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4F988: 4BFFFC11  bl 0x82e4f598
	ctx.lr = 0x82E4F98C;
	sub_82E4F598(ctx, base);
	// 82E4F98C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F990: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4F994: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E4F998: 4BFE9911  bl 0x82e392a8
	ctx.lr = 0x82E4F99C;
	sub_82E392A8(ctx, base);
	// 82E4F99C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4F9A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4F9A4: 419A0008  beq cr6, 0x82e4f9ac
	if ctx.cr[6].eq {
	pc = 0x82E4F9AC; continue 'dispatch;
	}
	// 82E4F9A8: 4B470EE9  bl 0x822c0890
	ctx.lr = 0x82E4F9AC;
	sub_822C0890(ctx, base);
	// 82E4F9AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F9B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4F9B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4F9B8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4F9BC: 4198FFC4  blt cr6, 0x82e4f980
	if ctx.cr[6].lt {
	pc = 0x82E4F980; continue 'dispatch;
	}
	// 82E4F9C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4F9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4F9C8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E4F9CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4F9D0: 483587E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4F9D8 size=176
    let mut pc: u32 = 0x82E4F9D8;
    'dispatch: loop {
        match pc {
            0x82E4F9D8 => {
    //   block [0x82E4F9D8..0x82E4FA88)
	// 82E4F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4F9DC: 48358791  bl 0x831a816c
	ctx.lr = 0x82E4F9E0;
	sub_831A8130(ctx, base);
	// 82E4F9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4F9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4F9E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4F9EC: 4BFAEE7D  bl 0x82dfe868
	ctx.lr = 0x82E4F9F0;
	sub_82DFE868(ctx, base);
	// 82E4F9F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4F9F4: 4082008C  bne 0x82e4fa80
	if !ctx.cr[0].eq {
	pc = 0x82E4FA80; continue 'dispatch;
	}
	// 82E4F9F8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E4F9FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FA00: 48000089  bl 0x82e4fa88
	ctx.lr = 0x82E4FA04;
	sub_82E4FA88(ctx, base);
	// 82E4FA04: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E4FA08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E4FA0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FA10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E4FA14: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E4FA18: 48000071  bl 0x82e4fa88
	ctx.lr = 0x82E4FA1C;
	sub_82E4FA88(ctx, base);
	// 82E4FA1C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82E4FA20: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82E4FA24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E4FA28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4FA2C: 4BFEB70D  bl 0x82e3b138
	ctx.lr = 0x82E4FA30;
	sub_82E3B138(ctx, base);
	// 82E4FA30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4FA34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E4FA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E4FA3C: 4099003C  ble cr6, 0x82e4fa78
	if !ctx.cr[6].gt {
	pc = 0x82E4FA78; continue 'dispatch;
	}
	// 82E4FA40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FA44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E4FA48: 4BFFFE59  bl 0x82e4f8a0
	ctx.lr = 0x82E4FA4C;
	sub_82E4F8A0(ctx, base);
	// 82E4FA4C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E4FA50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4FA54: 4BFE9855  bl 0x82e392a8
	ctx.lr = 0x82E4FA58;
	sub_82E392A8(ctx, base);
	// 82E4FA58: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E4FA5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E4FA60: 419A0008  beq cr6, 0x82e4fa68
	if ctx.cr[6].eq {
	pc = 0x82E4FA68; continue 'dispatch;
	}
	// 82E4FA64: 4B470E2D  bl 0x822c0890
	ctx.lr = 0x82E4FA68;
	sub_822C0890(ctx, base);
	// 82E4FA68: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4FA6C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E4FA70: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E4FA74: 4198FFCC  blt cr6, 0x82e4fa40
	if ctx.cr[6].lt {
	pc = 0x82E4FA40; continue 'dispatch;
	}
	// 82E4FA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FA7C: 4BFAEDDD  bl 0x82dfe858
	ctx.lr = 0x82E4FA80;
	sub_82DFE858(ctx, base);
	// 82E4FA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4FA84: 48358738  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4FA88 size=20
    let mut pc: u32 = 0x82E4FA88;
    'dispatch: loop {
        match pc {
            0x82E4FA88 => {
    //   block [0x82E4FA88..0x82E4FA9C)
	// 82E4FA88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FA8C: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 82E4FA90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E4FA94: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FAA0 size=76
    let mut pc: u32 = 0x82E4FAA0;
    'dispatch: loop {
        match pc {
            0x82E4FAA0 => {
    //   block [0x82E4FAA0..0x82E4FAEC)
	// 82E4FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4FAA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4FAAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FAB0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FAB4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E4FAB8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E4FABC: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82E4FAC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E4FAC4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E4FAC8: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E4FACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FAD0: 4E800421  bctrl
	ctx.lr = 0x82E4FAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FAD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FAD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4FADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4FAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4FAE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4FAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FAF0 size=748
    let mut pc: u32 = 0x82E4FAF0;
    'dispatch: loop {
        match pc {
            0x82E4FAF0 => {
    //   block [0x82E4FAF0..0x82E4FDDC)
	// 82E4FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FAF4: 48358671  bl 0x831a8164
	ctx.lr = 0x82E4FAF8;
	sub_831A8130(ctx, base);
	// 82E4FAF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FAFC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E4FB00: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FB04: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82E4FB08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4FB0C: 3BAB67DC  addi r29, r11, 0x67dc
	ctx.r[29].s64 = ctx.r[11].s64 + 26588;
	// 82E4FB10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E4FB14: 389D0018  addi r4, r29, 0x18
	ctx.r[4].s64 = ctx.r[29].s64 + 24;
	// 82E4FB18: 4BFA4531  bl 0x82df4048
	ctx.lr = 0x82E4FB1C;
	sub_82DF4048(ctx, base);
	// 82E4FB1C: 3FC08212  lis r30, -0x7dee
	ctx.r[30].s64 = -2112749568;
	// 82E4FB20: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FB24: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FB28: 409A004C  bne cr6, 0x82e4fb74
	if !ctx.cr[6].eq {
	pc = 0x82E4FB74; continue 'dispatch;
	}
	// 82E4FB2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FB30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FB34: 388BC7D8  addi r4, r11, -0x3828
	ctx.r[4].s64 = ctx.r[11].s64 + -14376;
	// 82E4FB38: 4BFA4491  bl 0x82df3fc8
	ctx.lr = 0x82E4FB3C;
	sub_82DF3FC8(ctx, base);
	// 82E4FB3C: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FB40: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FB44: 409A0030  bne cr6, 0x82e4fb74
	if !ctx.cr[6].eq {
	pc = 0x82E4FB74; continue 'dispatch;
	}
	// 82E4FB48: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FB4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FB50: 388BC7D0  addi r4, r11, -0x3830
	ctx.r[4].s64 = ctx.r[11].s64 + -14384;
	// 82E4FB54: 4BFA3EB5  bl 0x82df3a08
	ctx.lr = 0x82E4FB58;
	sub_82DF3A08(ctx, base);
	// 82E4FB58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E4FB5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FB60: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82E4FB64: 4BFA37A5  bl 0x82df3308
	ctx.lr = 0x82E4FB68;
	sub_82DF3308(ctx, base);
	// 82E4FB68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4FB6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E4FB70: 41820008  beq 0x82e4fb78
	if ctx.cr[0].eq {
	pc = 0x82E4FB78; continue 'dispatch;
	}
	// 82E4FB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E4FB78: 578A07FF  clrlwi. r10, r28, 0x1f
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E4FB7C: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E4FB80: 4182000C  beq 0x82e4fb8c
	if ctx.cr[0].eq {
	pc = 0x82E4FB8C; continue 'dispatch;
	}
	// 82E4FB84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FB88: 4BFA38A1  bl 0x82df3428
	ctx.lr = 0x82E4FB8C;
	sub_82DF3428(ctx, base);
	// 82E4FB8C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4FB90: 41820014  beq 0x82e4fba4
	if ctx.cr[0].eq {
	pc = 0x82E4FBA4; continue 'dispatch;
	}
	// 82E4FB94: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FB98: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FB9C: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82E4FBA0: 48000228  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FBA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E4FBA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FBAC: 4BFA449D  bl 0x82df4048
	ctx.lr = 0x82E4FBB0;
	sub_82DF4048(ctx, base);
	// 82E4FBB0: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FBB4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FBB8: 409A0208  bne cr6, 0x82e4fdc0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDC0; continue 'dispatch;
	}
	// 82E4FBBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4FBC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FBC4: 388BC634  addi r4, r11, -0x39cc
	ctx.r[4].s64 = ctx.r[11].s64 + -14796;
	// 82E4FBC8: 4BFA4401  bl 0x82df3fc8
	ctx.lr = 0x82E4FBCC;
	sub_82DF3FC8(ctx, base);
	// 82E4FBCC: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FBD0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FBD4: 409A01EC  bne cr6, 0x82e4fdc0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDC0; continue 'dispatch;
	}
	// 82E4FBD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FBDC: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82E4FBE0: 4BFA4469  bl 0x82df4048
	ctx.lr = 0x82E4FBE4;
	sub_82DF4048(ctx, base);
	// 82E4FBE4: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FBE8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FBEC: 409A01C4  bne cr6, 0x82e4fdb0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDB0; continue 'dispatch;
	}
	// 82E4FBF0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4FBF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FBF8: 388B018C  addi r4, r11, 0x18c
	ctx.r[4].s64 = ctx.r[11].s64 + 396;
	// 82E4FBFC: 4BFA43CD  bl 0x82df3fc8
	ctx.lr = 0x82E4FC00;
	sub_82DF3FC8(ctx, base);
	// 82E4FC00: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC04: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC08: 409A01A8  bne cr6, 0x82e4fdb0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDB0; continue 'dispatch;
	}
	// 82E4FC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC10: 389D0008  addi r4, r29, 8
	ctx.r[4].s64 = ctx.r[29].s64 + 8;
	// 82E4FC14: 4BFA4435  bl 0x82df4048
	ctx.lr = 0x82E4FC18;
	sub_82DF4048(ctx, base);
	// 82E4FC18: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC1C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC20: 409A0180  bne cr6, 0x82e4fda0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDA0; continue 'dispatch;
	}
	// 82E4FC24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E4FC28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC2C: 388B1A40  addi r4, r11, 0x1a40
	ctx.r[4].s64 = ctx.r[11].s64 + 6720;
	// 82E4FC30: 4BFA4399  bl 0x82df3fc8
	ctx.lr = 0x82E4FC34;
	sub_82DF3FC8(ctx, base);
	// 82E4FC34: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC38: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC3C: 409A0164  bne cr6, 0x82e4fda0
	if !ctx.cr[6].eq {
	pc = 0x82E4FDA0; continue 'dispatch;
	}
	// 82E4FC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC44: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82E4FC48: 4BFA4401  bl 0x82df4048
	ctx.lr = 0x82E4FC4C;
	sub_82DF4048(ctx, base);
	// 82E4FC4C: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC50: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC54: 409A013C  bne cr6, 0x82e4fd90
	if !ctx.cr[6].eq {
	pc = 0x82E4FD90; continue 'dispatch;
	}
	// 82E4FC58: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E4FC5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC60: 388BE72C  addi r4, r11, -0x18d4
	ctx.r[4].s64 = ctx.r[11].s64 + -6356;
	// 82E4FC64: 4BFA4365  bl 0x82df3fc8
	ctx.lr = 0x82E4FC68;
	sub_82DF3FC8(ctx, base);
	// 82E4FC68: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC6C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC70: 409A0120  bne cr6, 0x82e4fd90
	if !ctx.cr[6].eq {
	pc = 0x82E4FD90; continue 'dispatch;
	}
	// 82E4FC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC78: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 82E4FC7C: 4BFA43CD  bl 0x82df4048
	ctx.lr = 0x82E4FC80;
	sub_82DF4048(ctx, base);
	// 82E4FC80: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FC84: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FC88: 409A00F8  bne cr6, 0x82e4fd80
	if !ctx.cr[6].eq {
	pc = 0x82E4FD80; continue 'dispatch;
	}
	// 82E4FC8C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FC90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FC94: 388BC7C8  addi r4, r11, -0x3838
	ctx.r[4].s64 = ctx.r[11].s64 + -14392;
	// 82E4FC98: 4BFA4331  bl 0x82df3fc8
	ctx.lr = 0x82E4FC9C;
	sub_82DF3FC8(ctx, base);
	// 82E4FC9C: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FCA0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FCA4: 409A00DC  bne cr6, 0x82e4fd80
	if !ctx.cr[6].eq {
	pc = 0x82E4FD80; continue 'dispatch;
	}
	// 82E4FCA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FCAC: 389D001C  addi r4, r29, 0x1c
	ctx.r[4].s64 = ctx.r[29].s64 + 28;
	// 82E4FCB0: 4BFA4399  bl 0x82df4048
	ctx.lr = 0x82E4FCB4;
	sub_82DF4048(ctx, base);
	// 82E4FCB4: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FCB8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FCBC: 409A00B4  bne cr6, 0x82e4fd70
	if !ctx.cr[6].eq {
	pc = 0x82E4FD70; continue 'dispatch;
	}
	// 82E4FCC0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FCC8: 388BC7C0  addi r4, r11, -0x3840
	ctx.r[4].s64 = ctx.r[11].s64 + -14400;
	// 82E4FCCC: 4BFA42FD  bl 0x82df3fc8
	ctx.lr = 0x82E4FCD0;
	sub_82DF3FC8(ctx, base);
	// 82E4FCD0: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FCD4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FCD8: 409A0098  bne cr6, 0x82e4fd70
	if !ctx.cr[6].eq {
	pc = 0x82E4FD70; continue 'dispatch;
	}
	// 82E4FCDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FCE0: 389D0024  addi r4, r29, 0x24
	ctx.r[4].s64 = ctx.r[29].s64 + 36;
	// 82E4FCE4: 4BFA4365  bl 0x82df4048
	ctx.lr = 0x82E4FCE8;
	sub_82DF4048(ctx, base);
	// 82E4FCE8: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FCEC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FCF0: 409A0070  bne cr6, 0x82e4fd60
	if !ctx.cr[6].eq {
	pc = 0x82E4FD60; continue 'dispatch;
	}
	// 82E4FCF4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FCFC: 388BC7B4  addi r4, r11, -0x384c
	ctx.r[4].s64 = ctx.r[11].s64 + -14412;
	// 82E4FD00: 4BFA42C9  bl 0x82df3fc8
	ctx.lr = 0x82E4FD04;
	sub_82DF3FC8(ctx, base);
	// 82E4FD04: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FD08: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FD0C: 409A0054  bne cr6, 0x82e4fd60
	if !ctx.cr[6].eq {
	pc = 0x82E4FD60; continue 'dispatch;
	}
	// 82E4FD10: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FD14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FD18: 388BC7B0  addi r4, r11, -0x3850
	ctx.r[4].s64 = ctx.r[11].s64 + -14416;
	// 82E4FD1C: 4BFA42AD  bl 0x82df3fc8
	ctx.lr = 0x82E4FD20;
	sub_82DF3FC8(ctx, base);
	// 82E4FD20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E4FD24: 40820014  bne 0x82e4fd38
	if !ctx.cr[0].eq {
	pc = 0x82E4FD38; continue 'dispatch;
	}
	// 82E4FD28: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD2C: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD30: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 82E4FD34: 48000094  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FD3C: 389D002C  addi r4, r29, 0x2c
	ctx.r[4].s64 = ctx.r[29].s64 + 44;
	// 82E4FD40: 4BFA4309  bl 0x82df4048
	ctx.lr = 0x82E4FD44;
	sub_82DF4048(ctx, base);
	// 82E4FD44: 817EB230  lwz r11, -0x4dd0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FD48: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FD4C: 419A0084  beq cr6, 0x82e4fdd0
	if ctx.cr[6].eq {
	pc = 0x82E4FDD0; continue 'dispatch;
	}
	// 82E4FD50: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD54: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD58: 386B002C  addi r3, r11, 0x2c
	ctx.r[3].s64 = ctx.r[11].s64 + 44;
	// 82E4FD5C: 4800006C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FD60: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD64: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD68: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 82E4FD6C: 4800005C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FD70: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD74: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD78: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 82E4FD7C: 4800004C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FD80: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD84: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD88: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 82E4FD8C: 4800003C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FD90: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FD94: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FD98: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82E4FD9C: 4800002C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FDA0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FDA4: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FDA8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E4FDAC: 4800001C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FDB0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FDB4: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FDB8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E4FDBC: 4800000C  b 0x82e4fdc8
	pc = 0x82E4FDC8; continue 'dispatch;
	// 82E4FDC0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FDC4: 386B67AC  addi r3, r11, 0x67ac
	ctx.r[3].s64 = ctx.r[11].s64 + 26540;
	// 82E4FDC8: 4BFA33E9  bl 0x82df31b0
	ctx.lr = 0x82E4FDCC;
	sub_82DF31B0(ctx, base);
	// 82E4FDCC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E4FDD0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E4FDD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4FDD8: 483583DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FDE0 size=68
    let mut pc: u32 = 0x82E4FDE0;
    'dispatch: loop {
        match pc {
            0x82E4FDE0 => {
    //   block [0x82E4FDE0..0x82E4FE24)
	// 82E4FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4FDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4FDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FDF0: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FDF4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FDF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4FDFC: 396B67AC  addi r11, r11, 0x67ac
	ctx.r[11].s64 = ctx.r[11].s64 + 26540;
	// 82E4FE00: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E4FE04: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E4FE08: 4BFA3DF9  bl 0x82df3c00
	ctx.lr = 0x82E4FE0C;
	sub_82DF3C00(ctx, base);
	// 82E4FE0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FE10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4FE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4FE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4FE1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4FE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FE28 size=108
    let mut pc: u32 = 0x82E4FE28;
    'dispatch: loop {
        match pc {
            0x82E4FE28 => {
    //   block [0x82E4FE28..0x82E4FE94)
	// 82E4FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FE2C: 4835833D  bl 0x831a8168
	ctx.lr = 0x82E4FE30;
	sub_831A8130(ctx, base);
	// 82E4FE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FE34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FE38: 4BFA3BD1  bl 0x82df3a08
	ctx.lr = 0x82E4FE3C;
	sub_82DF3A08(ctx, base);
	// 82E4FE3C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E4FE40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E4FE44: 3BAB67DC  addi r29, r11, 0x67dc
	ctx.r[29].s64 = ctx.r[11].s64 + 26588;
	// 82E4FE48: 3F808212  lis r28, -0x7dee
	ctx.r[28].s64 = -2112749568;
	// 82E4FE4C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E4FE50: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E4FE54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FE58: 4BFA41F1  bl 0x82df4048
	ctx.lr = 0x82E4FE5C;
	sub_82DF4048(ctx, base);
	// 82E4FE5C: 817CB230  lwz r11, -0x4dd0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E4FE60: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FE64: 409A001C  bne cr6, 0x82e4fe80
	if !ctx.cr[6].eq {
	pc = 0x82E4FE80; continue 'dispatch;
	}
	// 82E4FE68: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E4FE6C: 397D0030  addi r11, r29, 0x30
	ctx.r[11].s64 = ctx.r[29].s64 + 48;
	// 82E4FE70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E4FE74: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E4FE78: 4198FFD8  blt cr6, 0x82e4fe50
	if ctx.cr[6].lt {
	pc = 0x82E4FE50; continue 'dispatch;
	}
	// 82E4FE7C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82E4FE80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E4FE84: 4BFA35A5  bl 0x82df3428
	ctx.lr = 0x82E4FE88;
	sub_82DF3428(ctx, base);
	// 82E4FE88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E4FE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E4FE90: 48358328  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4FE98 size=12
    let mut pc: u32 = 0x82E4FE98;
    'dispatch: loop {
        match pc {
            0x82E4FE98 => {
    //   block [0x82E4FE98..0x82E4FEA4)
	// 82E4FE98: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FE9C: 386B0050  addi r3, r11, 0x50
	ctx.r[3].s64 = ctx.r[11].s64 + 80;
	// 82E4FEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E4FEA8 size=12
    let mut pc: u32 = 0x82E4FEA8;
    'dispatch: loop {
        match pc {
            0x82E4FEA8 => {
    //   block [0x82E4FEA8..0x82E4FEB4)
	// 82E4FEA8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FEAC: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82E4FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E4FEB8 size=20
    let mut pc: u32 = 0x82E4FEB8;
    'dispatch: loop {
        match pc {
            0x82E4FEB8 => {
    //   block [0x82E4FEB8..0x82E4FECC)
	// 82E4FEB8: 3964001C  addi r11, r4, 0x1c
	ctx.r[11].s64 = ctx.r[4].s64 + 28;
	// 82E4FEBC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FEC0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E4FEC4: 7C2B552E  stfsx f1, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82E4FEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E4FED0 size=120
    let mut pc: u32 = 0x82E4FED0;
    'dispatch: loop {
        match pc {
            0x82E4FED0 => {
    //   block [0x82E4FED0..0x82E4FF48)
	// 82E4FED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FED4: 48358291  bl 0x831a8164
	ctx.lr = 0x82E4FED8;
	sub_831A8130(ctx, base);
	// 82E4FED8: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82E4FEDC: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E4FEE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FEE4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E4FEE8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E4FEEC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E4FEF0: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E4FEF4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E4FEF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E4FEFC: 3BCBC77C  addi r30, r11, -0x3884
	ctx.r[30].s64 = ctx.r[11].s64 + -14468;
	// 82E4FF00: 3BA0000C  li r29, 0xc
	ctx.r[29].s64 = 12;
	// 82E4FF04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FF08: 7D6BD839  and. r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E4FF0C: 4182001C  beq 0x82e4ff28
	if ctx.cr[0].eq {
	pc = 0x82E4FF28; continue 'dispatch;
	}
	// 82E4FF10: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FF14: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82E4FF18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E4FF1C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E4FF20: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82E4FF24: 48002BDD  bl 0x82e52b00
	ctx.lr = 0x82E4FF28;
	sub_82E52B00(ctx, base);
	// 82E4FF28: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E4FF2C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E4FF30: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E4FF34: 4082FFD0  bne 0x82e4ff04
	if !ctx.cr[0].eq {
	pc = 0x82E4FF04; continue 'dispatch;
	}
	// 82E4FF38: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E4FF3C: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E4FF40: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E4FF44: 48358270  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4FF48 size=88
    let mut pc: u32 = 0x82E4FF48;
    'dispatch: loop {
        match pc {
            0x82E4FF48 => {
    //   block [0x82E4FF48..0x82E4FFA0)
	// 82E4FF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4FF50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E4FF54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4FF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FF5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4FF60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E4FF64: 4BFA3C6D  bl 0x82df3bd0
	ctx.lr = 0x82E4FF68;
	sub_82DF3BD0(ctx, base);
	// 82E4FF68: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E4FF6C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E4FF70: 4BFA3C61  bl 0x82df3bd0
	ctx.lr = 0x82E4FF74;
	sub_82DF3BD0(ctx, base);
	// 82E4FF74: C01E0008  lfs f0, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4FF78: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E4FF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E4FF80: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4FF84: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E4FF88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E4FF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E4FF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E4FF94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E4FF98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E4FF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E4FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E4FFA0 size=108
    let mut pc: u32 = 0x82E4FFA0;
    'dispatch: loop {
        match pc {
            0x82E4FFA0 => {
    //   block [0x82E4FFA0..0x82E5000C)
	// 82E4FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E4FFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E4FFA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E4FFAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E4FFB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E4FFB4: 4800343D  bl 0x82e533f0
	ctx.lr = 0x82E4FFB8;
	sub_82E533F0(ctx, base);
	// 82E4FFB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FFBC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FFC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FFC4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E4FFC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FFCC: 4E800421  bctrl
	ctx.lr = 0x82E4FFD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FFD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FFD4: 806B013C  lwz r3, 0x13c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82E4FFD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E4FFDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E4FFE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E4FFE4: 4E800421  bctrl
	ctx.lr = 0x82E4FFE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E4FFE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E4FFEC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E4FFF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E4FFF4: D00B0140  stfs f0, 0x140(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 82E4FFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E4FFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50010 size=140
    let mut pc: u32 = 0x82E50010;
    'dispatch: loop {
        match pc {
            0x82E50010 => {
    //   block [0x82E50010..0x82E5009C)
	// 82E50010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5001C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50024: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50028: 806B013C  lwz r3, 0x13c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82E5002C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50030: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50038: 4E800421  bctrl
	ctx.lr = 0x82E5003C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5003C: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E50040: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50044: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E50048: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E5004C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E50050: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E50054: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E50058: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E5005C: C16B0140  lfs f11, 0x140(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(320 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E50060: C00908A0  lfs f0, 0x8a0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E50064: C1AAC7AC  lfs f13, -0x3854(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14420 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E50068: EC0C583A  fmadds f0, f12, f0, f11
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 82E5006C: D00B0140  stfs f0, 0x140(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 82E50070: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50074: C00B0140  lfs f0, 0x140(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E50078: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E5007C: 4198000C  blt cr6, 0x82e50088
	if ctx.cr[6].lt {
	pc = 0x82E50088; continue 'dispatch;
	}
	// 82E50080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50084: 4BFFFF1D  bl 0x82e4ffa0
	ctx.lr = 0x82E50088;
	sub_82E4FFA0(ctx, base);
	// 82E50088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E500A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E500A0 size=72
    let mut pc: u32 = 0x82E500A0;
    'dispatch: loop {
        match pc {
            0x82E500A0 => {
    //   block [0x82E500A0..0x82E500E8)
	// 82E500A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E500A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E500A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E500AC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E500B0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82E500B4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E500B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E500BC: 419A0018  beq cr6, 0x82e500d4
	if ctx.cr[6].eq {
	pc = 0x82E500D4; continue 'dispatch;
	}
	// 82E500C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E500C4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E500C8: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E500CC: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E500D0: 48002E81  bl 0x82e52f50
	ctx.lr = 0x82E500D4;
	sub_82E52F50(ctx, base);
	// 82E500D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E500D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E500DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E500E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E500E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E500E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E500E8 size=16
    let mut pc: u32 = 0x82E500E8;
    'dispatch: loop {
        match pc {
            0x82E500E8 => {
    //   block [0x82E500E8..0x82E500F8)
	// 82E500E8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E500EC: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E500F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E500F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E500F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E500F8 size=8
    let mut pc: u32 = 0x82E500F8;
    'dispatch: loop {
        match pc {
            0x82E500F8 => {
    //   block [0x82E500F8..0x82E50100)
	// 82E500F8: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E500FC: 48002C9C  b 0x82e52d98
	sub_82E52D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E50100 size=4
    let mut pc: u32 = 0x82E50100;
    'dispatch: loop {
        match pc {
            0x82E50100 => {
    //   block [0x82E50100..0x82E50104)
	// 82E50100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E50108 size=12
    let mut pc: u32 = 0x82E50108;
    'dispatch: loop {
        match pc {
            0x82E50108 => {
    //   block [0x82E50108..0x82E50114)
	// 82E50108: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5010C: 806B0148  lwz r3, 0x148(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E50110: 48003050  b 0x82e53160
	sub_82E53160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E50118 size=12
    let mut pc: u32 = 0x82E50118;
    'dispatch: loop {
        match pc {
            0x82E50118 => {
    //   block [0x82E50118..0x82E50124)
	// 82E50118: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5011C: 806B0148  lwz r3, 0x148(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E50120: 480030E8  b 0x82e53208
	sub_82E53208(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E50128 size=20
    let mut pc: u32 = 0x82E50128;
    'dispatch: loop {
        match pc {
            0x82E50128 => {
    //   block [0x82E50128..0x82E5013C)
	// 82E50128: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5012C: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82E50130: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E50140 size=20
    let mut pc: u32 = 0x82E50140;
    'dispatch: loop {
        match pc {
            0x82E50140 => {
    //   block [0x82E50140..0x82E50154)
	// 82E50140: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50144: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82E50148: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E50158 size=12
    let mut pc: u32 = 0x82E50158;
    'dispatch: loop {
        match pc {
            0x82E50158 => {
    //   block [0x82E50158..0x82E50164)
	// 82E50158: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5015C: 806B0148  lwz r3, 0x148(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E50160: 48002EF8  b 0x82e53058
	sub_82E53058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50168 size=188
    let mut pc: u32 = 0x82E50168;
    'dispatch: loop {
        match pc {
            0x82E50168 => {
    //   block [0x82E50168..0x82E50224)
	// 82E50168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5016C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E50174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5017C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E50184: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E50188: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E5018C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E50190: 4B4707A9  bl 0x822c0938
	ctx.lr = 0x82E50194;
	sub_822C0938(ctx, base);
	// 82E50194: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E50198: 41820028  beq 0x82e501c0
	if ctx.cr[0].eq {
	pc = 0x82E501C0; continue 'dispatch;
	}
	// 82E5019C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E501A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E501A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E501A8: 392BC7DC  addi r9, r11, -0x3824
	ctx.r[9].s64 = ctx.r[11].s64 + -14372;
	// 82E501AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E501B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E501B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E501B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E501BC: 48000008  b 0x82e501c4
	pc = 0x82E501C4; continue 'dispatch;
	// 82E501C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E501C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E501C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E501CC: 409A003C  bne cr6, 0x82e50208
	if !ctx.cr[6].eq {
	pc = 0x82E50208; continue 'dispatch;
	}
	// 82E501D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E501D4: 419A0014  beq cr6, 0x82e501e8
	if ctx.cr[6].eq {
	pc = 0x82E501E8; continue 'dispatch;
	}
	// 82E501D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E501DC: 48002ACD  bl 0x82e52ca8
	ctx.lr = 0x82E501E0;
	sub_82E52CA8(ctx, base);
	// 82E501E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E501E4: 4B470085  bl 0x822c0268
	ctx.lr = 0x82E501E8;
	sub_822C0268(ctx, base);
	// 82E501E8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E501EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E501F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E501F4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E501F8: 816BAFB0  lwz r11, -0x5050(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20560 as u32) ) } as u64;
	// 82E501FC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E50200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E50204: 4B46FDFD  bl 0x822c0000
	ctx.lr = 0x82E50208;
	sub_822C0000(ctx, base);
	// 82E50208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5020C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E50210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5021C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50228 size=64
    let mut pc: u32 = 0x82E50228;
    'dispatch: loop {
        match pc {
            0x82E50228 => {
    //   block [0x82E50228..0x82E50268)
	// 82E50228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5022C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50238: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5023C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E50240: 419A0014  beq cr6, 0x82e50254
	if ctx.cr[6].eq {
	pc = 0x82E50254; continue 'dispatch;
	}
	// 82E50244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50248: 48002A61  bl 0x82e52ca8
	ctx.lr = 0x82E5024C;
	sub_82E52CA8(ctx, base);
	// 82E5024C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50250: 4B470019  bl 0x822c0268
	ctx.lr = 0x82E50254;
	sub_822C0268(ctx, base);
	// 82E50254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E50258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5025C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50268 size=92
    let mut pc: u32 = 0x82E50268;
    'dispatch: loop {
        match pc {
            0x82E50268 => {
    //   block [0x82E50268..0x82E502C4)
	// 82E50268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E50274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5027C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50280: 83CB0028  lwz r30, 0x28(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50284: 83EB0024  lwz r31, 0x24(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E50288: 4800001C  b 0x82e502a4
	pc = 0x82E502A4; continue 'dispatch;
	// 82E5028C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50290: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50294: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5029C: 4E800421  bctrl
	ctx.lr = 0x82E502A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E502A0: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E502A4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E502A8: 409AFFE4  bne cr6, 0x82e5028c
	if !ctx.cr[6].eq {
	pc = 0x82E5028C; continue 'dispatch;
	}
	// 82E502AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E502B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E502B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E502B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E502BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E502C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E502C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E502C8 size=128
    let mut pc: u32 = 0x82E502C8;
    'dispatch: loop {
        match pc {
            0x82E502C8 => {
    //   block [0x82E502C8..0x82E50348)
	// 82E502C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E502CC: 48357E99  bl 0x831a8164
	ctx.lr = 0x82E502D0;
	sub_831A8130(ctx, base);
	// 82E502D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E502D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E502D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E502DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E502E0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E502E4: 83EB0024  lwz r31, 0x24(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E502E8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E502EC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E502F0: 419A0050  beq cr6, 0x82e50340
	if ctx.cr[6].eq {
	pc = 0x82E50340; continue 'dispatch;
	}
	// 82E502F4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E502F8: 3BABC77C  addi r29, r11, -0x3884
	ctx.r[29].s64 = ctx.r[11].s64 + -14468;
	// 82E502FC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50300: 48001779  bl 0x82e51a78
	ctx.lr = 0x82E50304;
	sub_82E51A78(ctx, base);
	// 82E50304: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E50308: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E5030C: 7D6BE039  and. r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50310: 4182001C  beq 0x82e5032c
	if ctx.cr[0].eq {
	pc = 0x82E5032C; continue 'dispatch;
	}
	// 82E50314: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50318: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5031C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50320: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E50324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50328: 4E800421  bctrl
	ctx.lr = 0x82E5032C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5032C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50330: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E50334: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50338: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E5033C: 409AFFC0  bne cr6, 0x82e502fc
	if !ctx.cr[6].eq {
	pc = 0x82E502FC; continue 'dispatch;
	}
	// 82E50340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E50344: 48357E70  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50348 size=108
    let mut pc: u32 = 0x82E50348;
    'dispatch: loop {
        match pc {
            0x82E50348 => {
    //   block [0x82E50348..0x82E503B4)
	// 82E50348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5034C: 48357E19  bl 0x831a8164
	ctx.lr = 0x82E50350;
	sub_831A8130(ctx, base);
	// 82E50350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50354: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50358: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E5035C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E50360: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50364: 83CB0158  lwz r30, 0x158(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E50368: 83EB0154  lwz r31, 0x154(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E5036C: 4800001C  b 0x82e50388
	pc = 0x82E50388; continue 'dispatch;
	// 82E50370: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50378: 4BFA3111  bl 0x82df3488
	ctx.lr = 0x82E5037C;
	sub_82DF3488(ctx, base);
	// 82E5037C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E50380: 4182001C  beq 0x82e5039c
	if ctx.cr[0].eq {
	pc = 0x82E5039C; continue 'dispatch;
	}
	// 82E50384: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E50388: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E5038C: 409AFFE4  bne cr6, 0x82e50370
	if !ctx.cr[6].eq {
	pc = 0x82E50370; continue 'dispatch;
	}
	// 82E50390: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E50394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E50398: 48357E1C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82E5039C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E503A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E503A4: 806B0148  lwz r3, 0x148(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E503A8: 48002C71  bl 0x82e53018
	ctx.lr = 0x82E503AC;
	sub_82E53018(ctx, base);
	// 82E503AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E503B0: 4BFFFFE4  b 0x82e50394
	pc = 0x82E50394; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E503B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E503B8 size=220
    let mut pc: u32 = 0x82E503B8;
    'dispatch: loop {
        match pc {
            0x82E503B8 => {
    //   block [0x82E503B8..0x82E50494)
	// 82E503B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E503BC: 48357DA9  bl 0x831a8164
	ctx.lr = 0x82E503C0;
	sub_831A8130(ctx, base);
	// 82E503C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E503C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E503C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E503CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E503D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E503D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E503D8: 419A001C  beq cr6, 0x82e503f4
	if ctx.cr[6].eq {
	pc = 0x82E503F4; continue 'dispatch;
	}
	// 82E503DC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E503E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E503E4: 419A0008  beq cr6, 0x82e503ec
	if ctx.cr[6].eq {
	pc = 0x82E503EC; continue 'dispatch;
	}
	// 82E503E8: 4B4704A9  bl 0x822c0890
	ctx.lr = 0x82E503EC;
	sub_822C0890(ctx, base);
	// 82E503EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E503F0: 4800009C  b 0x82e5048c
	pc = 0x82E5048C; continue 'dispatch;
	// 82E503F4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E503F8: 83AB0158  lwz r29, 0x158(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E503FC: 83EB0154  lwz r31, 0x154(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E50400: 48000070  b 0x82e50470
	pc = 0x82E50470; continue 'dispatch;
	// 82E50404: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50408: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5040C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50410: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E50414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50418: 4E800421  bctrl
	ctx.lr = 0x82E5041C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5041C: 4BFA2D95  bl 0x82df31b0
	ctx.lr = 0x82E50420;
	sub_82DF31B0(ctx, base);
	// 82E50420: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E50424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50428: 4BFA3061  bl 0x82df3488
	ctx.lr = 0x82E5042C;
	sub_82DF3488(ctx, base);
	// 82E5042C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E50430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50434: 557BDFFE  rlwinm r27, r11, 0x1b, 0x1f, 0x1f
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E50438: 4BFA2FF1  bl 0x82df3428
	ctx.lr = 0x82E5043C;
	sub_82DF3428(ctx, base);
	// 82E5043C: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E50440: 4182002C  beq 0x82e5046c
	if ctx.cr[0].eq {
	pc = 0x82E5046C; continue 'dispatch;
	}
	// 82E50444: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50448: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5044C: 80AB0148  lwz r5, 0x148(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E50450: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50454: 80850008  lwz r4, 8(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50458: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5045C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50460: 4E800421  bctrl
	ctx.lr = 0x82E50464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50464: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50468: 4082FF74  bne 0x82e503dc
	if !ctx.cr[0].eq {
	pc = 0x82E503DC; continue 'dispatch;
	}
	// 82E5046C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E50470: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E50474: 409AFF90  bne cr6, 0x82e50404
	if !ctx.cr[6].eq {
	pc = 0x82E50404; continue 'dispatch;
	}
	// 82E50478: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5047C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50480: 419A0008  beq cr6, 0x82e50488
	if ctx.cr[6].eq {
	pc = 0x82E50488; continue 'dispatch;
	}
	// 82E50484: 4B47040D  bl 0x822c0890
	ctx.lr = 0x82E50488;
	sub_822C0890(ctx, base);
	// 82E50488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E5048C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E50490: 48357D24  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50498 size=336
    let mut pc: u32 = 0x82E50498;
    'dispatch: loop {
        match pc {
            0x82E50498 => {
    //   block [0x82E50498..0x82E505E8)
	// 82E50498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5049C: 48357CD1  bl 0x831a816c
	ctx.lr = 0x82E504A0;
	sub_831A8130(ctx, base);
	// 82E504A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E504A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E504A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E504AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E504B0: 48003931  bl 0x82e53de0
	ctx.lr = 0x82E504B4;
	sub_82E53DE0(ctx, base);
	// 82E504B4: 48003C7D  bl 0x82e54130
	ctx.lr = 0x82E504B8;
	sub_82E54130(ctx, base);
	// 82E504B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E504BC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E504C0: 4830F561  bl 0x8315fa20
	ctx.lr = 0x82E504C4;
	sub_8315FA20(ctx, base);
	// 82E504C4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E504C8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E504CC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E504D0: 388AC7F0  addi r4, r10, -0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + -14352;
	// 82E504D4: 38A00145  li r5, 0x145
	ctx.r[5].s64 = 325;
	// 82E504D8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E504DC: 4BFA1BED  bl 0x82df20c8
	ctx.lr = 0x82E504E0;
	sub_82DF20C8(ctx, base);
	// 82E504E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E504E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E504E8: 906B000C  stw r3, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82E504EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E504F0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E504F4: 4830F2D5  bl 0x8315f7c8
	ctx.lr = 0x82E504F8;
	sub_8315F7C8(ctx, base);
	// 82E504F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E504FC: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E50500: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E50504: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82E50508: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5050C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E50510: 806A0008  lwz r3, 8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50514: C02BC7EC  lfs f1, -0x3814(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14356 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E50518: 482EE699  bl 0x8313ebb0
	ctx.lr = 0x82E5051C;
	sub_8313EBB0(ctx, base);
	// 82E5051C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50520: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E50524: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E50528: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5052C: 388BFAA0  addi r4, r11, -0x560
	ctx.r[4].s64 = ctx.r[11].s64 + -1376;
	// 82E50530: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50534: 80AA0004  lwz r5, 4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5053C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50544: 4E800421  bctrl
	ctx.lr = 0x82E50548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50548: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5054C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50554: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5055C: 4E800421  bctrl
	ctx.lr = 0x82E50560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50560: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82E50564: 4B4703D5  bl 0x822c0938
	ctx.lr = 0x82E50568;
	sub_822C0938(ctx, base);
	// 82E50568: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5056C: 41820020  beq 0x82e5058c
	if ctx.cr[0].eq {
	pc = 0x82E5058C; continue 'dispatch;
	}
	// 82E50570: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50574: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E50578: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5057C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50580: 480028D9  bl 0x82e52e58
	ctx.lr = 0x82E50584;
	sub_82E52E58(ctx, base);
	// 82E50584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50588: 48000008  b 0x82e50590
	pc = 0x82E50590; continue 'dispatch;
	// 82E5058C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E50590: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82E50594: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E50598: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E5059C: 4BFFFBCD  bl 0x82e50168
	ctx.lr = 0x82E505A0;
	sub_82E50168(ctx, base);
	// 82E505A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E505A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E505A8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E505AC: 4B46FA55  bl 0x822c0000
	ctx.lr = 0x82E505B0;
	sub_822C0000(ctx, base);
	// 82E505B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E505B4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E505B8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E505BC: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E505C0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E505C4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E505C8: 4B473E99  bl 0x822c4460
	ctx.lr = 0x82E505CC;
	sub_822C4460(ctx, base);
	// 82E505CC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E505D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E505D4: 419A0008  beq cr6, 0x82e505dc
	if ctx.cr[6].eq {
	pc = 0x82E505DC; continue 'dispatch;
	}
	// 82E505D8: 4B4702B9  bl 0x822c0890
	ctx.lr = 0x82E505DC;
	sub_822C0890(ctx, base);
	// 82E505DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E505E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E505E4: 48357BD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E505E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E505E8 size=88
    let mut pc: u32 = 0x82E505E8;
    'dispatch: loop {
        match pc {
            0x82E505E8 => {
    //   block [0x82E505E8..0x82E50640)
	// 82E505E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E505EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E505F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E505F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E505F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E505FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E50600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50604: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E50608: 419A0020  beq cr6, 0x82e50628
	if ctx.cr[6].eq {
	pc = 0x82E50628; continue 'dispatch;
	}
	// 82E5060C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E50610: 4BFA2E19  bl 0x82df3428
	ctx.lr = 0x82E50614;
	sub_82DF3428(ctx, base);
	// 82E50614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50618: 4BFA2E11  bl 0x82df3428
	ctx.lr = 0x82E5061C;
	sub_82DF3428(ctx, base);
	// 82E5061C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E50620: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E50624: 409AFFE8  bne cr6, 0x82e5060c
	if !ctx.cr[6].eq {
	pc = 0x82E5060C; continue 'dispatch;
	}
	// 82E50628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5062C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E50638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5063C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50640 size=88
    let mut pc: u32 = 0x82E50640;
    'dispatch: loop {
        match pc {
            0x82E50640 => {
    //   block [0x82E50640..0x82E50698)
	// 82E50640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50648: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5064C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50654: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5065C: 419A0018  beq cr6, 0x82e50674
	if ctx.cr[6].eq {
	pc = 0x82E50674; continue 'dispatch;
	}
	// 82E50660: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50664: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50668: 4BFFFF81  bl 0x82e505e8
	ctx.lr = 0x82E5066C;
	sub_82E505E8(ctx, base);
	// 82E5066C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50670: 4B46FBF9  bl 0x822c0268
	ctx.lr = 0x82E50674;
	sub_822C0268(ctx, base);
	// 82E50674: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E50678: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5067C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E50680: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E50684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E50688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5068C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50698 size=128
    let mut pc: u32 = 0x82E50698;
    'dispatch: loop {
        match pc {
            0x82E50698 => {
    //   block [0x82E50698..0x82E50718)
	// 82E50698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5069C: 48357AC5  bl 0x831a8160
	ctx.lr = 0x82E506A0;
	sub_831A8130(ctx, base);
	// 82E506A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E506A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E506A8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E506AC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E506B0: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 82E506B4: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E506B8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E506BC: 419A0020  beq cr6, 0x82e506dc
	if ctx.cr[6].eq {
	pc = 0x82E506DC; continue 'dispatch;
	}
	// 82E506C0: 7F7FE050  subf r27, r31, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82E506C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E506C8: 7C7BFA14  add r3, r27, r31
	ctx.r[3].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82E506CC: 4BFA3505  bl 0x82df3bd0
	ctx.lr = 0x82E506D0;
	sub_82DF3BD0(ctx, base);
	// 82E506D0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E506D4: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E506D8: 409AFFEC  bne cr6, 0x82e506c4
	if !ctx.cr[6].eq {
	pc = 0x82E506C4; continue 'dispatch;
	}
	// 82E506DC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E506E0: 3BFDFFFC  addi r31, r29, -4
	ctx.r[31].s64 = ctx.r[29].s64 + -4;
	// 82E506E4: 48000010  b 0x82e506f4
	pc = 0x82E506F4; continue 'dispatch;
	// 82E506E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E506EC: 4BFA2D3D  bl 0x82df3428
	ctx.lr = 0x82E506F0;
	sub_82DF3428(ctx, base);
	// 82E506F0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E506F4: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E506F8: 409AFFF0  bne cr6, 0x82e506e8
	if !ctx.cr[6].eq {
	pc = 0x82E506E8; continue 'dispatch;
	}
	// 82E506FC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50700: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E50704: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E50708: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82E5070C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E50710: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E50714: 48357A9C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50718 size=112
    let mut pc: u32 = 0x82E50718;
    'dispatch: loop {
        match pc {
            0x82E50718 => {
    //   block [0x82E50718..0x82E50788)
	// 82E50718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5071C: 48357A45  bl 0x831a8160
	ctx.lr = 0x82E50720;
	sub_831A8130(ctx, base);
	// 82E50720: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50724: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E50728: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E5072C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E50730: 3BFD0010  addi r31, r29, 0x10
	ctx.r[31].s64 = ctx.r[29].s64 + 16;
	// 82E50734: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50738: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E5073C: 419A0020  beq cr6, 0x82e5075c
	if ctx.cr[6].eq {
	pc = 0x82E5075C; continue 'dispatch;
	}
	// 82E50740: 7F5FE850  subf r26, r31, r29
	ctx.r[26].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82E50744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E50748: 7C7AFA14  add r3, r26, r31
	ctx.r[3].u64 = ctx.r[26].u64 + ctx.r[31].u64;
	// 82E5074C: 4BFFF7FD  bl 0x82e4ff48
	ctx.lr = 0x82E50750;
	sub_82E4FF48(ctx, base);
	// 82E50750: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E50754: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E50758: 409AFFEC  bne cr6, 0x82e50744
	if !ctx.cr[6].eq {
	pc = 0x82E50744; continue 'dispatch;
	}
	// 82E5075C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50760: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E50764: 3864FFF0  addi r3, r4, -0x10
	ctx.r[3].s64 = ctx.r[4].s64 + -16;
	// 82E50768: 4BFFFE81  bl 0x82e505e8
	ctx.lr = 0x82E5076C;
	sub_82E505E8(ctx, base);
	// 82E5076C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50770: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E50774: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E50778: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82E5077C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E50780: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E50784: 48357A2C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50788 size=116
    let mut pc: u32 = 0x82E50788;
    'dispatch: loop {
        match pc {
            0x82E50788 => {
    //   block [0x82E50788..0x82E507FC)
	// 82E50788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5078C: 483579D5  bl 0x831a8160
	ctx.lr = 0x82E50790;
	sub_831A8130(ctx, base);
	// 82E50790: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50794: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E50798: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E5079C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E507A0: 7F1B3040  cmplw cr6, r27, r6
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82E507A4: 419A0048  beq cr6, 0x82e507ec
	if ctx.cr[6].eq {
	pc = 0x82E507EC; continue 'dispatch;
	}
	// 82E507A8: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E507AC: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E507B0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E507B4: 7F06E840  cmplw cr6, r6, r29
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E507B8: 419A0020  beq cr6, 0x82e507d8
	if ctx.cr[6].eq {
	pc = 0x82E507D8; continue 'dispatch;
	}
	// 82E507BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E507C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E507C4: 4BFFF785  bl 0x82e4ff48
	ctx.lr = 0x82E507C8;
	sub_82E4FF48(ctx, base);
	// 82E507C8: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82E507CC: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E507D0: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E507D4: 409AFFE8  bne cr6, 0x82e507bc
	if !ctx.cr[6].eq {
	pc = 0x82E507BC; continue 'dispatch;
	}
	// 82E507D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E507DC: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E507E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E507E4: 4BFFFE05  bl 0x82e505e8
	ctx.lr = 0x82E507E8;
	sub_82E505E8(ctx, base);
	// 82E507E8: 93FC0008  stw r31, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E507EC: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E507F0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E507F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E507F8: 483579B8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50800 size=140
    let mut pc: u32 = 0x82E50800;
    'dispatch: loop {
        match pc {
            0x82E50800 => {
    //   block [0x82E50800..0x82E5088C)
	// 82E50800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50804: 48357965  bl 0x831a8168
	ctx.lr = 0x82E50808;
	sub_831A8130(ctx, base);
	// 82E50808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5080C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50810: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E50814: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50818: 814B0148  lwz r10, 0x148(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E5081C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E50820: 419A000C  beq cr6, 0x82e5082c
	if ctx.cr[6].eq {
	pc = 0x82E5082C; continue 'dispatch;
	}
	// 82E50824: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E50828: 48002D09  bl 0x82e53530
	ctx.lr = 0x82E5082C;
	sub_82E53530(ctx, base);
	// 82E5082C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50830: 4BFFF2C1  bl 0x82e4faf0
	ctx.lr = 0x82E50834;
	sub_82E4FAF0(ctx, base);
	// 82E50834: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E50838: 4182004C  beq 0x82e50884
	if ctx.cr[0].eq {
	pc = 0x82E50884; continue 'dispatch;
	}
	// 82E5083C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50840: 83CB0158  lwz r30, 0x158(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E50844: 83EB0154  lwz r31, 0x154(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E50848: 4800001C  b 0x82e50864
	pc = 0x82E50864; continue 'dispatch;
	// 82E5084C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E50850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50854: 4BFA2C35  bl 0x82df3488
	ctx.lr = 0x82E50858;
	sub_82DF3488(ctx, base);
	// 82E50858: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E5085C: 41820014  beq 0x82e50870
	if ctx.cr[0].eq {
	pc = 0x82E50870; continue 'dispatch;
	}
	// 82E50860: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E50864: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E50868: 409AFFE4  bne cr6, 0x82e5084c
	if !ctx.cr[6].eq {
	pc = 0x82E5084C; continue 'dispatch;
	}
	// 82E5086C: 48000018  b 0x82e50884
	pc = 0x82E50884; continue 'dispatch;
	// 82E50870: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50874: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5087C: 388B0150  addi r4, r11, 0x150
	ctx.r[4].s64 = ctx.r[11].s64 + 336;
	// 82E50880: 4BFFFE19  bl 0x82e50698
	ctx.lr = 0x82E50884;
	sub_82E50698(ctx, base);
	// 82E50884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E50888: 48357930  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50890 size=224
    let mut pc: u32 = 0x82E50890;
    'dispatch: loop {
        match pc {
            0x82E50890 => {
    //   block [0x82E50890..0x82E50970)
	// 82E50890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50894: 483578D5  bl 0x831a8168
	ctx.lr = 0x82E50898;
	sub_831A8130(ctx, base);
	// 82E50898: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E5089C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E508A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E508A4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E508A8: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E508AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E508B0: 419A00B0  beq cr6, 0x82e50960
	if ctx.cr[6].eq {
	pc = 0x82E50960; continue 'dispatch;
	}
	// 82E508B4: 812B0038  lwz r9, 0x38(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E508B8: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E508BC: 7D4A2671  srawi. r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E508C0: 418200A0  beq 0x82e50960
	if ctx.cr[0].eq {
	pc = 0x82E50960; continue 'dispatch;
	}
	// 82E508C4: 83EB0034  lwz r31, 0x34(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E508C8: 553E003E  slwi r30, r9, 0
	ctx.r[30].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E508CC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E508D0: 419A0090  beq cr6, 0x82e50960
	if ctx.cr[6].eq {
	pc = 0x82E50960; continue 'dispatch;
	}
	// 82E508D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E508D8: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E508DC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E508E0: 4BFA28D1  bl 0x82df31b0
	ctx.lr = 0x82E508E4;
	sub_82DF31B0(ctx, base);
	// 82E508E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E508E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E508EC: 4BFA28C5  bl 0x82df31b0
	ctx.lr = 0x82E508F0;
	sub_82DF31B0(ctx, base);
	// 82E508F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E508F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E508F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E508FC: 4BFFFA4D  bl 0x82e50348
	ctx.lr = 0x82E50900;
	sub_82E50348(ctx, base);
	// 82E50900: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50904: 4082002C  bne 0x82e50930
	if !ctx.cr[0].eq {
	pc = 0x82E50930; continue 'dispatch;
	}
	// 82E50908: C01F000C  lfs f0, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E5090C: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E50910: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E50914: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E50918: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E5091C: 40980030  bge cr6, 0x82e5094c
	if !ctx.cr[6].lt {
	pc = 0x82E5094C; continue 'dispatch;
	}
	// 82E50920: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E50924: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E50928: 409AFFB4  bne cr6, 0x82e508dc
	if !ctx.cr[6].eq {
	pc = 0x82E508DC; continue 'dispatch;
	}
	// 82E5092C: 48000034  b 0x82e50960
	pc = 0x82E50960; continue 'dispatch;
	// 82E50930: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50934: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50938: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5093C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82E50940: 4BFFFDD9  bl 0x82e50718
	ctx.lr = 0x82E50944;
	sub_82E50718(ctx, base);
	// 82E50944: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E50948: 4800001C  b 0x82e50964
	pc = 0x82E50964; continue 'dispatch;
	// 82E5094C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50950: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50958: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82E5095C: 4BFFFDBD  bl 0x82e50718
	ctx.lr = 0x82E50960;
	sub_82E50718(ctx, base);
	// 82E50960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E50964: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E50968: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E5096C: 4835784C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50970 size=176
    let mut pc: u32 = 0x82E50970;
    'dispatch: loop {
        match pc {
            0x82E50970 => {
    //   block [0x82E50970..0x82E50A20)
	// 82E50970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50974: 483577F1  bl 0x831a8164
	ctx.lr = 0x82E50978;
	sub_831A8130(ctx, base);
	// 82E50978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5097C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50980: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E50984: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50988: 83CB0024  lwz r30, 0x24(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E5098C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50990: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E50994: 419A0084  beq cr6, 0x82e50a18
	if ctx.cr[6].eq {
	pc = 0x82E50A18; continue 'dispatch;
	}
	// 82E50998: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5099C: 3B8BC77C  addi r28, r11, -0x3884
	ctx.r[28].s64 = ctx.r[11].s64 + -14468;
	// 82E509A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E509A4: 480010D5  bl 0x82e51a78
	ctx.lr = 0x82E509A8;
	sub_82E51A78(ctx, base);
	// 82E509A8: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E509AC: 7D6BE02E  lwzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82E509B0: 7D6BD839  and. r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E509B4: 41820050  beq 0x82e50a04
	if ctx.cr[0].eq {
	pc = 0x82E50A04; continue 'dispatch;
	}
	// 82E509B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E509BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E509C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E509C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E509C8: 4E800421  bctrl
	ctx.lr = 0x82E509CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E509CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E509D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E509D4: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E509D8: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 82E509DC: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E509E0: 48002A69  bl 0x82e53448
	ctx.lr = 0x82E509E4;
	sub_82E53448(ctx, base);
	// 82E509E4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E509E8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E509EC: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82E509F0: 4BFFE601  bl 0x82e4eff0
	ctx.lr = 0x82E509F4;
	sub_82E4EFF0(ctx, base);
	// 82E509F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E509F8: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82E509FC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E50A00: 48000008  b 0x82e50a08
	pc = 0x82E50A08; continue 'dispatch;
	// 82E50A04: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E50A08: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50A0C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50A10: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E50A14: 409AFF8C  bne cr6, 0x82e509a0
	if !ctx.cr[6].eq {
	pc = 0x82E509A0; continue 'dispatch;
	}
	// 82E50A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E50A1C: 48357798  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50A20 size=544
    let mut pc: u32 = 0x82E50A20;
    'dispatch: loop {
        match pc {
            0x82E50A20 => {
    //   block [0x82E50A20..0x82E50C40)
	// 82E50A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50A24: 48357745  bl 0x831a8168
	ctx.lr = 0x82E50A28;
	sub_831A8130(ctx, base);
	// 82E50A28: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 82E50A2C: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82E50A30: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E50A34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50A38: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E50A3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E50A40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E50A44: 3BEBC7F0  addi r31, r11, -0x3810
	ctx.r[31].s64 = ctx.r[11].s64 + -14352;
	// 82E50A48: 48000B79  bl 0x82e515c0
	ctx.lr = 0x82E50A4C;
	sub_82E515C0(ctx, base);
	// 82E50A4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E50A50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E50A54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50A58: 38C00450  li r6, 0x450
	ctx.r[6].s64 = 1104;
	// 82E50A5C: 4BFA8DDD  bl 0x82df9838
	ctx.lr = 0x82E50A60;
	sub_82DF9838(ctx, base);
	// 82E50A60: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50A64: 83EB0024  lwz r31, 0x24(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E50A68: 480001B0  b 0x82e50c18
	pc = 0x82E50C18; continue 'dispatch;
	// 82E50A6C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50A70: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50A74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E50A78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E50A7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E50A80: 419A0024  beq cr6, 0x82e50aa4
	if ctx.cr[6].eq {
	pc = 0x82E50AA4; continue 'dispatch;
	}
	// 82E50A84: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E50A88: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E50A8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E50A90: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E50A94: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E50A98: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E50A9C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E50AA0: 4082FFE8  bne 0x82e50a88
	if !ctx.cr[0].eq {
	pc = 0x82E50A88; continue 'dispatch;
	}
	// 82E50AA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E50AA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E50AAC: 4BFFF90D  bl 0x82e503b8
	ctx.lr = 0x82E50AB0;
	sub_82E503B8(ctx, base);
	// 82E50AB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50AB4: 418200B8  beq 0x82e50b6c
	if ctx.cr[0].eq {
	pc = 0x82E50B6C; continue 'dispatch;
	}
	// 82E50AB8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50ABC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50AC0: 388B0050  addi r4, r11, 0x50
	ctx.r[4].s64 = ctx.r[11].s64 + 80;
	// 82E50AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50AC8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E50ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50AD0: 4E800421  bctrl
	ctx.lr = 0x82E50AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50AD4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50AD8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50ADC: 388B0060  addi r4, r11, 0x60
	ctx.r[4].s64 = ctx.r[11].s64 + 96;
	// 82E50AE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50AE4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E50AE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50AEC: 4E800421  bctrl
	ctx.lr = 0x82E50AF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50AF0: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50AF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E50AF8: 48000F81  bl 0x82e51a78
	ctx.lr = 0x82E50AFC;
	sub_82E51A78(ctx, base);
	// 82E50AFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50B00: 546A2036  slwi r10, r3, 4
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E50B04: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E50B08: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82E50B0C: 48001F8D  bl 0x82e52a98
	ctx.lr = 0x82E50B10;
	sub_82E52A98(ctx, base);
	// 82E50B10: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B14: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E50B18: 481B8311  bl 0x83008e28
	ctx.lr = 0x82E50B1C;
	sub_83008E28(ctx, base);
	// 82E50B1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E50B20: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50B24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E50B28: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B2C: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82E50B30: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E50B34: 7FAB542E  lfsx f29, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E50B38: 48038311  bl 0x82e88e48
	ctx.lr = 0x82E50B3C;
	sub_82E88E48(ctx, base);
	// 82E50B3C: EC010772  fmuls f0, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[29].f64) as f32) as f64);
	// 82E50B40: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E50B44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E50B48: EC2007B2  fmuls f1, f0, f30
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E50B4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50B50: 4E800421  bctrl
	ctx.lr = 0x82E50B54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50B54: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E50B5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50B68: 4E800421  bctrl
	ctx.lr = 0x82E50B6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50B6C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B74: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E50B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50B7C: 4E800421  bctrl
	ctx.lr = 0x82E50B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50B80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50B84: 41820020  beq 0x82e50ba4
	if ctx.cr[0].eq {
	pc = 0x82E50BA4; continue 'dispatch;
	}
	// 82E50B88: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50B90: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E50B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50B98: 4E800421  bctrl
	ctx.lr = 0x82E50B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50B9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E50BA0: 41820070  beq 0x82e50c10
	if ctx.cr[0].eq {
	pc = 0x82E50C10; continue 'dispatch;
	}
	// 82E50BA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50BA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E50BAC: 419A000C  beq cr6, 0x82e50bb8
	if ctx.cr[6].eq {
	pc = 0x82E50BB8; continue 'dispatch;
	}
	// 82E50BB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50BB4: 48000008  b 0x82e50bbc
	pc = 0x82E50BBC; continue 'dispatch;
	// 82E50BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E50BBC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E50BC0: 409A0050  bne cr6, 0x82e50c10
	if !ctx.cr[6].eq {
	pc = 0x82E50C10; continue 'dispatch;
	}
	// 82E50BC4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50BCC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50BD4: 4E800421  bctrl
	ctx.lr = 0x82E50BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50BD8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50BDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E50BE0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E50BE4: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 82E50BE8: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50BEC: 4800285D  bl 0x82e53448
	ctx.lr = 0x82E50BF0;
	sub_82E53448(ctx, base);
	// 82E50BF0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50BF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E50BF8: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82E50BFC: 4BFFE3F5  bl 0x82e4eff0
	ctx.lr = 0x82E50C00;
	sub_82E4EFF0(ctx, base);
	// 82E50C00: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50C04: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82E50C08: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E50C0C: 48000008  b 0x82e50c14
	pc = 0x82E50C14; continue 'dispatch;
	// 82E50C10: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E50C14: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50C18: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E50C1C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E50C20: 409AFE4C  bne cr6, 0x82e50a6c
	if !ctx.cr[6].eq {
	pc = 0x82E50A6C; continue 'dispatch;
	}
	// 82E50C24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E50C28: 4B47E9D9  bl 0x822cf600
	ctx.lr = 0x82E50C2C;
	sub_822CF600(ctx, base);
	// 82E50C2C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E50C30: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E50C34: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E50C38: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E50C3C: 4835757C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50C40 size=128
    let mut pc: u32 = 0x82E50C40;
    'dispatch: loop {
        match pc {
            0x82E50C40 => {
    //   block [0x82E50C40..0x82E50CC0)
	// 82E50C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50C44: 48357525  bl 0x831a8168
	ctx.lr = 0x82E50C48;
	sub_831A8130(ctx, base);
	// 82E50C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50C4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50C50: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E50C54: 3B9F007C  addi r28, r31, 0x7c
	ctx.r[28].s64 = ctx.r[31].s64 + 124;
	// 82E50C58: 3BA0000B  li r29, 0xb
	ctx.r[29].s64 = 11;
	// 82E50C5C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82E50C60: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82E50C64: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E50C68: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82E50C6C: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82E50C70: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82E50C74: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82E50C78: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82E50C7C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82E50C80: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82E50C84: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82E50C88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E50C8C: 48001EE5  bl 0x82e52b70
	ctx.lr = 0x82E50C90;
	sub_82E52B70(ctx, base);
	// 82E50C90: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E50C94: 3B9C0010  addi r28, r28, 0x10
	ctx.r[28].s64 = ctx.r[28].s64 + 16;
	// 82E50C98: 4080FFF0  bge 0x82e50c88
	if !ctx.cr[0].lt {
	pc = 0x82E50C88; continue 'dispatch;
	}
	// 82E50C9C: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 82E50CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50CA4: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 82E50CA8: 93DF014C  stw r30, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[30].u32 ) };
	// 82E50CAC: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82E50CB0: 93DF0158  stw r30, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[30].u32 ) };
	// 82E50CB4: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 82E50CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E50CBC: 483574FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50CC0 size=136
    let mut pc: u32 = 0x82E50CC0;
    'dispatch: loop {
        match pc {
            0x82E50CC0 => {
    //   block [0x82E50CC0..0x82E50D48)
	// 82E50CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50CCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50CD4: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 82E50CD8: 4BFCB0B1  bl 0x82e1bd88
	ctx.lr = 0x82E50CDC;
	sub_82E1BD88(ctx, base);
	// 82E50CDC: 807F014C  lwz r3, 0x14c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82E50CE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50CE4: 419A0008  beq cr6, 0x82e50cec
	if ctx.cr[6].eq {
	pc = 0x82E50CEC; continue 'dispatch;
	}
	// 82E50CE8: 4B46FBA9  bl 0x822c0890
	ctx.lr = 0x82E50CEC;
	sub_822C0890(ctx, base);
	// 82E50CEC: 807F013C  lwz r3, 0x13c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82E50CF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50CF4: 419A0018  beq cr6, 0x82e50d0c
	if ctx.cr[6].eq {
	pc = 0x82E50D0C; continue 'dispatch;
	}
	// 82E50CF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50CFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E50D00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50D08: 4E800421  bctrl
	ctx.lr = 0x82E50D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50D0C: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E50D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50D14: 419A0008  beq cr6, 0x82e50d1c
	if ctx.cr[6].eq {
	pc = 0x82E50D1C; continue 'dispatch;
	}
	// 82E50D18: 4B46FB79  bl 0x822c0890
	ctx.lr = 0x82E50D1C;
	sub_822C0890(ctx, base);
	// 82E50D1C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E50D20: 4BFFF921  bl 0x82e50640
	ctx.lr = 0x82E50D24;
	sub_82E50640(ctx, base);
	// 82E50D24: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E50D28: 4BFDC099  bl 0x82e2cdc0
	ctx.lr = 0x82E50D2C;
	sub_82E2CDC0(ctx, base);
	// 82E50D2C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E50D30: 4BFDC091  bl 0x82e2cdc0
	ctx.lr = 0x82E50D34;
	sub_82E2CDC0(ctx, base);
	// 82E50D34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E50D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50D48 size=308
    let mut pc: u32 = 0x82E50D48;
    'dispatch: loop {
        match pc {
            0x82E50D48 => {
    //   block [0x82E50D48..0x82E50E7C)
	// 82E50D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50D50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E50D54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50D5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50D60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E50D64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50D68: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E50D6C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E50D70: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50D74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50D78: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E50D7C: 419A0008  beq cr6, 0x82e50d84
	if ctx.cr[6].eq {
	pc = 0x82E50D84; continue 'dispatch;
	}
	// 82E50D80: 4B46FB11  bl 0x822c0890
	ctx.lr = 0x82E50D84;
	sub_822C0890(ctx, base);
	// 82E50D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E50D88: 4BFFF4E1  bl 0x82e50268
	ctx.lr = 0x82E50D8C;
	sub_82E50268(ctx, base);
	// 82E50D8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50D90: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E50D94: 4804791D  bl 0x82e986b0
	ctx.lr = 0x82E50D98;
	sub_82E986B0(ctx, base);
	// 82E50D98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50DA0: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82E50DA4: 80CB0038  lwz r6, 0x38(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E50DA8: 80AB0034  lwz r5, 0x34(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E50DAC: 4BFFF9DD  bl 0x82e50788
	ctx.lr = 0x82E50DB0;
	sub_82E50788(ctx, base);
	// 82E50DB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50DB4: 396B0148  addi r11, r11, 0x148
	ctx.r[11].s64 = ctx.r[11].s64 + 328;
	// 82E50DB8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E50DBC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50DC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50DC4: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E50DC8: 419A0008  beq cr6, 0x82e50dd0
	if ctx.cr[6].eq {
	pc = 0x82E50DD0; continue 'dispatch;
	}
	// 82E50DCC: 4B46FAC5  bl 0x822c0890
	ctx.lr = 0x82E50DD0;
	sub_822C0890(ctx, base);
	// 82E50DD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50DD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50DD8: 388B0150  addi r4, r11, 0x150
	ctx.r[4].s64 = ctx.r[11].s64 + 336;
	// 82E50DDC: 80CB0158  lwz r6, 0x158(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E50DE0: 80AB0154  lwz r5, 0x154(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E50DE4: 4B58157D  bl 0x823d2360
	ctx.lr = 0x82E50DE8;
	sub_823D2360(ctx, base);
	// 82E50DE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E50DF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E50DF4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50DFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E04: 4E800421  bctrl
	ctx.lr = 0x82E50E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E0C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E50E10: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E20: 4E800421  bctrl
	ctx.lr = 0x82E50E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E28: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E50E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E50E38: 4E800421  bctrl
	ctx.lr = 0x82E50E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E50E3C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E40: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E50E44: 4830EA65  bl 0x8315f8a8
	ctx.lr = 0x82E50E48;
	sub_8315F8A8(ctx, base);
	// 82E50E48: 4830E7D1  bl 0x8315f618
	ctx.lr = 0x82E50E4C;
	sub_8315F618(ctx, base);
	// 82E50E4C: 48003015  bl 0x82e53e60
	ctx.lr = 0x82E50E50;
	sub_82E53E60(ctx, base);
	// 82E50E50: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E50E54: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E50E58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E5C: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E50E60: 4BFA1329  bl 0x82df2188
	ctx.lr = 0x82E50E64;
	sub_82DF2188(ctx, base);
	// 82E50E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E50E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50E70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E50E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50E80 size=136
    let mut pc: u32 = 0x82E50E80;
    'dispatch: loop {
        match pc {
            0x82E50E80 => {
    //   block [0x82E50E80..0x82E50F08)
	// 82E50E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50E8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E50E94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50E98: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E50E9C: 48047815  bl 0x82e986b0
	ctx.lr = 0x82E50EA0;
	sub_82E986B0(ctx, base);
	// 82E50EA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50EA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E50EA8: 396B0148  addi r11, r11, 0x148
	ctx.r[11].s64 = ctx.r[11].s64 + 328;
	// 82E50EAC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E50EB0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50EB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E50EB8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E50EBC: 419A0008  beq cr6, 0x82e50ec4
	if ctx.cr[6].eq {
	pc = 0x82E50EC4; continue 'dispatch;
	}
	// 82E50EC0: 4B46F9D1  bl 0x822c0890
	ctx.lr = 0x82E50EC4;
	sub_822C0890(ctx, base);
	// 82E50EC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50EC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50ECC: 388B0150  addi r4, r11, 0x150
	ctx.r[4].s64 = ctx.r[11].s64 + 336;
	// 82E50ED0: 80CB0158  lwz r6, 0x158(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E50ED4: 80AB0154  lwz r5, 0x154(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E50ED8: 4B581489  bl 0x823d2360
	ctx.lr = 0x82E50EDC;
	sub_823D2360(ctx, base);
	// 82E50EDC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50EE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E50EE4: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82E50EE8: 80CB0038  lwz r6, 0x38(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E50EEC: 80AB0034  lwz r5, 0x34(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E50EF0: 4BFFF899  bl 0x82e50788
	ctx.lr = 0x82E50EF4;
	sub_82E50788(ctx, base);
	// 82E50EF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E50EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50F00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E50F08 size=116
    let mut pc: u32 = 0x82E50F08;
    'dispatch: loop {
        match pc {
            0x82E50F08 => {
    //   block [0x82E50F08..0x82E50F7C)
	// 82E50F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E50F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E50F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E50F18: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E50F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50F20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50F24: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E50F28: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E50F2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50F30: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E50F34: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E50F38: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82E50F3C: 48001B65  bl 0x82e52aa0
	ctx.lr = 0x82E50F40;
	sub_82E52AA0(ctx, base);
	// 82E50F40: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E50F44: 2B1F00C0  cmplwi cr6, r31, 0xc0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 192 as u32, &mut ctx.xer);
	// 82E50F48: 4198FFE4  blt cr6, 0x82e50f2c
	if ctx.cr[6].lt {
	pc = 0x82E50F2C; continue 'dispatch;
	}
	// 82E50F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E50F50: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E50F54: 4BFFFACD  bl 0x82e50a20
	ctx.lr = 0x82E50F58;
	sub_82E50A20(ctx, base);
	// 82E50F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E50F5C: 4BFFF935  bl 0x82e50890
	ctx.lr = 0x82E50F60;
	sub_82E50890(ctx, base);
	// 82E50F60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E50F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E50F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E50F6C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E50F70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E50F74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E50F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E50F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E50F80 size=836
    let mut pc: u32 = 0x82E50F80;
    'dispatch: loop {
        match pc {
            0x82E50F80 => {
    //   block [0x82E50F80..0x82E512C4)
	// 82E50F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E50F84: 483571E9  bl 0x831a816c
	ctx.lr = 0x82E50F88;
	sub_831A8130(ctx, base);
	// 82E50F88: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E50F8C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E50F90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E50F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E50F98: 4BFA1541  bl 0x82df24d8
	ctx.lr = 0x82E50F9C;
	sub_82DF24D8(ctx, base);
	// 82E50F9C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E50FA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E50FA4: 388BC7F0  addi r4, r11, -0x3810
	ctx.r[4].s64 = ctx.r[11].s64 + -14352;
	// 82E50FA8: 38A000D5  li r5, 0xd5
	ctx.r[5].s64 = 213;
	// 82E50FAC: 38600160  li r3, 0x160
	ctx.r[3].s64 = 352;
	// 82E50FB0: 4BFA1439  bl 0x82df23e8
	ctx.lr = 0x82E50FB4;
	sub_82DF23E8(ctx, base);
	// 82E50FB4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E50FB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E50FBC: 4182000C  beq 0x82e50fc8
	if ctx.cr[0].eq {
	pc = 0x82E50FC8; continue 'dispatch;
	}
	// 82E50FC0: 4BFFFC81  bl 0x82e50c40
	ctx.lr = 0x82E50FC4;
	sub_82E50C40(ctx, base);
	// 82E50FC4: 48000008  b 0x82e50fcc
	pc = 0x82E50FCC; continue 'dispatch;
	// 82E50FC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E50FCC: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E50FD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E50FD4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E50FD8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E50FDC: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50FE0: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82E50FE4: 93A80004  stw r29, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E50FE8: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50FEC: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E50FF0: 93A8000C  stw r29, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E50FF4: C3C908A8  lfs f30, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E50FF8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E50FFC: D3EB0060  stfs f31, 0x60(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E51000: D3CB0064  stfs f30, 0x64(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E51004: D3EB0068  stfs f31, 0x68(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E51008: D3CB006C  stfs f30, 0x6c(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E5100C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51010: D3EB0050  stfs f31, 0x50(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E51014: D3EB0054  stfs f31, 0x54(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E51018: D3EB0058  stfs f31, 0x58(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E5101C: D3CB005C  stfs f30, 0x5c(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E51020: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51024: 7FCA5D2E  stfsx f30, r10, r11
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 82E51028: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E5102C: 2B0A007C  cmplwi cr6, r10, 0x7c
	ctx.cr[6].compare_u32(ctx.r[10].u32, 124 as u32, &mut ctx.xer);
	// 82E51030: 4198FFF0  blt cr6, 0x82e51020
	if ctx.cr[6].lt {
	pc = 0x82E51020; continue 'dispatch;
	}
	// 82E51034: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82E51038: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5103C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82E51040: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82E51044: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82E51048: 48001B19  bl 0x82e52b60
	ctx.lr = 0x82E5104C;
	sub_82E52B60(ctx, base);
	// 82E5104C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82E51050: 2B1F00C0  cmplwi cr6, r31, 0xc0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 192 as u32, &mut ctx.xer);
	// 82E51054: 4198FFE4  blt cr6, 0x82e51038
	if ctx.cr[6].lt {
	pc = 0x82E51038; continue 'dispatch;
	}
	// 82E51058: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5105C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E51060: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E51064: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51068: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5106C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E51070: 419A0008  beq cr6, 0x82e51078
	if ctx.cr[6].eq {
	pc = 0x82E51078; continue 'dispatch;
	}
	// 82E51074: 4B46F81D  bl 0x822c0890
	ctx.lr = 0x82E51078;
	sub_822C0890(ctx, base);
	// 82E51078: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E5107C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E51080: 3BEB67AC  addi r31, r11, 0x67ac
	ctx.r[31].s64 = ctx.r[11].s64 + 26540;
	// 82E51084: 388AC97C  addi r4, r10, -0x3684
	ctx.r[4].s64 = ctx.r[10].s64 + -13956;
	// 82E51088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5108C: 4BFA27ED  bl 0x82df3878
	ctx.lr = 0x82E51090;
	sub_82DF3878(ctx, base);
	// 82E51090: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51094: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E51098: 388BC964  addi r4, r11, -0x369c
	ctx.r[4].s64 = ctx.r[11].s64 + -13980;
	// 82E5109C: 4BFA27DD  bl 0x82df3878
	ctx.lr = 0x82E510A0;
	sub_82DF3878(ctx, base);
	// 82E510A0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510A4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E510A8: 388BC950  addi r4, r11, -0x36b0
	ctx.r[4].s64 = ctx.r[11].s64 + -14000;
	// 82E510AC: 4BFA27CD  bl 0x82df3878
	ctx.lr = 0x82E510B0;
	sub_82DF3878(ctx, base);
	// 82E510B0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510B4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E510B8: 388BC938  addi r4, r11, -0x36c8
	ctx.r[4].s64 = ctx.r[11].s64 + -14024;
	// 82E510BC: 4BFA27BD  bl 0x82df3878
	ctx.lr = 0x82E510C0;
	sub_82DF3878(ctx, base);
	// 82E510C0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510C4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E510C8: 388BC924  addi r4, r11, -0x36dc
	ctx.r[4].s64 = ctx.r[11].s64 + -14044;
	// 82E510CC: 4BFA27AD  bl 0x82df3878
	ctx.lr = 0x82E510D0;
	sub_82DF3878(ctx, base);
	// 82E510D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510D4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E510D8: 388BC90C  addi r4, r11, -0x36f4
	ctx.r[4].s64 = ctx.r[11].s64 + -14068;
	// 82E510DC: 4BFA279D  bl 0x82df3878
	ctx.lr = 0x82E510E0;
	sub_82DF3878(ctx, base);
	// 82E510E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510E4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E510E8: 388BC8F8  addi r4, r11, -0x3708
	ctx.r[4].s64 = ctx.r[11].s64 + -14088;
	// 82E510EC: 4BFA278D  bl 0x82df3878
	ctx.lr = 0x82E510F0;
	sub_82DF3878(ctx, base);
	// 82E510F0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E510F4: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82E510F8: 388BC8E0  addi r4, r11, -0x3720
	ctx.r[4].s64 = ctx.r[11].s64 + -14112;
	// 82E510FC: 4BFA277D  bl 0x82df3878
	ctx.lr = 0x82E51100;
	sub_82DF3878(ctx, base);
	// 82E51100: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51104: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E51108: 388BC8C8  addi r4, r11, -0x3738
	ctx.r[4].s64 = ctx.r[11].s64 + -14136;
	// 82E5110C: 4BFA276D  bl 0x82df3878
	ctx.lr = 0x82E51110;
	sub_82DF3878(ctx, base);
	// 82E51110: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51114: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E51118: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 82E5111C: 4BFA275D  bl 0x82df3878
	ctx.lr = 0x82E51120;
	sub_82DF3878(ctx, base);
	// 82E51120: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51124: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E51128: 388BC898  addi r4, r11, -0x3768
	ctx.r[4].s64 = ctx.r[11].s64 + -14184;
	// 82E5112C: 4BFA274D  bl 0x82df3878
	ctx.lr = 0x82E51130;
	sub_82DF3878(ctx, base);
	// 82E51130: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51134: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E51138: 388BC884  addi r4, r11, -0x377c
	ctx.r[4].s64 = ctx.r[11].s64 + -14204;
	// 82E5113C: 4BFA273D  bl 0x82df3878
	ctx.lr = 0x82E51140;
	sub_82DF3878(ctx, base);
	// 82E51140: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E51144: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E51148: 3BEB67DC  addi r31, r11, 0x67dc
	ctx.r[31].s64 = ctx.r[11].s64 + 26588;
	// 82E5114C: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 82E51150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51154: 4BFA2725  bl 0x82df3878
	ctx.lr = 0x82E51158;
	sub_82DF3878(ctx, base);
	// 82E51158: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5115C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E51160: 388BC874  addi r4, r11, -0x378c
	ctx.r[4].s64 = ctx.r[11].s64 + -14220;
	// 82E51164: 4BFA2715  bl 0x82df3878
	ctx.lr = 0x82E51168;
	sub_82DF3878(ctx, base);
	// 82E51168: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5116C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E51170: 388BC86C  addi r4, r11, -0x3794
	ctx.r[4].s64 = ctx.r[11].s64 + -14228;
	// 82E51174: 4BFA2705  bl 0x82df3878
	ctx.lr = 0x82E51178;
	sub_82DF3878(ctx, base);
	// 82E51178: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5117C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E51180: 388BC864  addi r4, r11, -0x379c
	ctx.r[4].s64 = ctx.r[11].s64 + -14236;
	// 82E51184: 4BFA26F5  bl 0x82df3878
	ctx.lr = 0x82E51188;
	sub_82DF3878(ctx, base);
	// 82E51188: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5118C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E51190: 388BC85C  addi r4, r11, -0x37a4
	ctx.r[4].s64 = ctx.r[11].s64 + -14244;
	// 82E51194: 4BFA26E5  bl 0x82df3878
	ctx.lr = 0x82E51198;
	sub_82DF3878(ctx, base);
	// 82E51198: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5119C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E511A0: 388BC854  addi r4, r11, -0x37ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14252;
	// 82E511A4: 4BFA26D5  bl 0x82df3878
	ctx.lr = 0x82E511A8;
	sub_82DF3878(ctx, base);
	// 82E511A8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511AC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E511B0: 388BC84C  addi r4, r11, -0x37b4
	ctx.r[4].s64 = ctx.r[11].s64 + -14260;
	// 82E511B4: 4BFA26C5  bl 0x82df3878
	ctx.lr = 0x82E511B8;
	sub_82DF3878(ctx, base);
	// 82E511B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511BC: 388BC844  addi r4, r11, -0x37bc
	ctx.r[4].s64 = ctx.r[11].s64 + -14268;
	// 82E511C0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82E511C4: 4BFA26B5  bl 0x82df3878
	ctx.lr = 0x82E511C8;
	sub_82DF3878(ctx, base);
	// 82E511C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511CC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E511D0: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 82E511D4: 4BFA26A5  bl 0x82df3878
	ctx.lr = 0x82E511D8;
	sub_82DF3878(ctx, base);
	// 82E511D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511DC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E511E0: 388BC830  addi r4, r11, -0x37d0
	ctx.r[4].s64 = ctx.r[11].s64 + -14288;
	// 82E511E4: 4BFA2695  bl 0x82df3878
	ctx.lr = 0x82E511E8;
	sub_82DF3878(ctx, base);
	// 82E511E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511EC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82E511F0: 388BC7B0  addi r4, r11, -0x3850
	ctx.r[4].s64 = ctx.r[11].s64 + -14416;
	// 82E511F4: 4BFA2685  bl 0x82df3878
	ctx.lr = 0x82E511F8;
	sub_82DF3878(ctx, base);
	// 82E511F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E511FC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E51200: 388BC828  addi r4, r11, -0x37d8
	ctx.r[4].s64 = ctx.r[11].s64 + -14296;
	// 82E51204: 4BFA2675  bl 0x82df3878
	ctx.lr = 0x82E51208;
	sub_82DF3878(ctx, base);
	// 82E51208: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5120C: 396B0148  addi r11, r11, 0x148
	ctx.r[11].s64 = ctx.r[11].s64 + 328;
	// 82E51210: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E51214: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51218: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E5121C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E51220: 419A0008  beq cr6, 0x82e51228
	if ctx.cr[6].eq {
	pc = 0x82E51228; continue 'dispatch;
	}
	// 82E51224: 4B46F66D  bl 0x822c0890
	ctx.lr = 0x82E51228;
	sub_822C0890(ctx, base);
	// 82E51228: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5122C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51230: 388B0150  addi r4, r11, 0x150
	ctx.r[4].s64 = ctx.r[11].s64 + 336;
	// 82E51234: 80CB0158  lwz r6, 0x158(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82E51238: 80AB0154  lwz r5, 0x154(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82E5123C: 4B581125  bl 0x823d2360
	ctx.lr = 0x82E51240;
	sub_823D2360(ctx, base);
	// 82E51240: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E51244: 806B1108  lwz r3, 0x1108(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4360 as u32) ) } as u64;
	// 82E51248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5124C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51254: 4E800421  bctrl
	ctx.lr = 0x82E51258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51258: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5125C: 814B013C  lwz r10, 0x13c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82E51260: 906B013C  stw r3, 0x13c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), ctx.r[3].u32 ) };
	// 82E51264: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E51268: 419A001C  beq cr6, 0x82e51284
	if ctx.cr[6].eq {
	pc = 0x82E51284; continue 'dispatch;
	}
	// 82E5126C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51270: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E51274: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82E51278: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5127C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51280: 4E800421  bctrl
	ctx.lr = 0x82E51284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51284: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51288: 806B013C  lwz r3, 0x13c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82E5128C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51290: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51298: 4E800421  bctrl
	ctx.lr = 0x82E5129C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5129C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E512A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E512A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E512A8: D3EA0140  stfs f31, 0x140(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 82E512AC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E512B0: 996A0144  stb r11, 0x144(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(324 as u32), ctx.r[11].u8 ) };
	// 82E512B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E512B8: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E512BC: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E512C0: 48356EFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E512C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E512C8 size=220
    let mut pc: u32 = 0x82E512C8;
    'dispatch: loop {
        match pc {
            0x82E512C8 => {
    //   block [0x82E512C8..0x82E513A4)
	// 82E512C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E512CC: 48356E99  bl 0x831a8164
	ctx.lr = 0x82E512D0;
	sub_831A8130(ctx, base);
	// 82E512D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E512D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E512D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E512DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E512E0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E512E4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E512E8: 4BFFE809  bl 0x82e4faf0
	ctx.lr = 0x82E512EC;
	sub_82E4FAF0(ctx, base);
	// 82E512EC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E512F0: 4082000C  bne 0x82e512fc
	if !ctx.cr[0].eq {
	pc = 0x82E512FC; continue 'dispatch;
	}
	// 82E512F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E512F8: 480000A4  b 0x82e5139c
	pc = 0x82E5139C; continue 'dispatch;
	// 82E512FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51300: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E51304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51308: 409A0050  bne cr6, 0x82e51358
	if !ctx.cr[6].eq {
	pc = 0x82E51358; continue 'dispatch;
	}
	// 82E5130C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51310: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E51314: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E51318: 388AC994  addi r4, r10, -0x366c
	ctx.r[4].s64 = ctx.r[10].s64 + -13932;
	// 82E5131C: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51320: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51324: 4800247D  bl 0x82e537a0
	ctx.lr = 0x82E51328;
	sub_82E537A0(ctx, base);
	// 82E51328: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E5132C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51330: 388A0004  addi r4, r10, 4
	ctx.r[4].s64 = ctx.r[10].s64 + 4;
	// 82E51334: 396B0148  addi r11, r11, 0x148
	ctx.r[11].s64 = ctx.r[11].s64 + 328;
	// 82E51338: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5133C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E51340: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E51344: 4B47311D  bl 0x822c4460
	ctx.lr = 0x82E51348;
	sub_822C4460(ctx, base);
	// 82E51348: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E5134C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51350: 419A0008  beq cr6, 0x82e51358
	if ctx.cr[6].eq {
	pc = 0x82E51358; continue 'dispatch;
	}
	// 82E51354: 4B46F53D  bl 0x822c0890
	ctx.lr = 0x82E51358;
	sub_822C0890(ctx, base);
	// 82E51358: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5135C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E51360: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E51364: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E51368: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5136C: 806B0148  lwz r3, 0x148(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E51370: 480024C9  bl 0x82e53838
	ctx.lr = 0x82E51374;
	sub_82E53838(ctx, base);
	// 82E51374: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E51378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5137C: 4BFA268D  bl 0x82df3a08
	ctx.lr = 0x82E51380;
	sub_82DF3A08(ctx, base);
	// 82E51380: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51384: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51388: 386B0150  addi r3, r11, 0x150
	ctx.r[3].s64 = ctx.r[11].s64 + 336;
	// 82E5138C: 4BFE2A75  bl 0x82e33e00
	ctx.lr = 0x82E51390;
	sub_82E33E00(ctx, base);
	// 82E51390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51394: 4BFA2095  bl 0x82df3428
	ctx.lr = 0x82E51398;
	sub_82DF3428(ctx, base);
	// 82E51398: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E5139C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E513A0: 48356E14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E513A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E513A8 size=84
    let mut pc: u32 = 0x82E513A8;
    'dispatch: loop {
        match pc {
            0x82E513A8 => {
    //   block [0x82E513A8..0x82E513FC)
	// 82E513A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E513AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E513B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E513B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E513B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E513BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E513C0: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E513C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E513C8: 419A0014  beq cr6, 0x82e513dc
	if ctx.cr[6].eq {
	pc = 0x82E513DC; continue 'dispatch;
	}
	// 82E513CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E513D0: 4BFFF8F1  bl 0x82e50cc0
	ctx.lr = 0x82E513D4;
	sub_82E50CC0(ctx, base);
	// 82E513D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E513D8: 4BFA1001  bl 0x82df23d8
	ctx.lr = 0x82E513DC;
	sub_82DF23D8(ctx, base);
	// 82E513DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E513E0: 4BFA1171  bl 0x82df2550
	ctx.lr = 0x82E513E4;
	sub_82DF2550(ctx, base);
	// 82E513E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E513E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E513EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E513F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E513F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E513F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51400 size=444
    let mut pc: u32 = 0x82E51400;
    'dispatch: loop {
        match pc {
            0x82E51400 => {
    //   block [0x82E51400..0x82E515BC)
	// 82E51400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5140C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51414: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E51418: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E5141C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E51420: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51424: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82E51428: 409A0088  bne cr6, 0x82e514b0
	if !ctx.cr[6].eq {
	pc = 0x82E514B0; continue 'dispatch;
	}
	// 82E5142C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51430: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E51434: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E51438: 419A0078  beq cr6, 0x82e514b0
	if ctx.cr[6].eq {
	pc = 0x82E514B0; continue 'dispatch;
	}
	// 82E5143C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E51440: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E51444: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51448: 480018E9  bl 0x82e52d30
	ctx.lr = 0x82E5144C;
	sub_82E52D30(ctx, base);
	// 82E5144C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51450: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E51454: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E51458: 4BFE7E51  bl 0x82e392a8
	ctx.lr = 0x82E5145C;
	sub_82E392A8(ctx, base);
	// 82E5145C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E51460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51464: 419A0008  beq cr6, 0x82e5146c
	if ctx.cr[6].eq {
	pc = 0x82E5146C; continue 'dispatch;
	}
	// 82E51468: 4B46F429  bl 0x822c0890
	ctx.lr = 0x82E5146C;
	sub_822C0890(ctx, base);
	// 82E5146C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51470: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E51474: 814BFFF8  lwz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51478: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E5147C: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E51480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51484: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51488: 419A0118  beq cr6, 0x82e515a0
	if ctx.cr[6].eq {
	pc = 0x82E515A0; continue 'dispatch;
	}
	// 82E5148C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E51490: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E51494: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E51498: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E5149C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E514A0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E514A4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E514A8: 4082FFE8  bne 0x82e51490
	if !ctx.cr[0].eq {
	pc = 0x82E51490; continue 'dispatch;
	}
	// 82E514AC: 480000F4  b 0x82e515a0
	pc = 0x82E515A0; continue 'dispatch;
	// 82E514B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E514B4: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E514B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E514BC: 419A007C  beq cr6, 0x82e51538
	if ctx.cr[6].eq {
	pc = 0x82E51538; continue 'dispatch;
	}
	// 82E514C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E514C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E514C8: 80EB0148  lwz r7, 0x148(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82E514CC: 80C70008  lwz r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E514D0: 48001109  bl 0x82e525d8
	ctx.lr = 0x82E514D4;
	sub_82E525D8(ctx, base);
	// 82E514D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E514D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E514DC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E514E0: 4BFE7DC9  bl 0x82e392a8
	ctx.lr = 0x82E514E4;
	sub_82E392A8(ctx, base);
	// 82E514E4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E514E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E514EC: 419A0008  beq cr6, 0x82e514f4
	if ctx.cr[6].eq {
	pc = 0x82E514F4; continue 'dispatch;
	}
	// 82E514F0: 4B46F3A1  bl 0x822c0890
	ctx.lr = 0x82E514F4;
	sub_822C0890(ctx, base);
	// 82E514F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E514F8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E514FC: 814BFFF8  lwz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51500: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E51504: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E51508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5150C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E51510: 419A0090  beq cr6, 0x82e515a0
	if ctx.cr[6].eq {
	pc = 0x82E515A0; continue 'dispatch;
	}
	// 82E51514: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E51518: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E5151C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E51520: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E51524: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E51528: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E5152C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E51530: 4082FFE8  bne 0x82e51518
	if !ctx.cr[0].eq {
	pc = 0x82E51518; continue 'dispatch;
	}
	// 82E51534: 4800006C  b 0x82e515a0
	pc = 0x82E515A0; continue 'dispatch;
	// 82E51538: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5153C: 48001355  bl 0x82e52890
	ctx.lr = 0x82E51540;
	sub_82E52890(ctx, base);
	// 82E51540: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51544: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E51548: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82E5154C: 4BFE7D5D  bl 0x82e392a8
	ctx.lr = 0x82E51550;
	sub_82E392A8(ctx, base);
	// 82E51550: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E51554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51558: 419A0008  beq cr6, 0x82e51560
	if ctx.cr[6].eq {
	pc = 0x82E51560; continue 'dispatch;
	}
	// 82E5155C: 4B46F335  bl 0x822c0890
	ctx.lr = 0x82E51560;
	sub_822C0890(ctx, base);
	// 82E51560: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51564: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E51568: 814BFFF8  lwz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5156C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E51570: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E51574: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51578: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5157C: 419A0024  beq cr6, 0x82e515a0
	if ctx.cr[6].eq {
	pc = 0x82E515A0; continue 'dispatch;
	}
	// 82E51580: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E51584: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E51588: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E5158C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E51590: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E51594: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E51598: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E5159C: 4082FFE8  bne 0x82e51584
	if !ctx.cr[0].eq {
	pc = 0x82E51584; continue 'dispatch;
	}
	// 82E515A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E515A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E515A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E515AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E515B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E515B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E515B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E515C0 size=12
    let mut pc: u32 = 0x82E515C0;
    'dispatch: loop {
        match pc {
            0x82E515C0 => {
    //   block [0x82E515C0..0x82E515CC)
	// 82E515C0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E515C4: 386B6818  addi r3, r11, 0x6818
	ctx.r[3].s64 = ctx.r[11].s64 + 26648;
	// 82E515C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E515D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E515D0 size=96
    let mut pc: u32 = 0x82E515D0;
    'dispatch: loop {
        match pc {
            0x82E515D0 => {
    //   block [0x82E515D0..0x82E51630)
	// 82E515D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E515D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E515D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E515DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E515E0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E515E4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E515E8: 3BEB680C  addi r31, r11, 0x680c
	ctx.r[31].s64 = ctx.r[11].s64 + 26636;
	// 82E515EC: 816A6814  lwz r11, 0x6814(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26644 as u32) ) } as u64;
	// 82E515F0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E515F4: 40820020  bne 0x82e51614
	if !ctx.cr[0].eq {
	pc = 0x82E51614; continue 'dispatch;
	}
	// 82E515F8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E515FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51600: 916A6814  stw r11, 0x6814(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26644 as u32), ctx.r[11].u32 ) };
	// 82E51604: 4BFFF97D  bl 0x82e50f80
	ctx.lr = 0x82E51608;
	sub_82E50F80(ctx, base);
	// 82E51608: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82E5160C: 386B1E48  addi r3, r11, 0x1e48
	ctx.r[3].s64 = ctx.r[11].s64 + 7752;
	// 82E51610: 48356EC9  bl 0x831a84d8
	ctx.lr = 0x82E51614;
	sub_831A84D8(ctx, base);
	// 82E51614: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E51618: 93EB271C  stw r31, 0x271c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(10012 as u32), ctx.r[31].u32 ) };
	// 82E5161C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51628: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5162C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51630 size=140
    let mut pc: u32 = 0x82E51630;
    'dispatch: loop {
        match pc {
            0x82E51630 => {
    //   block [0x82E51630..0x82E516BC)
	// 82E51630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5163C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51640: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 82E51644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E51648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5164C: 809F271C  lwz r4, 0x271c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82E51650: 4BFA05A9  bl 0x82df1bf8
	ctx.lr = 0x82E51654;
	sub_82DF1BF8(ctx, base);
	// 82E51654: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51658: 4BFFF829  bl 0x82e50e80
	ctx.lr = 0x82E5165C;
	sub_82E50E80(ctx, base);
	// 82E5165C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51660: 4BFA0631  bl 0x82df1c90
	ctx.lr = 0x82E51664;
	sub_82DF1C90(ctx, base);
	// 82E51664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E51668: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5166C: 809F271C  lwz r4, 0x271c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82E51670: 4BFA0589  bl 0x82df1bf8
	ctx.lr = 0x82E51674;
	sub_82DF1BF8(ctx, base);
	// 82E51674: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E51678: 4BFFF6D1  bl 0x82e50d48
	ctx.lr = 0x82E5167C;
	sub_82E50D48(ctx, base);
	// 82E5167C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E51680: 4BFA0611  bl 0x82df1c90
	ctx.lr = 0x82E51684;
	sub_82DF1C90(ctx, base);
	// 82E51684: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E51688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5168C: 392A6818  addi r9, r10, 0x6818
	ctx.r[9].s64 = ctx.r[10].s64 + 26648;
	// 82E51690: 916A6818  stw r11, 0x6818(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26648 as u32), ctx.r[11].u32 ) };
	// 82E51694: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51698: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E5169C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E516A0: 41820008  beq 0x82e516a8
	if ctx.cr[0].eq {
	pc = 0x82E516A8; continue 'dispatch;
	}
	// 82E516A4: 4B46F1ED  bl 0x822c0890
	ctx.lr = 0x82E516A8;
	sub_822C0890(ctx, base);
	// 82E516A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E516AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E516B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E516B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E516B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E516C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E516C0 size=68
    let mut pc: u32 = 0x82E516C0;
    'dispatch: loop {
        match pc {
            0x82E516C0 => {
    //   block [0x82E516C0..0x82E51704)
	// 82E516C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E516C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E516C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E516CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E516D0: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82E516D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E516D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E516DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E516E0: 4E800421  bctrl
	ctx.lr = 0x82E516E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E516E4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E516E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E516EC: 386B6818  addi r3, r11, 0x6818
	ctx.r[3].s64 = ctx.r[11].s64 + 26648;
	// 82E516F0: 4B69AAB9  bl 0x824ec1a8
	ctx.lr = 0x82E516F4;
	sub_824EC1A8(ctx, base);
	// 82E516F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E516F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E516FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51708 size=108
    let mut pc: u32 = 0x82E51708;
    'dispatch: loop {
        match pc {
            0x82E51708 => {
    //   block [0x82E51708..0x82E51774)
	// 82E51708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E51714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5171C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E51724: 4BFFFF9D  bl 0x82e516c0
	ctx.lr = 0x82E51728;
	sub_82E516C0(ctx, base);
	// 82E51728: 4BFFFEA9  bl 0x82e515d0
	ctx.lr = 0x82E5172C;
	sub_82E515D0(ctx, base);
	// 82E5172C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E51730: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E51734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51738: 808B271C  lwz r4, 0x271c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82E5173C: 4BFA04BD  bl 0x82df1bf8
	ctx.lr = 0x82E51740;
	sub_82DF1BF8(ctx, base);
	// 82E51740: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E51744: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51748: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5174C: 4BFFED4D  bl 0x82e50498
	ctx.lr = 0x82E51750;
	sub_82E50498(ctx, base);
	// 82E51750: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51754: 4BFA053D  bl 0x82df1c90
	ctx.lr = 0x82E51758;
	sub_82DF1C90(ctx, base);
	// 82E51758: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E5175C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5176C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E51778 size=68
    let mut pc: u32 = 0x82E51778;
    'dispatch: loop {
        match pc {
            0x82E51778 => {
    //   block [0x82E51778..0x82E517BC)
	// 82E51778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51784: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51788: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E5178C: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E51790: ED6D0032  fmuls f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E51794: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E51798: C1430000  lfs f10, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E5179C: EC2A5B38  fmsubs f1, f10, f12, f11
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E517A0: EC4D02BA  fmadds f2, f13, f10, f0
	ctx.f[2].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E517A4: 48359815  bl 0x831aafb8
	ctx.lr = 0x82E517A8;
	sub_831AAFB8(ctx, base);
	// 82E517A8: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82E517AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E517B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E517B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E517B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E517C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E517C0 size=108
    let mut pc: u32 = 0x82E517C0;
    'dispatch: loop {
        match pc {
            0x82E517C0 => {
    //   block [0x82E517C0..0x82E5182C)
	// 82E517C0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51830 size=92
    let mut pc: u32 = 0x82E51830;
    'dispatch: loop {
        match pc {
            0x82E51830 => {
    //   block [0x82E51830..0x82E5188C)
	// 82E51830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5183C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51848: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5184C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51850: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E51854: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51858: 4E800421  bctrl
	ctx.lr = 0x82E5185C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5185C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51860: 4082000C  bne 0x82e5186c
	if !ctx.cr[0].eq {
	pc = 0x82E5186C; continue 'dispatch;
	}
	// 82E51864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51868: 4800000C  b 0x82e51874
	pc = 0x82E51874; continue 'dispatch;
	// 82E5186C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82E51870: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5187C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51890 size=92
    let mut pc: u32 = 0x82E51890;
    'dispatch: loop {
        match pc {
            0x82E51890 => {
    //   block [0x82E51890..0x82E518EC)
	// 82E51890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51898: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5189C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E518A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E518A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E518A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E518AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E518B0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E518B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E518B8: 4E800421  bctrl
	ctx.lr = 0x82E518BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E518BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E518C0: 4082000C  bne 0x82e518cc
	if !ctx.cr[0].eq {
	pc = 0x82E518CC; continue 'dispatch;
	}
	// 82E518C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E518C8: 4800000C  b 0x82e518d4
	pc = 0x82E518D4; continue 'dispatch;
	// 82E518CC: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82E518D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E518D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E518D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E518DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E518E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E518E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E518E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E518F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E518F0 size=12
    let mut pc: u32 = 0x82E518F0;
    'dispatch: loop {
        match pc {
            0x82E518F0 => {
    //   block [0x82E518F0..0x82E518FC)
	// 82E518F0: D0230038  stfs f1, 0x38(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E518F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E518F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51900 size=8
    let mut pc: u32 = 0x82E51900;
    'dispatch: loop {
        match pc {
            0x82E51900 => {
    //   block [0x82E51900..0x82E51908)
	// 82E51900: 98830036  stb r4, 0x36(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(54 as u32), ctx.r[4].u8 ) };
	// 82E51904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51908 size=8
    let mut pc: u32 = 0x82E51908;
    'dispatch: loop {
        match pc {
            0x82E51908 => {
    //   block [0x82E51908..0x82E51910)
	// 82E51908: 88630036  lbz r3, 0x36(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E5190C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E51910 size=148
    let mut pc: u32 = 0x82E51910;
    'dispatch: loop {
        match pc {
            0x82E51910 => {
    //   block [0x82E51910..0x82E519A4)
	// 82E51910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51914: 48356855  bl 0x831a8168
	ctx.lr = 0x82E51918;
	sub_831A8130(ctx, base);
	// 82E51918: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5191C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51920: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 82E51924: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E51928: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E5192C: 3B810058  addi r28, r1, 0x58
	ctx.r[28].s64 = ctx.r[1].s64 + 88;
	// 82E51930: 4BFA1881  bl 0x82df31b0
	ctx.lr = 0x82E51934;
	sub_82DF31B0(ctx, base);
	// 82E51934: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E51938: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5193C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E51940: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E51944: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E51948: 48001971  bl 0x82e532b8
	ctx.lr = 0x82E5194C;
	sub_82E532B8(ctx, base);
	// 82E5194C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51950: 40820010  bne 0x82e51960
	if !ctx.cr[0].eq {
	pc = 0x82E51960; continue 'dispatch;
	}
	// 82E51954: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E51958: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E5195C: 48000040  b 0x82e5199c
	pc = 0x82E5199C; continue 'dispatch;
	// 82E51960: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51964: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E51968: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E5196C: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E51970: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E51974: C1AADFB0  lfs f13, -0x2050(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51978: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E5197C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E51980: 4198FFD4  blt cr6, 0x82e51954
	if ctx.cr[6].lt {
	pc = 0x82E51954; continue 'dispatch;
	}
	// 82E51984: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E51988: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E5198C: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E51990: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E51994: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E51998: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E5199C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E519A0: 48356818  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E519A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E519A8 size=208
    let mut pc: u32 = 0x82E519A8;
    'dispatch: loop {
        match pc {
            0x82E519A8 => {
    //   block [0x82E519A8..0x82E51A78)
	// 82E519A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E519AC: 483567BD  bl 0x831a8168
	ctx.lr = 0x82E519B0;
	sub_831A8130(ctx, base);
	// 82E519B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E519B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E519B8: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 82E519BC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E519C0: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E519C4: 3B810058  addi r28, r1, 0x58
	ctx.r[28].s64 = ctx.r[1].s64 + 88;
	// 82E519C8: 4BFA17E9  bl 0x82df31b0
	ctx.lr = 0x82E519CC;
	sub_82DF31B0(ctx, base);
	// 82E519CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E519D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E519D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E519D8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E519DC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E519E0: 480018D9  bl 0x82e532b8
	ctx.lr = 0x82E519E4;
	sub_82E532B8(ctx, base);
	// 82E519E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E519E8: 40820010  bne 0x82e519f8
	if !ctx.cr[0].eq {
	pc = 0x82E519F8; continue 'dispatch;
	}
	// 82E519EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E519F0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E519F4: 4800007C  b 0x82e51a70
	pc = 0x82E51A70; continue 'dispatch;
	// 82E519F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E519FC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E51A00: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82E51A04: C8010060  lfd f0, 0x60(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E51A08: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82E51A0C: C00ADFB0  lfs f0, -0x2050(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51A10: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E51A14: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E51A18: 4198FFD4  blt cr6, 0x82e519ec
	if ctx.cr[6].lt {
	pc = 0x82E519EC; continue 'dispatch;
	}
	// 82E51A1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51A24: 419AFFC8  beq cr6, 0x82e519ec
	if ctx.cr[6].eq {
	pc = 0x82E519EC; continue 'dispatch;
	}
	// 82E51A28: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E51A2C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 82E51A30: 388B81F0  addi r4, r11, -0x7e10
	ctx.r[4].s64 = ctx.r[11].s64 + -32272;
	// 82E51A34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51A38: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E51A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51A40: 4E800421  bctrl
	ctx.lr = 0x82E51A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51A44: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51A48: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E51A4C: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 82E51A50: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E51A54: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E51A58: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82E51A5C: C9A10060  lfd f13, 0x60(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E51A60: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E51A64: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E51A68: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E51A6C: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E51A70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E51A74: 48356744  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E51A78 size=8
    let mut pc: u32 = 0x82E51A78;
    'dispatch: loop {
        match pc {
            0x82E51A78 => {
    //   block [0x82E51A78..0x82E51A80)
	// 82E51A78: 80630040  lwz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E51A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51A80 size=160
    let mut pc: u32 = 0x82E51A80;
    'dispatch: loop {
        match pc {
            0x82E51A80 => {
    //   block [0x82E51A80..0x82E51B20)
	// 82E51A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51A88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E51A8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51A98: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E51A9C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E51AA0: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82E51AA4: 482EFB9D  bl 0x83141640
	ctx.lr = 0x82E51AA8;
	sub_83141640(ctx, base);
	// 82E51AA8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82E51AAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51AB0: 41820054  beq 0x82e51b04
	if ctx.cr[0].eq {
	pc = 0x82E51B04; continue 'dispatch;
	}
	// 82E51AB4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E51AB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51ABC: 409A0048  bne cr6, 0x82e51b04
	if !ctx.cr[6].eq {
	pc = 0x82E51B04; continue 'dispatch;
	}
	// 82E51AC0: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82E51AC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51AC8: 4BFA2139  bl 0x82df3c00
	ctx.lr = 0x82E51ACC;
	sub_82DF3C00(ctx, base);
	// 82E51ACC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51AD0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51AD4: 4BFA16DD  bl 0x82df31b0
	ctx.lr = 0x82E51AD8;
	sub_82DF31B0(ctx, base);
	// 82E51AD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E51ADC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E51AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51AE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51AE8: 4E800421  bctrl
	ctx.lr = 0x82E51AEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51AEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E51AF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51AF4: 997F0035  stb r11, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[11].u8 ) };
	// 82E51AF8: 4BFA1931  bl 0x82df3428
	ctx.lr = 0x82E51AFC;
	sub_82DF3428(ctx, base);
	// 82E51AFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51B00: 48000008  b 0x82e51b08
	pc = 0x82E51B08; continue 'dispatch;
	// 82E51B04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51B20 size=52
    let mut pc: u32 = 0x82E51B20;
    'dispatch: loop {
        match pc {
            0x82E51B20 => {
    //   block [0x82E51B20..0x82E51B54)
	// 82E51B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51B30: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 82E51B34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51B38: 4BFA20C9  bl 0x82df3c00
	ctx.lr = 0x82E51B3C;
	sub_82DF3C00(ctx, base);
	// 82E51B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51B40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51B58 size=68
    let mut pc: u32 = 0x82E51B58;
    'dispatch: loop {
        match pc {
            0x82E51B58 => {
    //   block [0x82E51B58..0x82E51B9C)
	// 82E51B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51B64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51B68: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E51B6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51B70: 4E800421  bctrl
	ctx.lr = 0x82E51B74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51B74: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E51B78: 41820010  beq 0x82e51b88
	if ctx.cr[0].eq {
	pc = 0x82E51B88; continue 'dispatch;
	}
	// 82E51B7C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82E51B80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51B84: 409A0008  bne cr6, 0x82e51b8c
	if !ctx.cr[6].eq {
	pc = 0x82E51B8C; continue 'dispatch;
	}
	// 82E51B88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51BA0 size=116
    let mut pc: u32 = 0x82E51BA0;
    'dispatch: loop {
        match pc {
            0x82E51BA0 => {
    //   block [0x82E51BA0..0x82E51C14)
	// 82E51BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51BAC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51BB4: 419A004C  beq cr6, 0x82e51c00
	if ctx.cr[6].eq {
	pc = 0x82E51C00; continue 'dispatch;
	}
	// 82E51BB8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E51BBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51BC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51BC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E51BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51BCC: 4E800421  bctrl
	ctx.lr = 0x82E51BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51BD0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82E51BD4: 4198002C  blt cr6, 0x82e51c00
	if ctx.cr[6].lt {
	pc = 0x82E51C00; continue 'dispatch;
	}
	// 82E51BD8: 419A0020  beq cr6, 0x82e51bf8
	if ctx.cr[6].eq {
	pc = 0x82E51BF8; continue 'dispatch;
	}
	// 82E51BDC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82E51BE0: 41980010  blt cr6, 0x82e51bf0
	if ctx.cr[6].lt {
	pc = 0x82E51BF0; continue 'dispatch;
	}
	// 82E51BE4: 409A001C  bne cr6, 0x82e51c00
	if !ctx.cr[6].eq {
	pc = 0x82E51C00; continue 'dispatch;
	}
	// 82E51BE8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82E51BEC: 48000018  b 0x82e51c04
	pc = 0x82E51C04; continue 'dispatch;
	// 82E51BF0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82E51BF4: 48000010  b 0x82e51c04
	pc = 0x82E51C04; continue 'dispatch;
	// 82E51BF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51BFC: 48000008  b 0x82e51c04
	pc = 0x82E51C04; continue 'dispatch;
	// 82E51C00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51C04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51C18 size=84
    let mut pc: u32 = 0x82E51C18;
    'dispatch: loop {
        match pc {
            0x82E51C18 => {
    //   block [0x82E51C18..0x82E51C6C)
	// 82E51C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51C24: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51C28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51C2C: 419A002C  beq cr6, 0x82e51c58
	if ctx.cr[6].eq {
	pc = 0x82E51C58; continue 'dispatch;
	}
	// 82E51C30: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E51C34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51C38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51C3C: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82E51C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51C44: 4E800421  bctrl
	ctx.lr = 0x82E51C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51C48: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E51C4C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E51C50: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82E51C54: 48000008  b 0x82e51c5c
	pc = 0x82E51C5C; continue 'dispatch;
	// 82E51C58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E51C70 size=216
    let mut pc: u32 = 0x82E51C70;
    'dispatch: loop {
        match pc {
            0x82E51C70 => {
    //   block [0x82E51C70..0x82E51D48)
	// 82E51C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51C7C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E51C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51C88: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E51C8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51C90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51C94: 409A000C  bne cr6, 0x82e51ca0
	if !ctx.cr[6].eq {
	pc = 0x82E51CA0; continue 'dispatch;
	}
	// 82E51C98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51C9C: 48000094  b 0x82e51d30
	pc = 0x82E51D30; continue 'dispatch;
	// 82E51CA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51CA4: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51CA8: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51CAC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E51CB0: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82E51CB4: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E51CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51CBC: 4E800421  bctrl
	ctx.lr = 0x82E51CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51CC0: 897F0034  lbz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E51CC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51CC8: 41820048  beq 0x82e51d10
	if ctx.cr[0].eq {
	pc = 0x82E51D10; continue 'dispatch;
	}
	// 82E51CCC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E51CD0: C1BF0024  lfs f13, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51CD4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51CD8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E51CDC: 409A0034  bne cr6, 0x82e51d10
	if !ctx.cr[6].eq {
	pc = 0x82E51D10; continue 'dispatch;
	}
	// 82E51CE0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51CE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51CE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51CEC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E51CF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51CF4: 4E800421  bctrl
	ctx.lr = 0x82E51CF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51CF8: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E51CFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51D00: 40820010  bne 0x82e51d10
	if !ctx.cr[0].eq {
	pc = 0x82E51D10; continue 'dispatch;
	}
	// 82E51D04: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51D08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51D0C: 409AFF8C  bne cr6, 0x82e51c98
	if !ctx.cr[6].eq {
	pc = 0x82E51C98; continue 'dispatch;
	}
	// 82E51D10: C01F0024  lfs f0, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51D14: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51D18: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E51D1C: 40980010  bge cr6, 0x82e51d2c
	if !ctx.cr[6].lt {
	pc = 0x82E51D2C; continue 'dispatch;
	}
	// 82E51D20: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E51D24: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E51D28: 4BFFFF70  b 0x82e51c98
	pc = 0x82E51C98; continue 'dispatch;
	// 82E51D2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51D30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51D3C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51D40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E51D48 size=240
    let mut pc: u32 = 0x82E51D48;
    'dispatch: loop {
        match pc {
            0x82E51D48 => {
    //   block [0x82E51D48..0x82E51E38)
	// 82E51D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51D54: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E51D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51D5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51D60: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E51D64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51D68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51D6C: 409A000C  bne cr6, 0x82e51d78
	if !ctx.cr[6].eq {
	pc = 0x82E51D78; continue 'dispatch;
	}
	// 82E51D70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E51D74: 480000AC  b 0x82e51e20
	pc = 0x82E51E20; continue 'dispatch;
	// 82E51D78: C01F0030  lfs f0, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51D7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E51D80: C1BF0028  lfs f13, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51D84: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51D88: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E51D8C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E51D90: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51D94: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E51D98: EC206828  fsubs f1, f0, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E51D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51DA0: 4E800421  bctrl
	ctx.lr = 0x82E51DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51DA4: C01F0028  lfs f0, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E51DA8: C1BF0030  lfs f13, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E51DAC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E51DB0: 40980010  bge cr6, 0x82e51dc0
	if !ctx.cr[6].lt {
	pc = 0x82E51DC0; continue 'dispatch;
	}
	// 82E51DB4: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E51DB8: D01F0028  stfs f0, 0x28(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E51DBC: 4BFFFFB4  b 0x82e51d70
	pc = 0x82E51D70; continue 'dispatch;
	// 82E51DC0: 897F0034  lbz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E51DC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51DC8: 41820034  beq 0x82e51dfc
	if ctx.cr[0].eq {
	pc = 0x82E51DFC; continue 'dispatch;
	}
	// 82E51DCC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51DD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51DD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51DD8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E51DDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51DE0: 4E800421  bctrl
	ctx.lr = 0x82E51DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51DE4: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E51DE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51DEC: 40820010  bne 0x82e51dfc
	if !ctx.cr[0].eq {
	pc = 0x82E51DFC; continue 'dispatch;
	}
	// 82E51DF0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E51DF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E51DF8: 409AFF78  bne cr6, 0x82e51d70
	if !ctx.cr[6].eq {
	pc = 0x82E51D70; continue 'dispatch;
	}
	// 82E51DFC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51E00: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E51E04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E51E08: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51E0C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E51E10: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E51E14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51E18: 4E800421  bctrl
	ctx.lr = 0x82E51E1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51E1C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E51E20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51E2C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51E38 size=188
    let mut pc: u32 = 0x82E51E38;
    'dispatch: loop {
        match pc {
            0x82E51E38 => {
    //   block [0x82E51E38..0x82E51EF4)
	// 82E51E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E51E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51E4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E51E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51E54: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E51E58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E51E5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E51E60: 4B46EAD9  bl 0x822c0938
	ctx.lr = 0x82E51E64;
	sub_822C0938(ctx, base);
	// 82E51E64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51E68: 41820028  beq 0x82e51e90
	if ctx.cr[0].eq {
	pc = 0x82E51E90; continue 'dispatch;
	}
	// 82E51E6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51E70: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E51E74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E51E78: 392BC9AC  addi r9, r11, -0x3654
	ctx.r[9].s64 = ctx.r[11].s64 + -13908;
	// 82E51E7C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E51E80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E51E84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E51E88: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E51E8C: 48000008  b 0x82e51e94
	pc = 0x82E51E94; continue 'dispatch;
	// 82E51E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E51E94: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E51E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51E9C: 409A003C  bne cr6, 0x82e51ed8
	if !ctx.cr[6].eq {
	pc = 0x82E51ED8; continue 'dispatch;
	}
	// 82E51EA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E51EA4: 419A0014  beq cr6, 0x82e51eb8
	if ctx.cr[6].eq {
	pc = 0x82E51EB8; continue 'dispatch;
	}
	// 82E51EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51EAC: 480023ED  bl 0x82e54298
	ctx.lr = 0x82E51EB0;
	sub_82E54298(ctx, base);
	// 82E51EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51EB4: 4BFA0525  bl 0x82df23d8
	ctx.lr = 0x82E51EB8;
	sub_82DF23D8(ctx, base);
	// 82E51EB8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E51EBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E51EC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E51EC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E51EC8: 816BAFBC  lwz r11, -0x5044(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20548 as u32) ) } as u64;
	// 82E51ECC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E51ED0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E51ED4: 4B46E12D  bl 0x822c0000
	ctx.lr = 0x82E51ED8;
	sub_822C0000(ctx, base);
	// 82E51ED8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E51EDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E51EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51EE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E51EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51EF8 size=64
    let mut pc: u32 = 0x82E51EF8;
    'dispatch: loop {
        match pc {
            0x82E51EF8 => {
    //   block [0x82E51EF8..0x82E51F38)
	// 82E51EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E51F00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E51F04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51F08: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51F0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E51F10: 419A0014  beq cr6, 0x82e51f24
	if ctx.cr[6].eq {
	pc = 0x82E51F24; continue 'dispatch;
	}
	// 82E51F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51F18: 48002381  bl 0x82e54298
	ctx.lr = 0x82E51F1C;
	sub_82E54298(ctx, base);
	// 82E51F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E51F20: 4BFA04B9  bl 0x82df23d8
	ctx.lr = 0x82E51F24;
	sub_82DF23D8(ctx, base);
	// 82E51F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E51F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E51F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E51F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E51F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51F38 size=108
    let mut pc: u32 = 0x82E51F38;
    'dispatch: loop {
        match pc {
            0x82E51F38 => {
    //   block [0x82E51F38..0x82E51FA4)
	// 82E51F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51F3C: 48356231  bl 0x831a816c
	ctx.lr = 0x82E51F40;
	sub_831A8130(ctx, base);
	// 82E51F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51F44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E51F48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E51F4C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E51F54: 419A0048  beq cr6, 0x82e51f9c
	if ctx.cr[6].eq {
	pc = 0x82E51F9C; continue 'dispatch;
	}
	// 82E51F58: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E51F5C: 3BABC9CC  addi r29, r11, -0x3634
	ctx.r[29].s64 = ctx.r[11].s64 + -13876;
	// 82E51F60: 4BFFF661  bl 0x82e515c0
	ctx.lr = 0x82E51F64;
	sub_82E515C0(ctx, base);
	// 82E51F64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E51F68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E51F6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E51F70: 38C00132  li r6, 0x132
	ctx.r[6].s64 = 306;
	// 82E51F74: 4BFA78C5  bl 0x82df9838
	ctx.lr = 0x82E51F78;
	sub_82DF9838(ctx, base);
	// 82E51F78: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51F7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E51F80: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E51F84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51F88: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E51F8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51F90: 4E800421  bctrl
	ctx.lr = 0x82E51F94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51F94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E51F98: 4B47D669  bl 0x822cf600
	ctx.lr = 0x82E51F9C;
	sub_822CF600(ctx, base);
	// 82E51F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E51FA0: 4835621C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E51FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E51FA8 size=752
    let mut pc: u32 = 0x82E51FA8;
    'dispatch: loop {
        match pc {
            0x82E51FA8 => {
    //   block [0x82E51FA8..0x82E52298)
	// 82E51FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E51FAC: 483561A5  bl 0x831a8150
	ctx.lr = 0x82E51FB0;
	sub_831A8130(ctx, base);
	// 82E51FB0: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82E51FB4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E51FB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E51FBC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E51FC0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E51FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E51FC8: 419A02C4  beq cr6, 0x82e5228c
	if ctx.cr[6].eq {
	pc = 0x82E5228C; continue 'dispatch;
	}
	// 82E51FCC: 897E0037  lbz r11, 0x37(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(55 as u32) ) } as u64;
	// 82E51FD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E51FD4: 41820018  beq 0x82e51fec
	if ctx.cr[0].eq {
	pc = 0x82E51FEC; continue 'dispatch;
	}
	// 82E51FD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E51FDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E51FE0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E51FE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E51FE8: 4E800421  bctrl
	ctx.lr = 0x82E51FEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E51FEC: 82FE0018  lwz r23, 0x18(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E51FF0: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E51FF4: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E51FF8: 419A0294  beq cr6, 0x82e5228c
	if ctx.cr[6].eq {
	pc = 0x82E5228C; continue 'dispatch;
	}
	// 82E51FFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82E52000: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E52004: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E52008: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82E5200C: 3CE08208  lis r7, -0x7df8
	ctx.r[7].s64 = -2113404928;
	// 82E52010: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82E52014: 3BAB0570  addi r29, r11, 0x570
	ctx.r[29].s64 = ctx.r[11].s64 + 1392;
	// 82E52018: 3B8ACA20  addi r28, r10, -0x35e0
	ctx.r[28].s64 = ctx.r[10].s64 + -13792;
	// 82E5201C: 3B69CA18  addi r27, r9, -0x35e8
	ctx.r[27].s64 = ctx.r[9].s64 + -13800;
	// 82E52020: 3B48CA10  addi r26, r8, -0x35f0
	ctx.r[26].s64 = ctx.r[8].s64 + -13808;
	// 82E52024: 3B27E7E4  addi r25, r7, -0x181c
	ctx.r[25].s64 = ctx.r[7].s64 + -6172;
	// 82E52028: 3B06CA08  addi r24, r6, -0x35f8
	ctx.r[24].s64 = ctx.r[6].s64 + -13816;
	// 82E5202C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E52030: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52034: 4BFA19D5  bl 0x82df3a08
	ctx.lr = 0x82E52038;
	sub_82DF3A08(ctx, base);
	// 82E52038: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E5203C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52040: 4BFA12C9  bl 0x82df3308
	ctx.lr = 0x82E52044;
	sub_82DF3308(ctx, base);
	// 82E52044: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E52048: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E5204C: 4BFA13DD  bl 0x82df3428
	ctx.lr = 0x82E52050;
	sub_82DF3428(ctx, base);
	// 82E52050: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52054: 408200F0  bne 0x82e52144
	if !ctx.cr[0].eq {
	pc = 0x82E52144; continue 'dispatch;
	}
	// 82E52058: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E5205C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E52060: 4BFA19A9  bl 0x82df3a08
	ctx.lr = 0x82E52064;
	sub_82DF3A08(ctx, base);
	// 82E52064: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E52068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5206C: 4BFA129D  bl 0x82df3308
	ctx.lr = 0x82E52070;
	sub_82DF3308(ctx, base);
	// 82E52070: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E52074: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E52078: 4BFA13B1  bl 0x82df3428
	ctx.lr = 0x82E5207C;
	sub_82DF3428(ctx, base);
	// 82E5207C: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52080: 40820174  bne 0x82e521f4
	if !ctx.cr[0].eq {
	pc = 0x82E521F4; continue 'dispatch;
	}
	// 82E52084: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E52088: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E5208C: 4BFA197D  bl 0x82df3a08
	ctx.lr = 0x82E52090;
	sub_82DF3A08(ctx, base);
	// 82E52090: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E52094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52098: 4BFA1271  bl 0x82df3308
	ctx.lr = 0x82E5209C;
	sub_82DF3308(ctx, base);
	// 82E5209C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E520A0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E520A4: 4BFA1385  bl 0x82df3428
	ctx.lr = 0x82E520A8;
	sub_82DF3428(ctx, base);
	// 82E520A8: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E520AC: 4082017C  bne 0x82e52228
	if !ctx.cr[0].eq {
	pc = 0x82E52228; continue 'dispatch;
	}
	// 82E520B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E520B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E520B8: 4BFA1951  bl 0x82df3a08
	ctx.lr = 0x82E520BC;
	sub_82DF3A08(ctx, base);
	// 82E520BC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E520C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E520C4: 4BFA1245  bl 0x82df3308
	ctx.lr = 0x82E520C8;
	sub_82DF3308(ctx, base);
	// 82E520C8: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E520CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E520D0: 4BFA1359  bl 0x82df3428
	ctx.lr = 0x82E520D4;
	sub_82DF3428(ctx, base);
	// 82E520D4: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E520D8: 40820170  bne 0x82e52248
	if !ctx.cr[0].eq {
	pc = 0x82E52248; continue 'dispatch;
	}
	// 82E520DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E520E0: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E520E4: 4BFA1925  bl 0x82df3a08
	ctx.lr = 0x82E520E8;
	sub_82DF3A08(ctx, base);
	// 82E520E8: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82E520EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E520F0: 4BFA1219  bl 0x82df3308
	ctx.lr = 0x82E520F4;
	sub_82DF3308(ctx, base);
	// 82E520F4: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E520F8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E520FC: 4BFA132D  bl 0x82df3428
	ctx.lr = 0x82E52100;
	sub_82DF3428(ctx, base);
	// 82E52100: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52104: 4082014C  bne 0x82e52250
	if !ctx.cr[0].eq {
	pc = 0x82E52250; continue 'dispatch;
	}
	// 82E52108: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E5210C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E52110: 4BFA18F9  bl 0x82df3a08
	ctx.lr = 0x82E52114;
	sub_82DF3A08(ctx, base);
	// 82E52114: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E52118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5211C: 4BFA11ED  bl 0x82df3308
	ctx.lr = 0x82E52120;
	sub_82DF3308(ctx, base);
	// 82E52120: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E52124: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E52128: 4BFA1301  bl 0x82df3428
	ctx.lr = 0x82E5212C;
	sub_82DF3428(ctx, base);
	// 82E5212C: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52130: 4082012C  bne 0x82e5225c
	if !ctx.cr[0].eq {
	pc = 0x82E5225C; continue 'dispatch;
	}
	// 82E52134: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E52138: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E5213C: 409AFEF0  bne cr6, 0x82e5202c
	if !ctx.cr[6].eq {
	pc = 0x82E5202C; continue 'dispatch;
	}
	// 82E52140: 4800014C  b 0x82e5228c
	pc = 0x82E5228C; continue 'dispatch;
	// 82E52144: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E52148: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5214C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52150: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E52154: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52158: 4E800421  bctrl
	ctx.lr = 0x82E5215C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5215C: 897E0036  lbz r11, 0x36(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E52160: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52164: 40820118  bne 0x82e5227c
	if !ctx.cr[0].eq {
	pc = 0x82E5227C; continue 'dispatch;
	}
	// 82E52168: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5216C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52170: 419A010C  beq cr6, 0x82e5227c
	if ctx.cr[6].eq {
	pc = 0x82E5227C; continue 'dispatch;
	}
	// 82E52174: 3BFE0020  addi r31, r30, 0x20
	ctx.r[31].s64 = ctx.r[30].s64 + 32;
	// 82E52178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5217C: 4BFA1035  bl 0x82df31b0
	ctx.lr = 0x82E52180;
	sub_82DF31B0(ctx, base);
	// 82E52180: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E52184: 396B9BC9  addi r11, r11, -0x6437
	ctx.r[11].s64 = ctx.r[11].s64 + -25655;
	// 82E52188: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5218C: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52190: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E52194: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E52198: 41820014  beq 0x82e521ac
	if ctx.cr[0].eq {
	pc = 0x82E521AC; continue 'dispatch;
	}
	// 82E5219C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E521A0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E521A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E521A8: 419AFFE0  beq cr6, 0x82e52188
	if ctx.cr[6].eq {
	pc = 0x82E52188; continue 'dispatch;
	}
	// 82E521AC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E521B0: 418200DC  beq 0x82e5228c
	if ctx.cr[0].eq {
	pc = 0x82E5228C; continue 'dispatch;
	}
	// 82E521B4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E521B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E521BC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E521C0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E521C4: 4BFA0FED  bl 0x82df31b0
	ctx.lr = 0x82E521C8;
	sub_82DF31B0(ctx, base);
	// 82E521C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E521CC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E521D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E521D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E521D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E521DC: 4E800421  bctrl
	ctx.lr = 0x82E521E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E521E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E521E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E521E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E521EC: 997E0037  stb r11, 0x37(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E521F0: 4800009C  b 0x82e5228c
	pc = 0x82E5228C; continue 'dispatch;
	// 82E521F4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E521F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E521FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52200: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E52204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52208: 4E800421  bctrl
	ctx.lr = 0x82E5220C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5220C: 897E0036  lbz r11, 0x36(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E52210: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52214: 40820068  bne 0x82e5227c
	if !ctx.cr[0].eq {
	pc = 0x82E5227C; continue 'dispatch;
	}
	// 82E52218: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E5221C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52220: 409A006C  bne cr6, 0x82e5228c
	if !ctx.cr[6].eq {
	pc = 0x82E5228C; continue 'dispatch;
	}
	// 82E52224: 48000058  b 0x82e5227c
	pc = 0x82E5227C; continue 'dispatch;
	// 82E52228: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E5222C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E52230: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E52234: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52238: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E5223C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52240: 4E800421  bctrl
	ctx.lr = 0x82E52244;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52244: 48000038  b 0x82e5227c
	pc = 0x82E5227C; continue 'dispatch;
	// 82E52248: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E5224C: 4BFFFFE0  b 0x82e5222c
	pc = 0x82E5222C; continue 'dispatch;
	// 82E52250: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52254: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E52258: 4800000C  b 0x82e52264
	pc = 0x82E52264; continue 'dispatch;
	// 82E5225C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52260: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E52264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E52268: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E5226C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52270: 4E800421  bctrl
	ctx.lr = 0x82E52274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52278: 41820014  beq 0x82e5228c
	if ctx.cr[0].eq {
	pc = 0x82E5228C; continue 'dispatch;
	}
	// 82E5227C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52280: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E52284: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E52288: 4BFFE411  bl 0x82e50698
	ctx.lr = 0x82E5228C;
	sub_82E50698(ctx, base);
	// 82E5228C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E52290: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82E52294: 48355F0C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52298 size=200
    let mut pc: u32 = 0x82E52298;
    'dispatch: loop {
        match pc {
            0x82E52298 => {
    //   block [0x82E52298..0x82E52360)
	// 82E52298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5229C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E522A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E522A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E522A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E522AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E522B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E522B4: 419A0084  beq cr6, 0x82e52338
	if ctx.cr[6].eq {
	pc = 0x82E52338; continue 'dispatch;
	}
	// 82E522B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E522BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E522C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E522C4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E522C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E522CC: 4E800421  bctrl
	ctx.lr = 0x82E522D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E522D0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E522D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E522D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E522DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E522E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E522E4: 4E800421  bctrl
	ctx.lr = 0x82E522E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E522E8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E522EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E522F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E522F4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E522F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E522FC: 4E800421  bctrl
	ctx.lr = 0x82E52300;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52300: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E52304: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E52308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5230C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E52310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52314: 4E800421  bctrl
	ctx.lr = 0x82E52318;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52318: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5231C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E52320: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52324: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5232C: 4E800421  bctrl
	ctx.lr = 0x82E52330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52334: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E52338: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82E5233C: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E52340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52344: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E52348: 4B580019  bl 0x823d2360
	ctx.lr = 0x82E5234C;
	sub_823D2360(ctx, base);
	// 82E5234C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E52350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5235C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52360 size=96
    let mut pc: u32 = 0x82E52360;
    'dispatch: loop {
        match pc {
            0x82E52360 => {
    //   block [0x82E52360..0x82E523C0)
	// 82E52360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5236C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52378: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5237C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E52380: 4BFA10A9  bl 0x82df3428
	ctx.lr = 0x82E52384;
	sub_82DF3428(ctx, base);
	// 82E52384: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E52388: 4BFC9A01  bl 0x82e1bd88
	ctx.lr = 0x82E5238C;
	sub_82E1BD88(ctx, base);
	// 82E5238C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E52390: 4BFA1099  bl 0x82df3428
	ctx.lr = 0x82E52394;
	sub_82DF3428(ctx, base);
	// 82E52394: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52398: 4182000C  beq 0x82e523a4
	if ctx.cr[0].eq {
	pc = 0x82E523A4; continue 'dispatch;
	}
	// 82E5239C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E523A0: 4BFA0039  bl 0x82df23d8
	ctx.lr = 0x82E523A4;
	sub_82DF23D8(ctx, base);
	// 82E523A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E523A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E523AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E523B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E523B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E523B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E523BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E523C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E523C0 size=176
    let mut pc: u32 = 0x82E523C0;
    'dispatch: loop {
        match pc {
            0x82E523C0 => {
    //   block [0x82E523C0..0x82E52470)
	// 82E523C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E523C4: 48355DA9  bl 0x831a816c
	ctx.lr = 0x82E523C8;
	sub_831A8130(ctx, base);
	// 82E523C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E523CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E523D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E523D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E523D8: 396BCA28  addi r11, r11, -0x35d8
	ctx.r[11].s64 = ctx.r[11].s64 + -13784;
	// 82E523DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E523E0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E523E4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E523E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E523EC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E523F0: 4BFA1619  bl 0x82df3a08
	ctx.lr = 0x82E523F4;
	sub_82DF3A08(ctx, base);
	// 82E523F4: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E523F8: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82E523FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E52400: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82E52404: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E52408: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E5240C: 388BCE10  addi r4, r11, -0x31f0
	ctx.r[4].s64 = ctx.r[11].s64 + -12784;
	// 82E52410: 4BFA15F9  bl 0x82df3a08
	ctx.lr = 0x82E52414;
	sub_82DF3A08(ctx, base);
	// 82E52414: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E52418: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E5241C: 9BDF0034  stb r30, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 82E52420: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52424: 9BDF0037  stb r30, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[30].u8 ) };
	// 82E52428: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E5242C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82E52430: 997F0036  stb r11, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[11].u8 ) };
	// 82E52434: C1AA08A4  lfs f13, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E52438: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82E5243C: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E52440: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E52444: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E52448: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E5244C: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E52450: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 82E52454: 409A000C  bne cr6, 0x82e52460
	if !ctx.cr[6].eq {
	pc = 0x82E52460; continue 'dispatch;
	}
	// 82E52458: 9BDF0035  stb r30, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[30].u8 ) };
	// 82E5245C: 48000008  b 0x82e52464
	pc = 0x82E52464; continue 'dispatch;
	// 82E52460: 997F0035  stb r11, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[11].u8 ) };
	// 82E52464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5246C: 48355D50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52470 size=184
    let mut pc: u32 = 0x82E52470;
    'dispatch: loop {
        match pc {
            0x82E52470 => {
    //   block [0x82E52470..0x82E52528)
	// 82E52470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52478: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5247C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E52488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5248C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E52490: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E52494: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52498: 4B46E4A1  bl 0x822c0938
	ctx.lr = 0x82E5249C;
	sub_822C0938(ctx, base);
	// 82E5249C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E524A0: 41820028  beq 0x82e524c8
	if ctx.cr[0].eq {
	pc = 0x82E524C8; continue 'dispatch;
	}
	// 82E524A4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E524A8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E524AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E524B0: 392BC9BC  addi r9, r11, -0x3644
	ctx.r[9].s64 = ctx.r[11].s64 + -13892;
	// 82E524B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E524B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E524BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E524C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E524C4: 48000008  b 0x82e524cc
	pc = 0x82E524CC; continue 'dispatch;
	// 82E524C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E524CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E524D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E524D4: 409A0038  bne cr6, 0x82e5250c
	if !ctx.cr[6].eq {
	pc = 0x82E5250C; continue 'dispatch;
	}
	// 82E524D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E524DC: 419A0010  beq cr6, 0x82e524ec
	if ctx.cr[6].eq {
	pc = 0x82E524EC; continue 'dispatch;
	}
	// 82E524E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E524E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E524E8: 4BFFFE79  bl 0x82e52360
	ctx.lr = 0x82E524EC;
	sub_82E52360(ctx, base);
	// 82E524EC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E524F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E524F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E524F8: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E524FC: 816BAFBC  lwz r11, -0x5044(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20548 as u32) ) } as u64;
	// 82E52500: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E52504: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E52508: 4B46DAF9  bl 0x822c0000
	ctx.lr = 0x82E5250C;
	sub_822C0000(ctx, base);
	// 82E5250C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E52510: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E52514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5251C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E52520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E52524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52528 size=12
    let mut pc: u32 = 0x82E52528;
    'dispatch: loop {
        match pc {
            0x82E52528 => {
    //   block [0x82E52528..0x82E52534)
	// 82E52528: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5252C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E52530: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E52534 size=8
    let mut pc: u32 = 0x82E52534;
    'dispatch: loop {
        match pc {
            0x82E52534 => {
    //   block [0x82E52534..0x82E5253C)
	// 82E52534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E52538: 4BFFFE28  b 0x82e52360
	sub_82E52360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E5253C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E5253C size=4
    let mut pc: u32 = 0x82E5253C;
    'dispatch: loop {
        match pc {
            0x82E5253C => {
    //   block [0x82E5253C..0x82E52540)
	// 82E5253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E52540 size=152
    let mut pc: u32 = 0x82E52540;
    'dispatch: loop {
        match pc {
            0x82E52540 => {
    //   block [0x82E52540..0x82E525D8)
	// 82E52540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52544: 48355C25  bl 0x831a8168
	ctx.lr = 0x82E52548;
	sub_831A8130(ctx, base);
	// 82E52548: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82E5254C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52550: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E52554: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E52558: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E5255C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82E52560: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E52564: 4BFE189D  bl 0x82e33e00
	ctx.lr = 0x82E52568;
	sub_82E33E00(ctx, base);
	// 82E52568: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5256C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52570: 388BCA20  addi r4, r11, -0x35e0
	ctx.r[4].s64 = ctx.r[11].s64 + -13792;
	// 82E52574: 4BFA1A55  bl 0x82df3fc8
	ctx.lr = 0x82E52578;
	sub_82DF3FC8(ctx, base);
	// 82E52578: 3FA08212  lis r29, -0x7dee
	ctx.r[29].s64 = -2112749568;
	// 82E5257C: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E52580: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E52584: 419A0018  beq cr6, 0x82e5259c
	if ctx.cr[6].eq {
	pc = 0x82E5259C; continue 'dispatch;
	}
	// 82E52588: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E5258C: D3FE002C  stfs f31, 0x2c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82E52590: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E52594: D01E0024  stfs f0, 0x24(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82E52598: 48000034  b 0x82e525cc
	pc = 0x82E525CC; continue 'dispatch;
	// 82E5259C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82E525A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E525A4: 388B0570  addi r4, r11, 0x570
	ctx.r[4].s64 = ctx.r[11].s64 + 1392;
	// 82E525A8: 4BFA1A21  bl 0x82df3fc8
	ctx.lr = 0x82E525AC;
	sub_82DF3FC8(ctx, base);
	// 82E525AC: 817DB230  lwz r11, -0x4dd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 82E525B0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E525B4: 419A0018  beq cr6, 0x82e525cc
	if ctx.cr[6].eq {
	pc = 0x82E525CC; continue 'dispatch;
	}
	// 82E525B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E525BC: D3FE0030  stfs f31, 0x30(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82E525C0: 9B9E0034  stb r28, 0x34(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[28].u8 ) };
	// 82E525C4: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E525C8: D01E0028  stfs f0, 0x28(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82E525CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E525D0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E525D4: 48355BE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E525D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E525D8 size=692
    let mut pc: u32 = 0x82E525D8;
    'dispatch: loop {
        match pc {
            0x82E525D8 => {
    //   block [0x82E525D8..0x82E5288C)
	// 82E525D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E525DC: 48355B7D  bl 0x831a8158
	ctx.lr = 0x82E525E0;
	sub_831A8130(ctx, base);
	// 82E525E0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E525E4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E525E8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E525EC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82E525F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E525F4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E525F8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82E525FC: 3BEBC9CC  addi r31, r11, -0x3634
	ctx.r[31].s64 = ctx.r[11].s64 + -13876;
	// 82E52600: 4BFFEFC1  bl 0x82e515c0
	ctx.lr = 0x82E52604;
	sub_82E515C0(ctx, base);
	// 82E52604: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E52608: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5260C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52610: 38C001E9  li r6, 0x1e9
	ctx.r[6].s64 = 489;
	// 82E52614: 4BFA7225  bl 0x82df9838
	ctx.lr = 0x82E52618;
	sub_82DF9838(ctx, base);
	// 82E52618: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5261C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E52620: 482EF021  bl 0x83141640
	ctx.lr = 0x82E52624;
	sub_83141640(ctx, base);
	// 82E52624: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E52628: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5262C: 4B47CFD5  bl 0x822cf600
	ctx.lr = 0x82E52630;
	sub_822CF600(ctx, base);
	// 82E52630: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E52634: 3B9A0004  addi r28, r26, 4
	ctx.r[28].s64 = ctx.r[26].s64 + 4;
	// 82E52638: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E5263C: 937A0004  stw r27, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E52640: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52644: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E52648: 409A004C  bne cr6, 0x82e52694
	if !ctx.cr[6].eq {
	pc = 0x82E52694; continue 'dispatch;
	}
	// 82E5264C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52650: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52654: 38A001F3  li r5, 0x1f3
	ctx.r[5].s64 = 499;
	// 82E52658: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 82E5265C: 4BF9FD8D  bl 0x82df23e8
	ctx.lr = 0x82E52660;
	sub_82DF23E8(ctx, base);
	// 82E52660: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52664: 41820018  beq 0x82e5267c
	if ctx.cr[0].eq {
	pc = 0x82E5267C; continue 'dispatch;
	}
	// 82E52668: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5266C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E52670: 48001C91  bl 0x82e54300
	ctx.lr = 0x82E52674;
	sub_82E54300(ctx, base);
	// 82E52674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52678: 48000008  b 0x82e52680
	pc = 0x82E52680; continue 'dispatch;
	// 82E5267C: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E52680: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52688: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E5268C: 4BFFF7AD  bl 0x82e51e38
	ctx.lr = 0x82E52690;
	sub_82E51E38(ctx, base);
	// 82E52690: 4800013C  b 0x82e527cc
	pc = 0x82E527CC; continue 'dispatch;
	// 82E52694: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52698: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E5269C: 419800EC  blt cr6, 0x82e52788
	if ctx.cr[6].lt {
	pc = 0x82E52788; continue 'dispatch;
	}
	// 82E526A0: 419A00A0  beq cr6, 0x82e52740
	if ctx.cr[6].eq {
	pc = 0x82E52740; continue 'dispatch;
	}
	// 82E526A4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E526A8: 41980050  blt cr6, 0x82e526f8
	if ctx.cr[6].lt {
	pc = 0x82E526F8; continue 'dispatch;
	}
	// 82E526AC: 409A0154  bne cr6, 0x82e52800
	if !ctx.cr[6].eq {
	pc = 0x82E52800; continue 'dispatch;
	}
	// 82E526B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E526B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E526B8: 38A0020A  li r5, 0x20a
	ctx.r[5].s64 = 522;
	// 82E526BC: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82E526C0: 4BF9FD29  bl 0x82df23e8
	ctx.lr = 0x82E526C4;
	sub_82DF23E8(ctx, base);
	// 82E526C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E526C8: 41820018  beq 0x82e526e0
	if ctx.cr[0].eq {
	pc = 0x82E526E0; continue 'dispatch;
	}
	// 82E526CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E526D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E526D4: 48002EF5  bl 0x82e555c8
	ctx.lr = 0x82E526D8;
	sub_82E555C8(ctx, base);
	// 82E526D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E526DC: 48000008  b 0x82e526e4
	pc = 0x82E526E4; continue 'dispatch;
	// 82E526E0: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E526E4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E526E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E526EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E526F0: 4BFFFD81  bl 0x82e52470
	ctx.lr = 0x82E526F4;
	sub_82E52470(ctx, base);
	// 82E526F4: 480000D8  b 0x82e527cc
	pc = 0x82E527CC; continue 'dispatch;
	// 82E526F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E526FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52700: 38A00205  li r5, 0x205
	ctx.r[5].s64 = 517;
	// 82E52704: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82E52708: 4BF9FCE1  bl 0x82df23e8
	ctx.lr = 0x82E5270C;
	sub_82DF23E8(ctx, base);
	// 82E5270C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52710: 41820018  beq 0x82e52728
	if ctx.cr[0].eq {
	pc = 0x82E52728; continue 'dispatch;
	}
	// 82E52714: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E52718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5271C: 48002B35  bl 0x82e55250
	ctx.lr = 0x82E52720;
	sub_82E55250(ctx, base);
	// 82E52720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52724: 48000008  b 0x82e5272c
	pc = 0x82E5272C; continue 'dispatch;
	// 82E52728: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E5272C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52730: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52734: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52738: 4BFFFD39  bl 0x82e52470
	ctx.lr = 0x82E5273C;
	sub_82E52470(ctx, base);
	// 82E5273C: 48000090  b 0x82e527cc
	pc = 0x82E527CC; continue 'dispatch;
	// 82E52740: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52748: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82E5274C: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82E52750: 4BF9FC99  bl 0x82df23e8
	ctx.lr = 0x82E52754;
	sub_82DF23E8(ctx, base);
	// 82E52754: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52758: 41820018  beq 0x82e52770
	if ctx.cr[0].eq {
	pc = 0x82E52770; continue 'dispatch;
	}
	// 82E5275C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E52760: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E52764: 48002995  bl 0x82e550f8
	ctx.lr = 0x82E52768;
	sub_82E550F8(ctx, base);
	// 82E52768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5276C: 48000008  b 0x82e52774
	pc = 0x82E52774; continue 'dispatch;
	// 82E52770: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E52774: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52778: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5277C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52780: 4BFFFCF1  bl 0x82e52470
	ctx.lr = 0x82E52784;
	sub_82E52470(ctx, base);
	// 82E52784: 48000048  b 0x82e527cc
	pc = 0x82E527CC; continue 'dispatch;
	// 82E52788: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52790: 38A001FB  li r5, 0x1fb
	ctx.r[5].s64 = 507;
	// 82E52794: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82E52798: 4BF9FC51  bl 0x82df23e8
	ctx.lr = 0x82E5279C;
	sub_82DF23E8(ctx, base);
	// 82E5279C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E527A0: 41820018  beq 0x82e527b8
	if ctx.cr[0].eq {
	pc = 0x82E527B8; continue 'dispatch;
	}
	// 82E527A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E527A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E527AC: 48002065  bl 0x82e54810
	ctx.lr = 0x82E527B0;
	sub_82E54810(ctx, base);
	// 82E527B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E527B4: 48000008  b 0x82e527bc
	pc = 0x82E527BC; continue 'dispatch;
	// 82E527B8: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E527BC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E527C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E527C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E527C8: 4BFFFCA9  bl 0x82e52470
	ctx.lr = 0x82E527CC;
	sub_82E52470(ctx, base);
	// 82E527CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E527D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E527D4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E527D8: 4B46D829  bl 0x822c0000
	ctx.lr = 0x82E527DC;
	sub_822C0000(ctx, base);
	// 82E527DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E527E0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E527E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E527E8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E527EC: 4B471C75  bl 0x822c4460
	ctx.lr = 0x82E527F0;
	sub_822C4460(ctx, base);
	// 82E527F0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E527F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E527F8: 419A0008  beq cr6, 0x82e52800
	if ctx.cr[6].eq {
	pc = 0x82E52800; continue 'dispatch;
	}
	// 82E527FC: 4B46E095  bl 0x822c0890
	ctx.lr = 0x82E52800;
	sub_822C0890(ctx, base);
	// 82E52800: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E52808: 419A0078  beq cr6, 0x82e52880
	if ctx.cr[6].eq {
	pc = 0x82E52880; continue 'dispatch;
	}
	// 82E5280C: 930B0004  stw r24, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82E52810: 3BF90008  addi r31, r25, 8
	ctx.r[31].s64 = ctx.r[25].s64 + 8;
	// 82E52814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52818: 4BFA0999  bl 0x82df31b0
	ctx.lr = 0x82E5281C;
	sub_82DF31B0(ctx, base);
	// 82E5281C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E52820: 396B9BC9  addi r11, r11, -0x6437
	ctx.r[11].s64 = ctx.r[11].s64 + -25655;
	// 82E52824: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52828: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5282C: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E52830: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E52834: 41820014  beq 0x82e52848
	if ctx.cr[0].eq {
	pc = 0x82E52848; continue 'dispatch;
	}
	// 82E52838: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5283C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E52840: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E52844: 419AFFE0  beq cr6, 0x82e52824
	if ctx.cr[6].eq {
	pc = 0x82E52824; continue 'dispatch;
	}
	// 82E52848: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E5284C: 41820028  beq 0x82e52874
	if ctx.cr[0].eq {
	pc = 0x82E52874; continue 'dispatch;
	}
	// 82E52850: 83DA0000  lwz r30, 0(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52858: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5285C: 4BFA0955  bl 0x82df31b0
	ctx.lr = 0x82E52860;
	sub_82DF31B0(ctx, base);
	// 82E52860: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E52864: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52868: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5286C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52870: 4E800421  bctrl
	ctx.lr = 0x82E52874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52874: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52878: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5287C: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82E52880: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E52884: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E52888: 48355920  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52890 size=516
    let mut pc: u32 = 0x82E52890;
    'dispatch: loop {
        match pc {
            0x82E52890 => {
    //   block [0x82E52890..0x82E52A94)
	// 82E52890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52894: 483558D1  bl 0x831a8164
	ctx.lr = 0x82E52898;
	sub_831A8130(ctx, base);
	// 82E52898: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5289C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E528A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E528A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E528A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E528AC: 3B9D0004  addi r28, r29, 4
	ctx.r[28].s64 = ctx.r[29].s64 + 4;
	// 82E528B0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E528B4: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E528B8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E528BC: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E528C0: 409A0044  bne cr6, 0x82e52904
	if !ctx.cr[6].eq {
	pc = 0x82E52904; continue 'dispatch;
	}
	// 82E528C4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E528C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E528CC: 388BC9CC  addi r4, r11, -0x3634
	ctx.r[4].s64 = ctx.r[11].s64 + -13876;
	// 82E528D0: 38A0022F  li r5, 0x22f
	ctx.r[5].s64 = 559;
	// 82E528D4: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 82E528D8: 4BF9FB11  bl 0x82df23e8
	ctx.lr = 0x82E528DC;
	sub_82DF23E8(ctx, base);
	// 82E528DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E528E0: 41820010  beq 0x82e528f0
	if ctx.cr[0].eq {
	pc = 0x82E528F0; continue 'dispatch;
	}
	// 82E528E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E528E8: 48001961  bl 0x82e54248
	ctx.lr = 0x82E528EC;
	sub_82E54248(ctx, base);
	// 82E528EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E528F0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E528F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E528F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E528FC: 4BFFF53D  bl 0x82e51e38
	ctx.lr = 0x82E52900;
	sub_82E51E38(ctx, base);
	// 82E52900: 4800011C  b 0x82e52a1c
	pc = 0x82E52A1C; continue 'dispatch;
	// 82E52904: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52908: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E5290C: 419800D4  blt cr6, 0x82e529e0
	if ctx.cr[6].lt {
	pc = 0x82E529E0; continue 'dispatch;
	}
	// 82E52910: 419A0090  beq cr6, 0x82e529a0
	if ctx.cr[6].eq {
	pc = 0x82E529A0; continue 'dispatch;
	}
	// 82E52914: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E52918: 41980048  blt cr6, 0x82e52960
	if ctx.cr[6].lt {
	pc = 0x82E52960; continue 'dispatch;
	}
	// 82E5291C: 409A0134  bne cr6, 0x82e52a50
	if !ctx.cr[6].eq {
	pc = 0x82E52A50; continue 'dispatch;
	}
	// 82E52920: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52928: 388BC9CC  addi r4, r11, -0x3634
	ctx.r[4].s64 = ctx.r[11].s64 + -13876;
	// 82E5292C: 38A00246  li r5, 0x246
	ctx.r[5].s64 = 582;
	// 82E52930: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82E52934: 4BF9FAB5  bl 0x82df23e8
	ctx.lr = 0x82E52938;
	sub_82DF23E8(ctx, base);
	// 82E52938: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5293C: 41820010  beq 0x82e5294c
	if ctx.cr[0].eq {
	pc = 0x82E5294C; continue 'dispatch;
	}
	// 82E52940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E52944: 48002C2D  bl 0x82e55570
	ctx.lr = 0x82E52948;
	sub_82E55570(ctx, base);
	// 82E52948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5294C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52950: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52954: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52958: 4BFFFB19  bl 0x82e52470
	ctx.lr = 0x82E5295C;
	sub_82E52470(ctx, base);
	// 82E5295C: 480000C0  b 0x82e52a1c
	pc = 0x82E52A1C; continue 'dispatch;
	// 82E52960: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52968: 388BC9CC  addi r4, r11, -0x3634
	ctx.r[4].s64 = ctx.r[11].s64 + -13876;
	// 82E5296C: 38A00241  li r5, 0x241
	ctx.r[5].s64 = 577;
	// 82E52970: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82E52974: 4BF9FA75  bl 0x82df23e8
	ctx.lr = 0x82E52978;
	sub_82DF23E8(ctx, base);
	// 82E52978: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5297C: 41820010  beq 0x82e5298c
	if ctx.cr[0].eq {
	pc = 0x82E5298C; continue 'dispatch;
	}
	// 82E52980: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E52984: 4800288D  bl 0x82e55210
	ctx.lr = 0x82E52988;
	sub_82E55210(ctx, base);
	// 82E52988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5298C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52990: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52994: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52998: 4BFFFAD9  bl 0x82e52470
	ctx.lr = 0x82E5299C;
	sub_82E52470(ctx, base);
	// 82E5299C: 48000080  b 0x82e52a1c
	pc = 0x82E52A1C; continue 'dispatch;
	// 82E529A0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E529A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E529A8: 388BC9CC  addi r4, r11, -0x3634
	ctx.r[4].s64 = ctx.r[11].s64 + -13876;
	// 82E529AC: 38A0023C  li r5, 0x23c
	ctx.r[5].s64 = 572;
	// 82E529B0: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 82E529B4: 4BF9FA35  bl 0x82df23e8
	ctx.lr = 0x82E529B8;
	sub_82DF23E8(ctx, base);
	// 82E529B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E529BC: 41820010  beq 0x82e529cc
	if ctx.cr[0].eq {
	pc = 0x82E529CC; continue 'dispatch;
	}
	// 82E529C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E529C4: 480023C5  bl 0x82e54d88
	ctx.lr = 0x82E529C8;
	sub_82E54D88(ctx, base);
	// 82E529C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E529CC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E529D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E529D4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E529D8: 4BFFFA99  bl 0x82e52470
	ctx.lr = 0x82E529DC;
	sub_82E52470(ctx, base);
	// 82E529DC: 48000040  b 0x82e52a1c
	pc = 0x82E52A1C; continue 'dispatch;
	// 82E529E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E529E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E529E8: 388BC9CC  addi r4, r11, -0x3634
	ctx.r[4].s64 = ctx.r[11].s64 + -13876;
	// 82E529EC: 38A00237  li r5, 0x237
	ctx.r[5].s64 = 567;
	// 82E529F0: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82E529F4: 4BF9F9F5  bl 0x82df23e8
	ctx.lr = 0x82E529F8;
	sub_82DF23E8(ctx, base);
	// 82E529F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E529FC: 41820010  beq 0x82e52a0c
	if ctx.cr[0].eq {
	pc = 0x82E52A0C; continue 'dispatch;
	}
	// 82E52A00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E52A04: 48001DC5  bl 0x82e547c8
	ctx.lr = 0x82E52A08;
	sub_82E547C8(ctx, base);
	// 82E52A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52A0C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52A10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52A14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52A18: 4BFFFA59  bl 0x82e52470
	ctx.lr = 0x82E52A1C;
	sub_82E52470(ctx, base);
	// 82E52A1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52A24: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52A28: 4B46D5D9  bl 0x822c0000
	ctx.lr = 0x82E52A2C;
	sub_822C0000(ctx, base);
	// 82E52A2C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E52A30: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E52A34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E52A38: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52A3C: 4B471A25  bl 0x822c4460
	ctx.lr = 0x82E52A40;
	sub_822C4460(ctx, base);
	// 82E52A40: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E52A44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E52A48: 419A0008  beq cr6, 0x82e52a50
	if ctx.cr[6].eq {
	pc = 0x82E52A50; continue 'dispatch;
	}
	// 82E52A4C: 4B46DE45  bl 0x822c0890
	ctx.lr = 0x82E52A50;
	sub_822C0890(ctx, base);
	// 82E52A50: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52A54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E52A58: 419A0030  beq cr6, 0x82e52a88
	if ctx.cr[6].eq {
	pc = 0x82E52A88; continue 'dispatch;
	}
	// 82E52A5C: 387B0008  addi r3, r27, 8
	ctx.r[3].s64 = ctx.r[27].s64 + 8;
	// 82E52A60: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52A64: 4BFA074D  bl 0x82df31b0
	ctx.lr = 0x82E52A68;
	sub_82DF31B0(ctx, base);
	// 82E52A68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E52A6C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52A74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52A78: 4E800421  bctrl
	ctx.lr = 0x82E52A7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52A7C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52A80: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52A84: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82E52A88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E52A8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E52A90: 48355724  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52A98 size=8
    let mut pc: u32 = 0x82E52A98;
    'dispatch: loop {
        match pc {
            0x82E52A98 => {
    //   block [0x82E52A98..0x82E52AA0)
	// 82E52A98: C0230000  lfs f1, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E52A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52AA0 size=32
    let mut pc: u32 = 0x82E52AA0;
    'dispatch: loop {
        match pc {
            0x82E52AA0 => {
    //   block [0x82E52AA0..0x82E52AC0)
	// 82E52AA0: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E52AA4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52AA8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E52AAC: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E52AB0: C1ABDFB0  lfs f13, -0x2050(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E52AB4: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82E52AB8: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82E52ABC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52AC0 size=40
    let mut pc: u32 = 0x82E52AC0;
    'dispatch: loop {
        match pc {
            0x82E52AC0 => {
    //   block [0x82E52AC0..0x82E52AE8)
	// 82E52AC0: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E52AC4: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E52AC8: C1630004  lfs f11, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E52ACC: EDAC687A  fmadds f13, f12, f1, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E52AD0: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E52AD4: FF0B0000  fcmpu cr6, f11, f0
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[0].f64);
	// 82E52AD8: 40980010  bge cr6, 0x82e52ae8
	if !ctx.cr[6].lt {
		sub_82E52AE8(ctx, base);
		return;
	}
	// 82E52ADC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E52AE0: 40980014  bge cr6, 0x82e52af4
	if !ctx.cr[6].lt {
		sub_82E52AE8(ctx, base);
		return;
	}
	// 82E52AE4: 4800000C  b 0x82e52af0
	sub_82E52AE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52AE8 size=20
    let mut pc: u32 = 0x82E52AE8;
    'dispatch: loop {
        match pc {
            0x82E52AE8 => {
    //   block [0x82E52AE8..0x82E52AFC)
	// 82E52AE8: FF0D0000  fcmpu cr6, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E52AEC: 40990008  ble cr6, 0x82e52af4
	if !ctx.cr[6].gt {
	pc = 0x82E52AF4; continue 'dispatch;
	}
	// 82E52AF0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82E52AF4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E52AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52B00 size=64
    let mut pc: u32 = 0x82E52B00;
    'dispatch: loop {
        match pc {
            0x82E52B00 => {
    //   block [0x82E52B00..0x82E52B40)
	// 82E52B00: FD801210  fabs f12, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[12].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 82E52B04: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52B08: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E52B0C: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E52B10: D0230008  stfs f1, 8(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E52B14: C00BDFB0  lfs f0, -0x2050(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E52B18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E52B1C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82E52B20: 41980008  blt cr6, 0x82e52b28
	if ctx.cr[6].lt {
	pc = 0x82E52B28; continue 'dispatch;
	}
	// 82E52B24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52B28: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E52B2C: 41820014  beq 0x82e52b40
	if ctx.cr[0].eq {
		sub_82E52B40(ctx, base);
		return;
	}
	// 82E52B30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E52B34: D0230000  stfs f1, 0(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E52B38: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E52B3C: 48000018  b 0x82e52b54
	sub_82E52B40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52B40 size=28
    let mut pc: u32 = 0x82E52B40;
    'dispatch: loop {
        match pc {
            0x82E52B40 => {
    //   block [0x82E52B40..0x82E52B5C)
	// 82E52B40: FF001000  fcmpu cr6, f0, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[2].f64);
	// 82E52B44: 41990008  bgt cr6, 0x82e52b4c
	if ctx.cr[6].gt {
	pc = 0x82E52B4C; continue 'dispatch;
	}
	// 82E52B48: FC001090  fmr f0, f2
	ctx.f[0].f64 = ctx.f[2].f64;
	// 82E52B4C: EDA16828  fsubs f13, f1, f13
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E52B50: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82E52B54: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E52B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E52B60 size=12
    let mut pc: u32 = 0x82E52B60;
    'dispatch: loop {
        match pc {
            0x82E52B60 => {
    //   block [0x82E52B60..0x82E52B6C)
	// 82E52B60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E52B64: C04B08A4  lfs f2, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E52B68: 4BFFFF98  b 0x82e52b00
	sub_82E52B00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E52B70 size=52
    let mut pc: u32 = 0x82E52B70;
    'dispatch: loop {
        match pc {
            0x82E52B70 => {
    //   block [0x82E52B70..0x82E52BA4)
	// 82E52B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52B7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E52B80: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E52B84: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82E52B88: C04B08A4  lfs f2, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E52B8C: C02908A8  lfs f1, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E52B90: 4BFFFF71  bl 0x82e52b00
	ctx.lr = 0x82E52B94;
	sub_82E52B00(ctx, base);
	// 82E52B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E52B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52BA8 size=188
    let mut pc: u32 = 0x82E52BA8;
    'dispatch: loop {
        match pc {
            0x82E52BA8 => {
    //   block [0x82E52BA8..0x82E52C64)
	// 82E52BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E52BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52BBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E52BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52BC4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E52BC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E52BCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52BD0: 4B46DD69  bl 0x822c0938
	ctx.lr = 0x82E52BD4;
	sub_822C0938(ctx, base);
	// 82E52BD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52BD8: 41820028  beq 0x82e52c00
	if ctx.cr[0].eq {
	pc = 0x82E52C00; continue 'dispatch;
	}
	// 82E52BDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52BE0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E52BE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E52BE8: 392BCA8C  addi r9, r11, -0x3574
	ctx.r[9].s64 = ctx.r[11].s64 + -13684;
	// 82E52BEC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E52BF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E52BF4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E52BF8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E52BFC: 48000008  b 0x82e52c04
	pc = 0x82E52C04; continue 'dispatch;
	// 82E52C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52C04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52C08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E52C0C: 409A003C  bne cr6, 0x82e52c48
	if !ctx.cr[6].eq {
	pc = 0x82E52C48; continue 'dispatch;
	}
	// 82E52C10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E52C14: 419A0014  beq cr6, 0x82e52c28
	if ctx.cr[6].eq {
	pc = 0x82E52C28; continue 'dispatch;
	}
	// 82E52C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52C1C: 48002B85  bl 0x82e557a0
	ctx.lr = 0x82E52C20;
	sub_82E557A0(ctx, base);
	// 82E52C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52C24: 4B46D645  bl 0x822c0268
	ctx.lr = 0x82E52C28;
	sub_822C0268(ctx, base);
	// 82E52C28: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E52C2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E52C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52C34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E52C38: 816BAFC4  lwz r11, -0x503c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20540 as u32) ) } as u64;
	// 82E52C3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E52C40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E52C44: 4B46D3BD  bl 0x822c0000
	ctx.lr = 0x82E52C48;
	sub_822C0000(ctx, base);
	// 82E52C48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E52C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E52C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E52C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E52C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52C68 size=64
    let mut pc: u32 = 0x82E52C68;
    'dispatch: loop {
        match pc {
            0x82E52C68 => {
    //   block [0x82E52C68..0x82E52CA8)
	// 82E52C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52C78: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E52C7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E52C80: 419A0014  beq cr6, 0x82e52c94
	if ctx.cr[6].eq {
	pc = 0x82E52C94; continue 'dispatch;
	}
	// 82E52C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52C88: 48002B19  bl 0x82e557a0
	ctx.lr = 0x82E52C8C;
	sub_82E557A0(ctx, base);
	// 82E52C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52C90: 4B46D5D9  bl 0x822c0268
	ctx.lr = 0x82E52C94;
	sub_822C0268(ctx, base);
	// 82E52C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E52C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E52CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52CA8 size=136
    let mut pc: u32 = 0x82E52CA8;
    'dispatch: loop {
        match pc {
            0x82E52CA8 => {
    //   block [0x82E52CA8..0x82E52D30)
	// 82E52CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52CB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E52CC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52CC4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E52CC8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E52CCC: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52CD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E52CD4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E52CD8: 419A0008  beq cr6, 0x82e52ce0
	if ctx.cr[6].eq {
	pc = 0x82E52CE0; continue 'dispatch;
	}
	// 82E52CDC: 4B46DBB5  bl 0x822c0890
	ctx.lr = 0x82E52CE0;
	sub_822C0890(ctx, base);
	// 82E52CE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52CE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E52CE8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52CF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E52CF8: 4E800421  bctrl
	ctx.lr = 0x82E52CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E52CFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52D00: 48311361  bl 0x83164060
	ctx.lr = 0x82E52D04;
	sub_83164060(ctx, base);
	// 82E52D04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52D08: 48311279  bl 0x83163f80
	ctx.lr = 0x82E52D0C;
	sub_83163F80(ctx, base);
	// 82E52D0C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E52D10: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52D14: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E52D18: 4BF9F471  bl 0x82df2188
	ctx.lr = 0x82E52D1C;
	sub_82DF2188(ctx, base);
	// 82E52D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E52D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E52D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52D30 size=104
    let mut pc: u32 = 0x82E52D30;
    'dispatch: loop {
        match pc {
            0x82E52D30 => {
    //   block [0x82E52D30..0x82E52D98)
	// 82E52D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E52D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E52D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52D40: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52D44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52D48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52D4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E52D50: 419A0028  beq cr6, 0x82e52d78
	if ctx.cr[6].eq {
	pc = 0x82E52D78; continue 'dispatch;
	}
	// 82E52D54: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E52D58: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E52D5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E52D60: 38AAC8C8  addi r5, r10, -0x3738
	ctx.r[5].s64 = ctx.r[10].s64 + -14136;
	// 82E52D64: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 82E52D68: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52D6C: 4BFFF86D  bl 0x82e525d8
	ctx.lr = 0x82E52D70;
	sub_82E525D8(ctx, base);
	// 82E52D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52D74: 48000010  b 0x82e52d84
	pc = 0x82E52D84; continue 'dispatch;
	// 82E52D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E52D7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52D80: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E52D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E52D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E52D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E52D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E52D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52D98 size=192
    let mut pc: u32 = 0x82E52D98;
    'dispatch: loop {
        match pc {
            0x82E52D98 => {
    //   block [0x82E52D98..0x82E52E58)
	// 82E52D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52D9C: 483553C9  bl 0x831a8164
	ctx.lr = 0x82E52DA0;
	sub_831A8130(ctx, base);
	// 82E52DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52DA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E52DA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E52DAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52DB0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52DB4: 4800084D  bl 0x82e53600
	ctx.lr = 0x82E52DB8;
	sub_82E53600(ctx, base);
	// 82E52DB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52DBC: 838B0014  lwz r28, 0x14(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E52DC0: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E52DC4: 4800003C  b 0x82e52e00
	pc = 0x82E52E00; continue 'dispatch;
	// 82E52DC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52DCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52DD0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E52DD4: 4BFA0E2D  bl 0x82df3c00
	ctx.lr = 0x82E52DD8;
	sub_82DF3C00(ctx, base);
	// 82E52DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E52DDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52DE0: 4BFA06A9  bl 0x82df3488
	ctx.lr = 0x82E52DE4;
	sub_82DF3488(ctx, base);
	// 82E52DE4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E52DE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52DEC: 557BDFFE  rlwinm r27, r11, 0x1b, 0x1f, 0x1f
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E52DF0: 4BFA0639  bl 0x82df3428
	ctx.lr = 0x82E52DF4;
	sub_82DF3428(ctx, base);
	// 82E52DF4: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52DF8: 40820014  bne 0x82e52e0c
	if !ctx.cr[0].eq {
	pc = 0x82E52E0C; continue 'dispatch;
	}
	// 82E52DFC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E52E00: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E52E04: 409AFFC4  bne cr6, 0x82e52dc8
	if !ctx.cr[6].eq {
	pc = 0x82E52DC8; continue 'dispatch;
	}
	// 82E52E08: 48000048  b 0x82e52e50
	pc = 0x82E52E50; continue 'dispatch;
	// 82E52E0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52E10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52E14: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E52E18: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82E52E1C: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E52E20: 48000629  bl 0x82e53448
	ctx.lr = 0x82E52E24;
	sub_82E53448(ctx, base);
	// 82E52E24: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52E28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52E2C: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82E52E30: 4BFFC1C1  bl 0x82e4eff0
	ctx.lr = 0x82E52E34;
	sub_82E4EFF0(ctx, base);
	// 82E52E34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E52E38: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82E52E3C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E52E40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52E44: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E52E48: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E52E4C: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E52E50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E52E54: 48355360  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52E58 size=244
    let mut pc: u32 = 0x82E52E58;
    'dispatch: loop {
        match pc {
            0x82E52E58 => {
    //   block [0x82E52E58..0x82E52F4C)
	// 82E52E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52E5C: 48355309  bl 0x831a8164
	ctx.lr = 0x82E52E60;
	sub_831A8130(ctx, base);
	// 82E52E60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52E64: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52E68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52E6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E52E70: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E52E74: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E52E78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E52E7C: 388BCAA0  addi r4, r11, -0x3560
	ctx.r[4].s64 = ctx.r[11].s64 + -13664;
	// 82E52E80: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 82E52E84: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E52E88: 4BF9F561  bl 0x82df23e8
	ctx.lr = 0x82E52E8C;
	sub_82DF23E8(ctx, base);
	// 82E52E8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52E90: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E52E94: 41820020  beq 0x82e52eb4
	if ctx.cr[0].eq {
	pc = 0x82E52EB4; continue 'dispatch;
	}
	// 82E52E98: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E52E9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E52EA0: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E52EA4: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E52EA8: 93A30014  stw r29, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82E52EAC: 93A30018  stw r29, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82E52EB0: 48000008  b 0x82e52eb8
	pc = 0x82E52EB8; continue 'dispatch;
	// 82E52EB4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E52EB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E52EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E52EC0: 48311021  bl 0x83163ee0
	ctx.lr = 0x82E52EC4;
	sub_83163EE0(ctx, base);
	// 82E52EC4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 82E52EC8: 3C800008  lis r4, 8
	ctx.r[4].s64 = 524288;
	// 82E52ECC: 38AB8328  addi r5, r11, -0x7cd8
	ctx.r[5].s64 = ctx.r[11].s64 + -31960;
	// 82E52ED0: 60847000  ori r4, r4, 0x7000
	ctx.r[4].u64 = ctx.r[4].u64 | 28672;
	// 82E52ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E52ED8: 48311119  bl 0x83163ff0
	ctx.lr = 0x82E52EDC;
	sub_83163FF0(ctx, base);
	// 82E52EDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E52EE0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E52EE4: 388BC8C8  addi r4, r11, -0x3738
	ctx.r[4].s64 = ctx.r[11].s64 + -14136;
	// 82E52EE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E52EEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E52EF0: 480008B1  bl 0x82e537a0
	ctx.lr = 0x82E52EF4;
	sub_82E537A0(ctx, base);
	// 82E52EF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52EF8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52EFC: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82E52F00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E52F04: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E52F08: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E52F0C: 4B471555  bl 0x822c4460
	ctx.lr = 0x82E52F10;
	sub_822C4460(ctx, base);
	// 82E52F10: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E52F14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E52F18: 419A0008  beq cr6, 0x82e52f20
	if ctx.cr[6].eq {
	pc = 0x82E52F20; continue 'dispatch;
	}
	// 82E52F1C: 4B46D975  bl 0x822c0890
	ctx.lr = 0x82E52F20;
	sub_822C0890(ctx, base);
	// 82E52F20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E52F24: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E52F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E52F2C: 4831198D  bl 0x831648b8
	ctx.lr = 0x82E52F30;
	sub_831648B8(ctx, base);
	// 82E52F30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52F34: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E52F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E52F3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52F40: 93AB001C  stw r29, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82E52F44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E52F48: 4835526C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E52F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E52F50 size=200
    let mut pc: u32 = 0x82E52F50;
    'dispatch: loop {
        match pc {
            0x82E52F50 => {
    //   block [0x82E52F50..0x82E53018)
	// 82E52F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E52F54: 48355211  bl 0x831a8164
	ctx.lr = 0x82E52F58;
	sub_831A8130(ctx, base);
	// 82E52F58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E52F5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E52F60: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82E52F64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E52F68: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E52F6C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E52F70: 4B46D9C9  bl 0x822c0938
	ctx.lr = 0x82E52F74;
	sub_822C0938(ctx, base);
	// 82E52F74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E52F78: 41820020  beq 0x82e52f98
	if ctx.cr[0].eq {
	pc = 0x82E52F98; continue 'dispatch;
	}
	// 82E52F7C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52F80: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E52F84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E52F88: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52F8C: 48002725  bl 0x82e556b0
	ctx.lr = 0x82E52F90;
	sub_82E556B0(ctx, base);
	// 82E52F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E52F94: 48000008  b 0x82e52f9c
	pc = 0x82E52F9C; continue 'dispatch;
	// 82E52F98: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E52F9C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E52FA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52FA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52FA8: 4BFFFC01  bl 0x82e52ba8
	ctx.lr = 0x82E52FAC;
	sub_82E52BA8(ctx, base);
	// 82E52FAC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E52FB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E52FB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E52FB8: 4B46D049  bl 0x822c0000
	ctx.lr = 0x82E52FBC;
	sub_822C0000(ctx, base);
	// 82E52FBC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52FC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E52FC4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82E52FC8: 4BFE62E1  bl 0x82e392a8
	ctx.lr = 0x82E52FCC;
	sub_82E392A8(ctx, base);
	// 82E52FCC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E52FD0: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82E52FD4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E52FD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E52FDC: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52FE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52FE4: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52FE8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E52FEC: 4800092D  bl 0x82e53918
	ctx.lr = 0x82E52FF0;
	sub_82E53918(ctx, base);
	// 82E52FF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E52FF4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E52FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E52FFC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53000: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E53004: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E53008: 419A0008  beq cr6, 0x82e53010
	if ctx.cr[6].eq {
	pc = 0x82E53010; continue 'dispatch;
	}
	// 82E5300C: 4B46D885  bl 0x822c0890
	ctx.lr = 0x82E53010;
	sub_822C0890(ctx, base);
	// 82E53010: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E53014: 483551A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53018 size=64
    let mut pc: u32 = 0x82E53018;
    'dispatch: loop {
        match pc {
            0x82E53018 => {
    //   block [0x82E53018..0x82E53058)
	// 82E53018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53024: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53028: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5302C: 409A001C  bne cr6, 0x82e53048
	if !ctx.cr[6].eq {
	pc = 0x82E53048; continue 'dispatch;
	}
	// 82E53030: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53034: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E53038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5303C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E53040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53044: 4E800421  bctrl
	ctx.lr = 0x82E53048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53058 size=64
    let mut pc: u32 = 0x82E53058;
    'dispatch: loop {
        match pc {
            0x82E53058 => {
    //   block [0x82E53058..0x82E53098)
	// 82E53058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5305C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53064: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5306C: 409A001C  bne cr6, 0x82e53088
	if !ctx.cr[6].eq {
	pc = 0x82E53088; continue 'dispatch;
	}
	// 82E53070: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53074: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E53078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5307C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E53080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53084: 4E800421  bctrl
	ctx.lr = 0x82E53088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5308C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53098 size=196
    let mut pc: u32 = 0x82E53098;
    'dispatch: loop {
        match pc {
            0x82E53098 => {
    //   block [0x82E53098..0x82E5315C)
	// 82E53098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5309C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E530A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E530A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E530A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E530AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E530B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E530B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E530B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E530BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E530C0: 4B46D879  bl 0x822c0938
	ctx.lr = 0x82E530C4;
	sub_822C0938(ctx, base);
	// 82E530C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E530C8: 41820028  beq 0x82e530f0
	if ctx.cr[0].eq {
	pc = 0x82E530F0; continue 'dispatch;
	}
	// 82E530CC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E530D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E530D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E530D8: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82E530DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E530E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E530E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E530E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E530EC: 48000008  b 0x82e530f4
	pc = 0x82E530F4; continue 'dispatch;
	// 82E530F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E530F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E530F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E530FC: 409A0044  bne cr6, 0x82e53140
	if !ctx.cr[6].eq {
	pc = 0x82E53140; continue 'dispatch;
	}
	// 82E53100: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E53104: 419A001C  beq cr6, 0x82e53120
	if ctx.cr[6].eq {
	pc = 0x82E53120; continue 'dispatch;
	}
	// 82E53108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5310C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53114: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5311C: 4E800421  bctrl
	ctx.lr = 0x82E53120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53120: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E53124: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E53128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5312C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E53130: 816BAFC8  lwz r11, -0x5038(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20536 as u32) ) } as u64;
	// 82E53134: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E53138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E5313C: 4B46CEC5  bl 0x822c0000
	ctx.lr = 0x82E53140;
	sub_822C0000(ctx, base);
	// 82E53140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E53144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5314C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E53154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53160 size=168
    let mut pc: u32 = 0x82E53160;
    'dispatch: loop {
        match pc {
            0x82E53160 => {
    //   block [0x82E53160..0x82E53208)
	// 82E53160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53164: 48355009  bl 0x831a816c
	ctx.lr = 0x82E53168;
	sub_831A8130(ctx, base);
	// 82E53168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5316C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53174: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53178: 3BABCAEC  addi r29, r11, -0x3514
	ctx.r[29].s64 = ctx.r[11].s64 + -13588;
	// 82E5317C: 4BFFE445  bl 0x82e515c0
	ctx.lr = 0x82E53180;
	sub_82E515C0(ctx, base);
	// 82E53180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53184: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53188: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5318C: 38C00139  li r6, 0x139
	ctx.r[6].s64 = 313;
	// 82E53190: 4BFA66A9  bl 0x82df9838
	ctx.lr = 0x82E53194;
	sub_82DF9838(ctx, base);
	// 82E53194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E53198: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5319C: 4BFA086D  bl 0x82df3a08
	ctx.lr = 0x82E531A0;
	sub_82DF3A08(ctx, base);
	// 82E531A0: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E531A4: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E531A8: 48000028  b 0x82e531d0
	pc = 0x82E531D0; continue 'dispatch;
	// 82E531AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E531B0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E531B4: 4BF9FFFD  bl 0x82df31b0
	ctx.lr = 0x82E531B8;
	sub_82DF31B0(ctx, base);
	// 82E531B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E531BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E531C0: 4BFA02C9  bl 0x82df3488
	ctx.lr = 0x82E531C4;
	sub_82DF3488(ctx, base);
	// 82E531C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E531C8: 41820018  beq 0x82e531e0
	if ctx.cr[0].eq {
	pc = 0x82E531E0; continue 'dispatch;
	}
	// 82E531CC: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E531D0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E531D4: 409AFFD8  bne cr6, 0x82e531ac
	if !ctx.cr[6].eq {
	pc = 0x82E531AC; continue 'dispatch;
	}
	// 82E531D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E531DC: 48000010  b 0x82e531ec
	pc = 0x82E531EC; continue 'dispatch;
	// 82E531E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E531E4: 48002755  bl 0x82e55938
	ctx.lr = 0x82E531E8;
	sub_82E55938(ctx, base);
	// 82E531E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E531EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E531F0: 4BFA0239  bl 0x82df3428
	ctx.lr = 0x82E531F4;
	sub_82DF3428(ctx, base);
	// 82E531F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E531F8: 4B47C409  bl 0x822cf600
	ctx.lr = 0x82E531FC;
	sub_822CF600(ctx, base);
	// 82E531FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E53204: 48354FB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53208 size=176
    let mut pc: u32 = 0x82E53208;
    'dispatch: loop {
        match pc {
            0x82E53208 => {
    //   block [0x82E53208..0x82E532B8)
	// 82E53208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5320C: 48354F5D  bl 0x831a8168
	ctx.lr = 0x82E53210;
	sub_831A8130(ctx, base);
	// 82E53210: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53214: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5321C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53220: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E53224: 3B8BCAEC  addi r28, r11, -0x3514
	ctx.r[28].s64 = ctx.r[11].s64 + -13588;
	// 82E53228: 4BFFE399  bl 0x82e515c0
	ctx.lr = 0x82E5322C;
	sub_82E515C0(ctx, base);
	// 82E5322C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53230: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53234: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E53238: 38C0014D  li r6, 0x14d
	ctx.r[6].s64 = 333;
	// 82E5323C: 4BFA65FD  bl 0x82df9838
	ctx.lr = 0x82E53240;
	sub_82DF9838(ctx, base);
	// 82E53240: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E53244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53248: 4BFA07C1  bl 0x82df3a08
	ctx.lr = 0x82E5324C;
	sub_82DF3A08(ctx, base);
	// 82E5324C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E53250: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E53254: 48000028  b 0x82e5327c
	pc = 0x82E5327C; continue 'dispatch;
	// 82E53258: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5325C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E53260: 4BF9FF51  bl 0x82df31b0
	ctx.lr = 0x82E53264;
	sub_82DF31B0(ctx, base);
	// 82E53264: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5326C: 4BFA021D  bl 0x82df3488
	ctx.lr = 0x82E53270;
	sub_82DF3488(ctx, base);
	// 82E53270: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E53274: 41820018  beq 0x82e5328c
	if ctx.cr[0].eq {
	pc = 0x82E5328C; continue 'dispatch;
	}
	// 82E53278: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E5327C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E53280: 409AFFD8  bne cr6, 0x82e53258
	if !ctx.cr[6].eq {
	pc = 0x82E53258; continue 'dispatch;
	}
	// 82E53284: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E53288: 48000014  b 0x82e5329c
	pc = 0x82E5329C; continue 'dispatch;
	// 82E5328C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E53290: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53294: 480026DD  bl 0x82e55970
	ctx.lr = 0x82E53298;
	sub_82E55970(ctx, base);
	// 82E53298: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5329C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E532A0: 4BFA0189  bl 0x82df3428
	ctx.lr = 0x82E532A4;
	sub_82DF3428(ctx, base);
	// 82E532A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E532A8: 4B47C359  bl 0x822cf600
	ctx.lr = 0x82E532AC;
	sub_822CF600(ctx, base);
	// 82E532AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E532B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E532B4: 48354F04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E532B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E532B8 size=308
    let mut pc: u32 = 0x82E532B8;
    'dispatch: loop {
        match pc {
            0x82E532B8 => {
    //   block [0x82E532B8..0x82E533EC)
	// 82E532B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E532BC: 48354E9D  bl 0x831a8158
	ctx.lr = 0x82E532C0;
	sub_831A8130(ctx, base);
	// 82E532C0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E532C4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E532C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E532CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E532D0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E532D4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82E532D8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82E532DC: 3BCBCAEC  addi r30, r11, -0x3514
	ctx.r[30].s64 = ctx.r[11].s64 + -13588;
	// 82E532E0: 4BFFE2E1  bl 0x82e515c0
	ctx.lr = 0x82E532E4;
	sub_82E515C0(ctx, base);
	// 82E532E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E532E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E532EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E532F0: 38C00195  li r6, 0x195
	ctx.r[6].s64 = 405;
	// 82E532F4: 4BFA6545  bl 0x82df9838
	ctx.lr = 0x82E532F8;
	sub_82DF9838(ctx, base);
	// 82E532F8: 837F0014  lwz r27, 0x14(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E532FC: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E53300: 48000064  b 0x82e53364
	pc = 0x82E53364; continue 'dispatch;
	// 82E53304: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53308: 48002631  bl 0x82e55938
	ctx.lr = 0x82E5330C;
	sub_82E55938(ctx, base);
	// 82E5330C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E53310: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E53314: 4182004C  beq 0x82e53360
	if ctx.cr[0].eq {
	pc = 0x82E53360; continue 'dispatch;
	}
	// 82E53318: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5331C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53320: 48002651  bl 0x82e55970
	ctx.lr = 0x82E53324;
	sub_82E55970(ctx, base);
	// 82E53324: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E53328: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5332C: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53330: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E53334: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E53338: 41820014  beq 0x82e5334c
	if ctx.cr[0].eq {
	pc = 0x82E5334C; continue 'dispatch;
	}
	// 82E5333C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53340: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E53344: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53348: 419AFFE0  beq cr6, 0x82e53328
	if ctx.cr[6].eq {
	pc = 0x82E53328; continue 'dispatch;
	}
	// 82E5334C: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E53350: 41820024  beq 0x82e53374
	if ctx.cr[0].eq {
	pc = 0x82E53374; continue 'dispatch;
	}
	// 82E53354: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E53358: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E5335C: 4198FFBC  blt cr6, 0x82e53318
	if ctx.cr[6].lt {
	pc = 0x82E53318; continue 'dispatch;
	}
	// 82E53360: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E53364: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E53368: 409AFF9C  bne cr6, 0x82e53304
	if !ctx.cr[6].eq {
	pc = 0x82E53304; continue 'dispatch;
	}
	// 82E5336C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E53370: 48000068  b 0x82e533d8
	pc = 0x82E533D8; continue 'dispatch;
	// 82E53374: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53378: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5337C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E53380: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53388: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5338C: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E53390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53394: 4E800421  bctrl
	ctx.lr = 0x82E53398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53398: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5339C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 82E533A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E533A4: 38CA81F0  addi r6, r10, -0x7e10
	ctx.r[6].s64 = ctx.r[10].s64 + -32272;
	// 82E533A8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E533AC: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E533B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E533B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E533B8: 4E800421  bctrl
	ctx.lr = 0x82E533BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E533BC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E533C0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E533C4: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E533C8: 81210068  lwz r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E533CC: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E533D0: 91590000  stw r10, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E533D4: 91380000  stw r9, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E533D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E533DC: 4B47C225  bl 0x822cf600
	ctx.lr = 0x82E533E0;
	sub_822CF600(ctx, base);
	// 82E533E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E533E4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E533E8: 48354DC0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E533F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E533F0 size=84
    let mut pc: u32 = 0x82E533F0;
    'dispatch: loop {
        match pc {
            0x82E533F0 => {
    //   block [0x82E533F0..0x82E53444)
	// 82E533F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E533F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E533F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E533FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53400: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53404: 3BEBCAEC  addi r31, r11, -0x3514
	ctx.r[31].s64 = ctx.r[11].s64 + -13588;
	// 82E53408: 4BFFE1B9  bl 0x82e515c0
	ctx.lr = 0x82E5340C;
	sub_82E515C0(ctx, base);
	// 82E5340C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53410: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53414: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E53418: 38C001CD  li r6, 0x1cd
	ctx.r[6].s64 = 461;
	// 82E5341C: 4BFA641D  bl 0x82df9838
	ctx.lr = 0x82E53420;
	sub_82DF9838(ctx, base);
	// 82E53420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53424: 482EED7D  bl 0x831421a0
	ctx.lr = 0x82E53428;
	sub_831421A0(ctx, base);
	// 82E53428: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5342C: 4B47C1D5  bl 0x822cf600
	ctx.lr = 0x82E53430;
	sub_822CF600(ctx, base);
	// 82E53430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5343C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53448 size=88
    let mut pc: u32 = 0x82E53448;
    'dispatch: loop {
        match pc {
            0x82E53448 => {
    //   block [0x82E53448..0x82E534A0)
	// 82E53448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5344C: 48354D1D  bl 0x831a8168
	ctx.lr = 0x82E53450;
	sub_831A8130(ctx, base);
	// 82E53450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53454: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E53458: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E5345C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E53460: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53464: 419A0030  beq cr6, 0x82e53494
	if ctx.cr[6].eq {
	pc = 0x82E53494; continue 'dispatch;
	}
	// 82E53468: 7F9F1850  subf r28, r31, r3
	ctx.r[28].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82E5346C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53470: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82E53474: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E53478: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E5347C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E53480: 4B470FE1  bl 0x822c4460
	ctx.lr = 0x82E53484;
	sub_822C4460(ctx, base);
	// 82E53484: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E53488: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82E5348C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53490: 409AFFDC  bne cr6, 0x82e5346c
	if !ctx.cr[6].eq {
	pc = 0x82E5346C; continue 'dispatch;
	}
	// 82E53494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5349C: 48354D1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E534A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E534A0 size=140
    let mut pc: u32 = 0x82E534A0;
    'dispatch: loop {
        match pc {
            0x82E534A0 => {
    //   block [0x82E534A0..0x82E5352C)
	// 82E534A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E534A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E534A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E534AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E534B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E534B4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E534B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E534BC: 396BCB24  addi r11, r11, -0x34dc
	ctx.r[11].s64 = ctx.r[11].s64 + -13532;
	// 82E534C0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E534C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E534C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E534CC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E534D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E534D4: 4E800421  bctrl
	ctx.lr = 0x82E534D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E534D8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E534DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E534E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E534E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E534E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E534EC: 4E800421  bctrl
	ctx.lr = 0x82E534F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E534F0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E534F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E534F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E534FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53504: 4E800421  bctrl
	ctx.lr = 0x82E53508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53508: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E5350C: 4BFD98B5  bl 0x82e2cdc0
	ctx.lr = 0x82E53510;
	sub_82E2CDC0(ctx, base);
	// 82E53510: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E53514: 4BF9FF15  bl 0x82df3428
	ctx.lr = 0x82E53518;
	sub_82DF3428(ctx, base);
	// 82E53518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5351C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53530 size=208
    let mut pc: u32 = 0x82E53530;
    'dispatch: loop {
        match pc {
            0x82E53530 => {
    //   block [0x82E53530..0x82E53600)
	// 82E53530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53534: 48354C35  bl 0x831a8168
	ctx.lr = 0x82E53538;
	sub_831A8130(ctx, base);
	// 82E53538: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5353C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53544: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E53548: 3BCBCAEC  addi r30, r11, -0x3514
	ctx.r[30].s64 = ctx.r[11].s64 + -13588;
	// 82E5354C: 4BFFE075  bl 0x82e515c0
	ctx.lr = 0x82E53550;
	sub_82E515C0(ctx, base);
	// 82E53550: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53554: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53558: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E5355C: 38C000A4  li r6, 0xa4
	ctx.r[6].s64 = 164;
	// 82E53560: 4BFA62D9  bl 0x82df9838
	ctx.lr = 0x82E53564;
	sub_82DF9838(ctx, base);
	// 82E53564: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E53568: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E5356C: 48000028  b 0x82e53594
	pc = 0x82E53594; continue 'dispatch;
	// 82E53570: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53574: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E53578: 4BF9FC39  bl 0x82df31b0
	ctx.lr = 0x82E5357C;
	sub_82DF31B0(ctx, base);
	// 82E5357C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53580: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E53584: 4BF9FF05  bl 0x82df3488
	ctx.lr = 0x82E53588;
	sub_82DF3488(ctx, base);
	// 82E53588: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E5358C: 41820014  beq 0x82e535a0
	if ctx.cr[0].eq {
	pc = 0x82E535A0; continue 'dispatch;
	}
	// 82E53590: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E53594: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53598: 409AFFD8  bne cr6, 0x82e53570
	if !ctx.cr[6].eq {
	pc = 0x82E53570; continue 'dispatch;
	}
	// 82E5359C: 48000054  b 0x82e535f0
	pc = 0x82E535F0; continue 'dispatch;
	// 82E535A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E535A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E535A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E535AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E535B0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E535B4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E535B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E535BC: 4E800421  bctrl
	ctx.lr = 0x82E535C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E535C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E535C4: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E535C8: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E535CC: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82E535D0: 4BFFFE79  bl 0x82e53448
	ctx.lr = 0x82E535D4;
	sub_82E53448(ctx, base);
	// 82E535D4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E535D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E535DC: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82E535E0: 4BFFBA11  bl 0x82e4eff0
	ctx.lr = 0x82E535E4;
	sub_82E4EFF0(ctx, base);
	// 82E535E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E535E8: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82E535EC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E535F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E535F4: 4B47C00D  bl 0x822cf600
	ctx.lr = 0x82E535F8;
	sub_822CF600(ctx, base);
	// 82E535F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E535FC: 48354BBC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53600 size=228
    let mut pc: u32 = 0x82E53600;
    'dispatch: loop {
        match pc {
            0x82E53600 => {
    //   block [0x82E53600..0x82E536E4)
	// 82E53600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53604: 48354B69  bl 0x831a816c
	ctx.lr = 0x82E53608;
	sub_831A8130(ctx, base);
	// 82E53608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5360C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53614: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53618: 3BABCAEC  addi r29, r11, -0x3514
	ctx.r[29].s64 = ctx.r[11].s64 + -13588;
	// 82E5361C: 4BFFDFA5  bl 0x82e515c0
	ctx.lr = 0x82E53620;
	sub_82E515C0(ctx, base);
	// 82E53620: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53624: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53628: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5362C: 38C000F1  li r6, 0xf1
	ctx.r[6].s64 = 241;
	// 82E53630: 4BFA6209  bl 0x82df9838
	ctx.lr = 0x82E53634;
	sub_82DF9838(ctx, base);
	// 82E53634: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E53638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5363C: 4BFA03CD  bl 0x82df3a08
	ctx.lr = 0x82E53640;
	sub_82DF3A08(ctx, base);
	// 82E53640: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E53644: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E53648: 48000028  b 0x82e53670
	pc = 0x82E53670; continue 'dispatch;
	// 82E5364C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53650: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E53654: 4BF9FB5D  bl 0x82df31b0
	ctx.lr = 0x82E53658;
	sub_82DF31B0(ctx, base);
	// 82E53658: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5365C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53660: 4BF9FE29  bl 0x82df3488
	ctx.lr = 0x82E53664;
	sub_82DF3488(ctx, base);
	// 82E53664: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E53668: 41820014  beq 0x82e5367c
	if ctx.cr[0].eq {
	pc = 0x82E5367C; continue 'dispatch;
	}
	// 82E5366C: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E53670: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53674: 409AFFD8  bne cr6, 0x82e5364c
	if !ctx.cr[6].eq {
	pc = 0x82E5364C; continue 'dispatch;
	}
	// 82E53678: 48000054  b 0x82e536cc
	pc = 0x82E536CC; continue 'dispatch;
	// 82E5367C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53680: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E53684: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53688: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5368C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53690: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53698: 4E800421  bctrl
	ctx.lr = 0x82E5369C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5369C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E536A0: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E536A4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E536A8: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82E536AC: 4BFFFD9D  bl 0x82e53448
	ctx.lr = 0x82E536B0;
	sub_82E53448(ctx, base);
	// 82E536B0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E536B4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E536B8: 3864FFF8  addi r3, r4, -8
	ctx.r[3].s64 = ctx.r[4].s64 + -8;
	// 82E536BC: 4BFFB935  bl 0x82e4eff0
	ctx.lr = 0x82E536C0;
	sub_82E4EFF0(ctx, base);
	// 82E536C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E536C4: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82E536C8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E536CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E536D0: 4BF9FD59  bl 0x82df3428
	ctx.lr = 0x82E536D4;
	sub_82DF3428(ctx, base);
	// 82E536D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E536D8: 4B47BF29  bl 0x822cf600
	ctx.lr = 0x82E536DC;
	sub_822CF600(ctx, base);
	// 82E536DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E536E0: 48354ADC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E536E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E536E8 size=100
    let mut pc: u32 = 0x82E536E8;
    'dispatch: loop {
        match pc {
            0x82E536E8 => {
    //   block [0x82E536E8..0x82E5374C)
	// 82E536E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E536EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E536F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E536F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E536F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E536FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53700: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53704: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E53708: 396BCB24  addi r11, r11, -0x34dc
	ctx.r[11].s64 = ctx.r[11].s64 + -13532;
	// 82E5370C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E53710: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53714: 4BFA02F5  bl 0x82df3a08
	ctx.lr = 0x82E53718;
	sub_82DF3A08(ctx, base);
	// 82E53718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5371C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E53720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53724: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E53728: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E5372C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E53730: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E53734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5373C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53740: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E53744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53750 size=76
    let mut pc: u32 = 0x82E53750;
    'dispatch: loop {
        match pc {
            0x82E53750 => {
    //   block [0x82E53750..0x82E5379C)
	// 82E53750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5375C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53768: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5376C: 4BFFFD35  bl 0x82e534a0
	ctx.lr = 0x82E53770;
	sub_82E534A0(ctx, base);
	// 82E53770: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53774: 4182000C  beq 0x82e53780
	if ctx.cr[0].eq {
	pc = 0x82E53780; continue 'dispatch;
	}
	// 82E53778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5377C: 4BF9EC5D  bl 0x82df23d8
	ctx.lr = 0x82E53780;
	sub_82DF23D8(ctx, base);
	// 82E53780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5378C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E53794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E537A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E537A0 size=152
    let mut pc: u32 = 0x82E537A0;
    'dispatch: loop {
        match pc {
            0x82E537A0 => {
    //   block [0x82E537A0..0x82E53838)
	// 82E537A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E537A4: 483549C9  bl 0x831a816c
	ctx.lr = 0x82E537A8;
	sub_831A8130(ctx, base);
	// 82E537A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E537AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E537B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E537B4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E537B8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E537BC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E537C0: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82E537C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E537C8: 482EED59  bl 0x83142520
	ctx.lr = 0x82E537CC;
	sub_83142520(ctx, base);
	// 82E537CC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E537D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E537D4: 388BCAEC  addi r4, r11, -0x3514
	ctx.r[4].s64 = ctx.r[11].s64 + -13588;
	// 82E537D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E537DC: 38A001C7  li r5, 0x1c7
	ctx.r[5].s64 = 455;
	// 82E537E0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E537E4: 4BF9EC05  bl 0x82df23e8
	ctx.lr = 0x82E537E8;
	sub_82DF23E8(ctx, base);
	// 82E537E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E537EC: 41820018  beq 0x82e53804
	if ctx.cr[0].eq {
	pc = 0x82E53804; continue 'dispatch;
	}
	// 82E537F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E537F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E537F8: 4BFFFEF1  bl 0x82e536e8
	ctx.lr = 0x82E537FC;
	sub_82E536E8(ctx, base);
	// 82E537FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53800: 48000008  b 0x82e53808
	pc = 0x82E53808; continue 'dispatch;
	// 82E53804: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E53808: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E5380C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82E53810: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E53814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E53818: 4BFFF881  bl 0x82e53098
	ctx.lr = 0x82E5381C;
	sub_82E53098(ctx, base);
	// 82E5381C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E53820: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E53824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E53828: 4B46C7D9  bl 0x822c0000
	ctx.lr = 0x82E5382C;
	sub_822C0000(ctx, base);
	// 82E5382C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E53830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E53834: 48354988  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53838 size=224
    let mut pc: u32 = 0x82E53838;
    'dispatch: loop {
        match pc {
            0x82E53838 => {
    //   block [0x82E53838..0x82E53918)
	// 82E53838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5383C: 48354925  bl 0x831a8160
	ctx.lr = 0x82E53840;
	sub_831A8130(ctx, base);
	// 82E53840: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53844: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5384C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53850: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E53854: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E53858: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E5385C: 3B4BCAEC  addi r26, r11, -0x3514
	ctx.r[26].s64 = ctx.r[11].s64 + -13588;
	// 82E53860: 4BFFDD61  bl 0x82e515c0
	ctx.lr = 0x82E53864;
	sub_82E515C0(ctx, base);
	// 82E53864: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53868: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5386C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E53870: 38C00082  li r6, 0x82
	ctx.r[6].s64 = 130;
	// 82E53874: 4BFA5FC5  bl 0x82df9838
	ctx.lr = 0x82E53878;
	sub_82DF9838(ctx, base);
	// 82E53878: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5387C: 4BF9F935  bl 0x82df31b0
	ctx.lr = 0x82E53880;
	sub_82DF31B0(ctx, base);
	// 82E53880: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E53884: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E53888: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5388C: 48002405  bl 0x82e55c90
	ctx.lr = 0x82E53890;
	sub_82E55C90(ctx, base);
	// 82E53890: 38DF001C  addi r6, r31, 0x1c
	ctx.r[6].s64 = ctx.r[31].s64 + 28;
	// 82E53894: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E53898: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E5389C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E538A0: 48002481  bl 0x82e55d20
	ctx.lr = 0x82E538A4;
	sub_82E55D20(ctx, base);
	// 82E538A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E538A8: 41820058  beq 0x82e53900
	if ctx.cr[0].eq {
	pc = 0x82E53900; continue 'dispatch;
	}
	// 82E538AC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E538B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E538B4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E538B8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E538BC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E538C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E538C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E538C8: 4E800421  bctrl
	ctx.lr = 0x82E538CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E538CC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E538D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E538D4: 4BFE59D5  bl 0x82e392a8
	ctx.lr = 0x82E538D8;
	sub_82E392A8(ctx, base);
	// 82E538D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E538DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E538E0: 419A0008  beq cr6, 0x82e538e8
	if ctx.cr[6].eq {
	pc = 0x82E538E8; continue 'dispatch;
	}
	// 82E538E4: 4B46CFAD  bl 0x822c0890
	ctx.lr = 0x82E538E8;
	sub_822C0890(ctx, base);
	// 82E538E8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E538EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E538F0: 4B47BD11  bl 0x822cf600
	ctx.lr = 0x82E538F4;
	sub_822CF600(ctx, base);
	// 82E538F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E538F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E538FC: 483548B4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E53900: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E53904: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E53908: 419A0008  beq cr6, 0x82e53910
	if ctx.cr[6].eq {
	pc = 0x82E53910; continue 'dispatch;
	}
	// 82E5390C: 4B46CF85  bl 0x822c0890
	ctx.lr = 0x82E53910;
	sub_822C0890(ctx, base);
	// 82E53910: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E53914: 4BFFFFD8  b 0x82e538ec
	pc = 0x82E538EC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53918 size=288
    let mut pc: u32 = 0x82E53918;
    'dispatch: loop {
        match pc {
            0x82E53918 => {
    //   block [0x82E53918..0x82E53A38)
	// 82E53918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5391C: 48354845  bl 0x831a8160
	ctx.lr = 0x82E53920;
	sub_831A8130(ctx, base);
	// 82E53920: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53924: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5392C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E53930: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E53934: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E53938: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E5393C: 3B4BCAEC  addi r26, r11, -0x3514
	ctx.r[26].s64 = ctx.r[11].s64 + -13588;
	// 82E53940: 4BFFDC81  bl 0x82e515c0
	ctx.lr = 0x82E53944;
	sub_82E515C0(ctx, base);
	// 82E53944: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E53948: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E5394C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E53950: 38C000C8  li r6, 0xc8
	ctx.r[6].s64 = 200;
	// 82E53954: 4BFA5EE5  bl 0x82df9838
	ctx.lr = 0x82E53958;
	sub_82DF9838(ctx, base);
	// 82E53958: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E5395C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E53960: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E53964: 4800232D  bl 0x82e55c90
	ctx.lr = 0x82E53968;
	sub_82E55C90(ctx, base);
	// 82E53968: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E5396C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53970: 4BFA0099  bl 0x82df3a08
	ctx.lr = 0x82E53974;
	sub_82DF3A08(ctx, base);
	// 82E53974: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E53978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5397C: 388BED54  addi r4, r11, -0x12ac
	ctx.r[4].s64 = ctx.r[11].s64 + -4780;
	// 82E53980: 4BF9FBF9  bl 0x82df3578
	ctx.lr = 0x82E53984;
	sub_82DF3578(ctx, base);
	// 82E53984: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53988: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 82E5398C: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E53990: 4BF9F821  bl 0x82df31b0
	ctx.lr = 0x82E53994;
	sub_82DF31B0(ctx, base);
	// 82E53994: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E53998: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E5399C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E539A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E539A4: 48001E45  bl 0x82e557e8
	ctx.lr = 0x82E539A8;
	sub_82E557E8(ctx, base);
	// 82E539A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E539AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E539B0: 40820030  bne 0x82e539e0
	if !ctx.cr[0].eq {
	pc = 0x82E539E0; continue 'dispatch;
	}
	// 82E539B4: 4BF9FA75  bl 0x82df3428
	ctx.lr = 0x82E539B8;
	sub_82DF3428(ctx, base);
	// 82E539B8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E539BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E539C0: 419A0008  beq cr6, 0x82e539c8
	if ctx.cr[6].eq {
	pc = 0x82E539C8; continue 'dispatch;
	}
	// 82E539C4: 4B46CECD  bl 0x822c0890
	ctx.lr = 0x82E539C8;
	sub_822C0890(ctx, base);
	// 82E539C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E539CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E539D0: 4B47BC31  bl 0x822cf600
	ctx.lr = 0x82E539D4;
	sub_822CF600(ctx, base);
	// 82E539D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E539D8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E539DC: 483547D4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E539E0: 4BF9FA49  bl 0x82df3428
	ctx.lr = 0x82E539E4;
	sub_82DF3428(ctx, base);
	// 82E539E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E539E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E539EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E539F0: 48001EC1  bl 0x82e558b0
	ctx.lr = 0x82E539F4;
	sub_82E558B0(ctx, base);
	// 82E539F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E539F8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E539FC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E53A00: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53A04: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A08: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E53A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53A10: 4E800421  bctrl
	ctx.lr = 0x82E53A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53A14: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E53A18: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E53A1C: 4BFE588D  bl 0x82e392a8
	ctx.lr = 0x82E53A20;
	sub_82E392A8(ctx, base);
	// 82E53A20: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E53A24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E53A28: 419A0008  beq cr6, 0x82e53a30
	if ctx.cr[6].eq {
	pc = 0x82E53A30; continue 'dispatch;
	}
	// 82E53A2C: 4B46CE65  bl 0x822c0890
	ctx.lr = 0x82E53A30;
	sub_822C0890(ctx, base);
	// 82E53A30: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E53A34: 4BFFFF98  b 0x82e539cc
	pc = 0x82E539CC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53A38 size=68
    let mut pc: u32 = 0x82E53A38;
    'dispatch: loop {
        match pc {
            0x82E53A38 => {
    //   block [0x82E53A38..0x82E53A7C)
	// 82E53A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53A48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53A4C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53A50: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53A54: 396BCB28  addi r11, r11, -0x34d8
	ctx.r[11].s64 = ctx.r[11].s64 + -13528;
	// 82E53A58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53A5C: 41820008  beq 0x82e53a64
	if ctx.cr[0].eq {
	pc = 0x82E53A64; continue 'dispatch;
	}
	// 82E53A60: 4B46C809  bl 0x822c0268
	ctx.lr = 0x82E53A64;
	sub_822C0268(ctx, base);
	// 82E53A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E53A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E53A80 size=16
    let mut pc: u32 = 0x82E53A80;
    'dispatch: loop {
        match pc {
            0x82E53A80 => {
    //   block [0x82E53A80..0x82E53A90)
	// 82E53A80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53A84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53A88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E53A8C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E53A90 size=16
    let mut pc: u32 = 0x82E53A90;
    'dispatch: loop {
        match pc {
            0x82E53A90 => {
    //   block [0x82E53A90..0x82E53AA0)
	// 82E53A90: 3D400018  lis r10, 0x18
	ctx.r[10].s64 = 1572864;
	// 82E53A94: 614A0048  ori r10, r10, 0x48
	ctx.r[10].u64 = ctx.r[10].u64 | 72;
	// 82E53A98: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E53A9C: 483848A4  b 0x831d8340
	sub_831D8340(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E53AA0 size=4
    let mut pc: u32 = 0x82E53AA0;
    'dispatch: loop {
        match pc {
            0x82E53AA0 => {
    //   block [0x82E53AA0..0x82E53AA4)
	// 82E53AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53AA8 size=196
    let mut pc: u32 = 0x82E53AA8;
    'dispatch: loop {
        match pc {
            0x82E53AA8 => {
    //   block [0x82E53AA8..0x82E53B6C)
	// 82E53AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53AAC: 483546BD  bl 0x831a8168
	ctx.lr = 0x82E53AB0;
	sub_831A8130(ctx, base);
	// 82E53AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53AB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53AB8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E53ABC: 3FBF0018  addis r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 1572864;
	// 82E53AC0: 3D600018  lis r11, 0x18
	ctx.r[11].s64 = 1572864;
	// 82E53AC4: 3BBD0044  addi r29, r29, 0x44
	ctx.r[29].s64 = ctx.r[29].s64 + 68;
	// 82E53AC8: 9B9F000C  stb r28, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u8 ) };
	// 82E53ACC: 617E004C  ori r30, r11, 0x4c
	ctx.r[30].u64 = ctx.r[11].u64 | 76;
	// 82E53AD0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53AD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E53AD8: 419A0014  beq cr6, 0x82e53aec
	if ctx.cr[6].eq {
	pc = 0x82E53AEC; continue 'dispatch;
	}
	// 82E53ADC: 48384865  bl 0x831d8340
	ctx.lr = 0x82E53AE0;
	sub_831D8340(ctx, base);
	// 82E53AE0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E53AE4: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E53AE8: 4BD7A219  bl 0x82bcdd00
	ctx.lr = 0x82E53AEC;
	sub_82BCDD00(ctx, base);
	// 82E53AEC: 7FDFF214  add r30, r31, r30
	ctx.r[30].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82E53AF0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53AF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E53AF8: 419A000C  beq cr6, 0x82e53b04
	if ctx.cr[6].eq {
	pc = 0x82E53B04; continue 'dispatch;
	}
	// 82E53AFC: 4BD78F25  bl 0x82bcca20
	ctx.lr = 0x82E53B00;
	sub_82BCCA20(ctx, base);
	// 82E53B00: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E53B04: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E53B08: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82E53B0C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53B10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E53B14: 419A000C  beq cr6, 0x82e53b20
	if ctx.cr[6].eq {
	pc = 0x82E53B20; continue 'dispatch;
	}
	// 82E53B18: 48384801  bl 0x831d8318
	ctx.lr = 0x82E53B1C;
	sub_831D8318(ctx, base);
	// 82E53B1C: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E53B20: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E53B24: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E53B28: 4082FFE4  bne 0x82e53b0c
	if !ctx.cr[0].eq {
	pc = 0x82E53B0C; continue 'dispatch;
	}
	// 82E53B2C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53B30: 483842F1  bl 0x831d7e20
	ctx.lr = 0x82E53B34;
	sub_831D7E20(ctx, base);
	// 82E53B34: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53B38: 48384171  bl 0x831d7ca8
	ctx.lr = 0x82E53B3C;
	sub_831D7CA8(ctx, base);
	// 82E53B3C: 3D600018  lis r11, 0x18
	ctx.r[11].s64 = 1572864;
	// 82E53B40: 616B0038  ori r11, r11, 0x38
	ctx.r[11].u64 = ctx.r[11].u64 | 56;
	// 82E53B44: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E53B48: 4B46C721  bl 0x822c0268
	ctx.lr = 0x82E53B4C;
	sub_822C0268(ctx, base);
	// 82E53B4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53B50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E53B58: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E53B5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E53B60: 4E800421  bctrl
	ctx.lr = 0x82E53B64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E53B68: 48354650  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53B70 size=60
    let mut pc: u32 = 0x82E53B70;
    'dispatch: loop {
        match pc {
            0x82E53B70 => {
    //   block [0x82E53B70..0x82E53BAC)
	// 82E53B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53B78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53B7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53B80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53B84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E53B88: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53B8C: 483842ED  bl 0x831d7e78
	ctx.lr = 0x82E53B90;
	sub_831D7E78(ctx, base);
	// 82E53B90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E53B94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E53B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E53B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53BA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53BB0 size=60
    let mut pc: u32 = 0x82E53BB0;
    'dispatch: loop {
        match pc {
            0x82E53BB0 => {
    //   block [0x82E53BB0..0x82E53BEC)
	// 82E53BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53BC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53BC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53BC8: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53BCC: 483842F5  bl 0x831d7ec0
	ctx.lr = 0x82E53BD0;
	sub_831D7EC0(ctx, base);
	// 82E53BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53BD4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E53BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E53BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E53BF0 size=12
    let mut pc: u32 = 0x82E53BF0;
    'dispatch: loop {
        match pc {
            0x82E53BF0 => {
    //   block [0x82E53BF0..0x82E53BFC)
	// 82E53BF0: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82E53BF4: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82E53BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53C00 size=76
    let mut pc: u32 = 0x82E53C00;
    'dispatch: loop {
        match pc {
            0x82E53C00 => {
    //   block [0x82E53C00..0x82E53C4C)
	// 82E53C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53C08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53C0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53C14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E53C18: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53C1C: 4838414D  bl 0x831d7d68
	ctx.lr = 0x82E53C20;
	sub_831D7D68(ctx, base);
	// 82E53C20: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E53C24: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53C28: 40820010  bne 0x82e53c38
	if !ctx.cr[0].eq {
	pc = 0x82E53C38; continue 'dispatch;
	}
	// 82E53C2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E53C30: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53C34: 48384245  bl 0x831d7e78
	ctx.lr = 0x82E53C38;
	sub_831D7E78(ctx, base);
	// 82E53C38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E53C50 size=396
    let mut pc: u32 = 0x82E53C50;
    'dispatch: loop {
        match pc {
            0x82E53C50 => {
    //   block [0x82E53C50..0x82E53DDC)
	// 82E53C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53C54: 48354515  bl 0x831a8168
	ctx.lr = 0x82E53C58;
	sub_831A8130(ctx, base);
	// 82E53C58: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53C60: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E53C64: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53C68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53C6C: 409A0010  bne cr6, 0x82e53c7c
	if !ctx.cr[6].eq {
	pc = 0x82E53C7C; continue 'dispatch;
	}
	// 82E53C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E53C74: 4838424D  bl 0x831d7ec0
	ctx.lr = 0x82E53C78;
	sub_831D7EC0(ctx, base);
	// 82E53C78: 4800015C  b 0x82e53dd4
	pc = 0x82E53DD4; continue 'dispatch;
	// 82E53C7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E53C80: 483840E9  bl 0x831d7d68
	ctx.lr = 0x82E53C84;
	sub_831D7D68(ctx, base);
	// 82E53C84: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E53C88: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53C8C: 41820148  beq 0x82e53dd4
	if ctx.cr[0].eq {
	pc = 0x82E53DD4; continue 'dispatch;
	}
	// 82E53C90: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E53C94: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E53C98: 419A013C  beq cr6, 0x82e53dd4
	if ctx.cr[6].eq {
	pc = 0x82E53DD4; continue 'dispatch;
	}
	// 82E53C9C: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E53CA0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E53CA4: 419A0024  beq cr6, 0x82e53cc8
	if ctx.cr[6].eq {
	pc = 0x82E53CC8; continue 'dispatch;
	}
	// 82E53CA8: 548B003E  slwi r11, r4, 0
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E53CAC: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82E53CB0: 395F0038  addi r10, r31, 0x38
	ctx.r[10].s64 = ctx.r[31].s64 + 56;
	// 82E53CB4: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E53CB8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53CBC: 3D4A0003  addis r10, r10, 3
	ctx.r[10].s64 = ctx.r[10].s64 + 196608;
	// 82E53CC0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E53CC4: 4082FFF0  bne 0x82e53cb4
	if !ctx.cr[0].eq {
	pc = 0x82E53CB4; continue 'dispatch;
	}
	// 82E53CC8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E53CCC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82E53CD0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E53CD4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82E53CD8: 4E800421  bctrl
	ctx.lr = 0x82E53CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E53CDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E53CE0: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82E53CE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E53CE8: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82E53CEC: 483544F5  bl 0x831a81e0
	ctx.lr = 0x82E53CF0;
	sub_831A81E0(ctx, base);
	// 82E53CF0: 3FDF0018  addis r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 1572864;
	// 82E53CF4: 39600C00  li r11, 0xc00
	ctx.r[11].s64 = 3072;
	// 82E53CF8: 3BDE003C  addi r30, r30, 0x3c
	ctx.r[30].s64 = ctx.r[30].s64 + 60;
	// 82E53CFC: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82E53D00: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E53D04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53D08: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82E53D0C: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E53D10: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E53D14: 38ABAFD0  addi r5, r11, -0x5030
	ctx.r[5].s64 = ctx.r[11].s64 + -20528;
	// 82E53D18: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E53D1C: 40980064  bge cr6, 0x82e53d80
	if !ctx.cr[6].lt {
	pc = 0x82E53D80; continue 'dispatch;
	}
	// 82E53D20: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E53D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53D28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E53D2C: 40990048  ble cr6, 0x82e53d74
	if !ctx.cr[6].gt {
	pc = 0x82E53D74; continue 'dispatch;
	}
	// 82E53D30: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E53D34: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E53D38: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53D3C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E53D40: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E53D44: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E53D48: 551C103A  slwi r28, r8, 2
	ctx.r[28].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82E53D4C: 7D0919D6  mullw r8, r9, r3
	ctx.r[8].s64 = (ctx.r[9].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82E53D50: 7C9C202E  lwzx r4, r28, r4
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82E53D54: 7C043C2E  lfsx f0, r4, r7
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E53D58: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E53D5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E53D60: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E53D64: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 82E53D68: 811F0030  lwz r8, 0x30(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E53D6C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E53D70: 4198FFC8  blt cr6, 0x82e53d38
	if ctx.cr[6].lt {
	pc = 0x82E53D38; continue 'dispatch;
	}
	// 82E53D74: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E53D78: 2B090080  cmplwi cr6, r9, 0x80
	ctx.cr[6].compare_u32(ctx.r[9].u32, 128 as u32, &mut ctx.xer);
	// 82E53D7C: 4198FF9C  blt cr6, 0x82e53d18
	if ctx.cr[6].lt {
	pc = 0x82E53D18; continue 'dispatch;
	}
	// 82E53D80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E53D84: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E53D88: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E53D8C: 48384035  bl 0x831d7dc0
	ctx.lr = 0x82E53D90;
	sub_831D7DC0(ctx, base);
	// 82E53D90: 3D7F0018  addis r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 1572864;
	// 82E53D94: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82E53D98: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53D9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E53DA0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E53DA4: 2B0A007F  cmplwi cr6, r10, 0x7f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 127 as u32, &mut ctx.xer);
	// 82E53DA8: 4099000C  ble cr6, 0x82e53db4
	if !ctx.cr[6].gt {
	pc = 0x82E53DB4; continue 'dispatch;
	}
	// 82E53DAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E53DB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E53DB4: 3D400018  lis r10, 0x18
	ctx.r[10].s64 = 1572864;
	// 82E53DB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E53DBC: 81210084  lwz r9, 0x84(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E53DC0: 614A0038  ori r10, r10, 0x38
	ctx.r[10].u64 = ctx.r[10].u64 | 56;
	// 82E53DC4: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82E53DC8: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E53DCC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E53DD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53DD4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82E53DD8: 483543E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53DE0 size=128
    let mut pc: u32 = 0x82E53DE0;
    'dispatch: loop {
        match pc {
            0x82E53DE0 => {
    //   block [0x82E53DE0..0x82E53E60)
	// 82E53DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53DEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53DF0: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82E53DF4: 817F6820  lwz r11, 0x6820(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26656 as u32) ) } as u64;
	// 82E53DF8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E53DFC: 419A004C  beq cr6, 0x82e53e48
	if ctx.cr[6].eq {
	pc = 0x82E53E48; continue 'dispatch;
	}
	// 82E53E00: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E53E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53E08: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 82E53E0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53E10: 3929FEA8  addi r9, r9, -0x158
	ctx.r[9].s64 = ctx.r[9].s64 + -344;
	// 82E53E14: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82E53E18: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E53E1C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E53E20: 48384411  bl 0x831d8230
	ctx.lr = 0x82E53E24;
	sub_831D8230(ctx, base);
	// 82E53E24: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E53E28: 40800018  bge 0x82e53e40
	if !ctx.cr[0].lt {
	pc = 0x82E53E40; continue 'dispatch;
	}
	// 82E53E2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53E30: 386BCB60  addi r3, r11, -0x34a0
	ctx.r[3].s64 = ctx.r[11].s64 + -13472;
	// 82E53E34: 4BD79465  bl 0x82bcd298
	ctx.lr = 0x82E53E38;
	sub_82BCD298(ctx, base);
	// 82E53E38: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E53E3C: 4BD78ABD  bl 0x82bcc8f8
	ctx.lr = 0x82E53E40;
	sub_82BCC8F8(ctx, base);
	// 82E53E40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E53E44: 917F6820  stw r11, 0x6820(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26656 as u32), ctx.r[11].u32 ) };
	// 82E53E48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E53E4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E53E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53E58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53E60 size=64
    let mut pc: u32 = 0x82E53E60;
    'dispatch: loop {
        match pc {
            0x82E53E60 => {
    //   block [0x82E53E60..0x82E53EA0)
	// 82E53E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E53E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E53E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53E70: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 82E53E74: 817F6820  lwz r11, 0x6820(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26656 as u32) ) } as u64;
	// 82E53E78: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E53E7C: 409A0008  bne cr6, 0x82e53e84
	if !ctx.cr[6].eq {
	pc = 0x82E53E84; continue 'dispatch;
	}
	// 82E53E80: 483842B9  bl 0x831d8138
	ctx.lr = 0x82E53E84;
	sub_831D8138(ctx, base);
	// 82E53E84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E53E88: 917F6820  stw r11, 0x6820(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(26656 as u32), ctx.r[11].u32 ) };
	// 82E53E8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E53E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E53E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E53E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E53E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E53EA0 size=92
    let mut pc: u32 = 0x82E53EA0;
    'dispatch: loop {
        match pc {
            0x82E53EA0 => {
    //   block [0x82E53EA0..0x82E53EFC)
	// 82E53EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53EA4: 483542C5  bl 0x831a8168
	ctx.lr = 0x82E53EA8;
	sub_831A8130(ctx, base);
	// 82E53EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53EAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E53EB0: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82E53EB4: 3FBE0018  addis r29, r30, 0x18
	ctx.r[29].s64 = ctx.r[30].s64 + 1572864;
	// 82E53EB8: 3BBD0044  addi r29, r29, 0x44
	ctx.r[29].s64 = ctx.r[29].s64 + 68;
	// 82E53EBC: 48000018  b 0x82e53ed4
	pc = 0x82E53ED4; continue 'dispatch;
	// 82E53EC0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82E53EC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E53EC8: 4BFFFD89  bl 0x82e53c50
	ctx.lr = 0x82E53ECC;
	sub_82E53C50(ctx, base);
	// 82E53ECC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E53ED0: 4082FFF4  bne 0x82e53ec4
	if !ctx.cr[0].eq {
	pc = 0x82E53EC4; continue 'dispatch;
	}
	// 82E53ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E53ED8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E53EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E53EE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E53EE4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82E53EE8: 48384469  bl 0x831d8350
	ctx.lr = 0x82E53EEC;
	sub_831D8350(ctx, base);
	// 82E53EEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E53EF0: 4082FFD0  bne 0x82e53ec0
	if !ctx.cr[0].eq {
	pc = 0x82E53EC0; continue 'dispatch;
	}
	// 82E53EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E53EF8: 483542C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E53F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E53F00 size=560
    let mut pc: u32 = 0x82E53F00;
    'dispatch: loop {
        match pc {
            0x82E53F00 => {
    //   block [0x82E53F00..0x82E54130)
	// 82E53F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E53F04: 48354259  bl 0x831a815c
	ctx.lr = 0x82E53F08;
	sub_831A8130(ctx, base);
	// 82E53F08: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E53F0C: 3C600018  lis r3, 0x18
	ctx.r[3].s64 = 1572864;
	// 82E53F10: 60630050  ori r3, r3, 0x50
	ctx.r[3].u64 = ctx.r[3].u64 | 80;
	// 82E53F14: 4B46CA25  bl 0x822c0938
	ctx.lr = 0x82E53F18;
	sub_822C0938(ctx, base);
	// 82E53F18: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E53F1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E53F20: 41820018  beq 0x82e53f38
	if ctx.cr[0].eq {
	pc = 0x82E53F38; continue 'dispatch;
	}
	// 82E53F24: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53F28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E53F2C: 396BCB44  addi r11, r11, -0x34bc
	ctx.r[11].s64 = ctx.r[11].s64 + -13500;
	// 82E53F30: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E53F34: 48000008  b 0x82e53f3c
	pc = 0x82E53F3C; continue 'dispatch;
	// 82E53F38: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82E53F3C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82E53F40: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82E53F44: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E53F48: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82E53F4C: 616BC000  ori r11, r11, 0xc000
	ctx.r[11].u64 = ctx.r[11].u64 | 49152;
	// 82E53F50: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82E53F54: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82E53F58: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E53F5C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E53F60: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82E53F64: 816A6820  lwz r11, 0x6820(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26656 as u32) ) } as u64;
	// 82E53F68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E53F6C: 409A0018  bne cr6, 0x82e53f84
	if !ctx.cr[6].eq {
	pc = 0x82E53F84; continue 'dispatch;
	}
	// 82E53F70: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53F74: 386BCBB0  addi r3, r11, -0x3450
	ctx.r[3].s64 = ctx.r[11].s64 + -13392;
	// 82E53F78: 4BD79321  bl 0x82bcd298
	ctx.lr = 0x82E53F7C;
	sub_82BCD298(ctx, base);
	// 82E53F7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E53F80: 480001A8  b 0x82e54128
	pc = 0x82E54128; continue 'dispatch;
	// 82E53F84: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E53F88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E53F8C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E53F90: 48354251  bl 0x831a81e0
	ctx.lr = 0x82E53F94;
	sub_831A81E0(ctx, base);
	// 82E53F94: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53F98: 3D4082E5  lis r10, -0x7d1b
	ctx.r[10].s64 = -2098921472;
	// 82E53F9C: 9B810050  stb r28, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82E53FA0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 82E53FA4: 39000080  li r8, 0x80
	ctx.r[8].s64 = 128;
	// 82E53FA8: 6129BB80  ori r9, r9, 0xbb80
	ctx.r[9].u64 = ctx.r[9].u64 | 48000;
	// 82E53FAC: C00BC7EC  lfs f0, -0x3814(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E53FB0: 394A3A80  addi r10, r10, 0x3a80
	ctx.r[10].s64 = ctx.r[10].s64 + 14976;
	// 82E53FB4: 3B3F001C  addi r25, r31, 0x1c
	ctx.r[25].s64 = ctx.r[31].s64 + 28;
	// 82E53FB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E53FBC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E53FC0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E53FC4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82E53FC8: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82E53FCC: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E53FD0: 9901008B  stb r8, 0x8b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(139 as u32), ctx.r[8].u8 ) };
	// 82E53FD4: 9141009C  stw r10, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 82E53FD8: 93E100A8  stw r31, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[31].u32 ) };
	// 82E53FDC: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82E53FE0: 48384259  bl 0x831d8238
	ctx.lr = 0x82E53FE4;
	sub_831D8238(ctx, base);
	// 82E53FE4: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E53FE8: 40800010  bge 0x82e53ff8
	if !ctx.cr[0].lt {
	pc = 0x82E53FF8; continue 'dispatch;
	}
	// 82E53FEC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E53FF0: 386BCB80  addi r3, r11, -0x3480
	ctx.r[3].s64 = ctx.r[11].s64 + -13440;
	// 82E53FF4: 4835BC15  bl 0x831afc08
	ctx.lr = 0x82E53FF8;
	sub_831AFC08(ctx, base);
	// 82E53FF8: 3C600006  lis r3, 6
	ctx.r[3].s64 = 393216;
	// 82E53FFC: 4BF9C585  bl 0x82df0580
	ctx.lr = 0x82E54000;
	sub_82DF0580(ctx, base);
	// 82E54000: 3F5F0018  addis r26, r31, 0x18
	ctx.r[26].s64 = ctx.r[31].s64 + 1572864;
	// 82E54004: 3CA00006  lis r5, 6
	ctx.r[5].s64 = 393216;
	// 82E54008: 3B5A0038  addi r26, r26, 0x38
	ctx.r[26].s64 = ctx.r[26].s64 + 56;
	// 82E5400C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E54010: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E54014: 483541CD  bl 0x831a81e0
	ctx.lr = 0x82E54018;
	sub_831A81E0(ctx, base);
	// 82E54018: 3FBF0018  addis r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 1572864;
	// 82E5401C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54020: 3FDF0018  addis r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 1572864;
	// 82E54024: 3BBD003C  addi r29, r29, 0x3c
	ctx.r[29].s64 = ctx.r[29].s64 + 60;
	// 82E54028: 3BDE0040  addi r30, r30, 0x40
	ctx.r[30].s64 = ctx.r[30].s64 + 64;
	// 82E5402C: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82E54030: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E54034: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 82E54038: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5403C: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E54040: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82E54044: 4835419D  bl 0x831a81e0
	ctx.lr = 0x82E54048;
	sub_831A81E0(ctx, base);
	// 82E54048: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82E5404C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54050: 39400C00  li r10, 0xc00
	ctx.r[10].s64 = 3072;
	// 82E54054: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E54058: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82E5405C: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E54060: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82E54064: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54068: 48383D59  bl 0x831d7dc0
	ctx.lr = 0x82E5406C;
	sub_831D7DC0(ctx, base);
	// 82E5406C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54070: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E54074: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54078: 2B0B007F  cmplwi cr6, r11, 0x7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 127 as u32, &mut ctx.xer);
	// 82E5407C: 40990008  ble cr6, 0x82e54084
	if !ctx.cr[6].gt {
	pc = 0x82E54084; continue 'dispatch;
	}
	// 82E54080: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E54084: 814100B4  lwz r10, 0xb4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82E54088: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E5408C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54090: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54094: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82E54098: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E5409C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E540A0: 4082FFAC  bne 0x82e5404c
	if !ctx.cr[0].eq {
	pc = 0x82E5404C; continue 'dispatch;
	}
	// 82E540A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E540A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E540AC: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82E540B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E540B4: 483841E5  bl 0x831d8298
	ctx.lr = 0x82E540B8;
	sub_831D8298(ctx, base);
	// 82E540B8: 3D600018  lis r11, 0x18
	ctx.r[11].s64 = 1572864;
	// 82E540BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E540C0: 616B0044  ori r11, r11, 0x44
	ctx.r[11].u64 = ctx.r[11].u64 | 68;
	// 82E540C4: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82E540C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E540CC: 483841CD  bl 0x831d8298
	ctx.lr = 0x82E540D0;
	sub_831D8298(ctx, base);
	// 82E540D0: 3D600018  lis r11, 0x18
	ctx.r[11].s64 = 1572864;
	// 82E540D4: 3D4082E5  lis r10, -0x7d1b
	ctx.r[10].s64 = -2098921472;
	// 82E540D8: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 82E540DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E540E0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82E540E4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E540E8: 38AA3EA0  addi r5, r10, 0x3ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 16032;
	// 82E540EC: 7C7F592E  stwx r3, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82E540F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E540F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E540F8: 48370A91  bl 0x831c4b88
	ctx.lr = 0x82E540FC;
	sub_831C4B88(ctx, base);
	// 82E540FC: 3FDF0018  addis r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 1572864;
	// 82E54100: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E54104: 3BDE004C  addi r30, r30, 0x4c
	ctx.r[30].s64 = ctx.r[30].s64 + 76;
	// 82E54108: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E5410C: 4BD79DF5  bl 0x82bcdf00
	ctx.lr = 0x82E54110;
	sub_82BCDF00(ctx, base);
	// 82E54110: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E54114: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54118: 4BD79C01  bl 0x82bcdd18
	ctx.lr = 0x82E5411C;
	sub_82BCDD18(ctx, base);
	// 82E5411C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54120: 48371031  bl 0x831c5150
	ctx.lr = 0x82E54124;
	sub_831C5150(ctx, base);
	// 82E54124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54128: 38210150  addi r1, r1, 0x150
	ctx.r[1].s64 = ctx.r[1].s64 + 336;
	// 82E5412C: 48354080  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E54130 size=4
    let mut pc: u32 = 0x82E54130;
    'dispatch: loop {
        match pc {
            0x82E54130 => {
    //   block [0x82E54130..0x82E54134)
	// 82E54130: 4BFFFDD0  b 0x82e53f00
	sub_82E53F00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54138 size=68
    let mut pc: u32 = 0x82E54138;
    'dispatch: loop {
        match pc {
            0x82E54138 => {
    //   block [0x82E54138..0x82E5417C)
	// 82E54138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5414C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E54150: 4BD78E29  bl 0x82bccf78
	ctx.lr = 0x82E54154;
	sub_82BCCF78(ctx, base);
	// 82E54154: 907F0044  stw r3, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 82E54158: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5415C: 4182000C  beq 0x82e54168
	if ctx.cr[0].eq {
	pc = 0x82E54168; continue 'dispatch;
	}
	// 82E54160: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 82E54164: 4BF9C165  bl 0x82df02c8
	ctx.lr = 0x82E54168;
	sub_82DF02C8(ctx, base);
	// 82E54168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5416C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54180 size=128
    let mut pc: u32 = 0x82E54180;
    'dispatch: loop {
        match pc {
            0x82E54180 => {
    //   block [0x82E54180..0x82E54200)
	// 82E54180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5418C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54194: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82E54198: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5419C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E541A0: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E541A4: 483EE3F9  bl 0x8324259c
	ctx.lr = 0x82E541A8;
	// extern call 0x8324259C → crate::xboxkrnl::XNotifyGetNext
	crate::xboxkrnl::XNotifyGetNext(ctx, base);
	// 82E541A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E541AC: 41820020  beq 0x82e541cc
	if ctx.cr[0].eq {
	pc = 0x82E541CC; continue 'dispatch;
	}
	// 82E541B0: 3D600A00  lis r11, 0xa00
	ctx.r[11].s64 = 167772160;
	// 82E541B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E541B8: 616B0003  ori r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64 | 3;
	// 82E541BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E541C0: 409A000C  bne cr6, 0x82e541cc
	if !ctx.cr[6].eq {
	pc = 0x82E541CC; continue 'dispatch;
	}
	// 82E541C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E541C8: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E541CC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E541D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E541D4: 419A0010  beq cr6, 0x82e541e4
	if ctx.cr[6].eq {
	pc = 0x82E541E4; continue 'dispatch;
	}
	// 82E541D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E541DC: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E541E0: 4800000C  b 0x82e541ec
	pc = 0x82E541EC; continue 'dispatch;
	// 82E541E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E541E8: C02B08A4  lfs f1, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E541EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E541F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E541F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E541F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E541FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54200 size=68
    let mut pc: u32 = 0x82E54200;
    'dispatch: loop {
        match pc {
            0x82E54200 => {
    //   block [0x82E54200..0x82E54244)
	// 82E54200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5420C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E54210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54218: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E5421C: 4BFFFF65  bl 0x82e54180
	ctx.lr = 0x82E54220;
	sub_82E54180(ctx, base);
	// 82E54220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54224: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E54228: 48000129  bl 0x82e54350
	ctx.lr = 0x82E5422C;
	sub_82E54350(ctx, base);
	// 82E5422C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E54230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54238: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E5423C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54248 size=80
    let mut pc: u32 = 0x82E54248;
    'dispatch: loop {
        match pc {
            0x82E54248 => {
    //   block [0x82E54248..0x82E54298)
	// 82E54248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5424C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5425C: 4800056D  bl 0x82e547c8
	ctx.lr = 0x82E54260;
	sub_82E547C8(ctx, base);
	// 82E54260: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E54264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54268: 394ACBF8  addi r10, r10, -0x3408
	ctx.r[10].s64 = ctx.r[10].s64 + -13320;
	// 82E5426C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E54270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54274: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E54278: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E5427C: 4BFFFEBD  bl 0x82e54138
	ctx.lr = 0x82E54280;
	sub_82E54138(ctx, base);
	// 82E54280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E54288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5428C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54298 size=100
    let mut pc: u32 = 0x82E54298;
    'dispatch: loop {
        match pc {
            0x82E54298 => {
    //   block [0x82E54298..0x82E542FC)
	// 82E54298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E542A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E542A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E542A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E542AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E542B0: 396BCBF8  addi r11, r11, -0x3408
	ctx.r[11].s64 = ctx.r[11].s64 + -13320;
	// 82E542B4: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E542B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E542BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E542C0: 419A0010  beq cr6, 0x82e542d0
	if ctx.cr[6].eq {
	pc = 0x82E542D0; continue 'dispatch;
	}
	// 82E542C4: 4BD7875D  bl 0x82bcca20
	ctx.lr = 0x82E542C8;
	sub_82BCCA20(ctx, base);
	// 82E542C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E542CC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E542D0: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E542D4: 4BF9F155  bl 0x82df3428
	ctx.lr = 0x82E542D8;
	sub_82DF3428(ctx, base);
	// 82E542D8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E542DC: 4BFC7AAD  bl 0x82e1bd88
	ctx.lr = 0x82E542E0;
	sub_82E1BD88(ctx, base);
	// 82E542E0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E542E4: 4BF9F145  bl 0x82df3428
	ctx.lr = 0x82E542E8;
	sub_82DF3428(ctx, base);
	// 82E542E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E542EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E542F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E542F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E542F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54300 size=80
    let mut pc: u32 = 0x82E54300;
    'dispatch: loop {
        match pc {
            0x82E54300 => {
    //   block [0x82E54300..0x82E54350)
	// 82E54300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5430C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54314: 480004FD  bl 0x82e54810
	ctx.lr = 0x82E54318;
	sub_82E54810(ctx, base);
	// 82E54318: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E5431C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E54320: 394ACBF8  addi r10, r10, -0x3408
	ctx.r[10].s64 = ctx.r[10].s64 + -13320;
	// 82E54324: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E54328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5432C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E54330: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E54334: 4BFFFE05  bl 0x82e54138
	ctx.lr = 0x82E54338;
	sub_82E54138(ctx, base);
	// 82E54338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5433C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E54340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5434C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54350 size=88
    let mut pc: u32 = 0x82E54350;
    'dispatch: loop {
        match pc {
            0x82E54350 => {
    //   block [0x82E54350..0x82E543A8)
	// 82E54350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5435C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54360: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54364: 419A0030  beq cr6, 0x82e54394
	if ctx.cr[6].eq {
	pc = 0x82E54394; continue 'dispatch;
	}
	// 82E54368: 89630035  lbz r11, 0x35(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E5436C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54370: 41820024  beq 0x82e54394
	if ctx.cr[0].eq {
	pc = 0x82E54394; continue 'dispatch;
	}
	// 82E54374: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54378: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E5437C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54380: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E54384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54388: 4E800421  bctrl
	ctx.lr = 0x82E5438C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5438C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54390: 48000008  b 0x82e54398
	pc = 0x82E54398; continue 'dispatch;
	// 82E54394: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E54398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5439C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E543A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E543A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E543A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E543A8 size=148
    let mut pc: u32 = 0x82E543A8;
    'dispatch: loop {
        match pc {
            0x82E543A8 => {
    //   block [0x82E543A8..0x82E5443C)
	// 82E543A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E543AC: 48353DC1  bl 0x831a816c
	ctx.lr = 0x82E543B0;
	sub_831A8130(ctx, base);
	// 82E543B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E543B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E543B8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E543BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E543C0: 4BF9F4B9  bl 0x82df3878
	ctx.lr = 0x82E543C4;
	sub_82DF3878(ctx, base);
	// 82E543C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E543C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E543CC: 419A0068  beq cr6, 0x82e54434
	if ctx.cr[6].eq {
	pc = 0x82E54434; continue 'dispatch;
	}
	// 82E543D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E543D4: 3BABCC60  addi r29, r11, -0x33a0
	ctx.r[29].s64 = ctx.r[11].s64 + -13216;
	// 82E543D8: 4BFFD1E9  bl 0x82e515c0
	ctx.lr = 0x82E543DC;
	sub_82E515C0(ctx, base);
	// 82E543DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E543E0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E543E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E543E8: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82E543EC: 4BFA544D  bl 0x82df9838
	ctx.lr = 0x82E543F0;
	sub_82DF9838(ctx, base);
	// 82E543F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E543F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E543F8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E543FC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54400: 4BF9EDB1  bl 0x82df31b0
	ctx.lr = 0x82E54404;
	sub_82DF31B0(ctx, base);
	// 82E54404: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54408: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5440C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54410: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54418: 4E800421  bctrl
	ctx.lr = 0x82E5441C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5441C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E54420: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54424: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E54428: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E5442C: 997F0037  stb r11, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E54430: 4B47B1D1  bl 0x822cf600
	ctx.lr = 0x82E54434;
	sub_822CF600(ctx, base);
	// 82E54434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54438: 48353D84  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54440 size=164
    let mut pc: u32 = 0x82E54440;
    'dispatch: loop {
        match pc {
            0x82E54440 => {
    //   block [0x82E54440..0x82E544E4)
	// 82E54440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5444C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54450: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E54454: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5445C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54460: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54464: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54468: 419A005C  beq cr6, 0x82e544c4
	if ctx.cr[6].eq {
	pc = 0x82E544C4; continue 'dispatch;
	}
	// 82E5446C: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54470: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54474: 41820050  beq 0x82e544c4
	if ctx.cr[0].eq {
	pc = 0x82E544C4; continue 'dispatch;
	}
	// 82E54478: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5447C: 3BCBCC60  addi r30, r11, -0x33a0
	ctx.r[30].s64 = ctx.r[11].s64 + -13216;
	// 82E54480: 4BFFD141  bl 0x82e515c0
	ctx.lr = 0x82E54484;
	sub_82E515C0(ctx, base);
	// 82E54484: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54488: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5448C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E54490: 38C0009E  li r6, 0x9e
	ctx.r[6].s64 = 158;
	// 82E54494: 4BFA53A5  bl 0x82df9838
	ctx.lr = 0x82E54498;
	sub_82DF9838(ctx, base);
	// 82E54498: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5449C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E544A0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E544A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E544A8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E544AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E544B0: 4E800421  bctrl
	ctx.lr = 0x82E544B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E544B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E544B8: 4B47B149  bl 0x822cf600
	ctx.lr = 0x82E544BC;
	sub_822CF600(ctx, base);
	// 82E544BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E544C0: 48000008  b 0x82e544c8
	pc = 0x82E544C8; continue 'dispatch;
	// 82E544C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E544C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E544CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E544D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E544D4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E544D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E544DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E544E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E544E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E544E8 size=148
    let mut pc: u32 = 0x82E544E8;
    'dispatch: loop {
        match pc {
            0x82E544E8 => {
    //   block [0x82E544E8..0x82E5457C)
	// 82E544E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E544EC: 48353C81  bl 0x831a816c
	ctx.lr = 0x82E544F0;
	sub_831A8130(ctx, base);
	// 82E544F0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E544F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E544F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E544FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54504: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54508: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5450C: 419A0060  beq cr6, 0x82e5456c
	if ctx.cr[6].eq {
	pc = 0x82E5456C; continue 'dispatch;
	}
	// 82E54510: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54514: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54518: 41820054  beq 0x82e5456c
	if ctx.cr[0].eq {
	pc = 0x82E5456C; continue 'dispatch;
	}
	// 82E5451C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54520: 3BABCC60  addi r29, r11, -0x33a0
	ctx.r[29].s64 = ctx.r[11].s64 + -13216;
	// 82E54524: 4BFFD09D  bl 0x82e515c0
	ctx.lr = 0x82E54528;
	sub_82E515C0(ctx, base);
	// 82E54528: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5452C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54530: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54534: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82E54538: 4BFA5301  bl 0x82df9838
	ctx.lr = 0x82E5453C;
	sub_82DF9838(ctx, base);
	// 82E5453C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54540: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E54544: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54548: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5454C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54550: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E54554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54558: 4E800421  bctrl
	ctx.lr = 0x82E5455C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5455C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54560: 4B47B0A1  bl 0x822cf600
	ctx.lr = 0x82E54564;
	sub_822CF600(ctx, base);
	// 82E54564: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54568: 48000008  b 0x82e54570
	pc = 0x82E54570; continue 'dispatch;
	// 82E5456C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E54570: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E54574: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E54578: 48353C44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54580 size=120
    let mut pc: u32 = 0x82E54580;
    'dispatch: loop {
        match pc {
            0x82E54580 => {
    //   block [0x82E54580..0x82E545F8)
	// 82E54580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54584: 48353BE9  bl 0x831a816c
	ctx.lr = 0x82E54588;
	sub_831A8130(ctx, base);
	// 82E54588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5458C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54590: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54594: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5459C: 419A0054  beq cr6, 0x82e545f0
	if ctx.cr[6].eq {
	pc = 0x82E545F0; continue 'dispatch;
	}
	// 82E545A0: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E545A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E545A8: 41820048  beq 0x82e545f0
	if ctx.cr[0].eq {
	pc = 0x82E545F0; continue 'dispatch;
	}
	// 82E545AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E545B0: 3BABCC60  addi r29, r11, -0x33a0
	ctx.r[29].s64 = ctx.r[11].s64 + -13216;
	// 82E545B4: 4BFFD00D  bl 0x82e515c0
	ctx.lr = 0x82E545B8;
	sub_82E515C0(ctx, base);
	// 82E545B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E545BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E545C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E545C4: 38C000C1  li r6, 0xc1
	ctx.r[6].s64 = 193;
	// 82E545C8: 4BFA5271  bl 0x82df9838
	ctx.lr = 0x82E545CC;
	sub_82DF9838(ctx, base);
	// 82E545CC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E545D0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E545D4: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E545D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E545DC: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E545E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E545E4: 4E800421  bctrl
	ctx.lr = 0x82E545E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E545E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E545EC: 4B47B015  bl 0x822cf600
	ctx.lr = 0x82E545F0;
	sub_822CF600(ctx, base);
	// 82E545F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E545F4: 48353BC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E545F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E545F8 size=144
    let mut pc: u32 = 0x82E545F8;
    'dispatch: loop {
        match pc {
            0x82E545F8 => {
    //   block [0x82E545F8..0x82E54688)
	// 82E545F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E545FC: 48353B71  bl 0x831a816c
	ctx.lr = 0x82E54600;
	sub_831A8130(ctx, base);
	// 82E54600: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E54604: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5460C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54610: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54614: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5461C: 419A0060  beq cr6, 0x82e5467c
	if ctx.cr[6].eq {
	pc = 0x82E5467C; continue 'dispatch;
	}
	// 82E54620: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54624: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54628: 41820054  beq 0x82e5467c
	if ctx.cr[0].eq {
	pc = 0x82E5467C; continue 'dispatch;
	}
	// 82E5462C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54630: 3BABCC60  addi r29, r11, -0x33a0
	ctx.r[29].s64 = ctx.r[11].s64 + -13216;
	// 82E54634: 4BFFCF8D  bl 0x82e515c0
	ctx.lr = 0x82E54638;
	sub_82E515C0(ctx, base);
	// 82E54638: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5463C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54640: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54644: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82E54648: 4BFA51F1  bl 0x82df9838
	ctx.lr = 0x82E5464C;
	sub_82DF9838(ctx, base);
	// 82E5464C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E54650: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54654: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E54658: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E5465C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E54660: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E54664: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54668: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E5466C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54670: 4E800421  bctrl
	ctx.lr = 0x82E54674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54674: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54678: 4B47AF89  bl 0x822cf600
	ctx.lr = 0x82E5467C;
	sub_822CF600(ctx, base);
	// 82E5467C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E54680: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E54684: 48353B38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54688 size=160
    let mut pc: u32 = 0x82E54688;
    'dispatch: loop {
        match pc {
            0x82E54688 => {
    //   block [0x82E54688..0x82E54728)
	// 82E54688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54698: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E5469C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E546A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E546A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E546A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E546AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E546B0: 419A005C  beq cr6, 0x82e5470c
	if ctx.cr[6].eq {
	pc = 0x82E5470C; continue 'dispatch;
	}
	// 82E546B4: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E546B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E546BC: 41820050  beq 0x82e5470c
	if ctx.cr[0].eq {
	pc = 0x82E5470C; continue 'dispatch;
	}
	// 82E546C0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E546C4: 3BCBCC60  addi r30, r11, -0x33a0
	ctx.r[30].s64 = ctx.r[11].s64 + -13216;
	// 82E546C8: 4BFFCEF9  bl 0x82e515c0
	ctx.lr = 0x82E546CC;
	sub_82E515C0(ctx, base);
	// 82E546CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E546D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E546D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E546D8: 38C000DF  li r6, 0xdf
	ctx.r[6].s64 = 223;
	// 82E546DC: 4BFA515D  bl 0x82df9838
	ctx.lr = 0x82E546E0;
	sub_82DF9838(ctx, base);
	// 82E546E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E546E4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E546E8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E546EC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E546F0: C04B08A4  lfs f2, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E546F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E546F8: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E546FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54700: 4E800421  bctrl
	ctx.lr = 0x82E54704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54704: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54708: 4B47AEF9  bl 0x822cf600
	ctx.lr = 0x82E5470C;
	sub_822CF600(ctx, base);
	// 82E5470C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54718: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E5471C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E54720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54728 size=160
    let mut pc: u32 = 0x82E54728;
    'dispatch: loop {
        match pc {
            0x82E54728 => {
    //   block [0x82E54728..0x82E547C8)
	// 82E54728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54730: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54734: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54738: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E5473C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54744: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54748: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5474C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54750: 419A005C  beq cr6, 0x82e547ac
	if ctx.cr[6].eq {
	pc = 0x82E547AC; continue 'dispatch;
	}
	// 82E54754: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54758: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5475C: 41820050  beq 0x82e547ac
	if ctx.cr[0].eq {
	pc = 0x82E547AC; continue 'dispatch;
	}
	// 82E54760: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54764: 3BCBCC60  addi r30, r11, -0x33a0
	ctx.r[30].s64 = ctx.r[11].s64 + -13216;
	// 82E54768: 4BFFCE59  bl 0x82e515c0
	ctx.lr = 0x82E5476C;
	sub_82E515C0(ctx, base);
	// 82E5476C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54770: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54774: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E54778: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 82E5477C: 4BFA50BD  bl 0x82df9838
	ctx.lr = 0x82E54780;
	sub_82DF9838(ctx, base);
	// 82E54780: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E54784: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54788: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E5478C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54790: C04B08A4  lfs f2, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E54794: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54798: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E5479C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E547A0: 4E800421  bctrl
	ctx.lr = 0x82E547A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E547A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E547A8: 4B47AE59  bl 0x822cf600
	ctx.lr = 0x82E547AC;
	sub_822CF600(ctx, base);
	// 82E547AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E547B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E547B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E547B8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E547BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E547C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E547C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E547C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E547C8 size=68
    let mut pc: u32 = 0x82E547C8;
    'dispatch: loop {
        match pc {
            0x82E547C8 => {
    //   block [0x82E547C8..0x82E5480C)
	// 82E547C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E547CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E547D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E547D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E547D8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E547DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E547E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E547E4: 4BFFDBDD  bl 0x82e523c0
	ctx.lr = 0x82E547E8;
	sub_82E523C0(ctx, base);
	// 82E547E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E547EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E547F0: 396BCCB0  addi r11, r11, -0x3350
	ctx.r[11].s64 = ctx.r[11].s64 + -13136;
	// 82E547F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E547F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E547FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54810 size=72
    let mut pc: u32 = 0x82E54810;
    'dispatch: loop {
        match pc {
            0x82E54810 => {
    //   block [0x82E54810..0x82E54858)
	// 82E54810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5481C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54820: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E54824: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E54828: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E5482C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54830: 4BFFDB91  bl 0x82e523c0
	ctx.lr = 0x82E54834;
	sub_82E523C0(ctx, base);
	// 82E54834: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5483C: 396BCCB0  addi r11, r11, -0x3350
	ctx.r[11].s64 = ctx.r[11].s64 + -13136;
	// 82E54840: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E54848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5484C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54858 size=80
    let mut pc: u32 = 0x82E54858;
    'dispatch: loop {
        match pc {
            0x82E54858 => {
    //   block [0x82E54858..0x82E548A8)
	// 82E54858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5485C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54868: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5486C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E54874: 388BCA08  addi r4, r11, -0x35f8
	ctx.r[4].s64 = ctx.r[11].s64 + -13816;
	// 82E54878: 4BF9F191  bl 0x82df3a08
	ctx.lr = 0x82E5487C;
	sub_82DF3A08(ctx, base);
	// 82E5487C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E54880: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E54884: 4BFDF57D  bl 0x82e33e00
	ctx.lr = 0x82E54888;
	sub_82E33E00(ctx, base);
	// 82E54888: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5488C: 4BF9EB9D  bl 0x82df3428
	ctx.lr = 0x82E54890;
	sub_82DF3428(ctx, base);
	// 82E54890: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E54898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E548A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E548A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E548A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E548A8 size=76
    let mut pc: u32 = 0x82E548A8;
    'dispatch: loop {
        match pc {
            0x82E548A8 => {
    //   block [0x82E548A8..0x82E548F4)
	// 82E548A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E548AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E548B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E548B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E548B8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82E548BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E548C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E548C4: 388BE7E4  addi r4, r11, -0x181c
	ctx.r[4].s64 = ctx.r[11].s64 + -6172;
	// 82E548C8: 4BF9F141  bl 0x82df3a08
	ctx.lr = 0x82E548CC;
	sub_82DF3A08(ctx, base);
	// 82E548CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E548D0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E548D4: 4BFDF52D  bl 0x82e33e00
	ctx.lr = 0x82E548D8;
	sub_82E33E00(ctx, base);
	// 82E548D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E548DC: 4BF9EB4D  bl 0x82df3428
	ctx.lr = 0x82E548E0;
	sub_82DF3428(ctx, base);
	// 82E548E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E548E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E548E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E548EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E548F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E548F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E548F8 size=100
    let mut pc: u32 = 0x82E548F8;
    'dispatch: loop {
        match pc {
            0x82E548F8 => {
    //   block [0x82E548F8..0x82E5495C)
	// 82E548F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E548FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54904: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5490C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E54914: 41820010  beq 0x82e54924
	if ctx.cr[0].eq {
	pc = 0x82E54924; continue 'dispatch;
	}
	// 82E54918: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5491C: 388BCA10  addi r4, r11, -0x35f0
	ctx.r[4].s64 = ctx.r[11].s64 + -13808;
	// 82E54920: 4800000C  b 0x82e5492c
	pc = 0x82E5492C; continue 'dispatch;
	// 82E54924: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54928: 388BCA18  addi r4, r11, -0x35e8
	ctx.r[4].s64 = ctx.r[11].s64 + -13800;
	// 82E5492C: 4BF9F0DD  bl 0x82df3a08
	ctx.lr = 0x82E54930;
	sub_82DF3A08(ctx, base);
	// 82E54930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E54934: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E54938: 4BFDF4C9  bl 0x82e33e00
	ctx.lr = 0x82E5493C;
	sub_82E33E00(ctx, base);
	// 82E5493C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E54940: 4BF9EAE9  bl 0x82df3428
	ctx.lr = 0x82E54944;
	sub_82DF3428(ctx, base);
	// 82E54944: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54948: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5494C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54960 size=328
    let mut pc: u32 = 0x82E54960;
    'dispatch: loop {
        match pc {
            0x82E54960 => {
    //   block [0x82E54960..0x82E54AA8)
	// 82E54960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54964: 48353809  bl 0x831a816c
	ctx.lr = 0x82E54968;
	sub_831A8130(ctx, base);
	// 82E54968: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E5496C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54974: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E5497C: 419A011C  beq cr6, 0x82e54a98
	if ctx.cr[6].eq {
	pc = 0x82E54A98; continue 'dispatch;
	}
	// 82E54980: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54984: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54988: 41820110  beq 0x82e54a98
	if ctx.cr[0].eq {
	pc = 0x82E54A98; continue 'dispatch;
	}
	// 82E5498C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E54990: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 82E54994: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 82E54998: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E5499C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E549A0: C02B95F4  lfs f1, -0x6a0c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27148 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E549A4: 4BFFCE1D  bl 0x82e517c0
	ctx.lr = 0x82E549A8;
	sub_82E517C0(ctx, base);
	// 82E549A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E549AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E549B0: D03F0080  stfs f1, 0x80(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E549B4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 82E549B8: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 82E549BC: 38CA81F0  addi r6, r10, -0x7e10
	ctx.r[6].s64 = ctx.r[10].s64 + -32272;
	// 82E549C0: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E549C4: 3889B93C  addi r4, r9, -0x46c4
	ctx.r[4].s64 = ctx.r[9].s64 + -18116;
	// 82E549C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E549CC: EC3F0828  fsubs f1, f31, f1
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E549D0: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E549D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E549D8: 4E800421  bctrl
	ctx.lr = 0x82E549DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E549DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E549E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E549E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E549E8: 4BFFCDD9  bl 0x82e517c0
	ctx.lr = 0x82E549EC;
	sub_82E517C0(ctx, base);
	// 82E549EC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E549F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E549F4: EC3F0828  fsubs f1, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82E549F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E549FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54A00: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E54A04: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82E54A08: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E54A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54A10: 4E800421  bctrl
	ctx.lr = 0x82E54A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54A14: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 82E54A18: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E54A1C: 48028A35  bl 0x82e7d450
	ctx.lr = 0x82E54A20;
	sub_82E7D450(ctx, base);
	// 82E54A20: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E54A24: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E54A28: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54AA8 size=148
    let mut pc: u32 = 0x82E54AA8;
    'dispatch: loop {
        match pc {
            0x82E54AA8 => {
    //   block [0x82E54AA8..0x82E54B3C)
	// 82E54AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54AAC: 483536C1  bl 0x831a816c
	ctx.lr = 0x82E54AB0;
	sub_831A8130(ctx, base);
	// 82E54AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54AB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54AB8: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E54ABC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E54AC0: 4BF9EDB9  bl 0x82df3878
	ctx.lr = 0x82E54AC4;
	sub_82DF3878(ctx, base);
	// 82E54AC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54AC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54ACC: 419A0068  beq cr6, 0x82e54b34
	if ctx.cr[6].eq {
	pc = 0x82E54B34; continue 'dispatch;
	}
	// 82E54AD0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54AD4: 3BABCD18  addi r29, r11, -0x32e8
	ctx.r[29].s64 = ctx.r[11].s64 + -13032;
	// 82E54AD8: 4BFFCAE9  bl 0x82e515c0
	ctx.lr = 0x82E54ADC;
	sub_82E515C0(ctx, base);
	// 82E54ADC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54AE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54AE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54AE8: 38C00099  li r6, 0x99
	ctx.r[6].s64 = 153;
	// 82E54AEC: 4BFA4D4D  bl 0x82df9838
	ctx.lr = 0x82E54AF0;
	sub_82DF9838(ctx, base);
	// 82E54AF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E54AF8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E54AFC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54B00: 4BF9E6B1  bl 0x82df31b0
	ctx.lr = 0x82E54B04;
	sub_82DF31B0(ctx, base);
	// 82E54B04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54B08: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54B0C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E54B10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54B18: 4E800421  bctrl
	ctx.lr = 0x82E54B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54B1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E54B20: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54B24: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E54B28: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E54B2C: 997F0037  stb r11, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E54B30: 4B47AAD1  bl 0x822cf600
	ctx.lr = 0x82E54B34;
	sub_822CF600(ctx, base);
	// 82E54B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54B38: 48353684  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54B40 size=164
    let mut pc: u32 = 0x82E54B40;
    'dispatch: loop {
        match pc {
            0x82E54B40 => {
    //   block [0x82E54B40..0x82E54BE4)
	// 82E54B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E54B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54B50: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E54B54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54B5C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54B60: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54B68: 419A005C  beq cr6, 0x82e54bc4
	if ctx.cr[6].eq {
	pc = 0x82E54BC4; continue 'dispatch;
	}
	// 82E54B6C: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54B70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54B74: 41820050  beq 0x82e54bc4
	if ctx.cr[0].eq {
	pc = 0x82E54BC4; continue 'dispatch;
	}
	// 82E54B78: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54B7C: 3BCBCD18  addi r30, r11, -0x32e8
	ctx.r[30].s64 = ctx.r[11].s64 + -13032;
	// 82E54B80: 4BFFCA41  bl 0x82e515c0
	ctx.lr = 0x82E54B84;
	sub_82E515C0(ctx, base);
	// 82E54B84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54B88: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54B8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E54B90: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 82E54B94: 4BFA4CA5  bl 0x82df9838
	ctx.lr = 0x82E54B98;
	sub_82DF9838(ctx, base);
	// 82E54B98: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54B9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E54BA0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54BA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54BA8: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E54BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54BB0: 4E800421  bctrl
	ctx.lr = 0x82E54BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54BB4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54BB8: 4B47AA49  bl 0x822cf600
	ctx.lr = 0x82E54BBC;
	sub_822CF600(ctx, base);
	// 82E54BBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54BC0: 48000008  b 0x82e54bc8
	pc = 0x82E54BC8; continue 'dispatch;
	// 82E54BC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E54BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54BD4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E54BD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E54BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54BE8 size=148
    let mut pc: u32 = 0x82E54BE8;
    'dispatch: loop {
        match pc {
            0x82E54BE8 => {
    //   block [0x82E54BE8..0x82E54C7C)
	// 82E54BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54BEC: 48353581  bl 0x831a816c
	ctx.lr = 0x82E54BF0;
	sub_831A8130(ctx, base);
	// 82E54BF0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E54BF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54BFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54C00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54C04: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54C08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54C0C: 419A0060  beq cr6, 0x82e54c6c
	if ctx.cr[6].eq {
	pc = 0x82E54C6C; continue 'dispatch;
	}
	// 82E54C10: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54C14: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54C18: 41820054  beq 0x82e54c6c
	if ctx.cr[0].eq {
	pc = 0x82E54C6C; continue 'dispatch;
	}
	// 82E54C1C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54C20: 3BABCD18  addi r29, r11, -0x32e8
	ctx.r[29].s64 = ctx.r[11].s64 + -13032;
	// 82E54C24: 4BFFC99D  bl 0x82e515c0
	ctx.lr = 0x82E54C28;
	sub_82E515C0(ctx, base);
	// 82E54C28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54C2C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54C30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54C34: 38C00111  li r6, 0x111
	ctx.r[6].s64 = 273;
	// 82E54C38: 4BFA4C01  bl 0x82df9838
	ctx.lr = 0x82E54C3C;
	sub_82DF9838(ctx, base);
	// 82E54C3C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54C40: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E54C44: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54C48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E54C4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54C50: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E54C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54C58: 4E800421  bctrl
	ctx.lr = 0x82E54C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54C5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54C60: 4B47A9A1  bl 0x822cf600
	ctx.lr = 0x82E54C64;
	sub_822CF600(ctx, base);
	// 82E54C64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E54C68: 48000008  b 0x82e54c70
	pc = 0x82E54C70; continue 'dispatch;
	// 82E54C6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E54C70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E54C74: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E54C78: 48353544  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54C80 size=120
    let mut pc: u32 = 0x82E54C80;
    'dispatch: loop {
        match pc {
            0x82E54C80 => {
    //   block [0x82E54C80..0x82E54CF8)
	// 82E54C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54C84: 483534E9  bl 0x831a816c
	ctx.lr = 0x82E54C88;
	sub_831A8130(ctx, base);
	// 82E54C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54C90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54C94: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54C98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54C9C: 419A0054  beq cr6, 0x82e54cf0
	if ctx.cr[6].eq {
	pc = 0x82E54CF0; continue 'dispatch;
	}
	// 82E54CA0: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54CA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54CA8: 41820048  beq 0x82e54cf0
	if ctx.cr[0].eq {
	pc = 0x82E54CF0; continue 'dispatch;
	}
	// 82E54CAC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54CB0: 3BABCD18  addi r29, r11, -0x32e8
	ctx.r[29].s64 = ctx.r[11].s64 + -13032;
	// 82E54CB4: 4BFFC90D  bl 0x82e515c0
	ctx.lr = 0x82E54CB8;
	sub_82E515C0(ctx, base);
	// 82E54CB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54CBC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54CC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54CC4: 38C00122  li r6, 0x122
	ctx.r[6].s64 = 290;
	// 82E54CC8: 4BFA4B71  bl 0x82df9838
	ctx.lr = 0x82E54CCC;
	sub_82DF9838(ctx, base);
	// 82E54CCC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54CD0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E54CD4: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E54CD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54CDC: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E54CE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54CE4: 4E800421  bctrl
	ctx.lr = 0x82E54CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54CE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54CEC: 4B47A915  bl 0x822cf600
	ctx.lr = 0x82E54CF0;
	sub_822CF600(ctx, base);
	// 82E54CF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E54CF4: 483534C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E54CF8 size=144
    let mut pc: u32 = 0x82E54CF8;
    'dispatch: loop {
        match pc {
            0x82E54CF8 => {
    //   block [0x82E54CF8..0x82E54D88)
	// 82E54CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54CFC: 48353471  bl 0x831a816c
	ctx.lr = 0x82E54D00;
	sub_831A8130(ctx, base);
	// 82E54D00: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E54D04: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54D08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54D0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54D10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E54D14: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54D1C: 419A0060  beq cr6, 0x82e54d7c
	if ctx.cr[6].eq {
	pc = 0x82E54D7C; continue 'dispatch;
	}
	// 82E54D20: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E54D24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54D28: 41820054  beq 0x82e54d7c
	if ctx.cr[0].eq {
	pc = 0x82E54D7C; continue 'dispatch;
	}
	// 82E54D2C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54D30: 3BABCD18  addi r29, r11, -0x32e8
	ctx.r[29].s64 = ctx.r[11].s64 + -13032;
	// 82E54D34: 4BFFC88D  bl 0x82e515c0
	ctx.lr = 0x82E54D38;
	sub_82E515C0(ctx, base);
	// 82E54D38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E54D3C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54D40: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E54D44: 38C00131  li r6, 0x131
	ctx.r[6].s64 = 305;
	// 82E54D48: 4BFA4AF1  bl 0x82df9838
	ctx.lr = 0x82E54D4C;
	sub_82DF9838(ctx, base);
	// 82E54D4C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E54D50: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54D54: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E54D58: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54D5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E54D60: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E54D64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54D68: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E54D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54D70: 4E800421  bctrl
	ctx.lr = 0x82E54D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54D74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54D78: 4B47A889  bl 0x822cf600
	ctx.lr = 0x82E54D7C;
	sub_822CF600(ctx, base);
	// 82E54D7C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E54D80: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E54D84: 48353438  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54D88 size=68
    let mut pc: u32 = 0x82E54D88;
    'dispatch: loop {
        match pc {
            0x82E54D88 => {
    //   block [0x82E54D88..0x82E54DCC)
	// 82E54D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E54D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E54D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54D98: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E54D9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E54DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E54DA4: 4BFFD61D  bl 0x82e523c0
	ctx.lr = 0x82E54DA8;
	sub_82E523C0(ctx, base);
	// 82E54DA8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E54DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54DB0: 396BCD60  addi r11, r11, -0x32a0
	ctx.r[11].s64 = ctx.r[11].s64 + -12960;
	// 82E54DB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E54DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E54DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E54DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E54DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E54DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E54DD0 size=16
    let mut pc: u32 = 0x82E54DD0;
    'dispatch: loop {
        match pc {
            0x82E54DD0 => {
    //   block [0x82E54DD0..0x82E54DE0)
	// 82E54DD0: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 82E54DD4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E54DE0 size=16
    let mut pc: u32 = 0x82E54DE0;
    'dispatch: loop {
        match pc {
            0x82E54DE0 => {
    //   block [0x82E54DE0..0x82E54DF0)
	// 82E54DE0: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 82E54DE4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E54DF0 size=16
    let mut pc: u32 = 0x82E54DF0;
    'dispatch: loop {
        match pc {
            0x82E54DF0 => {
    //   block [0x82E54DF0..0x82E54E00)
	// 82E54DF0: 39600070  li r11, 0x70
	ctx.r[11].s64 = 112;
	// 82E54DF4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E54E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E54E00 size=756
    let mut pc: u32 = 0x82E54E00;
    'dispatch: loop {
        match pc {
            0x82E54E00 => {
    //   block [0x82E54E00..0x82E550F4)
	// 82E54E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E54E04: 4835334D  bl 0x831a8150
	ctx.lr = 0x82E54E08;
	sub_831A8130(ctx, base);
	// 82E54E08: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82E54E0C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E54E10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E54E14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E54E18: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54E1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E54E20: 419A02C8  beq cr6, 0x82e550e8
	if ctx.cr[6].eq {
	pc = 0x82E550E8; continue 'dispatch;
	}
	// 82E54E24: 4BFFFB3D  bl 0x82e54960
	ctx.lr = 0x82E54E28;
	sub_82E54960(ctx, base);
	// 82E54E28: 897E0037  lbz r11, 0x37(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(55 as u32) ) } as u64;
	// 82E54E2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54E30: 4182001C  beq 0x82e54e4c
	if ctx.cr[0].eq {
	pc = 0x82E54E4C; continue 'dispatch;
	}
	// 82E54E34: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54E38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E54E3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54E40: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E54E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54E48: 4E800421  bctrl
	ctx.lr = 0x82E54E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54E4C: 82FE0018  lwz r23, 0x18(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E54E50: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E54E54: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E54E58: 419A0290  beq cr6, 0x82e550e8
	if ctx.cr[6].eq {
	pc = 0x82E550E8; continue 'dispatch;
	}
	// 82E54E5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82E54E60: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E54E64: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E54E68: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82E54E6C: 3CE08208  lis r7, -0x7df8
	ctx.r[7].s64 = -2113404928;
	// 82E54E70: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82E54E74: 3BAB0570  addi r29, r11, 0x570
	ctx.r[29].s64 = ctx.r[11].s64 + 1392;
	// 82E54E78: 3B8ACA20  addi r28, r10, -0x35e0
	ctx.r[28].s64 = ctx.r[10].s64 + -13792;
	// 82E54E7C: 3B69CA18  addi r27, r9, -0x35e8
	ctx.r[27].s64 = ctx.r[9].s64 + -13800;
	// 82E54E80: 3B48CA10  addi r26, r8, -0x35f0
	ctx.r[26].s64 = ctx.r[8].s64 + -13808;
	// 82E54E84: 3B27E7E4  addi r25, r7, -0x181c
	ctx.r[25].s64 = ctx.r[7].s64 + -6172;
	// 82E54E88: 3B06CA08  addi r24, r6, -0x35f8
	ctx.r[24].s64 = ctx.r[6].s64 + -13816;
	// 82E54E8C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E54E90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E54E94: 4BF9EB75  bl 0x82df3a08
	ctx.lr = 0x82E54E98;
	sub_82DF3A08(ctx, base);
	// 82E54E98: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E54E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54EA0: 4BF9E469  bl 0x82df3308
	ctx.lr = 0x82E54EA4;
	sub_82DF3308(ctx, base);
	// 82E54EA4: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54EA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E54EAC: 4BF9E57D  bl 0x82df3428
	ctx.lr = 0x82E54EB0;
	sub_82DF3428(ctx, base);
	// 82E54EB0: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54EB4: 4082011C  bne 0x82e54fd0
	if !ctx.cr[0].eq {
	pc = 0x82E54FD0; continue 'dispatch;
	}
	// 82E54EB8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E54EBC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54EC0: 4BF9EB49  bl 0x82df3a08
	ctx.lr = 0x82E54EC4;
	sub_82DF3A08(ctx, base);
	// 82E54EC4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E54EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54ECC: 4BF9E43D  bl 0x82df3308
	ctx.lr = 0x82E54ED0;
	sub_82DF3308(ctx, base);
	// 82E54ED0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54ED4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E54ED8: 4BF9E551  bl 0x82df3428
	ctx.lr = 0x82E54EDC;
	sub_82DF3428(ctx, base);
	// 82E54EDC: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54EE0: 408201A0  bne 0x82e55080
	if !ctx.cr[0].eq {
	pc = 0x82E55080; continue 'dispatch;
	}
	// 82E54EE4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E54EE8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E54EEC: 4BF9EB1D  bl 0x82df3a08
	ctx.lr = 0x82E54EF0;
	sub_82DF3A08(ctx, base);
	// 82E54EF0: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E54EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54EF8: 4BF9E411  bl 0x82df3308
	ctx.lr = 0x82E54EFC;
	sub_82DF3308(ctx, base);
	// 82E54EFC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54F00: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E54F04: 4BF9E525  bl 0x82df3428
	ctx.lr = 0x82E54F08;
	sub_82DF3428(ctx, base);
	// 82E54F08: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54F0C: 408201A8  bne 0x82e550b4
	if !ctx.cr[0].eq {
	pc = 0x82E550B4; continue 'dispatch;
	}
	// 82E54F10: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E54F14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E54F18: 4BF9EAF1  bl 0x82df3a08
	ctx.lr = 0x82E54F1C;
	sub_82DF3A08(ctx, base);
	// 82E54F1C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E54F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54F24: 4BF9E3E5  bl 0x82df3308
	ctx.lr = 0x82E54F28;
	sub_82DF3308(ctx, base);
	// 82E54F28: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54F2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E54F30: 4BF9E4F9  bl 0x82df3428
	ctx.lr = 0x82E54F34;
	sub_82DF3428(ctx, base);
	// 82E54F34: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54F38: 40820184  bne 0x82e550bc
	if !ctx.cr[0].eq {
	pc = 0x82E550BC; continue 'dispatch;
	}
	// 82E54F3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E54F40: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E54F44: 4BF9EAC5  bl 0x82df3a08
	ctx.lr = 0x82E54F48;
	sub_82DF3A08(ctx, base);
	// 82E54F48: 38810064  addi r4, r1, 0x64
	ctx.r[4].s64 = ctx.r[1].s64 + 100;
	// 82E54F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54F50: 4BF9E3B9  bl 0x82df3308
	ctx.lr = 0x82E54F54;
	sub_82DF3308(ctx, base);
	// 82E54F54: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54F58: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E54F5C: 4BF9E4CD  bl 0x82df3428
	ctx.lr = 0x82E54F60;
	sub_82DF3428(ctx, base);
	// 82E54F60: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54F64: 41820010  beq 0x82e54f74
	if ctx.cr[0].eq {
	pc = 0x82E54F74; continue 'dispatch;
	}
	// 82E54F68: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54F6C: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E54F70: 48000038  b 0x82e54fa8
	pc = 0x82E54FA8; continue 'dispatch;
	// 82E54F74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E54F78: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E54F7C: 4BF9EA8D  bl 0x82df3a08
	ctx.lr = 0x82E54F80;
	sub_82DF3A08(ctx, base);
	// 82E54F80: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E54F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E54F88: 4BF9E381  bl 0x82df3308
	ctx.lr = 0x82E54F8C;
	sub_82DF3308(ctx, base);
	// 82E54F8C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82E54F90: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E54F94: 4BF9E495  bl 0x82df3428
	ctx.lr = 0x82E54F98;
	sub_82DF3428(ctx, base);
	// 82E54F98: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54F9C: 41820024  beq 0x82e54fc0
	if ctx.cr[0].eq {
	pc = 0x82E54FC0; continue 'dispatch;
	}
	// 82E54FA0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54FA4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E54FA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E54FAC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E54FB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54FB4: 4E800421  bctrl
	ctx.lr = 0x82E54FB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54FB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54FBC: 4082011C  bne 0x82e550d8
	if !ctx.cr[0].eq {
	pc = 0x82E550D8; continue 'dispatch;
	}
	// 82E54FC0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E54FC4: 7F1FB840  cmplw cr6, r31, r23
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82E54FC8: 409AFEC4  bne cr6, 0x82e54e8c
	if !ctx.cr[6].eq {
	pc = 0x82E54E8C; continue 'dispatch;
	}
	// 82E54FCC: 4800011C  b 0x82e550e8
	pc = 0x82E550E8; continue 'dispatch;
	// 82E54FD0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E54FD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E54FD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E54FDC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E54FE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E54FE4: 4E800421  bctrl
	ctx.lr = 0x82E54FE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E54FE8: 897E0036  lbz r11, 0x36(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E54FEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E54FF0: 408200E8  bne 0x82e550d8
	if !ctx.cr[0].eq {
	pc = 0x82E550D8; continue 'dispatch;
	}
	// 82E54FF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E54FF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E54FFC: 419A00DC  beq cr6, 0x82e550d8
	if ctx.cr[6].eq {
	pc = 0x82E550D8; continue 'dispatch;
	}
	// 82E55000: 3BFE0020  addi r31, r30, 0x20
	ctx.r[31].s64 = ctx.r[30].s64 + 32;
	// 82E55004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55008: 4BF9E1A9  bl 0x82df31b0
	ctx.lr = 0x82E5500C;
	sub_82DF31B0(ctx, base);
	// 82E5500C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E55010: 396B9BC9  addi r11, r11, -0x6437
	ctx.r[11].s64 = ctx.r[11].s64 + -25655;
	// 82E55014: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55018: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5501C: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E55020: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E55024: 41820014  beq 0x82e55038
	if ctx.cr[0].eq {
	pc = 0x82E55038; continue 'dispatch;
	}
	// 82E55028: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E5502C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E55030: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E55034: 419AFFE0  beq cr6, 0x82e55014
	if ctx.cr[6].eq {
	pc = 0x82E55014; continue 'dispatch;
	}
	// 82E55038: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E5503C: 418200AC  beq 0x82e550e8
	if ctx.cr[0].eq {
	pc = 0x82E550E8; continue 'dispatch;
	}
	// 82E55040: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55048: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E5504C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55050: 4BF9E161  bl 0x82df31b0
	ctx.lr = 0x82E55054;
	sub_82DF31B0(ctx, base);
	// 82E55054: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55058: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5505C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55060: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E55064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55068: 4E800421  bctrl
	ctx.lr = 0x82E5506C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5506C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55070: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E55074: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E55078: 997E0037  stb r11, 0x37(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E5507C: 4800006C  b 0x82e550e8
	pc = 0x82E550E8; continue 'dispatch;
	// 82E55080: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55084: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55088: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E5508C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E55090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55094: 4E800421  bctrl
	ctx.lr = 0x82E55098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55098: 897E0036  lbz r11, 0x36(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E5509C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E550A0: 40820038  bne 0x82e550d8
	if !ctx.cr[0].eq {
	pc = 0x82E550D8; continue 'dispatch;
	}
	// 82E550A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E550A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E550AC: 409A003C  bne cr6, 0x82e550e8
	if !ctx.cr[6].eq {
	pc = 0x82E550E8; continue 'dispatch;
	}
	// 82E550B0: 48000028  b 0x82e550d8
	pc = 0x82E550D8; continue 'dispatch;
	// 82E550B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E550B8: 48000008  b 0x82e550c0
	pc = 0x82E550C0; continue 'dispatch;
	// 82E550BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E550C0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E550C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E550C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E550CC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E550D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E550D4: 4E800421  bctrl
	ctx.lr = 0x82E550D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E550D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E550DC: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82E550E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E550E4: 4BFFB5B5  bl 0x82e50698
	ctx.lr = 0x82E550E8;
	sub_82E50698(ctx, base);
	// 82E550E8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E550EC: CBE1FFA0  lfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 82E550F0: 483530B0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E550F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E550F8 size=132
    let mut pc: u32 = 0x82E550F8;
    'dispatch: loop {
        match pc {
            0x82E550F8 => {
    //   block [0x82E550F8..0x82E5517C)
	// 82E550F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E550FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55108: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E5510C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E55110: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E55114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55118: 4BFFD2A9  bl 0x82e523c0
	ctx.lr = 0x82E5511C;
	sub_82E523C0(ctx, base);
	// 82E5511C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E55120: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E55124: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82E55128: 394ACD60  addi r10, r10, -0x32a0
	ctx.r[10].s64 = ctx.r[10].s64 + -12960;
	// 82E5512C: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82E55130: 39000060  li r8, 0x60
	ctx.r[8].s64 = 96;
	// 82E55134: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E55138: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82E5513C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82E55140: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E55144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55180 size=96
    let mut pc: u32 = 0x82E55180;
    'dispatch: loop {
        match pc {
            0x82E55180 => {
    //   block [0x82E55180..0x82E551E0)
	// 82E55180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5518C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55194: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55198: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E5519C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E551A0: 4E800421  bctrl
	ctx.lr = 0x82E551A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E551A4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E551A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E551AC: 388BCA08  addi r4, r11, -0x35f8
	ctx.r[4].s64 = ctx.r[11].s64 + -13816;
	// 82E551B0: 4BF9E859  bl 0x82df3a08
	ctx.lr = 0x82E551B4;
	sub_82DF3A08(ctx, base);
	// 82E551B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E551B8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E551BC: 4BFDEC45  bl 0x82e33e00
	ctx.lr = 0x82E551C0;
	sub_82E33E00(ctx, base);
	// 82E551C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E551C4: 4BF9E265  bl 0x82df3428
	ctx.lr = 0x82E551C8;
	sub_82DF3428(ctx, base);
	// 82E551C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E551CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E551D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E551D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E551D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E551DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E551E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E551E0 size=24
    let mut pc: u32 = 0x82E551E0;
    'dispatch: loop {
        match pc {
            0x82E551E0 => {
    //   block [0x82E551E0..0x82E551F8)
	// 82E551E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E551E4: 39400060  li r10, 0x60
	ctx.r[10].s64 = 96;
	// 82E551E8: 396B4BC0  addi r11, r11, 0x4bc0
	ctx.r[11].s64 = ctx.r[11].s64 + 19392;
	// 82E551EC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E551F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E551F8 size=24
    let mut pc: u32 = 0x82E551F8;
    'dispatch: loop {
        match pc {
            0x82E551F8 => {
    //   block [0x82E551F8..0x82E55210)
	// 82E551F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E551FC: 39400070  li r10, 0x70
	ctx.r[10].s64 = 112;
	// 82E55200: 396B2D60  addi r11, r11, 0x2d60
	ctx.r[11].s64 = ctx.r[11].s64 + 11616;
	// 82E55204: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55210 size=60
    let mut pc: u32 = 0x82E55210;
    'dispatch: loop {
        match pc {
            0x82E55210 => {
    //   block [0x82E55210..0x82E5524C)
	// 82E55210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5521C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55224: 4BFFFB65  bl 0x82e54d88
	ctx.lr = 0x82E55228;
	sub_82E54D88(ctx, base);
	// 82E55228: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5522C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55230: 396BCDC8  addi r11, r11, -0x3238
	ctx.r[11].s64 = ctx.r[11].s64 + -12856;
	// 82E55234: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5523C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55250 size=60
    let mut pc: u32 = 0x82E55250;
    'dispatch: loop {
        match pc {
            0x82E55250 => {
    //   block [0x82E55250..0x82E5528C)
	// 82E55250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5525C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55264: 4BFFFE95  bl 0x82e550f8
	ctx.lr = 0x82E55268;
	sub_82E550F8(ctx, base);
	// 82E55268: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55270: 396BCDC8  addi r11, r11, -0x3238
	ctx.r[11].s64 = ctx.r[11].s64 + -12856;
	// 82E55274: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5527C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55290 size=148
    let mut pc: u32 = 0x82E55290;
    'dispatch: loop {
        match pc {
            0x82E55290 => {
    //   block [0x82E55290..0x82E55324)
	// 82E55290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55294: 48352ED9  bl 0x831a816c
	ctx.lr = 0x82E55298;
	sub_831A8130(ctx, base);
	// 82E55298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5529C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E552A0: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E552A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E552A8: 4BF9E5D1  bl 0x82df3878
	ctx.lr = 0x82E552AC;
	sub_82DF3878(ctx, base);
	// 82E552AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E552B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E552B4: 419A0068  beq cr6, 0x82e5531c
	if ctx.cr[6].eq {
	pc = 0x82E5531C; continue 'dispatch;
	}
	// 82E552B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E552BC: 3BABCE30  addi r29, r11, -0x31d0
	ctx.r[29].s64 = ctx.r[11].s64 + -12752;
	// 82E552C0: 4BFFC301  bl 0x82e515c0
	ctx.lr = 0x82E552C4;
	sub_82E515C0(ctx, base);
	// 82E552C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E552C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E552CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E552D0: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82E552D4: 4BFA4565  bl 0x82df9838
	ctx.lr = 0x82E552D8;
	sub_82DF9838(ctx, base);
	// 82E552D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E552DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E552E0: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E552E4: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E552E8: 4BF9DEC9  bl 0x82df31b0
	ctx.lr = 0x82E552EC;
	sub_82DF31B0(ctx, base);
	// 82E552EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E552F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E552F4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E552F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E552FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55300: 4E800421  bctrl
	ctx.lr = 0x82E55304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55304: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55308: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5530C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E55310: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E55314: 997F0037  stb r11, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E55318: 4B47A2E9  bl 0x822cf600
	ctx.lr = 0x82E5531C;
	sub_822CF600(ctx, base);
	// 82E5531C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E55320: 48352E9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55328 size=164
    let mut pc: u32 = 0x82E55328;
    'dispatch: loop {
        match pc {
            0x82E55328 => {
    //   block [0x82E55328..0x82E553CC)
	// 82E55328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5532C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55338: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E5533C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55344: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E55348: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5534C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55350: 419A005C  beq cr6, 0x82e553ac
	if ctx.cr[6].eq {
	pc = 0x82E553AC; continue 'dispatch;
	}
	// 82E55354: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E55358: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E5535C: 41820050  beq 0x82e553ac
	if ctx.cr[0].eq {
	pc = 0x82E553AC; continue 'dispatch;
	}
	// 82E55360: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55364: 3BCBCE30  addi r30, r11, -0x31d0
	ctx.r[30].s64 = ctx.r[11].s64 + -12752;
	// 82E55368: 4BFFC259  bl 0x82e515c0
	ctx.lr = 0x82E5536C;
	sub_82E515C0(ctx, base);
	// 82E5536C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55370: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55374: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E55378: 38C000A2  li r6, 0xa2
	ctx.r[6].s64 = 162;
	// 82E5537C: 4BFA44BD  bl 0x82df9838
	ctx.lr = 0x82E55380;
	sub_82DF9838(ctx, base);
	// 82E55380: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55384: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E55388: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E5538C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55390: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E55394: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55398: 4E800421  bctrl
	ctx.lr = 0x82E5539C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5539C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E553A0: 4B47A261  bl 0x822cf600
	ctx.lr = 0x82E553A4;
	sub_822CF600(ctx, base);
	// 82E553A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E553A8: 48000008  b 0x82e553b0
	pc = 0x82E553B0; continue 'dispatch;
	// 82E553AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E553B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E553B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E553B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E553BC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E553C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E553C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E553C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E553D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E553D0 size=148
    let mut pc: u32 = 0x82E553D0;
    'dispatch: loop {
        match pc {
            0x82E553D0 => {
    //   block [0x82E553D0..0x82E55464)
	// 82E553D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E553D4: 48352D99  bl 0x831a816c
	ctx.lr = 0x82E553D8;
	sub_831A8130(ctx, base);
	// 82E553D8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E553DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E553E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E553E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E553E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E553EC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E553F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E553F4: 419A0060  beq cr6, 0x82e55454
	if ctx.cr[6].eq {
	pc = 0x82E55454; continue 'dispatch;
	}
	// 82E553F8: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E553FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55400: 41820054  beq 0x82e55454
	if ctx.cr[0].eq {
	pc = 0x82E55454; continue 'dispatch;
	}
	// 82E55404: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55408: 3BABCE30  addi r29, r11, -0x31d0
	ctx.r[29].s64 = ctx.r[11].s64 + -12752;
	// 82E5540C: 4BFFC1B5  bl 0x82e515c0
	ctx.lr = 0x82E55410;
	sub_82E515C0(ctx, base);
	// 82E55410: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55414: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55418: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5541C: 38C000B4  li r6, 0xb4
	ctx.r[6].s64 = 180;
	// 82E55420: 4BFA4419  bl 0x82df9838
	ctx.lr = 0x82E55424;
	sub_82DF9838(ctx, base);
	// 82E55424: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55428: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E5542C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E55430: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E55434: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55438: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E5543C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55440: 4E800421  bctrl
	ctx.lr = 0x82E55444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55444: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55448: 4B47A1B9  bl 0x822cf600
	ctx.lr = 0x82E5544C;
	sub_822CF600(ctx, base);
	// 82E5544C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E55450: 48000008  b 0x82e55458
	pc = 0x82E55458; continue 'dispatch;
	// 82E55454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E55458: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E5545C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E55460: 48352D5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55468 size=120
    let mut pc: u32 = 0x82E55468;
    'dispatch: loop {
        match pc {
            0x82E55468 => {
    //   block [0x82E55468..0x82E554E0)
	// 82E55468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5546C: 48352D01  bl 0x831a816c
	ctx.lr = 0x82E55470;
	sub_831A8130(ctx, base);
	// 82E55470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55478: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5547C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55484: 419A0054  beq cr6, 0x82e554d8
	if ctx.cr[6].eq {
	pc = 0x82E554D8; continue 'dispatch;
	}
	// 82E55488: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E5548C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55490: 41820048  beq 0x82e554d8
	if ctx.cr[0].eq {
	pc = 0x82E554D8; continue 'dispatch;
	}
	// 82E55494: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55498: 3BABCE30  addi r29, r11, -0x31d0
	ctx.r[29].s64 = ctx.r[11].s64 + -12752;
	// 82E5549C: 4BFFC125  bl 0x82e515c0
	ctx.lr = 0x82E554A0;
	sub_82E515C0(ctx, base);
	// 82E554A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E554A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E554A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E554AC: 38C000C5  li r6, 0xc5
	ctx.r[6].s64 = 197;
	// 82E554B0: 4BFA4389  bl 0x82df9838
	ctx.lr = 0x82E554B4;
	sub_82DF9838(ctx, base);
	// 82E554B4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E554B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E554BC: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82E554C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E554C4: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E554C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E554CC: 4E800421  bctrl
	ctx.lr = 0x82E554D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E554D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E554D4: 4B47A12D  bl 0x822cf600
	ctx.lr = 0x82E554D8;
	sub_822CF600(ctx, base);
	// 82E554D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E554DC: 48352CE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E554E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E554E0 size=144
    let mut pc: u32 = 0x82E554E0;
    'dispatch: loop {
        match pc {
            0x82E554E0 => {
    //   block [0x82E554E0..0x82E55570)
	// 82E554E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E554E4: 48352C89  bl 0x831a816c
	ctx.lr = 0x82E554E8;
	sub_831A8130(ctx, base);
	// 82E554E8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E554EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E554F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E554F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E554F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E554FC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55504: 419A0060  beq cr6, 0x82e55564
	if ctx.cr[6].eq {
	pc = 0x82E55564; continue 'dispatch;
	}
	// 82E55508: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E5550C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55510: 41820054  beq 0x82e55564
	if ctx.cr[0].eq {
	pc = 0x82E55564; continue 'dispatch;
	}
	// 82E55514: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55518: 3BABCE30  addi r29, r11, -0x31d0
	ctx.r[29].s64 = ctx.r[11].s64 + -12752;
	// 82E5551C: 4BFFC0A5  bl 0x82e515c0
	ctx.lr = 0x82E55520;
	sub_82E515C0(ctx, base);
	// 82E55520: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55524: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55528: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E5552C: 38C000D4  li r6, 0xd4
	ctx.r[6].s64 = 212;
	// 82E55530: 4BFA4309  bl 0x82df9838
	ctx.lr = 0x82E55534;
	sub_82DF9838(ctx, base);
	// 82E55534: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E55538: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E5553C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E55540: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E55544: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E55548: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E5554C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55550: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E55554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55558: 4E800421  bctrl
	ctx.lr = 0x82E5555C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5555C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55560: 4B47A0A1  bl 0x822cf600
	ctx.lr = 0x82E55564;
	sub_822CF600(ctx, base);
	// 82E55564: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E55568: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E5556C: 48352C50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55570 size=88
    let mut pc: u32 = 0x82E55570;
    'dispatch: loop {
        match pc {
            0x82E55570 => {
    //   block [0x82E55570..0x82E555C8)
	// 82E55570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5557C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55584: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5558C: 38ABC8C8  addi r5, r11, -0x3738
	ctx.r[5].s64 = ctx.r[11].s64 + -14136;
	// 82E55590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E55594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55598: 4BFFCE29  bl 0x82e523c0
	ctx.lr = 0x82E5559C;
	sub_82E523C0(ctx, base);
	// 82E5559C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E555A0: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82E555A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E555A8: 396BCE78  addi r11, r11, -0x3188
	ctx.r[11].s64 = ctx.r[11].s64 + -12680;
	// 82E555AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E555B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E555B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E555B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E555BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E555C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E555C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E555C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E555C8 size=84
    let mut pc: u32 = 0x82E555C8;
    'dispatch: loop {
        match pc {
            0x82E555C8 => {
    //   block [0x82E555C8..0x82E5561C)
	// 82E555C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E555CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E555D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E555D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E555D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E555DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E555E0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E555E4: 38ABC8C8  addi r5, r11, -0x3738
	ctx.r[5].s64 = ctx.r[11].s64 + -14136;
	// 82E555E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E555EC: 4BFFCDD5  bl 0x82e523c0
	ctx.lr = 0x82E555F0;
	sub_82E523C0(ctx, base);
	// 82E555F0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E555F4: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82E555F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E555FC: 396BCE78  addi r11, r11, -0x3188
	ctx.r[11].s64 = ctx.r[11].s64 + -12680;
	// 82E55600: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55604: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5560C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55610: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55620 size=144
    let mut pc: u32 = 0x82E55620;
    'dispatch: loop {
        match pc {
            0x82E55620 => {
    //   block [0x82E55620..0x82E556B0)
	// 82E55620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55624: 48352B49  bl 0x831a816c
	ctx.lr = 0x82E55628;
	sub_831A8130(ctx, base);
	// 82E55628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5562C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55630: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E55634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55638: 4BF9E241  bl 0x82df3878
	ctx.lr = 0x82E5563C;
	sub_82DF3878(ctx, base);
	// 82E5563C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55640: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55644: 419A003C  beq cr6, 0x82e55680
	if ctx.cr[6].eq {
	pc = 0x82E55680; continue 'dispatch;
	}
	// 82E55648: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 82E5564C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55650: 41820030  beq 0x82e55680
	if ctx.cr[0].eq {
	pc = 0x82E55680; continue 'dispatch;
	}
	// 82E55654: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E5565C: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 82E55660: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55664: 4BF9DB4D  bl 0x82df31b0
	ctx.lr = 0x82E55668;
	sub_82DF31B0(ctx, base);
	// 82E55668: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E5566C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55670: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E55674: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E55678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E5567C: 4E800421  bctrl
	ctx.lr = 0x82E55680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55680: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55688: 388BCA08  addi r4, r11, -0x35f8
	ctx.r[4].s64 = ctx.r[11].s64 + -13816;
	// 82E5568C: 4BF9E37D  bl 0x82df3a08
	ctx.lr = 0x82E55690;
	sub_82DF3A08(ctx, base);
	// 82E55690: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55694: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E55698: 4BFDE769  bl 0x82e33e00
	ctx.lr = 0x82E5569C;
	sub_82E33E00(ctx, base);
	// 82E5569C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E556A0: 4BF9DD89  bl 0x82df3428
	ctx.lr = 0x82E556A4;
	sub_82DF3428(ctx, base);
	// 82E556A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E556A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E556AC: 48352B10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E556B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E556B0 size=240
    let mut pc: u32 = 0x82E556B0;
    'dispatch: loop {
        match pc {
            0x82E556B0 => {
    //   block [0x82E556B0..0x82E557A0)
	// 82E556B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E556B4: 48352AB5  bl 0x831a8168
	ctx.lr = 0x82E556B8;
	sub_831A8130(ctx, base);
	// 82E556B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E556BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E556C0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E556C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E556C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E556CC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E556D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E556D4: 4BF9E335  bl 0x82df3a08
	ctx.lr = 0x82E556D8;
	sub_82DF3A08(ctx, base);
	// 82E556D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E556DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E556E0: 4BF9E329  bl 0x82df3a08
	ctx.lr = 0x82E556E4;
	sub_82DF3A08(ctx, base);
	// 82E556E4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E556E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E556EC: 388BCEDC  addi r4, r11, -0x3124
	ctx.r[4].s64 = ctx.r[11].s64 + -12580;
	// 82E556F0: 4BF9DE89  bl 0x82df3578
	ctx.lr = 0x82E556F4;
	sub_82DF3578(ctx, base);
	// 82E556F4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E556F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E556FC: 48310F9D  bl 0x83166698
	ctx.lr = 0x82E55700;
	sub_83166698(ctx, base);
	// 82E55700: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E55704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55708: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5570C: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 82E55710: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55714: 4BF9DA9D  bl 0x82df31b0
	ctx.lr = 0x82E55718;
	sub_82DF31B0(ctx, base);
	// 82E55718: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E5571C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55720: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E55724: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E55728: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E5572C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E55730: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82E55734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55738: 4E800421  bctrl
	ctx.lr = 0x82E5573C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5573C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E55740: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 82E55744: 419A0048  beq cr6, 0x82e5578c
	if ctx.cr[6].eq {
	pc = 0x82E5578C; continue 'dispatch;
	}
	// 82E55748: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82E5574C: 419A0040  beq cr6, 0x82e5578c
	if ctx.cr[6].eq {
	pc = 0x82E5578C; continue 'dispatch;
	}
	// 82E55750: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E55754: 419A0038  beq cr6, 0x82e5578c
	if ctx.cr[6].eq {
	pc = 0x82E5578C; continue 'dispatch;
	}
	// 82E55758: 48000014  b 0x82e5576c
	pc = 0x82E5576C; continue 'dispatch;
	// 82E5575C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82E55760: 419A002C  beq cr6, 0x82e5578c
	if ctx.cr[6].eq {
	pc = 0x82E5578C; continue 'dispatch;
	}
	// 82E55764: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E55768: 4BF9CE81  bl 0x82df25e8
	ctx.lr = 0x82E5576C;
	sub_82DF25E8(ctx, base);
	// 82E5576C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55770: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E55774: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55778: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E5577C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55780: 4E800421  bctrl
	ctx.lr = 0x82E55784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55784: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E55788: 409AFFD4  bne cr6, 0x82e5575c
	if !ctx.cr[6].eq {
	pc = 0x82E5575C; continue 'dispatch;
	}
	// 82E5578C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55790: 4BF9DC99  bl 0x82df3428
	ctx.lr = 0x82E55794;
	sub_82DF3428(ctx, base);
	// 82E55794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E5579C: 48352A1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E557A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E557A0 size=72
    let mut pc: u32 = 0x82E557A0;
    'dispatch: loop {
        match pc {
            0x82E557A0 => {
    //   block [0x82E557A0..0x82E557E8)
	// 82E557A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E557A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E557A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E557AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E557B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E557B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E557B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E557BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E557C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E557C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E557C8: 4E800421  bctrl
	ctx.lr = 0x82E557CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E557CC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E557D0: 4BF9DC59  bl 0x82df3428
	ctx.lr = 0x82E557D4;
	sub_82DF3428(ctx, base);
	// 82E557D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E557D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E557DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E557E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E557E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E557E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E557E8 size=196
    let mut pc: u32 = 0x82E557E8;
    'dispatch: loop {
        match pc {
            0x82E557E8 => {
    //   block [0x82E557E8..0x82E558AC)
	// 82E557E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E557EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E557F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E557F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E557F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E557FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55800: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E55804: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E55808: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E5580C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55810: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55818: 4E800421  bctrl
	ctx.lr = 0x82E5581C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5581C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55820: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 82E55824: 409A0028  bne cr6, 0x82e5584c
	if !ctx.cr[6].eq {
	pc = 0x82E5584C; continue 'dispatch;
	}
	// 82E55828: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E5582C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E55834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5583C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55848: 4E800020  blr
	return;
	// 82E5584C: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82E55850: 409A000C  bne cr6, 0x82e5585c
	if !ctx.cr[6].eq {
	pc = 0x82E5585C; continue 'dispatch;
	}
	// 82E55854: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82E55858: 4BFFFFD4  b 0x82e5582c
	pc = 0x82E5582C; continue 'dispatch;
	// 82E5585C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E55860: 409A0024  bne cr6, 0x82e55884
	if !ctx.cr[6].eq {
	pc = 0x82E55884; continue 'dispatch;
	}
	// 82E55864: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E55868: 4BFFFFC4  b 0x82e5582c
	pc = 0x82E5582C; continue 'dispatch;
	// 82E5586C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82E55870: 419AFFC0  beq cr6, 0x82e55830
	if ctx.cr[6].eq {
	pc = 0x82E55830; continue 'dispatch;
	}
	// 82E55874: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E55878: 4BF9CD71  bl 0x82df25e8
	ctx.lr = 0x82E5587C;
	sub_82DF25E8(ctx, base);
	// 82E5587C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55880: 482EC921  bl 0x831421a0
	ctx.lr = 0x82E55884;
	sub_831421A0(ctx, base);
	// 82E55884: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55888: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5588C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55890: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E55894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55898: 4E800421  bctrl
	ctx.lr = 0x82E5589C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5589C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E558A0: 409AFFCC  bne cr6, 0x82e5586c
	if !ctx.cr[6].eq {
	pc = 0x82E5586C; continue 'dispatch;
	}
	// 82E558A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E558A8: 4BFFFF8C  b 0x82e55834
	pc = 0x82E55834; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E558B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E558B0 size=132
    let mut pc: u32 = 0x82E558B0;
    'dispatch: loop {
        match pc {
            0x82E558B0 => {
    //   block [0x82E558B0..0x82E55934)
	// 82E558B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E558B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E558B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E558BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E558C0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E558C4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E558C8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82E558CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E558D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E558D4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E558D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E558DC: 4E800421  bctrl
	ctx.lr = 0x82E558E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E558E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E558E4: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 82E558E8: 409A0014  bne cr6, 0x82e558fc
	if !ctx.cr[6].eq {
	pc = 0x82E558FC; continue 'dispatch;
	}
	// 82E558EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E558F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E558F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E558F8: 48000028  b 0x82e55920
	pc = 0x82E55920; continue 'dispatch;
	// 82E558FC: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82E55900: 409A000C  bne cr6, 0x82e5590c
	if !ctx.cr[6].eq {
	pc = 0x82E5590C; continue 'dispatch;
	}
	// 82E55904: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82E55908: 4BFFFFE8  b 0x82e558f0
	pc = 0x82E558F0; continue 'dispatch;
	// 82E5590C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E55910: 409A000C  bne cr6, 0x82e5591c
	if !ctx.cr[6].eq {
	pc = 0x82E5591C; continue 'dispatch;
	}
	// 82E55914: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E55918: 4BFFFFD8  b 0x82e558f0
	pc = 0x82E558F0; continue 'dispatch;
	// 82E5591C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E55920: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E5592C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55938 size=52
    let mut pc: u32 = 0x82E55938;
    'dispatch: loop {
        match pc {
            0x82E55938 => {
    //   block [0x82E55938..0x82E5596C)
	// 82E55938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55944: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55948: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E5594C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55950: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E55954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55958: 4E800421  bctrl
	ctx.lr = 0x82E5595C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E5595C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E55960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55970 size=52
    let mut pc: u32 = 0x82E55970;
    'dispatch: loop {
        match pc {
            0x82E55970 => {
    //   block [0x82E55970..0x82E559A4)
	// 82E55970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5597C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55980: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E55984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55988: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E5598C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55990: 4E800421  bctrl
	ctx.lr = 0x82E55994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E55998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E559A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E559A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E559A8 size=72
    let mut pc: u32 = 0x82E559A8;
    'dispatch: loop {
        match pc {
            0x82E559A8 => {
    //   block [0x82E559A8..0x82E559F0)
	// 82E559A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E559AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E559B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E559B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E559B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E559BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E559C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E559C4: 388BB00C  addi r4, r11, -0x4ff4
	ctx.r[4].s64 = ctx.r[11].s64 + -20468;
	// 82E559C8: 48352731  bl 0x831a80f8
	ctx.lr = 0x82E559CC;
	sub_831A80F8(ctx, base);
	// 82E559CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E559D0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E559D4: 40820008  bne 0x82e559dc
	if !ctx.cr[0].eq {
	pc = 0x82E559DC; continue 'dispatch;
	}
	// 82E559D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E559DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E559E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E559E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E559E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E559EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E559F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E559F0 size=100
    let mut pc: u32 = 0x82E559F0;
    'dispatch: loop {
        match pc {
            0x82E559F0 => {
    //   block [0x82E559F0..0x82E55A54)
	// 82E559F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E559F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E559F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E559FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55A00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55A04: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55A08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55A0C: 396BCEF8  addi r11, r11, -0x3108
	ctx.r[11].s64 = ctx.r[11].s64 + -12552;
	// 82E55A10: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55A14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55A18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55A1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55A20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55A24: 4E800421  bctrl
	ctx.lr = 0x82E55A28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55A28: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E55A2C: 4BF9D9FD  bl 0x82df3428
	ctx.lr = 0x82E55A30;
	sub_82DF3428(ctx, base);
	// 82E55A30: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55A34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E55A38: 419A0008  beq cr6, 0x82e55a40
	if ctx.cr[6].eq {
	pc = 0x82E55A40; continue 'dispatch;
	}
	// 82E55A3C: 4B46AE55  bl 0x822c0890
	ctx.lr = 0x82E55A40;
	sub_822C0890(ctx, base);
	// 82E55A40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55A58 size=196
    let mut pc: u32 = 0x82E55A58;
    'dispatch: loop {
        match pc {
            0x82E55A58 => {
    //   block [0x82E55A58..0x82E55B1C)
	// 82E55A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55A68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55A6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55A74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E55A78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E55A7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55A80: 4B46AEB9  bl 0x822c0938
	ctx.lr = 0x82E55A84;
	sub_822C0938(ctx, base);
	// 82E55A84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55A88: 41820028  beq 0x82e55ab0
	if ctx.cr[0].eq {
	pc = 0x82E55AB0; continue 'dispatch;
	}
	// 82E55A8C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55A90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E55A94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E55A98: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82E55A9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E55AA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E55AA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E55AA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E55AAC: 48000008  b 0x82e55ab4
	pc = 0x82E55AB4; continue 'dispatch;
	// 82E55AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55AB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55ABC: 409A0044  bne cr6, 0x82e55b00
	if !ctx.cr[6].eq {
	pc = 0x82E55B00; continue 'dispatch;
	}
	// 82E55AC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E55AC4: 419A001C  beq cr6, 0x82e55ae0
	if ctx.cr[6].eq {
	pc = 0x82E55AE0; continue 'dispatch;
	}
	// 82E55AC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55ACC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E55AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55AD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55ADC: 4E800421  bctrl
	ctx.lr = 0x82E55AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55AE0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E55AE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E55AE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55AEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E55AF0: 816BB008  lwz r11, -0x4ff8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20472 as u32) ) } as u64;
	// 82E55AF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E55AF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E55AFC: 4B46A505  bl 0x822c0000
	ctx.lr = 0x82E55B00;
	sub_822C0000(ctx, base);
	// 82E55B00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55B04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55B20 size=180
    let mut pc: u32 = 0x82E55B20;
    'dispatch: loop {
        match pc {
            0x82E55B20 => {
    //   block [0x82E55B20..0x82E55BD4)
	// 82E55B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55B34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55B3C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E55B40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E55B44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55B48: 4B46ADF1  bl 0x822c0938
	ctx.lr = 0x82E55B4C;
	sub_822C0938(ctx, base);
	// 82E55B4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55B50: 41820028  beq 0x82e55b78
	if ctx.cr[0].eq {
	pc = 0x82E55B78; continue 'dispatch;
	}
	// 82E55B54: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55B58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E55B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E55B60: 392BCEE8  addi r9, r11, -0x3118
	ctx.r[9].s64 = ctx.r[11].s64 + -12568;
	// 82E55B64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E55B68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E55B6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E55B70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E55B74: 48000008  b 0x82e55b7c
	pc = 0x82E55B7C; continue 'dispatch;
	// 82E55B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55B7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55B80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55B84: 409A0034  bne cr6, 0x82e55bb8
	if !ctx.cr[6].eq {
	pc = 0x82E55BB8; continue 'dispatch;
	}
	// 82E55B88: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E55B8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E55B90: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E55B94: 4BF9C5F5  bl 0x82df2188
	ctx.lr = 0x82E55B98;
	sub_82DF2188(ctx, base);
	// 82E55B98: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E55B9C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E55BA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55BA4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E55BA8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E55BAC: 816BB008  lwz r11, -0x4ff8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20472 as u32) ) } as u64;
	// 82E55BB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E55BB4: 4B46A44D  bl 0x822c0000
	ctx.lr = 0x82E55BB8;
	sub_822C0000(ctx, base);
	// 82E55BB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55BBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55BC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E55BD8 size=16
    let mut pc: u32 = 0x82E55BD8;
    'dispatch: loop {
        match pc {
            0x82E55BD8 => {
    //   block [0x82E55BD8..0x82E55BE8)
	// 82E55BD8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E55BDC: 8083000C  lwz r4, 0xc(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E55BE0: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E55BE4: 4BF9C5A4  b 0x82df2188
	sub_82DF2188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55BE8 size=84
    let mut pc: u32 = 0x82E55BE8;
    'dispatch: loop {
        match pc {
            0x82E55BE8 => {
    //   block [0x82E55BE8..0x82E55C3C)
	// 82E55BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55BF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55BF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55BFC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E55C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E55C04: 394ACEF8  addi r10, r10, -0x3108
	ctx.r[10].s64 = ctx.r[10].s64 + -12552;
	// 82E55C08: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E55C0C: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E55C10: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E55C14: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E55C18: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E55C1C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E55C20: 4BF9DDE9  bl 0x82df3a08
	ctx.lr = 0x82E55C24;
	sub_82DF3A08(ctx, base);
	// 82E55C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E55C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55C40 size=76
    let mut pc: u32 = 0x82E55C40;
    'dispatch: loop {
        match pc {
            0x82E55C40 => {
    //   block [0x82E55C40..0x82E55C8C)
	// 82E55C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55C58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E55C5C: 4BFFFD95  bl 0x82e559f0
	ctx.lr = 0x82E55C60;
	sub_82E559F0(ctx, base);
	// 82E55C60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55C64: 4182000C  beq 0x82e55c70
	if ctx.cr[0].eq {
	pc = 0x82E55C70; continue 'dispatch;
	}
	// 82E55C68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55C6C: 4BF9C76D  bl 0x82df23d8
	ctx.lr = 0x82E55C70;
	sub_82DF23D8(ctx, base);
	// 82E55C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55C90 size=140
    let mut pc: u32 = 0x82E55C90;
    'dispatch: loop {
        match pc {
            0x82E55C90 => {
    //   block [0x82E55C90..0x82E55D1C)
	// 82E55C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55C94: 483524D9  bl 0x831a816c
	ctx.lr = 0x82E55C98;
	sub_831A8130(ctx, base);
	// 82E55C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55C9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E55CA0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E55CA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55CA8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E55CAC: 482EF18D  bl 0x83144e38
	ctx.lr = 0x82E55CB0;
	sub_83144E38(ctx, base);
	// 82E55CB0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E55CB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55CB8: 388BCEFC  addi r4, r11, -0x3104
	ctx.r[4].s64 = ctx.r[11].s64 + -12548;
	// 82E55CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E55CC0: 38A0003E  li r5, 0x3e
	ctx.r[5].s64 = 62;
	// 82E55CC4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E55CC8: 4BF9C721  bl 0x82df23e8
	ctx.lr = 0x82E55CCC;
	sub_82DF23E8(ctx, base);
	// 82E55CCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55CD0: 41820018  beq 0x82e55ce8
	if ctx.cr[0].eq {
	pc = 0x82E55CE8; continue 'dispatch;
	}
	// 82E55CD4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E55CD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E55CDC: 4BFFFF0D  bl 0x82e55be8
	ctx.lr = 0x82E55CE0;
	sub_82E55BE8(ctx, base);
	// 82E55CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55CE4: 48000008  b 0x82e55cec
	pc = 0x82E55CEC; continue 'dispatch;
	// 82E55CE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E55CEC: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E55CF0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82E55CF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E55CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55CFC: 4BFFFD5D  bl 0x82e55a58
	ctx.lr = 0x82E55D00;
	sub_82E55A58(ctx, base);
	// 82E55D00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E55D04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E55D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E55D0C: 4B46A2F5  bl 0x822c0000
	ctx.lr = 0x82E55D10;
	sub_822C0000(ctx, base);
	// 82E55D10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E55D14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E55D18: 483524A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55D20 size=260
    let mut pc: u32 = 0x82E55D20;
    'dispatch: loop {
        match pc {
            0x82E55D20 => {
    //   block [0x82E55D20..0x82E55E24)
	// 82E55D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55D24: 4835243D  bl 0x831a8160
	ctx.lr = 0x82E55D28;
	sub_831A8130(ctx, base);
	// 82E55D28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55D2C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E55D30: 8BE10050  lbz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E55D34: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E55D38: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E55D3C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E55D40: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E55D44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E55D48: 388ACEFC  addi r4, r10, -0x3104
	ctx.r[4].s64 = ctx.r[10].s64 + -12548;
	// 82E55D4C: 38A000D5  li r5, 0xd5
	ctx.r[5].s64 = 213;
	// 82E55D50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E55D54: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E55D58: 4BF9C371  bl 0x82df20c8
	ctx.lr = 0x82E55D5C;
	sub_82DF20C8(ctx, base);
	// 82E55D5C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E55D60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E55D64: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82E55D68: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E55D6C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E55D70: 4BFFFDB1  bl 0x82e55b20
	ctx.lr = 0x82E55D74;
	sub_82E55B20(ctx, base);
	// 82E55D74: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E55D78: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E55D7C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E55D80: 4B46A281  bl 0x822c0000
	ctx.lr = 0x82E55D84;
	sub_822C0000(ctx, base);
	// 82E55D84: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E55D88: 3BFD0008  addi r31, r29, 8
	ctx.r[31].s64 = ctx.r[29].s64 + 8;
	// 82E55D8C: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E55D90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E55D94: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E55D98: 4B46E6C9  bl 0x822c4460
	ctx.lr = 0x82E55D9C;
	sub_822C4460(ctx, base);
	// 82E55D9C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E55DA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E55DA4: 419A0008  beq cr6, 0x82e55dac
	if ctx.cr[6].eq {
	pc = 0x82E55DAC; continue 'dispatch;
	}
	// 82E55DA8: 4B46AAE9  bl 0x822c0890
	ctx.lr = 0x82E55DAC;
	sub_822C0890(ctx, base);
	// 82E55DAC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E55DB0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55DB4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E55DB8: 48352759  bl 0x831a8510
	ctx.lr = 0x82E55DBC;
	sub_831A8510(ctx, base);
	// 82E55DBC: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55DC0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55DC4: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82E55DC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E55DCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55DD0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E55DD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E55DD8: 4E800421  bctrl
	ctx.lr = 0x82E55DDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E55DDC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E55DE0: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 82E55DE4: 409A0014  bne cr6, 0x82e55df8
	if !ctx.cr[6].eq {
	pc = 0x82E55DF8; continue 'dispatch;
	}
	// 82E55DE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E55DEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E55DF0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55DF4: 48000028  b 0x82e55e1c
	pc = 0x82E55E1C; continue 'dispatch;
	// 82E55DF8: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 82E55DFC: 409A000C  bne cr6, 0x82e55e08
	if !ctx.cr[6].eq {
	pc = 0x82E55E08; continue 'dispatch;
	}
	// 82E55E00: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82E55E04: 4BFFFFE8  b 0x82e55dec
	pc = 0x82E55DEC; continue 'dispatch;
	// 82E55E08: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E55E0C: 409A000C  bne cr6, 0x82e55e18
	if !ctx.cr[6].eq {
	pc = 0x82E55E18; continue 'dispatch;
	}
	// 82E55E10: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E55E14: 4BFFFFD8  b 0x82e55dec
	pc = 0x82E55DEC; continue 'dispatch;
	// 82E55E18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E55E1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E55E20: 48352390  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55E28 size=156
    let mut pc: u32 = 0x82E55E28;
    'dispatch: loop {
        match pc {
            0x82E55E28 => {
    //   block [0x82E55E28..0x82E55EC4)
	// 82E55E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55E3C: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E55E40: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E55E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55E48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E55E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55E50: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 82E55E54: 41820028  beq 0x82e55e7c
	if ctx.cr[0].eq {
	pc = 0x82E55E7C; continue 'dispatch;
	}
	// 82E55E58: 4BF9DBB1  bl 0x82df3a08
	ctx.lr = 0x82E55E5C;
	sub_82DF3A08(ctx, base);
	// 82E55E5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E55E60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55E64: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E55E68: 480003D9  bl 0x82e56240
	ctx.lr = 0x82E55E6C;
	sub_82E56240(ctx, base);
	// 82E55E6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55E70: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55E74: 4BFAB36D  bl 0x82e011e0
	ctx.lr = 0x82E55E78;
	sub_82E011E0(ctx, base);
	// 82E55E78: 48000024  b 0x82e55e9c
	pc = 0x82E55E9C; continue 'dispatch;
	// 82E55E7C: 4BF9DB8D  bl 0x82df3a08
	ctx.lr = 0x82E55E80;
	sub_82DF3A08(ctx, base);
	// 82E55E80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E55E84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E55E88: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E55E8C: 480003B5  bl 0x82e56240
	ctx.lr = 0x82E55E90;
	sub_82E56240(ctx, base);
	// 82E55E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E55E94: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55E98: 4BFAB6E1  bl 0x82e01578
	ctx.lr = 0x82E55E9C;
	sub_82E01578(ctx, base);
	// 82E55E9C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E55EA0: 4BF9D589  bl 0x82df3428
	ctx.lr = 0x82E55EA4;
	sub_82DF3428(ctx, base);
	// 82E55EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55EA8: 4BF9D581  bl 0x82df3428
	ctx.lr = 0x82E55EAC;
	sub_82DF3428(ctx, base);
	// 82E55EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55EB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55EC8 size=112
    let mut pc: u32 = 0x82E55EC8;
    'dispatch: loop {
        match pc {
            0x82E55EC8 => {
    //   block [0x82E55EC8..0x82E55F38)
	// 82E55EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E55ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E55ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E55ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55EDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E55EE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E55EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E55EE8: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E55EEC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E55EF0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E55EF4: 4BF9C4F5  bl 0x82df23e8
	ctx.lr = 0x82E55EF8;
	sub_82DF23E8(ctx, base);
	// 82E55EF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E55EFC: 41820010  beq 0x82e55f0c
	if ctx.cr[0].eq {
	pc = 0x82E55F0C; continue 'dispatch;
	}
	// 82E55F00: 483A6069  bl 0x831fbf68
	ctx.lr = 0x82E55F04;
	sub_831FBF68(ctx, base);
	// 82E55F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55F08: 48000008  b 0x82e55f10
	pc = 0x82E55F10; continue 'dispatch;
	// 82E55F0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E55F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55F14: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E55F18: 4BFA8961  bl 0x82dfe878
	ctx.lr = 0x82E55F1C;
	sub_82DFE878(ctx, base);
	// 82E55F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55F20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E55F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E55F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E55F2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E55F30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E55F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55F38 size=164
    let mut pc: u32 = 0x82E55F38;
    'dispatch: loop {
        match pc {
            0x82E55F38 => {
    //   block [0x82E55F38..0x82E55FDC)
	// 82E55F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55F3C: 48352231  bl 0x831a816c
	ctx.lr = 0x82E55F40;
	sub_831A8130(ctx, base);
	// 82E55F40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55F44: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E55F48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55F4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E55F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E55F54: 396B5EC8  addi r11, r11, 0x5ec8
	ctx.r[11].s64 = ctx.r[11].s64 + 24264;
	// 82E55F58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E55F5C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E55F60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E55F64: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E55F68: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E55F6C: 4BFA898D  bl 0x82dfe8f8
	ctx.lr = 0x82E55F70;
	sub_82DFE8F8(ctx, base);
	// 82E55F70: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E55F74: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E55F78: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E55F7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E55F80: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E55F84: 4BFAB915  bl 0x82e01898
	ctx.lr = 0x82E55F88;
	sub_82E01898(ctx, base);
	// 82E55F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E55F8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E55F90: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E55F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E55F98: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E55F9C: 419A0024  beq cr6, 0x82e55fc0
	if ctx.cr[6].eq {
	pc = 0x82E55FC0; continue 'dispatch;
	}
	// 82E55FA0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E55FA4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E55FA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E55FAC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E55FB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E55FB4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E55FB8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E55FBC: 4082FFE8  bne 0x82e55fa4
	if !ctx.cr[0].eq {
	pc = 0x82E55FA4; continue 'dispatch;
	}
	// 82E55FC0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E55FC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E55FC8: 419A0008  beq cr6, 0x82e55fd0
	if ctx.cr[6].eq {
	pc = 0x82E55FD0; continue 'dispatch;
	}
	// 82E55FCC: 4B46A8C5  bl 0x822c0890
	ctx.lr = 0x82E55FD0;
	sub_822C0890(ctx, base);
	// 82E55FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E55FD4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E55FD8: 483521E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E55FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E55FE0 size=108
    let mut pc: u32 = 0x82E55FE0;
    'dispatch: loop {
        match pc {
            0x82E55FE0 => {
    //   block [0x82E55FE0..0x82E5604C)
	// 82E55FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E55FE4: 48352185  bl 0x831a8168
	ctx.lr = 0x82E55FE8;
	sub_831A8130(ctx, base);
	// 82E55FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E55FEC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E55FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E55FF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E55FF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E55FFC: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 82E56000: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E56004: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E56008: 4BF9DA01  bl 0x82df3a08
	ctx.lr = 0x82E5600C;
	sub_82DF3A08(ctx, base);
	// 82E5600C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E56010: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E56014: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E56018: 48000229  bl 0x82e56240
	ctx.lr = 0x82E5601C;
	sub_82E56240(ctx, base);
	// 82E5601C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E56020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56024: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56028: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E5602C: 4BFFFF0D  bl 0x82e55f38
	ctx.lr = 0x82E56030;
	sub_82E55F38(ctx, base);
	// 82E56030: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E56034: 4BF9D3F5  bl 0x82df3428
	ctx.lr = 0x82E56038;
	sub_82DF3428(ctx, base);
	// 82E56038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E5603C: 4BF9D3ED  bl 0x82df3428
	ctx.lr = 0x82E56040;
	sub_82DF3428(ctx, base);
	// 82E56040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E56048: 48352170  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56050 size=112
    let mut pc: u32 = 0x82E56050;
    'dispatch: loop {
        match pc {
            0x82E56050 => {
    //   block [0x82E56050..0x82E560C0)
	// 82E56050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5605C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56064: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E56068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5606C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E56074: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 82E56078: 4BF9D991  bl 0x82df3a08
	ctx.lr = 0x82E5607C;
	sub_82DF3A08(ctx, base);
	// 82E5607C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E56080: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E56084: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E56088: 480001B9  bl 0x82e56240
	ctx.lr = 0x82E5608C;
	sub_82E56240(ctx, base);
	// 82E5608C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E56090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56094: 4BFAC3A5  bl 0x82e02438
	ctx.lr = 0x82E56098;
	sub_82E02438(ctx, base);
	// 82E56098: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E5609C: 4BF9D38D  bl 0x82df3428
	ctx.lr = 0x82E560A0;
	sub_82DF3428(ctx, base);
	// 82E560A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E560A4: 4BF9D385  bl 0x82df3428
	ctx.lr = 0x82E560A8;
	sub_82DF3428(ctx, base);
	// 82E560A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E560AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E560B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E560B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E560B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E560BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E560C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E560C0 size=228
    let mut pc: u32 = 0x82E560C0;
    'dispatch: loop {
        match pc {
            0x82E560C0 => {
    //   block [0x82E560C0..0x82E561A4)
	// 82E560C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E560C4: 483520A5  bl 0x831a8168
	ctx.lr = 0x82E560C8;
	sub_831A8130(ctx, base);
	// 82E560C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E560CC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E560D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E560D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E560D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E560DC: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 82E560E0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E560E4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E560E8: 4BF9D921  bl 0x82df3a08
	ctx.lr = 0x82E560EC;
	sub_82DF3A08(ctx, base);
	// 82E560EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E560F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E560F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E560F8: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E560FC: 48000145  bl 0x82e56240
	ctx.lr = 0x82E56100;
	sub_82E56240(ctx, base);
	// 82E56100: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E56104: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E56108: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E5610C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E56110: 4BFFFE29  bl 0x82e55f38
	ctx.lr = 0x82E56114;
	sub_82E55F38(ctx, base);
	// 82E56114: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E56118: 4BF9D311  bl 0x82df3428
	ctx.lr = 0x82E5611C;
	sub_82DF3428(ctx, base);
	// 82E5611C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E56120: 4BF9D309  bl 0x82df3428
	ctx.lr = 0x82E56124;
	sub_82DF3428(ctx, base);
	// 82E56124: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E56128: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E5612C: 419A0060  beq cr6, 0x82e5618c
	if ctx.cr[6].eq {
	pc = 0x82E5618C; continue 'dispatch;
	}
	// 82E56130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E56134: 4BFA8735  bl 0x82dfe868
	ctx.lr = 0x82E56138;
	sub_82DFE868(ctx, base);
	// 82E56138: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E5613C: 40820050  bne 0x82e5618c
	if !ctx.cr[0].eq {
	pc = 0x82E5618C; continue 'dispatch;
	}
	// 82E56140: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E56144: 419A0048  beq cr6, 0x82e5618c
	if ctx.cr[6].eq {
	pc = 0x82E5618C; continue 'dispatch;
	}
	// 82E56148: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E5614C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E56150: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E56154: 808B271C  lwz r4, 0x271c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10012 as u32) ) } as u64;
	// 82E56158: 4BF9BAA1  bl 0x82df1bf8
	ctx.lr = 0x82E5615C;
	sub_82DF1BF8(ctx, base);
	// 82E5615C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E56160: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E56164: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E56168: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5616C: 4BFFB15D  bl 0x82e512c8
	ctx.lr = 0x82E56170;
	sub_82E512C8(ctx, base);
	// 82E56170: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E56174: 4BF9BB1D  bl 0x82df1c90
	ctx.lr = 0x82E56178;
	sub_82DF1C90(ctx, base);
	// 82E56178: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5617C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E56180: 483A5DE1  bl 0x831fbf60
	ctx.lr = 0x82E56184;
	sub_831FBF60(ctx, base);
	// 82E56184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E56188: 4BFA86D1  bl 0x82dfe858
	ctx.lr = 0x82E5618C;
	sub_82DFE858(ctx, base);
	// 82E5618C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E56190: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E56194: 419A0008  beq cr6, 0x82e5619c
	if ctx.cr[6].eq {
	pc = 0x82E5619C; continue 'dispatch;
	}
	// 82E56198: 4B46A6F9  bl 0x822c0890
	ctx.lr = 0x82E5619C;
	sub_822C0890(ctx, base);
	// 82E5619C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E561A0: 48352018  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E561A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E561A8 size=148
    let mut pc: u32 = 0x82E561A8;
    'dispatch: loop {
        match pc {
            0x82E561A8 => {
    //   block [0x82E561A8..0x82E5623C)
	// 82E561A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E561AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E561B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E561B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E561B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E561BC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E561C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E561C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E561C8: 388BCF34  addi r4, r11, -0x30cc
	ctx.r[4].s64 = ctx.r[11].s64 + -12492;
	// 82E561CC: 4BF9D83D  bl 0x82df3a08
	ctx.lr = 0x82E561D0;
	sub_82DF3A08(ctx, base);
	// 82E561D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E561D4: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E561D8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82E561DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E561E0: 388B6050  addi r4, r11, 0x6050
	ctx.r[4].s64 = ctx.r[11].s64 + 24656;
	// 82E561E4: 4B47BF3D  bl 0x822d2120
	ctx.lr = 0x82E561E8;
	sub_822D2120(ctx, base);
	// 82E561E8: 3D6082E5  lis r11, -0x7d1b
	ctx.r[11].s64 = -2098921472;
	// 82E561EC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E561F0: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 82E561F4: 396B60C0  addi r11, r11, 0x60c0
	ctx.r[11].s64 = ctx.r[11].s64 + 24768;
	// 82E561F8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E561FC: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E56200: 4BD66869  bl 0x82bbca68
	ctx.lr = 0x82E56204;
	sub_82BBCA68(ctx, base);
	// 82E56204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E56208: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E5620C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82E56210: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E56214: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E56218: 4BFACFF1  bl 0x82e03208
	ctx.lr = 0x82E5621C;
	sub_82E03208(ctx, base);
	// 82E5621C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E56220: 4BF9D209  bl 0x82df3428
	ctx.lr = 0x82E56224;
	sub_82DF3428(ctx, base);
	// 82E56224: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E56228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5622C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56230: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56240 size=132
    let mut pc: u32 = 0x82E56240;
    'dispatch: loop {
        match pc {
            0x82E56240 => {
    //   block [0x82E56240..0x82E562C4)
	// 82E56240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56244: 48351F25  bl 0x831a8168
	ctx.lr = 0x82E56248;
	sub_831A8130(ctx, base);
	// 82E56248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E5624C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56250: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E56254: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E56258: 4BF9CE99  bl 0x82df30f0
	ctx.lr = 0x82E5625C;
	sub_82DF30F0(ctx, base);
	// 82E5625C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E56260: 4BF9D1D1  bl 0x82df3430
	ctx.lr = 0x82E56264;
	sub_82DF3430(ctx, base);
	// 82E56264: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E56268: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E5626C: 4BF9D1C5  bl 0x82df3430
	ctx.lr = 0x82E56270;
	sub_82DF3430(ctx, base);
	// 82E56270: 7D7C1A14  add r11, r28, r3
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[3].u64;
	// 82E56274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56278: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82E5627C: 48271865  bl 0x830c7ae0
	ctx.lr = 0x82E56280;
	sub_830C7AE0(ctx, base);
	// 82E56280: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E56284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56288: 388BCF38  addi r4, r11, -0x30c8
	ctx.r[4].s64 = ctx.r[11].s64 + -12488;
	// 82E5628C: 4BF9D2ED  bl 0x82df3578
	ctx.lr = 0x82E56290;
	sub_82DF3578(ctx, base);
	// 82E56290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56294: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E56298: 4BF9D441  bl 0x82df36d8
	ctx.lr = 0x82E5629C;
	sub_82DF36D8(ctx, base);
	// 82E5629C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E562A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E562A4: 388B9FD8  addi r4, r11, -0x6028
	ctx.r[4].s64 = ctx.r[11].s64 + -24616;
	// 82E562A8: 4BF9D2D1  bl 0x82df3578
	ctx.lr = 0x82E562AC;
	sub_82DF3578(ctx, base);
	// 82E562AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E562B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E562B4: 4BF9D425  bl 0x82df36d8
	ctx.lr = 0x82E562B8;
	sub_82DF36D8(ctx, base);
	// 82E562B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E562BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E562C0: 48351EF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E562C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E562C8 size=104
    let mut pc: u32 = 0x82E562C8;
    'dispatch: loop {
        match pc {
            0x82E562C8 => {
    //   block [0x82E562C8..0x82E56330)
	// 82E562C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E562CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E562D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E562D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E562D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E562DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E562E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E562E4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E562E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E562EC: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 82E562F0: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82E562F4: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82E562F8: 4BF9C0F1  bl 0x82df23e8
	ctx.lr = 0x82E562FC;
	sub_82DF23E8(ctx, base);
	// 82E562FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E56300: 41820014  beq 0x82e56314
	if ctx.cr[0].eq {
	pc = 0x82E56314; continue 'dispatch;
	}
	// 82E56304: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E56308: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5630C: 480179B5  bl 0x82e6dcc0
	ctx.lr = 0x82E56310;
	sub_82E6DCC0(ctx, base);
	// 82E56310: 48000008  b 0x82e56318
	pc = 0x82E56318; continue 'dispatch;
	// 82E56314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E56318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E5631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56324: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E56330 size=56
    let mut pc: u32 = 0x82E56330;
    'dispatch: loop {
        match pc {
            0x82E56330 => {
    //   block [0x82E56330..0x82E56368)
	// 82E56330: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E56334: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E56338: 390B682C  addi r8, r11, 0x682c
	ctx.r[8].s64 = ctx.r[11].s64 + 26668;
	// 82E5633C: 816A6830  lwz r11, 0x6830(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26672 as u32) ) } as u64;
	// 82E56340: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E56344: 40820018  bne 0x82e5635c
	if !ctx.cr[0].eq {
	pc = 0x82E5635C; continue 'dispatch;
	}
	// 82E56348: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82E5634C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E56350: 3929CF44  addi r9, r9, -0x30bc
	ctx.r[9].s64 = ctx.r[9].s64 + -12476;
	// 82E56354: 916A6830  stw r11, 0x6830(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26672 as u32), ctx.r[11].u32 ) };
	// 82E56358: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E5635C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E56360: 910B6CAC  stw r8, 0x6cac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(27820 as u32), ctx.r[8].u32 ) };
	// 82E56364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56368 size=68
    let mut pc: u32 = 0x82E56368;
    'dispatch: loop {
        match pc {
            0x82E56368 => {
    //   block [0x82E56368..0x82E563AC)
	// 82E56368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E5636C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56370: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56378: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E5637C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E56380: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E56384: 396BCF4C  addi r11, r11, -0x30b4
	ctx.r[11].s64 = ctx.r[11].s64 + -12468;
	// 82E56388: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5638C: 41820008  beq 0x82e56394
	if ctx.cr[0].eq {
	pc = 0x82E56394; continue 'dispatch;
	}
	// 82E56390: 4B469ED9  bl 0x822c0268
	ctx.lr = 0x82E56394;
	sub_822C0268(ctx, base);
	// 82E56394: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E5639C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E563A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E563A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E563A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E563B0 size=12
    let mut pc: u32 = 0x82E563B0;
    'dispatch: loop {
        match pc {
            0x82E563B0 => {
    //   block [0x82E563B0..0x82E563BC)
	// 82E563B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E563B4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82E563B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E563C0 size=8
    let mut pc: u32 = 0x82E563C0;
    'dispatch: loop {
        match pc {
            0x82E563C0 => {
    //   block [0x82E563C0..0x82E563C8)
	// 82E563C0: 88630008  lbz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E563C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E563C8 size=8
    let mut pc: u32 = 0x82E563C8;
    'dispatch: loop {
        match pc {
            0x82E563C8 => {
    //   block [0x82E563C8..0x82E563D0)
	// 82E563C8: 98830024  stb r4, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u8 ) };
	// 82E563CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E563D0 size=8
    let mut pc: u32 = 0x82E563D0;
    'dispatch: loop {
        match pc {
            0x82E563D0 => {
    //   block [0x82E563D0..0x82E563D8)
	// 82E563D0: 88630024  lbz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E563D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E563D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E563D8 size=56
    let mut pc: u32 = 0x82E563D8;
    'dispatch: loop {
        match pc {
            0x82E563D8 => {
    //   block [0x82E563D8..0x82E56410)
	// 82E563D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E563DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E563E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E563E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E563E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E563EC: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E563F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E563F4: 4BF9B805  bl 0x82df1bf8
	ctx.lr = 0x82E563F8;
	sub_82DF1BF8(ctx, base);
	// 82E563F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E563FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E56400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E56404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E5640C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56410 size=108
    let mut pc: u32 = 0x82E56410;
    'dispatch: loop {
        match pc {
            0x82E56410 => {
    //   block [0x82E56410..0x82E5647C)
	// 82E56410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5641C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56428: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5642C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E56430: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E56434: 419A002C  beq cr6, 0x82e56460
	if ctx.cr[6].eq {
	pc = 0x82E56460; continue 'dispatch;
	}
	// 82E56438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E5643C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E56440: 4BF9B7B9  bl 0x82df1bf8
	ctx.lr = 0x82E56444;
	sub_82DF1BF8(ctx, base);
	// 82E56444: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E56448: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E5644C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E56450: 4800D4A9  bl 0x82e638f8
	ctx.lr = 0x82E56454;
	sub_82E638F8(ctx, base);
	// 82E56454: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E56458: 4BF9B839  bl 0x82df1c90
	ctx.lr = 0x82E5645C;
	sub_82DF1C90(ctx, base);
	// 82E5645C: 48000008  b 0x82e56464
	pc = 0x82E56464; continue 'dispatch;
	// 82E56460: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E56464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E56468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5646C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56480 size=124
    let mut pc: u32 = 0x82E56480;
    'dispatch: loop {
        match pc {
            0x82E56480 => {
    //   block [0x82E56480..0x82E564FC)
	// 82E56480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E5648C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56494: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E56498: 396BCF58  addi r11, r11, -0x30a8
	ctx.r[11].s64 = ctx.r[11].s64 + -12456;
	// 82E5649C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E564A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E564A4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E564A8: 419A0024  beq cr6, 0x82e564cc
	if ctx.cr[6].eq {
	pc = 0x82E564CC; continue 'dispatch;
	}
	// 82E564AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E564B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E564B4: 4BF9B745  bl 0x82df1bf8
	ctx.lr = 0x82E564B8;
	sub_82DF1BF8(ctx, base);
	// 82E564B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E564BC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E564C0: 4800D269  bl 0x82e63728
	ctx.lr = 0x82E564C4;
	sub_82E63728(ctx, base);
	// 82E564C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E564C8: 4BF9B7C9  bl 0x82df1c90
	ctx.lr = 0x82E564CC;
	sub_82DF1C90(ctx, base);
	// 82E564CC: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E564D0: 4BF9CF59  bl 0x82df3428
	ctx.lr = 0x82E564D4;
	sub_82DF3428(ctx, base);
	// 82E564D4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E564D8: 4BFA8C01  bl 0x82dff0d8
	ctx.lr = 0x82E564DC;
	sub_82DFF0D8(ctx, base);
	// 82E564DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E564E0: 396BCF4C  addi r11, r11, -0x30b4
	ctx.r[11].s64 = ctx.r[11].s64 + -12468;
	// 82E564E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E564E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E564EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E564F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E564F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E564F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56500 size=76
    let mut pc: u32 = 0x82E56500;
    'dispatch: loop {
        match pc {
            0x82E56500 => {
    //   block [0x82E56500..0x82E5654C)
	// 82E56500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5650C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56518: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E5651C: 4BFFFF65  bl 0x82e56480
	ctx.lr = 0x82E56520;
	sub_82E56480(ctx, base);
	// 82E56520: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E56524: 4182000C  beq 0x82e56530
	if ctx.cr[0].eq {
	pc = 0x82E56530; continue 'dispatch;
	}
	// 82E56528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E5652C: 4BF9BEAD  bl 0x82df23d8
	ctx.lr = 0x82E56530;
	sub_82DF23D8(ctx, base);
	// 82E56530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E56534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E56538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E5653C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E56540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E56544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E56548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E56550 size=104
    let mut pc: u32 = 0x82E56550;
    'dispatch: loop {
        match pc {
            0x82E56550 => {
    //   block [0x82E56550..0x82E565B8)
	// 82E56550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E56554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E56558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E5655C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E56560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E56564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E56568: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E5656C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E56570: 396BCF58  addi r11, r11, -0x30a8
	ctx.r[11].s64 = ctx.r[11].s64 + -12456;
	// 82E56574: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E56578: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E5657C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E56580: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82E56584: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E56588: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82E5658C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82E56590: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E56594: 4BF9CB5D  bl 0x82df30f0
	ctx.lr = 0x82E56598;
	sub_82DF30F0(ctx, base);
	// 82E56598: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82E5659C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E565A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E565A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E565A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E565AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E565B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E565B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E565B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E565B8 size=64
    let mut pc: u32 = 0x82E565B8;
    'dispatch: loop {
        match pc {
            0x82E565B8 => {
    //   block [0x82E565B8..0x82E565F8)
	// 82E565B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E565BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E565C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E565C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E565C8: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E565CC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 82E565D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E565D4: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82E565D8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82E565DC: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E565E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E565E4: 480428D5  bl 0x82e98eb8
	ctx.lr = 0x82E565E8;
	sub_82E98EB8(ctx, base);
	// 82E565E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E565EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E565F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E565F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E565F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E565F8 size=16
    let mut pc: u32 = 0x82E565F8;
    'dispatch: loop {
        match pc {
            0x82E565F8 => {
    //   block [0x82E565F8..0x82E56608)
	// 82E565F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E565FC: 396BCF6C  addi r11, r11, -0x3094
	ctx.r[11].s64 = ctx.r[11].s64 + -12436;
	// 82E56600: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E56604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E56608 size=24
    let mut pc: u32 = 0x82E56608;
    'dispatch: loop {
        match pc {
            0x82E56608 => {
    //   block [0x82E56608..0x82E56620)
	// 82E56608: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E5660C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82E56610: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E56614: 996A6835  stb r11, 0x6835(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(26677 as u32), ctx.r[11].u8 ) };
	// 82E56618: 9069B22C  stw r3, -0x4dd4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-19924 as u32), ctx.r[3].u32 ) };
	// 82E5661C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E56620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E56620 size=28
    let mut pc: u32 = 0x82E56620;
    'dispatch: loop {
        match pc {
            0x82E56620 => {
    //   block [0x82E56620..0x82E5663C)
	// 82E56620: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82E56624: 3D008332  lis r8, -0x7cce
	ctx.r[8].s64 = -2093875200;
	// 82E56628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E5662C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E56630: 99696835  stb r11, 0x6835(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(26677 as u32), ctx.r[11].u8 ) };
	// 82E56634: 9148B22C  stw r10, -0x4dd4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-19924 as u32), ctx.r[10].u32 ) };
	// 82E56638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


