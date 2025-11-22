pub fn sub_830AA678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA678 size=60
    let mut pc: u32 = 0x830AA678;
    'dispatch: loop {
        match pc {
            0x830AA678 => {
    //   block [0x830AA678..0x830AA6B4)
	// 830AA678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA68C: 4BFFCB6D  bl 0x830a71f8
	ctx.lr = 0x830AA690;
	sub_830A71F8(ctx, base);
	// 830AA690: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA698: 396B03C8  addi r11, r11, 0x3c8
	ctx.r[11].s64 = ctx.r[11].s64 + 968;
	// 830AA69C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AA6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AA6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA6B8 size=60
    let mut pc: u32 = 0x830AA6B8;
    'dispatch: loop {
        match pc {
            0x830AA6B8 => {
    //   block [0x830AA6B8..0x830AA6F4)
	// 830AA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA6C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA6C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA6C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA6CC: 4BFFCC5D  bl 0x830a7328
	ctx.lr = 0x830AA6D0;
	sub_830A7328(ctx, base);
	// 830AA6D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AA6D4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA6D8: 3880005E  li r4, 0x5e
	ctx.r[4].s64 = 94;
	// 830AA6DC: 4BFF919D  bl 0x830a3878
	ctx.lr = 0x830AA6E0;
	sub_830A3878(ctx, base);
	// 830AA6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AA6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA6EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA6F8 size=60
    let mut pc: u32 = 0x830AA6F8;
    'dispatch: loop {
        match pc {
            0x830AA6F8 => {
    //   block [0x830AA6F8..0x830AA734)
	// 830AA6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA700: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA704: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA70C: 4BFFCC1D  bl 0x830a7328
	ctx.lr = 0x830AA710;
	sub_830A7328(ctx, base);
	// 830AA710: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AA714: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA718: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 830AA71C: 4BFF915D  bl 0x830a3878
	ctx.lr = 0x830AA720;
	sub_830A3878(ctx, base);
	// 830AA720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AA724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA72C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA738 size=92
    let mut pc: u32 = 0x830AA738;
    'dispatch: loop {
        match pc {
            0x830AA738 => {
    //   block [0x830AA738..0x830AA794)
	// 830AA738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AA744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA74C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA750: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AA754: 4BFFCBD5  bl 0x830a7328
	ctx.lr = 0x830AA758;
	sub_830A7328(ctx, base);
	// 830AA758: 83FF0030  lwz r31, 0x30(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AA760: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AA764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA768: 4BFF8D49  bl 0x830a34b0
	ctx.lr = 0x830AA76C;
	sub_830A34B0(ctx, base);
	// 830AA76C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AA770: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AA774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA778: 4BFF8E49  bl 0x830a35c0
	ctx.lr = 0x830AA77C;
	sub_830A35C0(ctx, base);
	// 830AA77C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AA780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA788: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AA78C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA798 size=72
    let mut pc: u32 = 0x830AA798;
    'dispatch: loop {
        match pc {
            0x830AA798 => {
    //   block [0x830AA798..0x830AA7E0)
	// 830AA798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA7A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AA7A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA7A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA7AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA7B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AA7B4: 4BFFCB75  bl 0x830a7328
	ctx.lr = 0x830AA7B8;
	sub_830A7328(ctx, base);
	// 830AA7B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AA7BC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA7C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AA7C4: 4BFF8CED  bl 0x830a34b0
	ctx.lr = 0x830AA7C8;
	sub_830A34B0(ctx, base);
	// 830AA7C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AA7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA7D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AA7D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA7E0 size=120
    let mut pc: u32 = 0x830AA7E0;
    'dispatch: loop {
        match pc {
            0x830AA7E0 => {
    //   block [0x830AA7E0..0x830AA858)
	// 830AA7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA7E4: 480FD989  bl 0x831a816c
	ctx.lr = 0x830AA7E8;
	sub_831A8130(ctx, base);
	// 830AA7E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA7EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA7F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830AA7F4: 4BFFCB35  bl 0x830a7328
	ctx.lr = 0x830AA7F8;
	sub_830A7328(ctx, base);
	// 830AA7F8: 83DF0030  lwz r30, 0x30(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA7FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA800: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA804: 4BFF8E65  bl 0x830a3668
	ctx.lr = 0x830AA808;
	sub_830A3668(ctx, base);
	// 830AA808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA80C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AA810: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AA814: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA818: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830AA81C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA820: 4E800421  bctrl
	ctx.lr = 0x830AA824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA824: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830AA828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA82C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA830: 4BFF8A71  bl 0x830a32a0
	ctx.lr = 0x830AA834;
	sub_830A32A0(ctx, base);
	// 830AA834: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AA838: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 830AA83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA840: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AA844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA848: 4E800421  bctrl
	ctx.lr = 0x830AA84C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA84C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AA854: 480FD968  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA858 size=156
    let mut pc: u32 = 0x830AA858;
    'dispatch: loop {
        match pc {
            0x830AA858 => {
    //   block [0x830AA858..0x830AA8F4)
	// 830AA858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AA860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AA864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AA868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA870: 4BFFCAB9  bl 0x830a7328
	ctx.lr = 0x830AA874;
	sub_830A7328(ctx, base);
	// 830AA874: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AA878: 83DF0030  lwz r30, 0x30(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA87C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA880: 4BFFF321  bl 0x830a9ba0
	ctx.lr = 0x830AA884;
	sub_830A9BA0(ctx, base);
	// 830AA884: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AA888: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AA890: 4BFF8B79  bl 0x830a3408
	ctx.lr = 0x830AA894;
	sub_830A3408(ctx, base);
	// 830AA894: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AA89C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 830AA8A0: 419A0030  beq cr6, 0x830aa8d0
	if ctx.cr[6].eq {
	pc = 0x830AA8D0; continue 'dispatch;
	}
	// 830AA8A4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA8A8: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AA8AC: 38C0007E  li r6, 0x7e
	ctx.r[6].s64 = 126;
	// 830AA8B0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AA8B4: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 830AA8B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA8BC: 4BFFC8CD  bl 0x830a7188
	ctx.lr = 0x830AA8C0;
	sub_830A7188(ctx, base);
	// 830AA8C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AA8C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AA8C8: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AA8CC: 4810635D  bl 0x831b0c28
	ctx.lr = 0x830AA8D0;
	sub_831B0C28(ctx, base);
	// 830AA8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA8D4: 4BFFCA55  bl 0x830a7328
	ctx.lr = 0x830AA8D8;
	sub_830A7328(ctx, base);
	// 830AA8D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AA8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AA8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AA8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AA8E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AA8EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AA8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AA8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AA8F8 size=1412
    let mut pc: u32 = 0x830AA8F8;
    'dispatch: loop {
        match pc {
            0x830AA8F8 => {
    //   block [0x830AA8F8..0x830AAE7C)
	// 830AA8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AA8FC: 480FD85D  bl 0x831a8158
	ctx.lr = 0x830AA900;
	sub_831A8130(ctx, base);
	// 830AA900: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AA904: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AA908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AA90C: A16BFFB8  lhz r11, -0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) } as u64;
	// 830AA910: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 830AA914: 4BFFCA15  bl 0x830a7328
	ctx.lr = 0x830AA918;
	sub_830A7328(ctx, base);
	// 830AA918: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AA91C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830AA920: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 830AA924: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 830AA928: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AA92C: 40820048  bne 0x830aa974
	if !ctx.cr[0].eq {
	pc = 0x830AA974; continue 'dispatch;
	}
	// 830AA930: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AA934: 2F0B005E  cmpwi cr6, r11, 0x5e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 94, &mut ctx.xer);
	// 830AA938: 409A003C  bne cr6, 0x830aa974
	if !ctx.cr[6].eq {
	pc = 0x830AA974; continue 'dispatch;
	}
	// 830AA93C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AA940: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 830AA944: 4BFFC9E5  bl 0x830a7328
	ctx.lr = 0x830AA948;
	sub_830A7328(ctx, base);
	// 830AA948: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA94C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA950: 4BFF8E21  bl 0x830a3770
	ctx.lr = 0x830AA954;
	sub_830A3770(ctx, base);
	// 830AA954: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830AA958: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830AA95C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA960: 80ABFD68  lwz r5, -0x298(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830AA964: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AA968: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AA96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AA970: 4E800421  bctrl
	ctx.lr = 0x830AA974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AA974: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AA978: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AA97C: 4BFF8DF5  bl 0x830a3770
	ctx.lr = 0x830AA980;
	sub_830A3770(ctx, base);
	// 830AA980: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830AA984: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AA988: 4800027C  b 0x830aac04
	pc = 0x830AAC04; continue 'dispatch;
	// 830AA98C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830AA990: 409A0018  bne cr6, 0x830aa9a8
	if !ctx.cr[6].eq {
	pc = 0x830AA9A8; continue 'dispatch;
	}
	// 830AA994: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AA998: 2F0A005D  cmpwi cr6, r10, 0x5d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 93, &mut ctx.xer);
	// 830AA99C: 409A000C  bne cr6, 0x830aa9a8
	if !ctx.cr[6].eq {
	pc = 0x830AA9A8; continue 'dispatch;
	}
	// 830AA9A0: 556A063F  clrlwi. r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830AA9A4: 418202A4  beq 0x830aac48
	if ctx.cr[0].eq {
	pc = 0x830AAC48; continue 'dispatch;
	}
	// 830AA9A8: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AA9AC: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 830AA9B0: 2F1B000A  cmpwi cr6, r27, 0xa
	ctx.cr[6].compare_i32(ctx.r[27].s32, 10, &mut ctx.xer);
	// 830AA9B4: 409A0104  bne cr6, 0x830aaab8
	if !ctx.cr[6].eq {
	pc = 0x830AAAB8; continue 'dispatch;
	}
	// 830AA9B8: 2F1E0063  cmpwi cr6, r30, 0x63
	ctx.cr[6].compare_i32(ctx.r[30].s32, 99, &mut ctx.xer);
	// 830AA9BC: 41990038  bgt cr6, 0x830aa9f4
	if ctx.cr[6].gt {
	pc = 0x830AA9F4; continue 'dispatch;
	}
	// 830AA9C0: 419A0098  beq cr6, 0x830aaa58
	if ctx.cr[6].eq {
	pc = 0x830AAA58; continue 'dispatch;
	}
	// 830AA9C4: 2F1E0043  cmpwi cr6, r30, 0x43
	ctx.cr[6].compare_i32(ctx.r[30].s32, 67, &mut ctx.xer);
	// 830AA9C8: 419A0090  beq cr6, 0x830aaa58
	if ctx.cr[6].eq {
	pc = 0x830AAA58; continue 'dispatch;
	}
	// 830AA9CC: 2F1E0044  cmpwi cr6, r30, 0x44
	ctx.cr[6].compare_i32(ctx.r[30].s32, 68, &mut ctx.xer);
	// 830AA9D0: 419A00B0  beq cr6, 0x830aaa80
	if ctx.cr[6].eq {
	pc = 0x830AAA80; continue 'dispatch;
	}
	// 830AA9D4: 2F1E0049  cmpwi cr6, r30, 0x49
	ctx.cr[6].compare_i32(ctx.r[30].s32, 73, &mut ctx.xer);
	// 830AA9D8: 419A0080  beq cr6, 0x830aaa58
	if ctx.cr[6].eq {
	pc = 0x830AAA58; continue 'dispatch;
	}
	// 830AA9DC: 2F1E0050  cmpwi cr6, r30, 0x50
	ctx.cr[6].compare_i32(ctx.r[30].s32, 80, &mut ctx.xer);
	// 830AA9E0: 419A0058  beq cr6, 0x830aaa38
	if ctx.cr[6].eq {
	pc = 0x830AAA38; continue 'dispatch;
	}
	// 830AA9E4: 2F1E0053  cmpwi cr6, r30, 0x53
	ctx.cr[6].compare_i32(ctx.r[30].s32, 83, &mut ctx.xer);
	// 830AA9E8: 419A0098  beq cr6, 0x830aaa80
	if ctx.cr[6].eq {
	pc = 0x830AAA80; continue 'dispatch;
	}
	// 830AA9EC: 2F1E0057  cmpwi cr6, r30, 0x57
	ctx.cr[6].compare_i32(ctx.r[30].s32, 87, &mut ctx.xer);
	// 830AA9F0: 48000028  b 0x830aaa18
	pc = 0x830AAA18; continue 'dispatch;
	// 830AA9F4: 2F1E0064  cmpwi cr6, r30, 0x64
	ctx.cr[6].compare_i32(ctx.r[30].s32, 100, &mut ctx.xer);
	// 830AA9F8: 419A0088  beq cr6, 0x830aaa80
	if ctx.cr[6].eq {
	pc = 0x830AAA80; continue 'dispatch;
	}
	// 830AA9FC: 2F1E0069  cmpwi cr6, r30, 0x69
	ctx.cr[6].compare_i32(ctx.r[30].s32, 105, &mut ctx.xer);
	// 830AAA00: 419A0058  beq cr6, 0x830aaa58
	if ctx.cr[6].eq {
	pc = 0x830AAA58; continue 'dispatch;
	}
	// 830AAA04: 2F1E0070  cmpwi cr6, r30, 0x70
	ctx.cr[6].compare_i32(ctx.r[30].s32, 112, &mut ctx.xer);
	// 830AAA08: 419A0030  beq cr6, 0x830aaa38
	if ctx.cr[6].eq {
	pc = 0x830AAA38; continue 'dispatch;
	}
	// 830AAA0C: 2F1E0073  cmpwi cr6, r30, 0x73
	ctx.cr[6].compare_i32(ctx.r[30].s32, 115, &mut ctx.xer);
	// 830AAA10: 419A0070  beq cr6, 0x830aaa80
	if ctx.cr[6].eq {
	pc = 0x830AAA80; continue 'dispatch;
	}
	// 830AAA14: 2F1E0077  cmpwi cr6, r30, 0x77
	ctx.cr[6].compare_i32(ctx.r[30].s32, 119, &mut ctx.xer);
	// 830AAA18: 419A0068  beq cr6, 0x830aaa80
	if ctx.cr[6].eq {
	pc = 0x830AAA80; continue 'dispatch;
	}
	// 830AAA1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAA20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAA24: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AAA28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAA2C: 4E800421  bctrl
	ctx.lr = 0x830AAA30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAA30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AAA34: 48000094  b 0x830aaac8
	pc = 0x830AAAC8; continue 'dispatch;
	// 830AAA38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AAA3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAA40: 4BFFE169  bl 0x830a8ba8
	ctx.lr = 0x830AAA44;
	sub_830A8BA8(ctx, base);
	// 830AAA44: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830AAA48: 41820228  beq 0x830aac70
	if ctx.cr[0].eq {
	pc = 0x830AAC70; continue 'dispatch;
	}
	// 830AAA4C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAA50: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AAA54: 48000050  b 0x830aaaa4
	pc = 0x830AAAA4; continue 'dispatch;
	// 830AAA58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAA5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AAA60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AAA64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAA68: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 830AAA6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAA70: 4E800421  bctrl
	ctx.lr = 0x830AAA74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAA74: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830AAA78: 40800050  bge 0x830aaac8
	if !ctx.cr[0].lt {
	pc = 0x830AAAC8; continue 'dispatch;
	}
	// 830AAA7C: 48000034  b 0x830aaab0
	pc = 0x830AAAB0; continue 'dispatch;
	// 830AAA80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAA84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AAA88: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAA8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAA90: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AAA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAA98: 4E800421  bctrl
	ctx.lr = 0x830AAA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAA9C: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AAAA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AAAA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AAAA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAAAC: 4E800421  bctrl
	ctx.lr = 0x830AAAB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAAB0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 830AAAB4: 48000014  b 0x830aaac8
	pc = 0x830AAAC8; continue 'dispatch;
	// 830AAAB8: 2F1B0018  cmpwi cr6, r27, 0x18
	ctx.cr[6].compare_i32(ctx.r[27].s32, 24, &mut ctx.xer);
	// 830AAABC: 409A000C  bne cr6, 0x830aaac8
	if !ctx.cr[6].eq {
	pc = 0x830AAAC8; continue 'dispatch;
	}
	// 830AAAC0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AAAC4: 418201D8  beq 0x830aac9c
	if ctx.cr[0].eq {
	pc = 0x830AAC9C; continue 'dispatch;
	}
	// 830AAAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAACC: 4BFFC85D  bl 0x830a7328
	ctx.lr = 0x830AAAD0;
	sub_830A7328(ctx, base);
	// 830AAAD0: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AAAD4: 4082012C  bne 0x830aac00
	if !ctx.cr[0].eq {
	pc = 0x830AAC00; continue 'dispatch;
	}
	// 830AAAD8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830AAADC: 409A001C  bne cr6, 0x830aaaf8
	if !ctx.cr[6].eq {
	pc = 0x830AAAF8; continue 'dispatch;
	}
	// 830AAAE0: 2F1E005B  cmpwi cr6, r30, 0x5b
	ctx.cr[6].compare_i32(ctx.r[30].s32, 91, &mut ctx.xer);
	// 830AAAE4: 419A0250  beq cr6, 0x830aad34
	if ctx.cr[6].eq {
	pc = 0x830AAD34; continue 'dispatch;
	}
	// 830AAAE8: 2F1E005D  cmpwi cr6, r30, 0x5d
	ctx.cr[6].compare_i32(ctx.r[30].s32, 93, &mut ctx.xer);
	// 830AAAEC: 419A0248  beq cr6, 0x830aad34
	if ctx.cr[6].eq {
	pc = 0x830AAD34; continue 'dispatch;
	}
	// 830AAAF0: 2F1E002D  cmpwi cr6, r30, 0x2d
	ctx.cr[6].compare_i32(ctx.r[30].s32, 45, &mut ctx.xer);
	// 830AAAF4: 419A0240  beq cr6, 0x830aad34
	if ctx.cr[6].eq {
	pc = 0x830AAD34; continue 'dispatch;
	}
	// 830AAAF8: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AAAFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AAB00: 408200E4  bne 0x830aabe4
	if !ctx.cr[0].eq {
	pc = 0x830AABE4; continue 'dispatch;
	}
	// 830AAB04: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AAB08: 2F0B002D  cmpwi cr6, r11, 0x2d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 45, &mut ctx.xer);
	// 830AAB0C: 409A00D8  bne cr6, 0x830aabe4
	if !ctx.cr[6].eq {
	pc = 0x830AABE4; continue 'dispatch;
	}
	// 830AAB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAB14: 4BFFC815  bl 0x830a7328
	ctx.lr = 0x830AAB18;
	sub_830A7328(ctx, base);
	// 830AAB18: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AAB1C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830AAB20: 419A025C  beq cr6, 0x830aad7c
	if ctx.cr[6].eq {
	pc = 0x830AAD7C; continue 'dispatch;
	}
	// 830AAB24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AAB28: 409A0010  bne cr6, 0x830aab38
	if !ctx.cr[6].eq {
	pc = 0x830AAB38; continue 'dispatch;
	}
	// 830AAB2C: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AAB30: 2F0A005D  cmpwi cr6, r10, 0x5d
	ctx.cr[6].compare_i32(ctx.r[10].s32, 93, &mut ctx.xer);
	// 830AAB34: 419A0274  beq cr6, 0x830aada8
	if ctx.cr[6].eq {
	pc = 0x830AADA8; continue 'dispatch;
	}
	// 830AAB38: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 830AAB3C: 419A026C  beq cr6, 0x830aada8
	if ctx.cr[6].eq {
	pc = 0x830AADA8; continue 'dispatch;
	}
	// 830AAB40: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AAB44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AAB48: B3210062  sth r25, 0x62(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[25].u16 ) };
	// 830AAB4C: B3A10060  sth r29, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u16 ) };
	// 830AAB50: 409A005C  bne cr6, 0x830aabac
	if !ctx.cr[6].eq {
	pc = 0x830AABAC; continue 'dispatch;
	}
	// 830AAB54: 2F1D005B  cmpwi cr6, r29, 0x5b
	ctx.cr[6].compare_i32(ctx.r[29].s32, 91, &mut ctx.xer);
	// 830AAB58: 419A0014  beq cr6, 0x830aab6c
	if ctx.cr[6].eq {
	pc = 0x830AAB6C; continue 'dispatch;
	}
	// 830AAB5C: 2F1D005D  cmpwi cr6, r29, 0x5d
	ctx.cr[6].compare_i32(ctx.r[29].s32, 93, &mut ctx.xer);
	// 830AAB60: 419A000C  beq cr6, 0x830aab6c
	if ctx.cr[6].eq {
	pc = 0x830AAB6C; continue 'dispatch;
	}
	// 830AAB64: 2F1D002D  cmpwi cr6, r29, 0x2d
	ctx.cr[6].compare_i32(ctx.r[29].s32, 45, &mut ctx.xer);
	// 830AAB68: 409A0064  bne cr6, 0x830aabcc
	if !ctx.cr[6].eq {
	pc = 0x830AABCC; continue 'dispatch;
	}
	// 830AAB6C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAB70: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAB74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AAB78: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AAB80: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 830AAB84: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830AAB88: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 830AAB8C: 38C0008E  li r6, 0x8e
	ctx.r[6].s64 = 142;
	// 830AAB90: 38A00128  li r5, 0x128
	ctx.r[5].s64 = 296;
	// 830AAB94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAB98: 4BFDD671  bl 0x83088208
	ctx.lr = 0x830AAB9C;
	sub_83088208(ctx, base);
	// 830AAB9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AABA0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AABA4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AABA8: 48106081  bl 0x831b0c28
	ctx.lr = 0x830AABAC;
	sub_831B0C28(ctx, base);
	// 830AABAC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 830AABB0: 409A001C  bne cr6, 0x830aabcc
	if !ctx.cr[6].eq {
	pc = 0x830AABCC; continue 'dispatch;
	}
	// 830AABB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AABB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AABBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AABC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AABC4: 4E800421  bctrl
	ctx.lr = 0x830AABC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AABC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830AABCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AABD0: 4BFFC759  bl 0x830a7328
	ctx.lr = 0x830AABD4;
	sub_830A7328(ctx, base);
	// 830AABD4: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830AABD8: 41990214  bgt cr6, 0x830aadec
	if ctx.cr[6].gt {
	pc = 0x830AADEC; continue 'dispatch;
	}
	// 830AABDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AABE0: 48000008  b 0x830aabe8
	pc = 0x830AABE8; continue 'dispatch;
	// 830AABE4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AABE8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AABEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AABF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AABF4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AABF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AABFC: 4E800421  bctrl
	ctx.lr = 0x830AAC00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAC00: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 830AAC04: A37F0020  lhz r27, 0x20(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AAC08: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 830AAC0C: 409AFD80  bne cr6, 0x830aa98c
	if !ctx.cr[6].eq {
	pc = 0x830AA98C; continue 'dispatch;
	}
	// 830AAC10: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AAC14: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830AAC18: 409A021C  bne cr6, 0x830aae34
	if !ctx.cr[6].eq {
	pc = 0x830AAE34; continue 'dispatch;
	}
	// 830AAC1C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAC20: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAC24: 38C0008A  li r6, 0x8a
	ctx.r[6].s64 = 138;
	// 830AAC28: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAC2C: 38A0013D  li r5, 0x13d
	ctx.r[5].s64 = 317;
	// 830AAC30: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAC34: 4BFFC555  bl 0x830a7188
	ctx.lr = 0x830AAC38;
	sub_830A7188(ctx, base);
	// 830AAC38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAC3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAC40: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AAC44: 48105FE5  bl 0x831b0c28
	ctx.lr = 0x830AAC48;
	sub_831B0C28(ctx, base);
	// 830AAC48: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AAC4C: 4182FFC4  beq 0x830aac10
	if ctx.cr[0].eq {
	pc = 0x830AAC10; continue 'dispatch;
	}
	// 830AAC50: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAC54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AAC58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830AAC5C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830AAC60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAC64: 4E800421  bctrl
	ctx.lr = 0x830AAC68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAC68: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 830AAC6C: 4BFFFFA4  b 0x830aac10
	pc = 0x830AAC10; continue 'dispatch;
	// 830AAC70: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAC74: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAC78: 38C00088  li r6, 0x88
	ctx.r[6].s64 = 136;
	// 830AAC7C: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAC80: 38A000E5  li r5, 0xe5
	ctx.r[5].s64 = 229;
	// 830AAC84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAC88: 4BFFC501  bl 0x830a7188
	ctx.lr = 0x830AAC8C;
	sub_830A7188(ctx, base);
	// 830AAC8C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAC90: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAC94: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AAC98: 48105F91  bl 0x831b0c28
	ctx.lr = 0x830AAC9C;
	sub_831B0C28(ctx, base);
	// 830AAC9C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AACA0: 41820020  beq 0x830aacc0
	if ctx.cr[0].eq {
	pc = 0x830AACC0; continue 'dispatch;
	}
	// 830AACA4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AACA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AACAC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830AACB0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830AACB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AACB8: 4E800421  bctrl
	ctx.lr = 0x830AACBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AACBC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 830AACC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AACC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AACC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AACCC: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 830AACD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AACD4: 4E800421  bctrl
	ctx.lr = 0x830AACD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AACD8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AACDC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AACE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AACE4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830AACE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AACEC: 4E800421  bctrl
	ctx.lr = 0x830AACF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AACF0: A17F0020  lhz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AACF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AACF8: 40820010  bne 0x830aad08
	if !ctx.cr[0].eq {
	pc = 0x830AAD08; continue 'dispatch;
	}
	// 830AACFC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AAD00: 2F0B005D  cmpwi cr6, r11, 0x5d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 93, &mut ctx.xer);
	// 830AAD04: 419AFF0C  beq cr6, 0x830aac10
	if ctx.cr[6].eq {
	pc = 0x830AAC10; continue 'dispatch;
	}
	// 830AAD08: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAD0C: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAD10: 38C0008D  li r6, 0x8d
	ctx.r[6].s64 = 141;
	// 830AAD14: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAD18: 38A000FC  li r5, 0xfc
	ctx.r[5].s64 = 252;
	// 830AAD1C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAD20: 4BFFC469  bl 0x830a7188
	ctx.lr = 0x830AAD24;
	sub_830A7188(ctx, base);
	// 830AAD24: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAD28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAD2C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AAD30: 48105EF9  bl 0x831b0c28
	ctx.lr = 0x830AAD34;
	sub_831B0C28(ctx, base);
	// 830AAD34: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAD38: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAD3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AAD40: B3C10060  sth r30, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u16 ) };
	// 830AAD44: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAD48: B3210062  sth r25, 0x62(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[25].u16 ) };
	// 830AAD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AAD50: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 830AAD54: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830AAD58: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 830AAD5C: 38C0008E  li r6, 0x8e
	ctx.r[6].s64 = 142;
	// 830AAD60: 38A0010B  li r5, 0x10b
	ctx.r[5].s64 = 267;
	// 830AAD64: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAD68: 4BFDD4A1  bl 0x83088208
	ctx.lr = 0x830AAD6C;
	sub_83088208(ctx, base);
	// 830AAD6C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAD70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAD74: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AAD78: 48105EB1  bl 0x831b0c28
	ctx.lr = 0x830AAD7C;
	sub_831B0C28(ctx, base);
	// 830AAD7C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAD80: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAD84: 38C0008A  li r6, 0x8a
	ctx.r[6].s64 = 138;
	// 830AAD88: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAD8C: 38A00115  li r5, 0x115
	ctx.r[5].s64 = 277;
	// 830AAD90: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAD94: 4BFFC3F5  bl 0x830a7188
	ctx.lr = 0x830AAD98;
	sub_830A7188(ctx, base);
	// 830AAD98: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAD9C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AADA0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AADA4: 48105E85  bl 0x831b0c28
	ctx.lr = 0x830AADA8;
	sub_831B0C28(ctx, base);
	// 830AADA8: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AADAC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AADB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AADB4: 390B0484  addi r8, r11, 0x484
	ctx.r[8].s64 = ctx.r[11].s64 + 1156;
	// 830AADB8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AADBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AADC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 830AADC4: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AADC8: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 830AADCC: 38C0008E  li r6, 0x8e
	ctx.r[6].s64 = 142;
	// 830AADD0: 38A0011B  li r5, 0x11b
	ctx.r[5].s64 = 283;
	// 830AADD4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AADD8: 4BFDD431  bl 0x83088208
	ctx.lr = 0x830AADDC;
	sub_83088208(ctx, base);
	// 830AADDC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AADE0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AADE4: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AADE8: 48105E41  bl 0x831b0c28
	ctx.lr = 0x830AADEC;
	sub_831B0C28(ctx, base);
	// 830AADEC: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AADF0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AADF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AADF8: B3C10064  sth r30, 0x64(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u16 ) };
	// 830AADFC: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAE00: B3210066  sth r25, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[25].u16 ) };
	// 830AAE04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AAE08: 39010064  addi r8, r1, 0x64
	ctx.r[8].s64 = ctx.r[1].s64 + 100;
	// 830AAE0C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830AAE10: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 830AAE14: 38C00091  li r6, 0x91
	ctx.r[6].s64 = 145;
	// 830AAE18: 38A00132  li r5, 0x132
	ctx.r[5].s64 = 306;
	// 830AAE1C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAE20: 4BFDD3E9  bl 0x83088208
	ctx.lr = 0x830AAE24;
	sub_83088208(ctx, base);
	// 830AAE24: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAE28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AAE2C: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AAE30: 48105DF9  bl 0x831b0c28
	ctx.lr = 0x830AAE34;
	sub_831B0C28(ctx, base);
	// 830AAE34: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAE38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AAE3C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830AAE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAE44: 4E800421  bctrl
	ctx.lr = 0x830AAE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAE48: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAE4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AAE50: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830AAE54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAE58: 4E800421  bctrl
	ctx.lr = 0x830AAE5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAE5C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAE64: A16BFFB4  lhz r11, -0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-76 as u32) ) } as u64;
	// 830AAE68: B17F0018  sth r11, 0x18(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u16 ) };
	// 830AAE6C: 4BFFC4BD  bl 0x830a7328
	ctx.lr = 0x830AAE70;
	sub_830A7328(ctx, base);
	// 830AAE70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AAE74: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 830AAE78: 480FD330  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAE80 size=96
    let mut pc: u32 = 0x830AAE80;
    'dispatch: loop {
        match pc {
            0x830AAE80 => {
    //   block [0x830AAE80..0x830AAEE0)
	// 830AAE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAE88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AAE8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AAE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAE94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAE98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830AAE9C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830AAEA0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAEA4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AAEA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAEAC: 4E800421  bctrl
	ctx.lr = 0x830AAEB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAEB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AAEB4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AAEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAEBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAEC0: 4E800421  bctrl
	ctx.lr = 0x830AAEC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAEC4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830AAEC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AAECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AAED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AAED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AAED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AAEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAEE0 size=56
    let mut pc: u32 = 0x830AAEE0;
    'dispatch: loop {
        match pc {
            0x830AAEE0 => {
    //   block [0x830AAEE0..0x830AAF18)
	// 830AAEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAEEC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAEF0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAEF4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AAEF8: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAEFC: 38A00150  li r5, 0x150
	ctx.r[5].s64 = 336;
	// 830AAF00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF04: 4BF26155  bl 0x82fd1058
	ctx.lr = 0x830AAF08;
	sub_82FD1058(ctx, base);
	// 830AAF08: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAF0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF10: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AAF14: 48105D15  bl 0x831b0c28
	ctx.lr = 0x830AAF18;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAF18 size=56
    let mut pc: u32 = 0x830AAF18;
    'dispatch: loop {
        match pc {
            0x830AAF18 => {
    //   block [0x830AAF18..0x830AAF50)
	// 830AAF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAF24: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAF28: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAF2C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AAF30: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAF34: 38A00156  li r5, 0x156
	ctx.r[5].s64 = 342;
	// 830AAF38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF3C: 4BF2611D  bl 0x82fd1058
	ctx.lr = 0x830AAF40;
	sub_82FD1058(ctx, base);
	// 830AAF40: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAF44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF48: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AAF4C: 48105CDD  bl 0x831b0c28
	ctx.lr = 0x830AAF50;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAF50 size=56
    let mut pc: u32 = 0x830AAF50;
    'dispatch: loop {
        match pc {
            0x830AAF50 => {
    //   block [0x830AAF50..0x830AAF88)
	// 830AAF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAF58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAF5C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAF60: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAF64: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AAF68: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAF6C: 38A0015C  li r5, 0x15c
	ctx.r[5].s64 = 348;
	// 830AAF70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF74: 4BF260E5  bl 0x82fd1058
	ctx.lr = 0x830AAF78;
	sub_82FD1058(ctx, base);
	// 830AAF78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAF80: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AAF84: 48105CA5  bl 0x831b0c28
	ctx.lr = 0x830AAF88;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAF88 size=56
    let mut pc: u32 = 0x830AAF88;
    'dispatch: loop {
        match pc {
            0x830AAF88 => {
    //   block [0x830AAF88..0x830AAFC0)
	// 830AAF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAF90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAF94: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AAF98: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AAF9C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AAFA0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AAFA4: 38A00162  li r5, 0x162
	ctx.r[5].s64 = 354;
	// 830AAFA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAFAC: 4BF260AD  bl 0x82fd1058
	ctx.lr = 0x830AAFB0;
	sub_82FD1058(ctx, base);
	// 830AAFB0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AAFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AAFB8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AAFBC: 48105C6D  bl 0x831b0c28
	ctx.lr = 0x830AAFC0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AAFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AAFC0 size=68
    let mut pc: u32 = 0x830AAFC0;
    'dispatch: loop {
        match pc {
            0x830AAFC0 => {
    //   block [0x830AAFC0..0x830AB004)
	// 830AAFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AAFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AAFC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AAFCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AAFD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AAFD4: 4BFFC355  bl 0x830a7328
	ctx.lr = 0x830AAFD8;
	sub_830A7328(ctx, base);
	// 830AAFD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AAFDC: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 830AAFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AAFE4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AAFE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AAFEC: 4E800421  bctrl
	ctx.lr = 0x830AAFF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AAFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AAFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AAFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AAFFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB008 size=68
    let mut pc: u32 = 0x830AB008;
    'dispatch: loop {
        match pc {
            0x830AB008 => {
    //   block [0x830AB008..0x830AB04C)
	// 830AB008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB01C: 4BFFC30D  bl 0x830a7328
	ctx.lr = 0x830AB020;
	sub_830A7328(ctx, base);
	// 830AB020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB024: 38800063  li r4, 0x63
	ctx.r[4].s64 = 99;
	// 830AB028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB02C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AB030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB034: 4E800421  bctrl
	ctx.lr = 0x830AB038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AB03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB050 size=56
    let mut pc: u32 = 0x830AB050;
    'dispatch: loop {
        match pc {
            0x830AB050 => {
    //   block [0x830AB050..0x830AB088)
	// 830AB050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB05C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB060: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB064: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB068: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB06C: 38A00174  li r5, 0x174
	ctx.r[5].s64 = 372;
	// 830AB070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB074: 4BF25FE5  bl 0x82fd1058
	ctx.lr = 0x830AB078;
	sub_82FD1058(ctx, base);
	// 830AB078: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB07C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB080: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB084: 48105BA5  bl 0x831b0c28
	ctx.lr = 0x830AB088;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB088 size=56
    let mut pc: u32 = 0x830AB088;
    'dispatch: loop {
        match pc {
            0x830AB088 => {
    //   block [0x830AB088..0x830AB0C0)
	// 830AB088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB094: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB098: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB09C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB0A0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB0A4: 38A0017A  li r5, 0x17a
	ctx.r[5].s64 = 378;
	// 830AB0A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB0AC: 4BF25FAD  bl 0x82fd1058
	ctx.lr = 0x830AB0B0;
	sub_82FD1058(ctx, base);
	// 830AB0B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB0B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB0B8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB0BC: 48105B6D  bl 0x831b0c28
	ctx.lr = 0x830AB0C0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB0C0 size=68
    let mut pc: u32 = 0x830AB0C0;
    'dispatch: loop {
        match pc {
            0x830AB0C0 => {
    //   block [0x830AB0C0..0x830AB104)
	// 830AB0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB0C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB0CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB0D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB0D4: 4BFFC255  bl 0x830a7328
	ctx.lr = 0x830AB0D8;
	sub_830A7328(ctx, base);
	// 830AB0D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB0DC: 38800049  li r4, 0x49
	ctx.r[4].s64 = 73;
	// 830AB0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB0E4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AB0E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB0EC: 4E800421  bctrl
	ctx.lr = 0x830AB0F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB0F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AB0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB0FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB108 size=68
    let mut pc: u32 = 0x830AB108;
    'dispatch: loop {
        match pc {
            0x830AB108 => {
    //   block [0x830AB108..0x830AB14C)
	// 830AB108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB11C: 4BFFC20D  bl 0x830a7328
	ctx.lr = 0x830AB120;
	sub_830A7328(ctx, base);
	// 830AB120: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB124: 38800069  li r4, 0x69
	ctx.r[4].s64 = 105;
	// 830AB128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB12C: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 830AB130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB134: 4E800421  bctrl
	ctx.lr = 0x830AB138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AB13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB150 size=56
    let mut pc: u32 = 0x830AB150;
    'dispatch: loop {
        match pc {
            0x830AB150 => {
    //   block [0x830AB150..0x830AB188)
	// 830AB150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB15C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB160: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB164: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB168: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB16C: 38A0018C  li r5, 0x18c
	ctx.r[5].s64 = 396;
	// 830AB170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB174: 4BF25EE5  bl 0x82fd1058
	ctx.lr = 0x830AB178;
	sub_82FD1058(ctx, base);
	// 830AB178: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB17C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB180: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB184: 48105AA5  bl 0x831b0c28
	ctx.lr = 0x830AB188;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB188 size=56
    let mut pc: u32 = 0x830AB188;
    'dispatch: loop {
        match pc {
            0x830AB188 => {
    //   block [0x830AB188..0x830AB1C0)
	// 830AB188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB194: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB198: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB19C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB1A0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB1A4: 38A00192  li r5, 0x192
	ctx.r[5].s64 = 402;
	// 830AB1A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB1AC: 4BF25EAD  bl 0x82fd1058
	ctx.lr = 0x830AB1B0;
	sub_82FD1058(ctx, base);
	// 830AB1B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB1B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB1B8: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB1BC: 48105A6D  bl 0x831b0c28
	ctx.lr = 0x830AB1C0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB1C0 size=56
    let mut pc: u32 = 0x830AB1C0;
    'dispatch: loop {
        match pc {
            0x830AB1C0 => {
    //   block [0x830AB1C0..0x830AB1F8)
	// 830AB1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB1CC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB1D0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB1D4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB1D8: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB1DC: 38A00198  li r5, 0x198
	ctx.r[5].s64 = 408;
	// 830AB1E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB1E4: 4BF25E75  bl 0x82fd1058
	ctx.lr = 0x830AB1E8;
	sub_82FD1058(ctx, base);
	// 830AB1E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB1EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB1F0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB1F4: 48105A35  bl 0x831b0c28
	ctx.lr = 0x830AB1F8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB1F8 size=56
    let mut pc: u32 = 0x830AB1F8;
    'dispatch: loop {
        match pc {
            0x830AB1F8 => {
    //   block [0x830AB1F8..0x830AB230)
	// 830AB1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB204: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB208: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB20C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB210: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB214: 38A0019E  li r5, 0x19e
	ctx.r[5].s64 = 414;
	// 830AB218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB21C: 4BF25E3D  bl 0x82fd1058
	ctx.lr = 0x830AB220;
	sub_82FD1058(ctx, base);
	// 830AB220: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB224: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB228: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB22C: 481059FD  bl 0x831b0c28
	ctx.lr = 0x830AB230;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB230 size=56
    let mut pc: u32 = 0x830AB230;
    'dispatch: loop {
        match pc {
            0x830AB230 => {
    //   block [0x830AB230..0x830AB268)
	// 830AB230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB23C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB240: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB244: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB248: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB24C: 38A001A4  li r5, 0x1a4
	ctx.r[5].s64 = 420;
	// 830AB250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB254: 4BF25E05  bl 0x82fd1058
	ctx.lr = 0x830AB258;
	sub_82FD1058(ctx, base);
	// 830AB258: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB25C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB260: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB264: 481059C5  bl 0x831b0c28
	ctx.lr = 0x830AB268;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB268 size=56
    let mut pc: u32 = 0x830AB268;
    'dispatch: loop {
        match pc {
            0x830AB268 => {
    //   block [0x830AB268..0x830AB2A0)
	// 830AB268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB274: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB278: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB27C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB280: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB284: 38A001AA  li r5, 0x1aa
	ctx.r[5].s64 = 426;
	// 830AB288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB28C: 4BF25DCD  bl 0x82fd1058
	ctx.lr = 0x830AB290;
	sub_82FD1058(ctx, base);
	// 830AB290: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB294: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB298: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB29C: 4810598D  bl 0x831b0c28
	ctx.lr = 0x830AB2A0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB2A0 size=56
    let mut pc: u32 = 0x830AB2A0;
    'dispatch: loop {
        match pc {
            0x830AB2A0 => {
    //   block [0x830AB2A0..0x830AB2D8)
	// 830AB2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB2A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB2AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB2B0: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB2B4: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB2B8: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB2BC: 38A001B0  li r5, 0x1b0
	ctx.r[5].s64 = 432;
	// 830AB2C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB2C4: 4BF25D95  bl 0x82fd1058
	ctx.lr = 0x830AB2C8;
	sub_82FD1058(ctx, base);
	// 830AB2C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB2CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB2D0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB2D4: 48105955  bl 0x831b0c28
	ctx.lr = 0x830AB2D8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB2D8 size=56
    let mut pc: u32 = 0x830AB2D8;
    'dispatch: loop {
        match pc {
            0x830AB2D8 => {
    //   block [0x830AB2D8..0x830AB310)
	// 830AB2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB2E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB2E4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB2E8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB2EC: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB2F0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB2F4: 38A001B6  li r5, 0x1b6
	ctx.r[5].s64 = 438;
	// 830AB2F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB2FC: 4BF25D5D  bl 0x82fd1058
	ctx.lr = 0x830AB300;
	sub_82FD1058(ctx, base);
	// 830AB300: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB304: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB308: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB30C: 4810591D  bl 0x831b0c28
	ctx.lr = 0x830AB310;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB310 size=56
    let mut pc: u32 = 0x830AB310;
    'dispatch: loop {
        match pc {
            0x830AB310 => {
    //   block [0x830AB310..0x830AB348)
	// 830AB310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB31C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB320: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB324: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB328: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB32C: 38A001BC  li r5, 0x1bc
	ctx.r[5].s64 = 444;
	// 830AB330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB334: 4BF25D25  bl 0x82fd1058
	ctx.lr = 0x830AB338;
	sub_82FD1058(ctx, base);
	// 830AB338: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB33C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB340: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB344: 481058E5  bl 0x831b0c28
	ctx.lr = 0x830AB348;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB348 size=56
    let mut pc: u32 = 0x830AB348;
    'dispatch: loop {
        match pc {
            0x830AB348 => {
    //   block [0x830AB348..0x830AB380)
	// 830AB348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB354: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB358: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB35C: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB360: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB364: 38A001C2  li r5, 0x1c2
	ctx.r[5].s64 = 450;
	// 830AB368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB36C: 4BF25CED  bl 0x82fd1058
	ctx.lr = 0x830AB370;
	sub_82FD1058(ctx, base);
	// 830AB370: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB374: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB378: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB37C: 481058AD  bl 0x831b0c28
	ctx.lr = 0x830AB380;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB380 size=76
    let mut pc: u32 = 0x830AB380;
    'dispatch: loop {
        match pc {
            0x830AB380 => {
    //   block [0x830AB380..0x830AB3CC)
	// 830AB380: 2F040063  cmpwi cr6, r4, 0x63
	ctx.cr[6].compare_i32(ctx.r[4].s32, 99, &mut ctx.xer);
	// 830AB384: 419900BC  bgt cr6, 0x830ab440
	if ctx.cr[6].gt {
		sub_830AB440(ctx, base);
		return;
	}
	// 830AB388: 419A00A0  beq cr6, 0x830ab428
	if ctx.cr[6].eq {
		sub_830AB428(ctx, base);
		return;
	}
	// 830AB38C: 2F040043  cmpwi cr6, r4, 0x43
	ctx.cr[6].compare_i32(ctx.r[4].s32, 67, &mut ctx.xer);
	// 830AB390: 419A0080  beq cr6, 0x830ab410
	if ctx.cr[6].eq {
		sub_830AB410(ctx, base);
		return;
	}
	// 830AB394: 2F040044  cmpwi cr6, r4, 0x44
	ctx.cr[6].compare_i32(ctx.r[4].s32, 68, &mut ctx.xer);
	// 830AB398: 419A0060  beq cr6, 0x830ab3f8
	if ctx.cr[6].eq {
		sub_830AB3F8(ctx, base);
		return;
	}
	// 830AB39C: 2F040049  cmpwi cr6, r4, 0x49
	ctx.cr[6].compare_i32(ctx.r[4].s32, 73, &mut ctx.xer);
	// 830AB3A0: 419A0040  beq cr6, 0x830ab3e0
	if ctx.cr[6].eq {
		sub_830AB3E0(ctx, base);
		return;
	}
	// 830AB3A4: 2F040053  cmpwi cr6, r4, 0x53
	ctx.cr[6].compare_i32(ctx.r[4].s32, 83, &mut ctx.xer);
	// 830AB3A8: 419A0024  beq cr6, 0x830ab3cc
	if ctx.cr[6].eq {
		sub_830AB3CC(ctx, base);
		return;
	}
	// 830AB3AC: 2F040057  cmpwi cr6, r4, 0x57
	ctx.cr[6].compare_i32(ctx.r[4].s32, 87, &mut ctx.xer);
	// 830AB3B0: 409A00B0  bne cr6, 0x830ab460
	if !ctx.cr[6].eq {
		sub_830AB440(ctx, base);
		return;
	}
	// 830AB3B4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB3B8: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB3BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AB3C0: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB3C4: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 830AB3C8: 4BFF8870  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB3CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB3CC size=20
    let mut pc: u32 = 0x830AB3CC;
    'dispatch: loop {
        match pc {
            0x830AB3CC => {
    //   block [0x830AB3CC..0x830AB3E0)
	// 830AB3CC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB3D0: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB3D4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AB3D8: 388B0330  addi r4, r11, 0x330
	ctx.r[4].s64 = ctx.r[11].s64 + 816;
	// 830AB3DC: 4BFF885C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB3E0 size=24
    let mut pc: u32 = 0x830AB3E0;
    'dispatch: loop {
        match pc {
            0x830AB3E0 => {
    //   block [0x830AB3E0..0x830AB3F8)
	// 830AB3E0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB3E4: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB3E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AB3EC: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB3F0: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 830AB3F4: 4BFF8844  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB3F8 size=24
    let mut pc: u32 = 0x830AB3F8;
    'dispatch: loop {
        match pc {
            0x830AB3F8 => {
    //   block [0x830AB3F8..0x830AB410)
	// 830AB3F8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB3FC: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB400: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AB404: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB408: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 830AB40C: 4BFF882C  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB410 size=24
    let mut pc: u32 = 0x830AB410;
    'dispatch: loop {
        match pc {
            0x830AB410 => {
    //   block [0x830AB410..0x830AB428)
	// 830AB410: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB414: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB418: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AB41C: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB420: 388B0048  addi r4, r11, 0x48
	ctx.r[4].s64 = ctx.r[11].s64 + 72;
	// 830AB424: 4BFF8814  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB428 size=24
    let mut pc: u32 = 0x830AB428;
    'dispatch: loop {
        match pc {
            0x830AB428 => {
    //   block [0x830AB428..0x830AB440)
	// 830AB428: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB42C: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB430: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AB434: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB438: 388B0048  addi r4, r11, 0x48
	ctx.r[4].s64 = ctx.r[11].s64 + 72;
	// 830AB43C: 4BFF87FC  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB440 size=40
    let mut pc: u32 = 0x830AB440;
    'dispatch: loop {
        match pc {
            0x830AB440 => {
    //   block [0x830AB440..0x830AB468)
	// 830AB440: 2F040064  cmpwi cr6, r4, 0x64
	ctx.cr[6].compare_i32(ctx.r[4].s32, 100, &mut ctx.xer);
	// 830AB444: 419A0068  beq cr6, 0x830ab4ac
	if ctx.cr[6].eq {
		sub_830AB4AC(ctx, base);
		return;
	}
	// 830AB448: 2F040069  cmpwi cr6, r4, 0x69
	ctx.cr[6].compare_i32(ctx.r[4].s32, 105, &mut ctx.xer);
	// 830AB44C: 419A0048  beq cr6, 0x830ab494
	if ctx.cr[6].eq {
		sub_830AB494(ctx, base);
		return;
	}
	// 830AB450: 2F040073  cmpwi cr6, r4, 0x73
	ctx.cr[6].compare_i32(ctx.r[4].s32, 115, &mut ctx.xer);
	// 830AB454: 419A002C  beq cr6, 0x830ab480
	if ctx.cr[6].eq {
		sub_830AB480(ctx, base);
		return;
	}
	// 830AB458: 2F040077  cmpwi cr6, r4, 0x77
	ctx.cr[6].compare_i32(ctx.r[4].s32, 119, &mut ctx.xer);
	// 830AB45C: 419A000C  beq cr6, 0x830ab468
	if ctx.cr[6].eq {
		sub_830AB468(ctx, base);
		return;
	}
	// 830AB460: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AB464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB468 size=24
    let mut pc: u32 = 0x830AB468;
    'dispatch: loop {
        match pc {
            0x830AB468 => {
    //   block [0x830AB468..0x830AB480)
	// 830AB468: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB46C: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB470: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AB474: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB478: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 830AB47C: 4BFF87BC  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB480 size=20
    let mut pc: u32 = 0x830AB480;
    'dispatch: loop {
        match pc {
            0x830AB480 => {
    //   block [0x830AB480..0x830AB494)
	// 830AB480: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB484: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AB48C: 388B0330  addi r4, r11, 0x330
	ctx.r[4].s64 = ctx.r[11].s64 + 816;
	// 830AB490: 4BFF87A8  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB494 size=24
    let mut pc: u32 = 0x830AB494;
    'dispatch: loop {
        match pc {
            0x830AB494 => {
    //   block [0x830AB494..0x830AB4AC)
	// 830AB494: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB498: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AB4A0: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB4A4: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 830AB4A8: 4BFF8790  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB4AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB4AC size=24
    let mut pc: u32 = 0x830AB4AC;
    'dispatch: loop {
        match pc {
            0x830AB4AC => {
    //   block [0x830AB4AC..0x830AB4C4)
	// 830AB4AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB4B0: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AB4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AB4B8: 396B0330  addi r11, r11, 0x330
	ctx.r[11].s64 = ctx.r[11].s64 + 816;
	// 830AB4BC: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 830AB4C0: 4BFF8778  b 0x830a3c38
	sub_830A3C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB4C8 size=304
    let mut pc: u32 = 0x830AB4C8;
    'dispatch: loop {
        match pc {
            0x830AB4C8 => {
    //   block [0x830AB4C8..0x830AB5F8)
	// 830AB4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB4D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB4D4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB4D8: A1630020  lhz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AB4DC: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 830AB4E0: 419A0030  beq cr6, 0x830ab510
	if ctx.cr[6].eq {
	pc = 0x830AB510; continue 'dispatch;
	}
	// 830AB4E4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB4E8: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB4EC: 38C0007A  li r6, 0x7a
	ctx.r[6].s64 = 122;
	// 830AB4F0: 388B0448  addi r4, r11, 0x448
	ctx.r[4].s64 = ctx.r[11].s64 + 1096;
	// 830AB4F4: 38A001F1  li r5, 0x1f1
	ctx.r[5].s64 = 497;
	// 830AB4F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AB4FC: 4BFFBC8D  bl 0x830a7188
	ctx.lr = 0x830AB500;
	sub_830A7188(ctx, base);
	// 830AB500: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB504: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AB508: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AB50C: 4810571D  bl 0x831b0c28
	ctx.lr = 0x830AB510;
	sub_831B0C28(ctx, base);
	// 830AB510: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 830AB514: 2F0B006E  cmpwi cr6, r11, 0x6e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 110, &mut ctx.xer);
	// 830AB518: 41990048  bgt cr6, 0x830ab560
	if ctx.cr[6].gt {
	pc = 0x830AB560; continue 'dispatch;
	}
	// 830AB51C: 419A003C  beq cr6, 0x830ab558
	if ctx.cr[6].eq {
	pc = 0x830AB558; continue 'dispatch;
	}
	// 830AB520: 2F0B0028  cmpwi cr6, r11, 0x28
	ctx.cr[6].compare_i32(ctx.r[11].s32, 40, &mut ctx.xer);
	// 830AB524: 4198005C  blt cr6, 0x830ab580
	if ctx.cr[6].lt {
	pc = 0x830AB580; continue 'dispatch;
	}
	// 830AB528: 2F0B002B  cmpwi cr6, r11, 0x2b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 43, &mut ctx.xer);
	// 830AB52C: 409900B4  ble cr6, 0x830ab5e0
	if !ctx.cr[6].gt {
	pc = 0x830AB5E0; continue 'dispatch;
	}
	// 830AB530: 2F0B002C  cmpwi cr6, r11, 0x2c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 44, &mut ctx.xer);
	// 830AB534: 4099004C  ble cr6, 0x830ab580
	if !ctx.cr[6].gt {
	pc = 0x830AB580; continue 'dispatch;
	}
	// 830AB538: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 830AB53C: 409900A4  ble cr6, 0x830ab5e0
	if !ctx.cr[6].gt {
	pc = 0x830AB5E0; continue 'dispatch;
	}
	// 830AB540: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 830AB544: 419A009C  beq cr6, 0x830ab5e0
	if ctx.cr[6].eq {
	pc = 0x830AB5E0; continue 'dispatch;
	}
	// 830AB548: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 830AB54C: 40990034  ble cr6, 0x830ab580
	if !ctx.cr[6].gt {
	pc = 0x830AB580; continue 'dispatch;
	}
	// 830AB550: 2F0B005E  cmpwi cr6, r11, 0x5e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 94, &mut ctx.xer);
	// 830AB554: 48000028  b 0x830ab57c
	pc = 0x830AB57C; continue 'dispatch;
	// 830AB558: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 830AB55C: 48000084  b 0x830ab5e0
	pc = 0x830AB5E0; continue 'dispatch;
	// 830AB560: 2F0B0072  cmpwi cr6, r11, 0x72
	ctx.cr[6].compare_i32(ctx.r[11].s32, 114, &mut ctx.xer);
	// 830AB564: 419A0078  beq cr6, 0x830ab5dc
	if ctx.cr[6].eq {
	pc = 0x830AB5DC; continue 'dispatch;
	}
	// 830AB568: 2F0B0074  cmpwi cr6, r11, 0x74
	ctx.cr[6].compare_i32(ctx.r[11].s32, 116, &mut ctx.xer);
	// 830AB56C: 419A0068  beq cr6, 0x830ab5d4
	if ctx.cr[6].eq {
	pc = 0x830AB5D4; continue 'dispatch;
	}
	// 830AB570: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 830AB574: 4099000C  ble cr6, 0x830ab580
	if !ctx.cr[6].gt {
	pc = 0x830AB580; continue 'dispatch;
	}
	// 830AB578: 2F0B007D  cmpwi cr6, r11, 0x7d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 125, &mut ctx.xer);
	// 830AB57C: 40990064  ble cr6, 0x830ab5e0
	if !ctx.cr[6].gt {
	pc = 0x830AB5E0; continue 'dispatch;
	}
	// 830AB580: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB584: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830AB588: B1610062  sth r11, 0x62(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(98 as u32), ctx.r[11].u16 ) };
	// 830AB58C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AB590: 3BE0005C  li r31, 0x5c
	ctx.r[31].s64 = 92;
	// 830AB594: 388A0448  addi r4, r10, 0x448
	ctx.r[4].s64 = ctx.r[10].s64 + 1096;
	// 830AB598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AB59C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830AB5A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AB5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830AB5A8: B1610064  sth r11, 0x64(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u16 ) };
	// 830AB5AC: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 830AB5B0: B3E10060  sth r31, 0x60(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u16 ) };
	// 830AB5B4: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 830AB5B8: 38A00211  li r5, 0x211
	ctx.r[5].s64 = 529;
	// 830AB5BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AB5C0: 4BFDCC49  bl 0x83088208
	ctx.lr = 0x830AB5C4;
	sub_83088208(ctx, base);
	// 830AB5C4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB5C8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830AB5CC: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 830AB5D0: 48105659  bl 0x831b0c28
	ctx.lr = 0x830AB5D4;
	sub_831B0C28(ctx, base);
	// 830AB5D4: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 830AB5D8: 48000008  b 0x830ab5e0
	pc = 0x830AB5E0; continue 'dispatch;
	// 830AB5DC: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 830AB5E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830AB5E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830AB5E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB5EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB5F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB5F8 size=88
    let mut pc: u32 = 0x830AB5F8;
    'dispatch: loop {
        match pc {
            0x830AB5F8 => {
    //   block [0x830AB5F8..0x830AB650)
	// 830AB5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AB604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB60C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB614: 396B03C8  addi r11, r11, 0x3c8
	ctx.r[11].s64 = ctx.r[11].s64 + 968;
	// 830AB618: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AB61C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB620: 4BFFBC59  bl 0x830a7278
	ctx.lr = 0x830AB624;
	sub_830A7278(ctx, base);
	// 830AB624: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AB628: 4182000C  beq 0x830ab634
	if ctx.cr[0].eq {
	pc = 0x830AB634; continue 'dispatch;
	}
	// 830AB62C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB630: 4BF2CCB1  bl 0x82fd82e0
	ctx.lr = 0x830AB634;
	sub_82FD82E0(ctx, base);
	// 830AB634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB638: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AB63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB644: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AB648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB650 size=40
    let mut pc: u32 = 0x830AB650;
    'dispatch: loop {
        match pc {
            0x830AB650 => {
    //   block [0x830AB650..0x830AB678)
	// 830AB650: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB654: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 830AB658: 394B0488  addi r10, r11, 0x488
	ctx.r[10].s64 = ctx.r[11].s64 + 1160;
	// 830AB65C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AB660: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AB664: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830AB668: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AB66C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830AB670: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AB674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB678 size=100
    let mut pc: u32 = 0x830AB678;
    'dispatch: loop {
        match pc {
            0x830AB678 => {
    //   block [0x830AB678..0x830AB6DC)
	// 830AB678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB68C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AB690: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AB694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB698: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB69C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB6A0: 4E800421  bctrl
	ctx.lr = 0x830AB6A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB6A4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AB6A8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AB6AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB6B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB6B8: 4E800421  bctrl
	ctx.lr = 0x830AB6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB6BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AB6C0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830AB6C4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AB6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AB6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB6D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB6E0 size=192
    let mut pc: u32 = 0x830AB6E0;
    'dispatch: loop {
        match pc {
            0x830AB6E0 => {
    //   block [0x830AB6E0..0x830AB7A0)
	// 830AB6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB6E4: 480FCA89  bl 0x831a816c
	ctx.lr = 0x830AB6E8;
	sub_831A8130(ctx, base);
	// 830AB6E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB6EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB6F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AB6F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB6F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AB6FC: 40990010  ble cr6, 0x830ab70c
	if !ctx.cr[6].gt {
	pc = 0x830AB70C; continue 'dispatch;
	}
	// 830AB700: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB704: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830AB708: 40980050  bge cr6, 0x830ab758
	if !ctx.cr[6].lt {
	pc = 0x830AB758; continue 'dispatch;
	}
	// 830AB70C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB710: 4BFFFF69  bl 0x830ab678
	ctx.lr = 0x830AB714;
	sub_830AB678(ctx, base);
	// 830AB714: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AB718: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830AB71C: 57DD103A  slwi r29, r30, 2
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 830AB720: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AB724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB728: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB730: 4E800421  bctrl
	ctx.lr = 0x830AB734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB734: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AB738: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830AB73C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AB740: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830AB744: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB748: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB74C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB750: 4E800421  bctrl
	ctx.lr = 0x830AB754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB754: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830AB758: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AB760: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830AB764: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AB768: 40990030  ble cr6, 0x830ab798
	if !ctx.cr[6].gt {
	pc = 0x830AB798; continue 'dispatch;
	}
	// 830AB76C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AB770: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830AB774: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AB778: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AB77C: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 830AB780: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AB784: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 830AB788: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830AB78C: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB790: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830AB794: 4198FFE0  blt cr6, 0x830ab774
	if ctx.cr[6].lt {
	pc = 0x830AB774; continue 'dispatch;
	}
	// 830AB798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AB79C: 480FCA20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB7A0 size=88
    let mut pc: u32 = 0x830AB7A0;
    'dispatch: loop {
        match pc {
            0x830AB7A0 => {
    //   block [0x830AB7A0..0x830AB7F8)
	// 830AB7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB7A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AB7AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB7B4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB7B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB7BC: 396B0488  addi r11, r11, 0x488
	ctx.r[11].s64 = ctx.r[11].s64 + 1160;
	// 830AB7C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AB7C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB7C8: 4BFFFEB1  bl 0x830ab678
	ctx.lr = 0x830AB7CC;
	sub_830AB678(ctx, base);
	// 830AB7CC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AB7D0: 4182000C  beq 0x830ab7dc
	if ctx.cr[0].eq {
	pc = 0x830AB7DC; continue 'dispatch;
	}
	// 830AB7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB7D8: 4BF2CB09  bl 0x82fd82e0
	ctx.lr = 0x830AB7DC;
	sub_82FD82E0(ctx, base);
	// 830AB7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB7E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AB7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB7EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AB7F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB7F8 size=144
    let mut pc: u32 = 0x830AB7F8;
    'dispatch: loop {
        match pc {
            0x830AB7F8 => {
    //   block [0x830AB7F8..0x830AB888)
	// 830AB7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB800: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB80C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AB810: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB814: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB818: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB81C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB820: 4E800421  bctrl
	ctx.lr = 0x830AB824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB824: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AB828: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AB82C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB830: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB838: 4E800421  bctrl
	ctx.lr = 0x830AB83C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB83C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AB840: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AB844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB848: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB850: 4E800421  bctrl
	ctx.lr = 0x830AB854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB854: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AB858: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AB85C: 41820018  beq 0x830ab874
	if ctx.cr[0].eq {
	pc = 0x830AB874; continue 'dispatch;
	}
	// 830AB860: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB864: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AB868: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB86C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB870: 4E800421  bctrl
	ctx.lr = 0x830AB874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AB878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AB87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AB880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AB884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB888 size=16
    let mut pc: u32 = 0x830AB888;
    'dispatch: loop {
        match pc {
            0x830AB888 => {
    //   block [0x830AB888..0x830AB898)
	// 830AB888: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB88C: 396B04A4  addi r11, r11, 0x4a4
	ctx.r[11].s64 = ctx.r[11].s64 + 1188;
	// 830AB890: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB894: 4BFFFF64  b 0x830ab7f8
	sub_830AB7F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB898 size=76
    let mut pc: u32 = 0x830AB898;
    'dispatch: loop {
        match pc {
            0x830AB898 => {
    //   block [0x830AB898..0x830AB8E4)
	// 830AB898: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB89C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AB8A0: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830AB8A4: 40810038  ble 0x830ab8dc
	if !ctx.cr[0].gt {
	pc = 0x830AB8DC; continue 'dispatch;
	}
	// 830AB8A8: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AB8AC: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 830AB8B0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB8B4: 552607FE  clrlwi r6, r9, 0x1f
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 830AB8B8: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 830AB8BC: 409A0010  bne cr6, 0x830ab8cc
	if !ctx.cr[6].eq {
	pc = 0x830AB8CC; continue 'dispatch;
	}
	// 830AB8C0: 7129000D  andi. r9, r9, 0xd
	ctx.r[9].u64 = ctx.r[9].u64 & 13;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830AB8C4: 2F09000D  cmpwi cr6, r9, 0xd
	ctx.cr[6].compare_i32(ctx.r[9].s32, 13, &mut ctx.xer);
	// 830AB8C8: 409A001C  bne cr6, 0x830ab8e4
	if !ctx.cr[6].eq {
		sub_830AB8E4(ctx, base);
		return;
	}
	// 830AB8CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AB8D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830AB8D4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 830AB8D8: 4198FFD8  blt cr6, 0x830ab8b0
	if ctx.cr[6].lt {
	pc = 0x830AB8B0; continue 'dispatch;
	}
	// 830AB8DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AB8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB8E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB8E4 size=12
    let mut pc: u32 = 0x830AB8E4;
    'dispatch: loop {
        match pc {
            0x830AB8E4 => {
    //   block [0x830AB8E4..0x830AB8F0)
	// 830AB8E4: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AB8E8: 7C6B382E  lwzx r3, r11, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830AB8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB8F0 size=56
    let mut pc: u32 = 0x830AB8F0;
    'dispatch: loop {
        match pc {
            0x830AB8F0 => {
    //   block [0x830AB8F0..0x830AB928)
	// 830AB8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB8FC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB900: 80E30020  lwz r7, 0x20(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AB904: 38C00126  li r6, 0x126
	ctx.r[6].s64 = 294;
	// 830AB908: 388B04C0  addi r4, r11, 0x4c0
	ctx.r[4].s64 = ctx.r[11].s64 + 1216;
	// 830AB90C: 38A00186  li r5, 0x186
	ctx.r[5].s64 = 390;
	// 830AB910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB914: 4BF25745  bl 0x82fd1058
	ctx.lr = 0x830AB918;
	sub_82FD1058(ctx, base);
	// 830AB918: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AB91C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AB920: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830AB924: 48105305  bl 0x831b0c28
	ctx.lr = 0x830AB928;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AB928 size=16
    let mut pc: u32 = 0x830AB928;
    'dispatch: loop {
        match pc {
            0x830AB928 => {
    //   block [0x830AB928..0x830AB938)
	// 830AB928: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB92C: 396B0508  addi r11, r11, 0x508
	ctx.r[11].s64 = ctx.r[11].s64 + 1288;
	// 830AB930: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB938 size=128
    let mut pc: u32 = 0x830AB938;
    'dispatch: loop {
        match pc {
            0x830AB938 => {
    //   block [0x830AB938..0x830AB9B8)
	// 830AB938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB93C: 480FC831  bl 0x831a816c
	ctx.lr = 0x830AB940;
	sub_831A8130(ctx, base);
	// 830AB940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB944: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB94C: 396B0508  addi r11, r11, 0x508
	ctx.r[11].s64 = ctx.r[11].s64 + 1288;
	// 830AB950: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AB954: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AB958: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830AB95C: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 830AB960: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 830AB964: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB968: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830AB96C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830AB970: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 830AB974: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830AB978: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AB97C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AB980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AB984: 4E800421  bctrl
	ctx.lr = 0x830AB988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AB988: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AB98C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830AB990: 419A001C  beq cr6, 0x830ab9ac
	if ctx.cr[6].eq {
	pc = 0x830AB9AC; continue 'dispatch;
	}
	// 830AB994: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830AB998: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AB99C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830AB9A0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 830AB9A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830AB9A8: 4082FFF0  bne 0x830ab998
	if !ctx.cr[0].eq {
	pc = 0x830AB998; continue 'dispatch;
	}
	// 830AB9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB9B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AB9B4: 480FC808  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AB9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AB9B8 size=88
    let mut pc: u32 = 0x830AB9B8;
    'dispatch: loop {
        match pc {
            0x830AB9B8 => {
    //   block [0x830AB9B8..0x830ABA10)
	// 830AB9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AB9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AB9C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AB9C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AB9C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AB9CC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AB9D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AB9D4: 396B04A4  addi r11, r11, 0x4a4
	ctx.r[11].s64 = ctx.r[11].s64 + 1188;
	// 830AB9D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AB9DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AB9E0: 4BFFFE19  bl 0x830ab7f8
	ctx.lr = 0x830AB9E4;
	sub_830AB7F8(ctx, base);
	// 830AB9E4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AB9E8: 4182000C  beq 0x830ab9f4
	if ctx.cr[0].eq {
	pc = 0x830AB9F4; continue 'dispatch;
	}
	// 830AB9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB9F0: 4BF2C8F1  bl 0x82fd82e0
	ctx.lr = 0x830AB9F4;
	sub_82FD82E0(ctx, base);
	// 830AB9F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AB9F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AB9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ABA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ABA04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ABA08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ABA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ABA10 size=8
    let mut pc: u32 = 0x830ABA10;
    'dispatch: loop {
        match pc {
            0x830ABA10 => {
    //   block [0x830ABA10..0x830ABA18)
	// 830ABA10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ABA14: 82180548  lwz r16, 0x548(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(1352 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABA18 size=352
    let mut pc: u32 = 0x830ABA18;
    'dispatch: loop {
        match pc {
            0x830ABA18 => {
    //   block [0x830ABA18..0x830ABB78)
	// 830ABA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABA1C: 480FC74D  bl 0x831a8168
	ctx.lr = 0x830ABA20;
	sub_831A8130(ctx, base);
	// 830ABA20: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830ABA24: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABA28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830ABA2C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830ABA30: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830ABA34: 419A013C  beq cr6, 0x830abb70
	if ctx.cr[6].eq {
	pc = 0x830ABB70; continue 'dispatch;
	}
	// 830ABA38: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ABA3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ABA40: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830ABA44: 4182000C  beq 0x830aba50
	if ctx.cr[0].eq {
	pc = 0x830ABA50; continue 'dispatch;
	}
	// 830ABA48: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABA4C: 48000008  b 0x830aba54
	pc = 0x830ABA54; continue 'dispatch;
	// 830ABA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ABA54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ABA58: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830ABA5C: 419A0114  beq cr6, 0x830abb70
	if ctx.cr[6].eq {
	pc = 0x830ABB70; continue 'dispatch;
	}
	// 830ABA60: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830ABA64: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABA68: 4BF2C831  bl 0x82fd8298
	ctx.lr = 0x830ABA6C;
	sub_82FD8298(ctx, base);
	// 830ABA6C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830ABA70: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830ABA74: 4182002C  beq 0x830abaa0
	if ctx.cr[0].eq {
	pc = 0x830ABAA0; continue 'dispatch;
	}
	// 830ABA78: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830ABA7C: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABA80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ABA84: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABA88: 4BFFFEB1  bl 0x830ab938
	ctx.lr = 0x830ABA8C;
	sub_830AB938(ctx, base);
	// 830ABA8C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830ABA90: 394B0520  addi r10, r11, 0x520
	ctx.r[10].s64 = ctx.r[11].s64 + 1312;
	// 830ABA94: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830ABA98: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830ABA9C: 48000008  b 0x830abaa4
	pc = 0x830ABAA4; continue 'dispatch;
	// 830ABAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ABAA4: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABAA8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830ABAAC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABAB0: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830ABAB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ABAB8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABABC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ABAC0: 4E800421  bctrl
	ctx.lr = 0x830ABAC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ABAC4: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABAC8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABACC: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830ABAD0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830ABAD4: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830ABAD8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ABADC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABAE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ABAE4: 4E800421  bctrl
	ctx.lr = 0x830ABAE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ABAE8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABAEC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABAF0: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830ABAF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830ABAF8: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830ABAFC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ABB00: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ABB08: 4E800421  bctrl
	ctx.lr = 0x830ABB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ABB0C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABB10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830ABB14: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830ABB18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ABB1C: 40990054  ble cr6, 0x830abb70
	if !ctx.cr[6].gt {
	pc = 0x830ABB70; continue 'dispatch;
	}
	// 830ABB20: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830ABB24: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABB28: 4BF2C771  bl 0x82fd8298
	ctx.lr = 0x830ABB2C;
	sub_82FD8298(ctx, base);
	// 830ABB2C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830ABB30: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 830ABB34: 41820020  beq 0x830abb54
	if ctx.cr[0].eq {
	pc = 0x830ABB54; continue 'dispatch;
	}
	// 830ABB38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830ABB3C: 80BE0020  lwz r5, 0x20(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABB40: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830ABB44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ABB48: 4BFBCC59  bl 0x830687a0
	ctx.lr = 0x830ABB4C;
	sub_830687A0(ctx, base);
	// 830ABB4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830ABB50: 48000008  b 0x830abb58
	pc = 0x830ABB58; continue 'dispatch;
	// 830ABB54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ABB58: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ABB5C: 4BF8F5F5  bl 0x8303b150
	ctx.lr = 0x830ABB60;
	sub_8303B150(ctx, base);
	// 830ABB60: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABB64: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830ABB68: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830ABB6C: 4198FFB4  blt cr6, 0x830abb20
	if ctx.cr[6].lt {
	pc = 0x830ABB20; continue 'dispatch;
	}
	// 830ABB70: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830ABB74: 480FC644  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABB78 size=48
    let mut pc: u32 = 0x830ABB78;
    'dispatch: loop {
        match pc {
            0x830ABB78 => {
    //   block [0x830ABB78..0x830ABBA8)
	// 830ABB78: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830ABB7C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABB80: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ABB84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABB88: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830ABB8C: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABB90: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ABB94: 4BF2C74D  bl 0x82fd82e0
	ctx.lr = 0x830ABB98;
	sub_82FD82E0(ctx, base);
	// 830ABB98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ABB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ABBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ABBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABBA8 size=48
    let mut pc: u32 = 0x830ABBA8;
    'dispatch: loop {
        match pc {
            0x830ABBA8 => {
    //   block [0x830ABBA8..0x830ABBD8)
	// 830ABBA8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830ABBAC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABBB0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ABBB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABBB8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830ABBBC: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABBC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ABBC4: 4BF2C71D  bl 0x82fd82e0
	ctx.lr = 0x830ABBC8;
	sub_82FD82E0(ctx, base);
	// 830ABBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ABBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ABBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ABBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABBD8 size=108
    let mut pc: u32 = 0x830ABBD8;
    'dispatch: loop {
        match pc {
            0x830ABBD8 => {
    //   block [0x830ABBD8..0x830ABC44)
	// 830ABBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABBDC: 480FC58D  bl 0x831a8168
	ctx.lr = 0x830ABBE0;
	sub_831A8130(ctx, base);
	// 830ABBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABBE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830ABBE8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830ABBEC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830ABBF0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABBF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ABBF8: 40990044  ble cr6, 0x830abc3c
	if !ctx.cr[6].gt {
	pc = 0x830ABC3C; continue 'dispatch;
	}
	// 830ABBFC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830ABC00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830ABC04: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ABC08: 4BF80C69  bl 0x8302c870
	ctx.lr = 0x830ABC0C;
	sub_8302C870(ctx, base);
	// 830ABC0C: 93830004  stw r28, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830ABC10: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABC14: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830ABC18: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 830ABC1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ABC20: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 830ABC24: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABC28: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 830ABC2C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830ABC30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABC34: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830ABC38: 4198FFC8  blt cr6, 0x830abc00
	if ctx.cr[6].lt {
	pc = 0x830ABC00; continue 'dispatch;
	}
	// 830ABC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830ABC40: 480FC578  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABC48 size=264
    let mut pc: u32 = 0x830ABC48;
    'dispatch: loop {
        match pc {
            0x830ABC48 => {
    //   block [0x830ABC48..0x830ABD50)
	// 830ABC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABC4C: 480FC519  bl 0x831a8164
	ctx.lr = 0x830ABC50;
	sub_831A8130(ctx, base);
	// 830ABC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830ABC58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830ABC5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830ABC60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830ABC64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABC68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ABC6C: 409900DC  ble cr6, 0x830abd48
	if !ctx.cr[6].gt {
	pc = 0x830ABD48; continue 'dispatch;
	}
	// 830ABC70: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830ABC74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830ABC78: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ABC7C: 4BF80BF5  bl 0x8302c870
	ctx.lr = 0x830ABC80;
	sub_8302C870(ctx, base);
	// 830ABC80: 4BFA9891  bl 0x83055510
	ctx.lr = 0x830ABC84;
	sub_83055510(ctx, base);
	// 830ABC84: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABC88: 7C7E592E  stwx r3, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 830ABC8C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ABC90: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABC94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ABC98: 40990014  ble cr6, 0x830abcac
	if !ctx.cr[6].gt {
	pc = 0x830ABCAC; continue 'dispatch;
	}
	// 830ABC9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ABCA0: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABCA4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 830ABCA8: 48000088  b 0x830abd30
	pc = 0x830ABD30; continue 'dispatch;
	// 830ABCAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ABCB0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830ABCB4: 40990030  ble cr6, 0x830abce4
	if !ctx.cr[6].gt {
	pc = 0x830ABCE4; continue 'dispatch;
	}
	// 830ABCB8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABCBC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ABCC0: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 830ABCC4: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 830ABCC8: 419A0014  beq cr6, 0x830abcdc
	if ctx.cr[6].eq {
	pc = 0x830ABCDC; continue 'dispatch;
	}
	// 830ABCCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830ABCD0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830ABCD4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830ABCD8: 4198FFE4  blt cr6, 0x830abcbc
	if ctx.cr[6].lt {
	pc = 0x830ABCBC; continue 'dispatch;
	}
	// 830ABCDC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830ABCE0: 41980054  blt cr6, 0x830abd34
	if ctx.cr[6].lt {
	pc = 0x830ABD34; continue 'dispatch;
	}
	// 830ABCE4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABCE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830ABCEC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830ABCF0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830ABCF4: 41820040  beq 0x830abd34
	if ctx.cr[0].eq {
	pc = 0x830ABD34; continue 'dispatch;
	}
	// 830ABCF8: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 830ABCFC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830ABD00: 419A0034  beq cr6, 0x830abd34
	if ctx.cr[6].eq {
	pc = 0x830ABD34; continue 'dispatch;
	}
	// 830ABD04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ABD08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830ABD0C: 815C0030  lwz r10, 0x30(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 830ABD10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ABD14: 80BC0020  lwz r5, 0x20(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 830ABD18: 554607FE  clrlwi r6, r10, 0x1f
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830ABD1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ABD20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ABD24: 4E800421  bctrl
	ctx.lr = 0x830ABD28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ABD28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABD2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830ABD30: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830ABD34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABD38: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830ABD3C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830ABD40: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830ABD44: 4198FF30  blt cr6, 0x830abc74
	if ctx.cr[6].lt {
	pc = 0x830ABC74; continue 'dispatch;
	}
	// 830ABD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830ABD4C: 480FC468  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ABD50 size=8
    let mut pc: u32 = 0x830ABD50;
    'dispatch: loop {
        match pc {
            0x830ABD50 => {
    //   block [0x830ABD50..0x830ABD58)
	// 830ABD50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ABD54: 821805DC  lwz r16, 0x5dc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(1500 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABD58 size=128
    let mut pc: u32 = 0x830ABD58;
    'dispatch: loop {
        match pc {
            0x830ABD58 => {
    //   block [0x830ABD58..0x830ABDD8)
	// 830ABD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ABD60: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830ABD64: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830ABD68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830ABD6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830ABD70: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830ABD74: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABD78: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830ABD7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830ABD80: 394B04A4  addi r10, r11, 0x4a4
	ctx.r[10].s64 = ctx.r[11].s64 + 1188;
	// 830ABD84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ABD88: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830ABD8C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830ABD90: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830ABD94: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830ABD98: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830ABD9C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830ABDA0: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830ABDA4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830ABDA8: 90BE001C  stw r5, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 830ABDAC: 90DE0020  stw r6, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 830ABDB0: 4BFFFC69  bl 0x830aba18
	ctx.lr = 0x830ABDB4;
	sub_830ABA18(ctx, base);
	// 830ABDB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ABDB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ABDBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830ABDC0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830ABDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ABDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ABDCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ABDD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ABDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABDE0 size=24
    let mut pc: u32 = 0x830ABDE0;
    'dispatch: loop {
        match pc {
            0x830ABDE0 => {
    //   block [0x830ABDE0..0x830ABDF8)
	// 830ABDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ABDE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABDEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ABDF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ABDF4: 48104E35  bl 0x831b0c28
	ctx.lr = 0x830ABDF8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABE00 size=36
    let mut pc: u32 = 0x830ABE00;
    'dispatch: loop {
        match pc {
            0x830ABE00 => {
    //   block [0x830ABE00..0x830ABE24)
	// 830ABE00: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830ABE04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABE08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ABE0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABE10: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830ABE14: 4BFFF9E5  bl 0x830ab7f8
	ctx.lr = 0x830ABE18;
	sub_830AB7F8(ctx, base);
	// 830ABE18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ABE1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ABE20: 48104E09  bl 0x831b0c28
	ctx.lr = 0x830ABE24;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ABE28 size=8
    let mut pc: u32 = 0x830ABE28;
    'dispatch: loop {
        match pc {
            0x830ABE28 => {
    //   block [0x830ABE28..0x830ABE30)
	// 830ABE28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ABE2C: 82180618  lwz r16, 0x618(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(1560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ABE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ABE30 size=1092
    let mut pc: u32 = 0x830ABE30;
    'dispatch: loop {
        match pc {
            0x830ABE30 => {
    //   block [0x830ABE30..0x830AC274)
	// 830ABE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ABE34: 480FC309  bl 0x831a813c
	ctx.lr = 0x830ABE38;
	sub_831A8130(ctx, base);
	// 830ABE38: 3BE1FEF0  addi r31, r1, -0x110
	ctx.r[31].s64 = ctx.r[1].s64 + -272;
	// 830ABE3C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ABE40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830ABE44: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 830ABE48: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 830ABE4C: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 830ABE50: 7CF13B78  mr r17, r7
	ctx.r[17].u64 = ctx.r[7].u64;
	// 830ABE54: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABE58: 7D134378  mr r19, r8
	ctx.r[19].u64 = ctx.r[8].u64;
	// 830ABE5C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 830ABE60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ABE64: 40990408  ble cr6, 0x830ac26c
	if !ctx.cr[6].gt {
	pc = 0x830AC26C; continue 'dispatch;
	}
	// 830ABE68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830ABE6C: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 830ABE70: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABE74: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830ABE78: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ABE7C: 7F9E582E  lwzx r28, r30, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABE80: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830ABE84: 4BF809ED  bl 0x8302c870
	ctx.lr = 0x830ABE88;
	sub_8302C870(ctx, base);
	// 830ABE88: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830ABE8C: 4BF3AD7D  bl 0x82fe6c08
	ctx.lr = 0x830ABE90;
	sub_82FE6C08(ctx, base);
	// 830ABE90: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABE94: 7D7E502E  lwzx r11, r30, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830ABE98: 716B0005  andi. r11, r11, 5
	ctx.r[11].u64 = ctx.r[11].u64 & 5;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ABE9C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830ABEA0: 419A03A8  beq cr6, 0x830ac248
	if ctx.cr[6].eq {
	pc = 0x830AC248; continue 'dispatch;
	}
	// 830ABEA4: 813D000C  lwz r9, 0xc(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ABEA8: 7D3E482E  lwzx r9, r30, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830ABEAC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830ABEB0: 41990398  bgt cr6, 0x830ac248
	if ctx.cr[6].gt {
	pc = 0x830AC248; continue 'dispatch;
	}
	// 830ABEB4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830ABEB8: 409A000C  bne cr6, 0x830abec4
	if !ctx.cr[6].eq {
	pc = 0x830ABEC4; continue 'dispatch;
	}
	// 830ABEBC: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 830ABEC0: 7D7E512E  stwx r11, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 830ABEC4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 830ABEC8: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 830ABECC: 4BF809A5  bl 0x8302c870
	ctx.lr = 0x830ABED0;
	sub_8302C870(ctx, base);
	// 830ABED0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830ABED4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABED8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ABEDC: 4182000C  beq 0x830abee8
	if ctx.cr[0].eq {
	pc = 0x830ABEE8; continue 'dispatch;
	}
	// 830ABEE0: 832B0008  lwz r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABEE4: 48000044  b 0x830abf28
	pc = 0x830ABF28; continue 'dispatch;
	// 830ABEE8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830ABEEC: 4800003C  b 0x830abf28
	pc = 0x830ABF28; continue 'dispatch;
	// 830ABEF0: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABEF4: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABEF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ABEFC: 4182000C  beq 0x830abf08
	if ctx.cr[0].eq {
	pc = 0x830ABF08; continue 'dispatch;
	}
	// 830ABF00: 4BF80971  bl 0x8302c870
	ctx.lr = 0x830ABF04;
	sub_8302C870(ctx, base);
	// 830ABF04: 48000008  b 0x830abf0c
	pc = 0x830ABF0C; continue 'dispatch;
	// 830ABF08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ABF0C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABF10: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830ABF14: 409A0024  bne cr6, 0x830abf38
	if !ctx.cr[6].eq {
	pc = 0x830ABF38; continue 'dispatch;
	}
	// 830ABF18: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABF1C: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF20: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830ABF24: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830ABF28: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABF2C: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF30: 7F0AC800  cmpw cr6, r10, r25
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830ABF34: 4198FFBC  blt cr6, 0x830abef0
	if ctx.cr[6].lt {
	pc = 0x830ABEF0; continue 'dispatch;
	}
	// 830ABF38: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABF3C: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF40: 7F0AC800  cmpw cr6, r10, r25
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830ABF44: 409A0010  bne cr6, 0x830abf54
	if !ctx.cr[6].eq {
	pc = 0x830ABF54; continue 'dispatch;
	}
	// 830ABF48: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ABF4C: 7E5E592E  stwx r18, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[18].u32) };
	// 830ABF50: 48000308  b 0x830ac258
	pc = 0x830AC258; continue 'dispatch;
	// 830ABF54: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 830ABF58: 48000044  b 0x830abf9c
	pc = 0x830ABF9C; continue 'dispatch;
	// 830ABF5C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABF60: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ABF68: 4182000C  beq 0x830abf74
	if ctx.cr[0].eq {
	pc = 0x830ABF74; continue 'dispatch;
	}
	// 830ABF6C: 4BF80905  bl 0x8302c870
	ctx.lr = 0x830ABF70;
	sub_8302C870(ctx, base);
	// 830ABF70: 48000008  b 0x830abf78
	pc = 0x830ABF78; continue 'dispatch;
	// 830ABF74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ABF78: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABF7C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830ABF80: 409A0024  bne cr6, 0x830abfa4
	if !ctx.cr[6].eq {
	pc = 0x830ABFA4; continue 'dispatch;
	}
	// 830ABF84: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABF88: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830ABF90: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830ABF94: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABF98: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABF9C: 7F0AC800  cmpw cr6, r10, r25
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830ABFA0: 4198FFBC  blt cr6, 0x830abf5c
	if ctx.cr[6].lt {
	pc = 0x830ABF5C; continue 'dispatch;
	}
	// 830ABFA4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830ABFA8: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ABFAC: 7E4B9378  mr r11, r18
	ctx.r[11].u64 = ctx.r[18].u64;
	// 830ABFB0: 7F04C000  cmpw cr6, r4, r24
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[24].s32, &mut ctx.xer);
	// 830ABFB4: 41990008  bgt cr6, 0x830abfbc
	if ctx.cr[6].gt {
	pc = 0x830ABFBC; continue 'dispatch;
	}
	// 830ABFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ABFBC: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830ABFC0: 7F04C800  cmpw cr6, r4, r25
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830ABFC4: 419A0284  beq cr6, 0x830ac248
	if ctx.cr[6].eq {
	pc = 0x830AC248; continue 'dispatch;
	}
	// 830ABFC8: 7F04E000  cmpw cr6, r4, r28
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[28].s32, &mut ctx.xer);
	// 830ABFCC: 419A000C  beq cr6, 0x830abfd8
	if ctx.cr[6].eq {
	pc = 0x830ABFD8; continue 'dispatch;
	}
	// 830ABFD0: 7F04C000  cmpw cr6, r4, r24
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[24].s32, &mut ctx.xer);
	// 830ABFD4: 409900D8  ble cr6, 0x830ac0ac
	if !ctx.cr[6].gt {
	pc = 0x830AC0AC; continue 'dispatch;
	}
	// 830ABFD8: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABFDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ABFE0: 4182000C  beq 0x830abfec
	if ctx.cr[0].eq {
	pc = 0x830ABFEC; continue 'dispatch;
	}
	// 830ABFE4: 4BF8088D  bl 0x8302c870
	ctx.lr = 0x830ABFE8;
	sub_8302C870(ctx, base);
	// 830ABFE8: 48000008  b 0x830abff0
	pc = 0x830ABFF0; continue 'dispatch;
	// 830ABFEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ABFF0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ABFF4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830ABFF8: 409A00B4  bne cr6, 0x830ac0ac
	if !ctx.cr[6].eq {
	pc = 0x830AC0AC; continue 'dispatch;
	}
	// 830ABFFC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC000: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC004: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC008: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC00C: 4182000C  beq 0x830ac018
	if ctx.cr[0].eq {
	pc = 0x830AC018; continue 'dispatch;
	}
	// 830AC010: 4BF80861  bl 0x8302c870
	ctx.lr = 0x830AC014;
	sub_8302C870(ctx, base);
	// 830AC014: 48000008  b 0x830ac01c
	pc = 0x830AC01C; continue 'dispatch;
	// 830AC018: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC01C: 83830008  lwz r28, 8(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC020: A17C0004  lhz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC024: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830AC028: 409A0074  bne cr6, 0x830ac09c
	if !ctx.cr[6].eq {
	pc = 0x830AC09C; continue 'dispatch;
	}
	// 830AC02C: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC030: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 830AC034: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 830AC038: 80FD0020  lwz r7, 0x20(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AC03C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC040: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC044: 4BF32BE5  bl 0x82fdec28
	ctx.lr = 0x830AC048;
	sub_82FDEC28(ctx, base);
	// 830AC048: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 830AC04C: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC050: 4BF32751  bl 0x82fde7a0
	ctx.lr = 0x830AC054;
	sub_82FDE7A0(ctx, base);
	// 830AC054: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AC058: 4082003C  bne 0x830ac094
	if !ctx.cr[0].eq {
	pc = 0x830AC094; continue 'dispatch;
	}
	// 830AC05C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC060: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC064: 7F0AC000  cmpw cr6, r10, r24
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[24].s32, &mut ctx.xer);
	// 830AC068: 40990014  ble cr6, 0x830ac07c
	if !ctx.cr[6].gt {
	pc = 0x830AC07C; continue 'dispatch;
	}
	// 830AC06C: 7F1E592E  stwx r24, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 830AC070: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC074: 4BF3297D  bl 0x82fde9f0
	ctx.lr = 0x830AC078;
	sub_82FDE9F0(ctx, base);
	// 830AC078: 480001E0  b 0x830ac258
	pc = 0x830AC258; continue 'dispatch;
	// 830AC07C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AC080: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC084: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AC088: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830AC08C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC090: 4BFFFFE4  b 0x830ac074
	pc = 0x830AC074; continue 'dispatch;
	// 830AC094: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC098: 4BF32959  bl 0x82fde9f0
	ctx.lr = 0x830AC09C;
	sub_82FDE9F0(ctx, base);
	// 830AC09C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC0A0: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC0A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AC0A8: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830AC0AC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC0B0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC0B4: 7F04C800  cmpw cr6, r4, r25
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830AC0B8: 409A001C  bne cr6, 0x830ac0d4
	if !ctx.cr[6].eq {
	pc = 0x830AC0D4; continue 'dispatch;
	}
	// 830AC0BC: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830AC0C0: 4182FE88  beq 0x830abf48
	if ctx.cr[0].eq {
	pc = 0x830ABF48; continue 'dispatch;
	}
	// 830AC0C4: 7F1E592E  stwx r24, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 830AC0C8: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830AC0CC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC0D0: 48000184  b 0x830ac254
	pc = 0x830AC254; continue 'dispatch;
	// 830AC0D4: 40980184  bge cr6, 0x830ac258
	if !ctx.cr[6].lt {
	pc = 0x830AC258; continue 'dispatch;
	}
	// 830AC0D8: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC0DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC0E0: 4182000C  beq 0x830ac0ec
	if ctx.cr[0].eq {
	pc = 0x830AC0EC; continue 'dispatch;
	}
	// 830AC0E4: 4BF8078D  bl 0x8302c870
	ctx.lr = 0x830AC0E8;
	sub_8302C870(ctx, base);
	// 830AC0E8: 48000008  b 0x830ac0f0
	pc = 0x830AC0F0; continue 'dispatch;
	// 830AC0EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC0F0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC0F4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830AC0F8: 409A0160  bne cr6, 0x830ac258
	if !ctx.cr[6].eq {
	pc = 0x830AC258; continue 'dispatch;
	}
	// 830AC0FC: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 830AC100: 419A011C  beq cr6, 0x830ac21c
	if ctx.cr[6].eq {
	pc = 0x830AC21C; continue 'dispatch;
	}
	// 830AC104: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC108: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC10C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC110: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC114: 4182000C  beq 0x830ac120
	if ctx.cr[0].eq {
	pc = 0x830AC120; continue 'dispatch;
	}
	// 830AC118: 4BF80759  bl 0x8302c870
	ctx.lr = 0x830AC11C;
	sub_8302C870(ctx, base);
	// 830AC11C: 48000008  b 0x830ac124
	pc = 0x830AC124; continue 'dispatch;
	// 830AC120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC124: 83630008  lwz r27, 8(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC128: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830AC12C: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 830AC130: 419A00EC  beq cr6, 0x830ac21c
	if ctx.cr[6].eq {
	pc = 0x830AC21C; continue 'dispatch;
	}
	// 830AC134: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AC138: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 830AC13C: 4BF4189D  bl 0x82fed9d8
	ctx.lr = 0x830AC140;
	sub_82FED9D8(ctx, base);
	// 830AC140: A17B0004  lhz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC144: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830AC148: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830AC14C: 409A0028  bne cr6, 0x830ac174
	if !ctx.cr[6].eq {
	pc = 0x830AC174; continue 'dispatch;
	}
	// 830AC150: 809A0010  lwz r4, 0x10(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC154: 807B0008  lwz r3, 8(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC158: 4BF32649  bl 0x82fde7a0
	ctx.lr = 0x830AC15C;
	sub_82FDE7A0(ctx, base);
	// 830AC15C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AC160: 40820014  bne 0x830ac174
	if !ctx.cr[0].eq {
	pc = 0x830AC174; continue 'dispatch;
	}
	// 830AC164: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830AC168: 7F1C9840  cmplw cr6, r28, r19
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[19].u32, &mut ctx.xer);
	// 830AC16C: 4198FFC8  blt cr6, 0x830ac134
	if ctx.cr[6].lt {
	pc = 0x830AC134; continue 'dispatch;
	}
	// 830AC170: 480000AC  b 0x830ac21c
	pc = 0x830AC21C; continue 'dispatch;
	// 830AC174: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC178: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC17C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AC180: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830AC184: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC188: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC18C: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830AC190: 409A008C  bne cr6, 0x830ac21c
	if !ctx.cr[6].eq {
	pc = 0x830AC21C; continue 'dispatch;
	}
	// 830AC194: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 830AC19C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AC1A0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 830AC1A4: 7D3E512E  stwx r9, r30, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 830AC1A8: 40990028  ble cr6, 0x830ac1d0
	if !ctx.cr[6].gt {
	pc = 0x830AC1D0; continue 'dispatch;
	}
	// 830AC1AC: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC1B0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AC1B4: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 830AC1B8: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 830AC1BC: 419A0014  beq cr6, 0x830ac1d0
	if ctx.cr[6].eq {
	pc = 0x830AC1D0; continue 'dispatch;
	}
	// 830AC1C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830AC1C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830AC1C8: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 830AC1CC: 4198FFE4  blt cr6, 0x830ac1b0
	if ctx.cr[6].lt {
	pc = 0x830AC1B0; continue 'dispatch;
	}
	// 830AC1D0: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 830AC1D4: 409A0048  bne cr6, 0x830ac21c
	if !ctx.cr[6].eq {
	pc = 0x830AC21C; continue 'dispatch;
	}
	// 830AC1D8: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC1DC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 830AC1E0: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830AC1E4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC1E8: 4BFB9F91  bl 0x83066178
	ctx.lr = 0x830AC1EC;
	sub_83066178(ctx, base);
	// 830AC1EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC1F0: 4182000C  beq 0x830ac1fc
	if ctx.cr[0].eq {
	pc = 0x830AC1FC; continue 'dispatch;
	}
	// 830AC1F4: 80A3002C  lwz r5, 0x2c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AC1F8: 48000008  b 0x830ac200
	pc = 0x830AC200; continue 'dispatch;
	// 830AC1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830AC200: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AC204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AC208: 809A000C  lwz r4, 0xc(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AC20C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AC210: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AC214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AC218: 4E800421  bctrl
	ctx.lr = 0x830AC21C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AC21C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC220: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC224: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830AC228: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830AC22C: 419A002C  beq cr6, 0x830ac258
	if ctx.cr[6].eq {
	pc = 0x830AC258; continue 'dispatch;
	}
	// 830AC230: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC234: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC238: 7F0AC000  cmpw cr6, r10, r24
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[24].s32, &mut ctx.xer);
	// 830AC23C: 4099000C  ble cr6, 0x830ac248
	if !ctx.cr[6].gt {
	pc = 0x830AC248; continue 'dispatch;
	}
	// 830AC240: 7F1E592E  stwx r24, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u32) };
	// 830AC244: 48000014  b 0x830ac258
	pc = 0x830AC258; continue 'dispatch;
	// 830AC248: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AC24C: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC250: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AC254: 7D5E592E  stwx r10, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830AC258: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC25C: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 830AC260: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830AC264: 7F175800  cmpw cr6, r23, r11
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AC268: 4198FC08  blt cr6, 0x830abe70
	if ctx.cr[6].lt {
	pc = 0x830ABE70; continue 'dispatch;
	}
	// 830AC26C: 383F0110  addi r1, r31, 0x110
	ctx.r[1].s64 = ctx.r[31].s64 + 272;
	// 830AC270: 480FBF1C  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC274 size=40
    let mut pc: u32 = 0x830AC274;
    'dispatch: loop {
        match pc {
            0x830AC274 => {
    //   block [0x830AC274..0x830AC29C)
	// 830AC274: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 830AC278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC284: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC288: 4BF32769  bl 0x82fde9f0
	ctx.lr = 0x830AC28C;
	sub_82FDE9F0(ctx, base);
	// 830AC28C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AC290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AC294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AC298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC2A0 size=8
    let mut pc: u32 = 0x830AC2A0;
    'dispatch: loop {
        match pc {
            0x830AC2A0 => {
    //   block [0x830AC2A0..0x830AC2A8)
	// 830AC2A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AC2A4: 82180680  lwz r16, 0x680(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(1664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC2A8 size=156
    let mut pc: u32 = 0x830AC2A8;
    'dispatch: loop {
        match pc {
            0x830AC2A8 => {
    //   block [0x830AC2A8..0x830AC344)
	// 830AC2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC2AC: 480FBEBD  bl 0x831a8168
	ctx.lr = 0x830AC2B0;
	sub_831A8130(ctx, base);
	// 830AC2B0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830AC2B4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC2B8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AC2BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AC2C0: 396B0520  addi r11, r11, 0x520
	ctx.r[11].s64 = ctx.r[11].s64 + 1312;
	// 830AC2C4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830AC2C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AC2CC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC2D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC2D4: 41820044  beq 0x830ac318
	if ctx.cr[0].eq {
	pc = 0x830AC318; continue 'dispatch;
	}
	// 830AC2D8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC2DC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830AC2E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AC2E4: 40990034  ble cr6, 0x830ac318
	if !ctx.cr[6].gt {
	pc = 0x830AC318; continue 'dispatch;
	}
	// 830AC2E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AC2EC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC2F0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830AC2F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC2F8: 4182000C  beq 0x830ac304
	if ctx.cr[0].eq {
	pc = 0x830AC304; continue 'dispatch;
	}
	// 830AC2FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC300: 4BFEE419  bl 0x8309a718
	ctx.lr = 0x830AC304;
	sub_8309A718(ctx, base);
	// 830AC304: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC308: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 830AC30C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830AC310: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC314: 4198FFD8  blt cr6, 0x830ac2ec
	if ctx.cr[6].lt {
	pc = 0x830AC2EC; continue 'dispatch;
	}
	// 830AC318: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AC31C: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC320: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AC324: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AC32C: 4E800421  bctrl
	ctx.lr = 0x830AC330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AC330: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AC334: 396B0508  addi r11, r11, 0x508
	ctx.r[11].s64 = ctx.r[11].s64 + 1288;
	// 830AC338: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AC33C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830AC340: 480FBE78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC344 size=40
    let mut pc: u32 = 0x830AC344;
    'dispatch: loop {
        match pc {
            0x830AC344 => {
    //   block [0x830AC344..0x830AC36C)
	// 830AC344: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830AC348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC354: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830AC358: 4BFFF5D1  bl 0x830ab928
	ctx.lr = 0x830AC35C;
	sub_830AB928(ctx, base);
	// 830AC35C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AC360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AC364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AC368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC370 size=140
    let mut pc: u32 = 0x830AC370;
    'dispatch: loop {
        match pc {
            0x830AC370 => {
    //   block [0x830AC370..0x830AC3FC)
	// 830AC370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC374: 480FBDF9  bl 0x831a816c
	ctx.lr = 0x830AC378;
	sub_831A8130(ctx, base);
	// 830AC378: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC37C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC380: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830AC384: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830AC388: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC38C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC390: 41980030  blt cr6, 0x830ac3c0
	if ctx.cr[6].lt {
	pc = 0x830AC3C0; continue 'dispatch;
	}
	// 830AC394: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AC398: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AC39C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 830AC3A0: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 830AC3A4: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 830AC3A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AC3AC: 4BF245AD  bl 0x82fd0958
	ctx.lr = 0x830AC3B0;
	sub_82FD0958(ctx, base);
	// 830AC3B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AC3B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AC3B8: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 830AC3BC: 4810486D  bl 0x831b0c28
	ctx.lr = 0x830AC3C0;
	sub_831B0C28(ctx, base);
	// 830AC3C0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC3C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC3C8: 41820020  beq 0x830ac3e8
	if ctx.cr[0].eq {
	pc = 0x830AC3E8; continue 'dispatch;
	}
	// 830AC3CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC3D0: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AC3D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC3D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC3DC: 4182000C  beq 0x830ac3e8
	if ctx.cr[0].eq {
	pc = 0x830AC3E8; continue 'dispatch;
	}
	// 830AC3E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC3E4: 4BFEE335  bl 0x8309a718
	ctx.lr = 0x830AC3E8;
	sub_8309A718(ctx, base);
	// 830AC3E8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC3EC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AC3F0: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 830AC3F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830AC3F8: 480FBDC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC400 size=116
    let mut pc: u32 = 0x830AC400;
    'dispatch: loop {
        match pc {
            0x830AC400 => {
    //   block [0x830AC400..0x830AC474)
	// 830AC400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC404: 480FBD65  bl 0x831a8168
	ctx.lr = 0x830AC408;
	sub_831A8130(ctx, base);
	// 830AC408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC40C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC410: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830AC414: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 830AC418: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC41C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AC420: 40990048  ble cr6, 0x830ac468
	if !ctx.cr[6].gt {
	pc = 0x830AC468; continue 'dispatch;
	}
	// 830AC424: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830AC428: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC42C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC430: 4182001C  beq 0x830ac44c
	if ctx.cr[0].eq {
	pc = 0x830AC44C; continue 'dispatch;
	}
	// 830AC434: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC438: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830AC43C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC440: 4182000C  beq 0x830ac44c
	if ctx.cr[0].eq {
	pc = 0x830AC44C; continue 'dispatch;
	}
	// 830AC444: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC448: 4BFEE2D1  bl 0x8309a718
	ctx.lr = 0x830AC44C;
	sub_8309A718(ctx, base);
	// 830AC44C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC450: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830AC454: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 830AC458: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830AC45C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC460: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC464: 4198FFC4  blt cr6, 0x830ac428
	if ctx.cr[6].lt {
	pc = 0x830AC428; continue 'dispatch;
	}
	// 830AC468: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830AC46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AC470: 480FBD48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC478 size=276
    let mut pc: u32 = 0x830AC478;
    'dispatch: loop {
        match pc {
            0x830AC478 => {
    //   block [0x830AC478..0x830AC58C)
	// 830AC478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AC484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AC488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC48C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AC494: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC498: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC49C: 41980030  blt cr6, 0x830ac4cc
	if ctx.cr[6].lt {
	pc = 0x830AC4CC; continue 'dispatch;
	}
	// 830AC4A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AC4A4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AC4A8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 830AC4AC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 830AC4B0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 830AC4B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AC4B8: 4BF244A1  bl 0x82fd0958
	ctx.lr = 0x830AC4BC;
	sub_82FD0958(ctx, base);
	// 830AC4BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830AC4C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830AC4C4: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 830AC4C8: 48104761  bl 0x831b0c28
	ctx.lr = 0x830AC4CC;
	sub_831B0C28(ctx, base);
	// 830AC4CC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC4D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC4D4: 41820020  beq 0x830ac4f4
	if ctx.cr[0].eq {
	pc = 0x830AC4F4; continue 'dispatch;
	}
	// 830AC4D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC4DC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AC4E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC4E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC4E8: 4182000C  beq 0x830ac4f4
	if ctx.cr[0].eq {
	pc = 0x830AC4F4; continue 'dispatch;
	}
	// 830AC4EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC4F0: 4BFEE229  bl 0x8309a718
	ctx.lr = 0x830AC4F4;
	sub_8309A718(ctx, base);
	// 830AC4F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC4F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830AC4FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC500: 409A0018  bne cr6, 0x830ac518
	if !ctx.cr[6].eq {
	pc = 0x830AC518; continue 'dispatch;
	}
	// 830AC504: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC508: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AC50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AC510: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 830AC514: 48000054  b 0x830ac568
	pc = 0x830AC568; continue 'dispatch;
	// 830AC518: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830AC51C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC520: 40980030  bge cr6, 0x830ac550
	if !ctx.cr[6].lt {
	pc = 0x830AC550; continue 'dispatch;
	}
	// 830AC524: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AC528: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC52C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AC530: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 830AC534: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830AC538: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC53C: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830AC540: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC544: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 830AC548: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830AC54C: 4198FFDC  blt cr6, 0x830ac528
	if ctx.cr[6].lt {
	pc = 0x830AC528; continue 'dispatch;
	}
	// 830AC550: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830AC558: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC55C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AC560: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830AC564: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 830AC568: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC56C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830AC570: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AC574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AC578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AC57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AC580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AC584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AC588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC590 size=12
    let mut pc: u32 = 0x830AC590;
    'dispatch: loop {
        match pc {
            0x830AC590 => {
    //   block [0x830AC590..0x830AC59C)
	// 830AC590: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC594: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC598: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC59C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC59C size=20
    let mut pc: u32 = 0x830AC59C;
    'dispatch: loop {
        match pc {
            0x830AC59C => {
    //   block [0x830AC59C..0x830AC5B0)
	// 830AC59C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830AC5A0: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC5A4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC5A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AC5AC: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC5B0 size=20
    let mut pc: u32 = 0x830AC5B0;
    'dispatch: loop {
        match pc {
            0x830AC5B0 => {
    //   block [0x830AC5B0..0x830AC5C4)
	// 830AC5B0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC5B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AC5B8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830AC5BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC5C0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC5C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC5C4 size=8
    let mut pc: u32 = 0x830AC5C4;
    'dispatch: loop {
        match pc {
            0x830AC5C4 => {
    //   block [0x830AC5C4..0x830AC5CC)
	// 830AC5C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC5C8: 4BFEE150  b 0x8309a718
	sub_8309A718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC5CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC5CC size=4
    let mut pc: u32 = 0x830AC5CC;
    'dispatch: loop {
        match pc {
            0x830AC5CC => {
    //   block [0x830AC5CC..0x830AC5D0)
	// 830AC5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC5D0 size=124
    let mut pc: u32 = 0x830AC5D0;
    'dispatch: loop {
        match pc {
            0x830AC5D0 => {
    //   block [0x830AC5D0..0x830AC64C)
	// 830AC5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC5D4: 480FBB99  bl 0x831a816c
	ctx.lr = 0x830AC5D8;
	sub_831A8130(ctx, base);
	// 830AC5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC5E0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC5E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC5E8: 41820044  beq 0x830ac62c
	if ctx.cr[0].eq {
	pc = 0x830AC62C; continue 'dispatch;
	}
	// 830AC5EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC5F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AC5F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AC5F8: 40990034  ble cr6, 0x830ac62c
	if !ctx.cr[6].gt {
	pc = 0x830AC62C; continue 'dispatch;
	}
	// 830AC5FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830AC600: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC604: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830AC608: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC60C: 4182000C  beq 0x830ac618
	if ctx.cr[0].eq {
	pc = 0x830AC618; continue 'dispatch;
	}
	// 830AC610: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AC614: 4BFEE105  bl 0x8309a718
	ctx.lr = 0x830AC618;
	sub_8309A718(ctx, base);
	// 830AC618: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC61C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830AC620: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830AC624: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830AC628: 4198FFD8  blt cr6, 0x830ac600
	if ctx.cr[6].lt {
	pc = 0x830AC600; continue 'dispatch;
	}
	// 830AC62C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830AC630: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AC634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AC638: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AC63C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AC640: 4E800421  bctrl
	ctx.lr = 0x830AC644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AC644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AC648: 480FBB74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC650 size=76
    let mut pc: u32 = 0x830AC650;
    'dispatch: loop {
        match pc {
            0x830AC650 => {
    //   block [0x830AC650..0x830AC69C)
	// 830AC650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AC65C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AC660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC664: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC668: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AC66C: 4BFFFC3D  bl 0x830ac2a8
	ctx.lr = 0x830AC670;
	sub_830AC2A8(ctx, base);
	// 830AC670: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AC674: 4182000C  beq 0x830ac680
	if ctx.cr[0].eq {
	pc = 0x830AC680; continue 'dispatch;
	}
	// 830AC678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AC67C: 4BF2BC65  bl 0x82fd82e0
	ctx.lr = 0x830AC680;
	sub_82FD82E0(ctx, base);
	// 830AC680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AC684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AC688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AC68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AC690: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AC694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AC698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC6A0 size=20
    let mut pc: u32 = 0x830AC6A0;
    'dispatch: loop {
        match pc {
            0x830AC6A0 => {
    //   block [0x830AC6A0..0x830AC6B4)
	// 830AC6A0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AC6A4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 830AC6A8: 396B0878  addi r11, r11, 0x878
	ctx.r[11].s64 = ctx.r[11].s64 + 2168;
	// 830AC6AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AC6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC6B8 size=12
    let mut pc: u32 = 0x830AC6B8;
    'dispatch: loop {
        match pc {
            0x830AC6B8 => {
    //   block [0x830AC6B8..0x830AC6C4)
	// 830AC6B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830AC6BC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 830AC6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC6C8 size=32
    let mut pc: u32 = 0x830AC6C8;
    'dispatch: loop {
        match pc {
            0x830AC6C8 => {
    //   block [0x830AC6C8..0x830AC6E8)
	// 830AC6C8: 548A043F  clrlwi. r10, r4, 0x10
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830AC6CC: 4082001C  bne 0x830ac6e8
	if !ctx.cr[0].eq {
		sub_830AC6E8(ctx, base);
		return;
	}
	// 830AC6D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 830AC6D4: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830AC6D8: 396B2DD8  addi r11, r11, 0x2dd8
	ctx.r[11].s64 = ctx.r[11].s64 + 11736;
	// 830AC6DC: 7D6A58AE  lbzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC6E0: 5563D7FE  rlwinm r3, r11, 0x1a, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 830AC6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC6E8 size=40
    let mut pc: u32 = 0x830AC6E8;
    'dispatch: loop {
        match pc {
            0x830AC6E8 => {
    //   block [0x830AC6E8..0x830AC710)
	// 830AC6E8: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830AC6EC: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 830AC6F0: 41980020  blt cr6, 0x830ac710
	if ctx.cr[6].lt {
		sub_830AC710(ctx, base);
		return;
	}
	// 830AC6F4: 2B0BDBFF  cmplwi cr6, r11, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56319 as u32, &mut ctx.xer);
	// 830AC6F8: 41990018  bgt cr6, 0x830ac710
	if ctx.cr[6].gt {
		sub_830AC710(ctx, base);
		return;
	}
	// 830AC6FC: 2B0ADC00  cmplwi cr6, r10, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56320 as u32, &mut ctx.xer);
	// 830AC700: 41980010  blt cr6, 0x830ac710
	if ctx.cr[6].lt {
		sub_830AC710(ctx, base);
		return;
	}
	// 830AC704: 2B0ADFFF  cmplwi cr6, r10, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57343 as u32, &mut ctx.xer);
	// 830AC708: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830AC70C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC710 size=8
    let mut pc: u32 = 0x830AC710;
    'dispatch: loop {
        match pc {
            0x830AC710 => {
    //   block [0x830AC710..0x830AC718)
	// 830AC710: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC718 size=32
    let mut pc: u32 = 0x830AC718;
    'dispatch: loop {
        match pc {
            0x830AC718 => {
    //   block [0x830AC718..0x830AC738)
	// 830AC718: 548A043F  clrlwi. r10, r4, 0x10
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830AC71C: 4082001C  bne 0x830ac738
	if !ctx.cr[0].eq {
		sub_830AC738(ctx, base);
		return;
	}
	// 830AC720: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 830AC724: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830AC728: 396B2DD8  addi r11, r11, 0x2dd8
	ctx.r[11].s64 = ctx.r[11].s64 + 11736;
	// 830AC72C: 7D6A58AE  lbzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC730: 5563D7FE  rlwinm r3, r11, 0x1a, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 830AC734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC738 size=40
    let mut pc: u32 = 0x830AC738;
    'dispatch: loop {
        match pc {
            0x830AC738 => {
    //   block [0x830AC738..0x830AC760)
	// 830AC738: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830AC73C: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 830AC740: 41980020  blt cr6, 0x830ac760
	if ctx.cr[6].lt {
		sub_830AC760(ctx, base);
		return;
	}
	// 830AC744: 2B0BDBFF  cmplwi cr6, r11, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56319 as u32, &mut ctx.xer);
	// 830AC748: 41990018  bgt cr6, 0x830ac760
	if ctx.cr[6].gt {
		sub_830AC760(ctx, base);
		return;
	}
	// 830AC74C: 2B0ADC00  cmplwi cr6, r10, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[10].u32, 56320 as u32, &mut ctx.xer);
	// 830AC750: 41980010  blt cr6, 0x830ac760
	if ctx.cr[6].lt {
		sub_830AC760(ctx, base);
		return;
	}
	// 830AC754: 2B0ADFFF  cmplwi cr6, r10, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57343 as u32, &mut ctx.xer);
	// 830AC758: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830AC75C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC760 size=8
    let mut pc: u32 = 0x830AC760;
    'dispatch: loop {
        match pc {
            0x830AC760 => {
    //   block [0x830AC760..0x830AC768)
	// 830AC760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC768 size=8
    let mut pc: u32 = 0x830AC768;
    'dispatch: loop {
        match pc {
            0x830AC768 => {
    //   block [0x830AC768..0x830AC770)
	// 830AC768: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AC76C: 821809FC  lwz r16, 0x9fc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2556 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC770 size=400
    //   switch @ 0x830AC7D4: r11 with 9 label(s)
    //       case  0 → 0x830AC7E4
    //       case  1 → 0x830AC7F0
    //       case  2 → 0x830AC7FC
    //       case  3 → 0x830AC808
    //       case  4 → 0x830AC814
    //       case  5 → 0x830AC820
    //       case  6 → 0x830AC82C
    //       case  7 → 0x830AC838
    //       case  8 → 0x830AC844
    let mut pc: u32 = 0x830AC770;
    'dispatch: loop {
        match pc {
            0x830AC770 => {
    //   block [0x830AC770..0x830AC7E4)
	// 830AC770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC774: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830AC778: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830AC77C: 480FB9ED  bl 0x831a8168
	ctx.lr = 0x830AC780;
	sub_831A8130(ctx, base);
	// 830AC780: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 830AC784: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC788: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830AC78C: 90BF0104  stw r5, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[5].u32 ) };
	// 830AC790: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 830AC794: 939F00FC  stw r28, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[28].u32 ) };
	// 830AC798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AC79C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AC7A0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC7A4: 4BFE347D  bl 0x8308fc20
	ctx.lr = 0x830AC7A8;
	sub_8308FC20(ctx, base);
	// 830AC7A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC7AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC7B0: 397CFFFB  addi r11, r28, -5
	ctx.r[11].s64 = ctx.r[28].s64 + -5;
	// 830AC7B4: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 830AC7B8: 41990124  bgt cr6, 0x830ac8dc
	if ctx.cr[6].gt {
	pc = 0x830AC8DC; continue 'dispatch;
	}
	// 830AC7BC: 3D808218  lis r12, -0x7de8
	ctx.r[12].s64 = -2112356352;
	// 830AC7C0: 398C0960  addi r12, r12, 0x960
	ctx.r[12].s64 = ctx.r[12].s64 + 2400;
	// 830AC7C4: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AC7C8: 3D80830B  lis r12, -0x7cf5
	ctx.r[12].s64 = -2096431104;
	// 830AC7CC: 398CC7E4  addi r12, r12, -0x381c
	ctx.r[12].s64 = ctx.r[12].s64 + -14364;
	// 830AC7D0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830AC7D4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830AC7D8: 60000000  nop
	// 830AC7DC: 60000000  nop
	// 830AC7E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x830AC7E4 => {
    //   block [0x830AC7E4..0x830AC7F0)
	// 830AC7E4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC7E8: 4BFE46F9  bl 0x83090ee0
	ctx.lr = 0x830AC7EC;
	sub_83090EE0(ctx, base);
	// 830AC7EC: 48000060  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC7F0 => {
    //   block [0x830AC7F0..0x830AC7FC)
	// 830AC7F0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC7F4: 4BFE4F95  bl 0x83091788
	ctx.lr = 0x830AC7F8;
	sub_83091788(ctx, base);
	// 830AC7F8: 48000054  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC7FC => {
    //   block [0x830AC7FC..0x830AC808)
	// 830AC7FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC800: 4BFE5041  bl 0x83091840
	ctx.lr = 0x830AC804;
	sub_83091840(ctx, base);
	// 830AC804: 48000048  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC808 => {
    //   block [0x830AC808..0x830AC814)
	// 830AC808: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC80C: 4BFE461D  bl 0x83090e28
	ctx.lr = 0x830AC810;
	sub_83090E28(ctx, base);
	// 830AC810: 4800003C  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC814 => {
    //   block [0x830AC814..0x830AC820)
	// 830AC814: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC818: 4BFE4669  bl 0x83090e80
	ctx.lr = 0x830AC81C;
	sub_83090E80(ctx, base);
	// 830AC81C: 48000030  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC820 => {
    //   block [0x830AC820..0x830AC82C)
	// 830AC820: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC824: 4BFE4025  bl 0x83090848
	ctx.lr = 0x830AC828;
	sub_83090848(ctx, base);
	// 830AC828: 48000024  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC82C => {
    //   block [0x830AC82C..0x830AC838)
	// 830AC82C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC830: 4BFE40D1  bl 0x83090900
	ctx.lr = 0x830AC834;
	sub_83090900(ctx, base);
	// 830AC834: 48000018  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC838 => {
    //   block [0x830AC838..0x830AC844)
	// 830AC838: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC83C: 4BFE3D75  bl 0x830905b0
	ctx.lr = 0x830AC840;
	sub_830905B0(ctx, base);
	// 830AC840: 4800000C  b 0x830ac84c
	pc = 0x830AC84C; continue 'dispatch;
            }
            0x830AC844 => {
    //   block [0x830AC844..0x830AC900)
	// 830AC844: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC848: 4BFE3EA1  bl 0x830906e8
	ctx.lr = 0x830AC84C;
	sub_830906E8(ctx, base);
	// 830AC84C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC850: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC854: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AC858: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AC85C: 4BF2BA3D  bl 0x82fd8298
	ctx.lr = 0x830AC860;
	sub_82FD8298(ctx, base);
	// 830AC860: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC864: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC868: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC86C: 4182001C  beq 0x830ac888
	if ctx.cr[0].eq {
	pc = 0x830AC888; continue 'dispatch;
	}
	// 830AC870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AC874: 93A3002C  stw r29, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830AC878: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AC87C: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830AC880: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AC884: 48000008  b 0x830ac88c
	pc = 0x830AC88C; continue 'dispatch;
	// 830AC888: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830AC88C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830AC890: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AC894: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830AC898: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830AC89C: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830AC8A0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830AC8A4: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830AC8A8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830AC8AC: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830AC8B0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830AC8B4: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830AC8B8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830AC8BC: C81F00A0  lfd f0, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	// 830AC8C0: D81E0020  stfd f0, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.f[0].u64 ) };
	// 830AC8C4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC8C8: 4BFE23D1  bl 0x8308ec98
	ctx.lr = 0x830AC8CC;
	sub_8308EC98(ctx, base);
	// 830AC8CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC8D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC8D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AC8D8: 48000020  b 0x830ac8f8
	pc = 0x830AC8F8; continue 'dispatch;
	// 830AC8DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC8E0: 4BFE23B9  bl 0x8308ec98
	ctx.lr = 0x830AC8E4;
	sub_8308EC98(ctx, base);
	// 830AC8E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC8E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AC8EC: 48000008  b 0x830ac8f4
	pc = 0x830AC8F4; continue 'dispatch;
	// 830AC8F0: 48000004  b 0x830ac8f4
	pc = 0x830AC8F4; continue 'dispatch;
	// 830AC8F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AC8F8: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830AC8FC: 480FB8BC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC900 size=8
    let mut pc: u32 = 0x830AC900;
    'dispatch: loop {
        match pc {
            0x830AC900 => {
    //   block [0x830AC900..0x830AC908)
	// 830AC900: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AC904: 821809FC  lwz r16, 0x9fc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2556 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830AC908 size=124
    let mut pc: u32 = 0x830AC908;
    'dispatch: loop {
        match pc {
            0x830AC908 => {
    //   block [0x830AC908..0x830AC984)
	// 830AC908: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830AC90C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC910: 817F00FC  lwz r11, 0xfc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 830AC914: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830AC918: 419A000C  beq cr6, 0x830ac924
	if ctx.cr[6].eq {
	pc = 0x830AC924; continue 'dispatch;
	}
	// 830AC91C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830AC920: 409A002C  bne cr6, 0x830ac94c
	if !ctx.cr[6].eq {
	pc = 0x830AC94C; continue 'dispatch;
	}
	// 830AC924: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AC928: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AC92C: 2F0B0157  cmpwi cr6, r11, 0x157
	ctx.cr[6].compare_i32(ctx.r[11].s32, 343, &mut ctx.xer);
	// 830AC930: 419A0024  beq cr6, 0x830ac954
	if ctx.cr[6].eq {
	pc = 0x830AC954; continue 'dispatch;
	}
	// 830AC934: 2F0B0158  cmpwi cr6, r11, 0x158
	ctx.cr[6].compare_i32(ctx.r[11].s32, 344, &mut ctx.xer);
	// 830AC938: 419A001C  beq cr6, 0x830ac954
	if ctx.cr[6].eq {
	pc = 0x830AC954; continue 'dispatch;
	}
	// 830AC93C: 2F0B0159  cmpwi cr6, r11, 0x159
	ctx.cr[6].compare_i32(ctx.r[11].s32, 345, &mut ctx.xer);
	// 830AC940: 419A0014  beq cr6, 0x830ac954
	if ctx.cr[6].eq {
	pc = 0x830AC954; continue 'dispatch;
	}
	// 830AC944: 2F0B0160  cmpwi cr6, r11, 0x160
	ctx.cr[6].compare_i32(ctx.r[11].s32, 352, &mut ctx.xer);
	// 830AC948: 419A000C  beq cr6, 0x830ac954
	if ctx.cr[6].eq {
	pc = 0x830AC954; continue 'dispatch;
	}
	// 830AC94C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AC950: 48000008  b 0x830ac958
	pc = 0x830AC958; continue 'dispatch;
	// 830AC954: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AC958: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830AC95C: 815F0104  lwz r10, 0x104(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 830AC960: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830AC964: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 830AC968: 3863C8F0  addi r3, r3, -0x3710
	ctx.r[3].s64 = ctx.r[3].s64 + -14096;
	// 830AC96C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 830AC970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AC974: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 830AC978: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 830AC97C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AC980: 480FB838  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AC984 size=8
    let mut pc: u32 = 0x830AC984;
    'dispatch: loop {
        match pc {
            0x830AC984 => {
    //   block [0x830AC984..0x830AC98C)
	// 830AC984: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AC988: 821809FC  lwz r16, 0x9fc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2556 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC98C size=36
    let mut pc: u32 = 0x830AC98C;
    'dispatch: loop {
        match pc {
            0x830AC98C => {
    //   block [0x830AC98C..0x830AC9B0)
	// 830AC98C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830AC990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC994: 815F0104  lwz r10, 0x104(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 830AC998: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830AC99C: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830AC9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AC9A4: 3863C8F4  addi r3, r3, -0x370c
	ctx.r[3].s64 = ctx.r[3].s64 + -14092;
	// 830AC9A8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AC9AC: 480FB80C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC9B0 size=40
    let mut pc: u32 = 0x830AC9B0;
    'dispatch: loop {
        match pc {
            0x830AC9B0 => {
    //   block [0x830AC9B0..0x830AC9D8)
	// 830AC9B0: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830AC9B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC9B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC9BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC9C0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AC9C4: 4BFE22D5  bl 0x8308ec98
	ctx.lr = 0x830AC9C8;
	sub_8308EC98(ctx, base);
	// 830AC9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AC9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AC9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AC9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AC9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AC9D8 size=108
    let mut pc: u32 = 0x830AC9D8;
    'dispatch: loop {
        match pc {
            0x830AC9D8 => {
    //   block [0x830AC9D8..0x830ACA44)
	// 830AC9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AC9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AC9E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AC9E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AC9E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AC9EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AC9F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AC9F4: 897F0028  lbz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830AC9F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AC9FC: 4182001C  beq 0x830aca18
	if ctx.cr[0].eq {
	pc = 0x830ACA18; continue 'dispatch;
	}
	// 830ACA00: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830ACA04: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACA08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACA0C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ACA14: 4E800421  bctrl
	ctx.lr = 0x830ACA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ACA18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ACA1C: 4182000C  beq 0x830aca28
	if ctx.cr[0].eq {
	pc = 0x830ACA28; continue 'dispatch;
	}
	// 830ACA20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACA24: 4BF2B8BD  bl 0x82fd82e0
	ctx.lr = 0x830ACA28;
	sub_82FD82E0(ctx, base);
	// 830ACA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACA2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830ACA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACA38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ACA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ACA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACA48 size=152
    let mut pc: u32 = 0x830ACA48;
    'dispatch: loop {
        match pc {
            0x830ACA48 => {
    //   block [0x830ACA48..0x830ACAE0)
	// 830ACA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACA4C: 480FB715  bl 0x831a8160
	ctx.lr = 0x830ACA50;
	sub_831A8130(ctx, base);
	// 830ACA50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACA54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830ACA58: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830ACA5C: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 830ACA60: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ACA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ACA68: 4099006C  ble cr6, 0x830acad4
	if !ctx.cr[6].gt {
	pc = 0x830ACAD4; continue 'dispatch;
	}
	// 830ACA6C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 830ACA70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACA74: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830ACA78: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACA7C: 4182003C  beq 0x830acab8
	if ctx.cr[0].eq {
	pc = 0x830ACAB8; continue 'dispatch;
	}
	// 830ACA80: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ACA84: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ACA88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACA8C: 41820018  beq 0x830acaa4
	if ctx.cr[0].eq {
	pc = 0x830ACAA4; continue 'dispatch;
	}
	// 830ACA90: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACA94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACA98: 4182000C  beq 0x830acaa4
	if ctx.cr[0].eq {
	pc = 0x830ACAA4; continue 'dispatch;
	}
	// 830ACA9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830ACAA0: 4BFFFF39  bl 0x830ac9d8
	ctx.lr = 0x830ACAA4;
	sub_830AC9D8(ctx, base);
	// 830ACAA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830ACAA8: 4BF2B839  bl 0x82fd82e0
	ctx.lr = 0x830ACAAC;
	sub_82FD82E0(ctx, base);
	// 830ACAAC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 830ACAB0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830ACAB4: 409AFFCC  bne cr6, 0x830aca80
	if !ctx.cr[6].eq {
	pc = 0x830ACA80; continue 'dispatch;
	}
	// 830ACAB8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACABC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 830ACAC0: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 830ACAC4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830ACAC8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ACACC: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830ACAD0: 4198FFA0  blt cr6, 0x830aca70
	if ctx.cr[6].lt {
	pc = 0x830ACA70; continue 'dispatch;
	}
	// 830ACAD4: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 830ACAD8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830ACADC: 480FB6D4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACAE0 size=84
    let mut pc: u32 = 0x830ACAE0;
    'dispatch: loop {
        match pc {
            0x830ACAE0 => {
    //   block [0x830ACAE0..0x830ACB34)
	// 830ACAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830ACAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830ACAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACAF4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830ACAF8: 83FEBC4C  lwz r31, -0x43b4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17332 as u32) ) } as u64;
	// 830ACAFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830ACB00: 419A0014  beq cr6, 0x830acb14
	if ctx.cr[6].eq {
	pc = 0x830ACB14; continue 'dispatch;
	}
	// 830ACB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACB08: 4BF48C81  bl 0x82ff5788
	ctx.lr = 0x830ACB0C;
	sub_82FF5788(ctx, base);
	// 830ACB0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACB10: 4BF2B7D1  bl 0x82fd82e0
	ctx.lr = 0x830ACB14;
	sub_82FD82E0(ctx, base);
	// 830ACB14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ACB18: 917EBC4C  stw r11, -0x43b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17332 as u32), ctx.r[11].u32 ) };
	// 830ACB1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830ACB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACB28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ACB2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ACB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACB38 size=84
    let mut pc: u32 = 0x830ACB38;
    'dispatch: loop {
        match pc {
            0x830ACB38 => {
    //   block [0x830ACB38..0x830ACB8C)
	// 830ACB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACB40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830ACB44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830ACB48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACB4C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830ACB50: 83FEBC50  lwz r31, -0x43b0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17328 as u32) ) } as u64;
	// 830ACB54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830ACB58: 419A0014  beq cr6, 0x830acb6c
	if ctx.cr[6].eq {
	pc = 0x830ACB6C; continue 'dispatch;
	}
	// 830ACB5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACB60: 4BFDC621  bl 0x83089180
	ctx.lr = 0x830ACB64;
	sub_83089180(ctx, base);
	// 830ACB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACB68: 4BF2B779  bl 0x82fd82e0
	ctx.lr = 0x830ACB6C;
	sub_82FD82E0(ctx, base);
	// 830ACB6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ACB70: 917EBC50  stw r11, -0x43b0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17328 as u32), ctx.r[11].u32 ) };
	// 830ACB74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830ACB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACB80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ACB84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ACB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACB90 size=100
    let mut pc: u32 = 0x830ACB90;
    'dispatch: loop {
        match pc {
            0x830ACB90 => {
    //   block [0x830ACB90..0x830ACBF4)
	// 830ACB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830ACB9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACBA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830ACBA4: 4BFFFEA5  bl 0x830aca48
	ctx.lr = 0x830ACBA8;
	sub_830ACA48(ctx, base);
	// 830ACBA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACBAC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACBB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACBB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACBB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ACBBC: 4E800421  bctrl
	ctx.lr = 0x830ACBC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ACBC0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830ACBC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACBC8: 41820018  beq 0x830acbe0
	if ctx.cr[0].eq {
	pc = 0x830ACBE0; continue 'dispatch;
	}
	// 830ACBCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACBD0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830ACBD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACBD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830ACBDC: 4E800421  bctrl
	ctx.lr = 0x830ACBE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830ACBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACBEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ACBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACBF8 size=212
    let mut pc: u32 = 0x830ACBF8;
    'dispatch: loop {
        match pc {
            0x830ACBF8 => {
    //   block [0x830ACBF8..0x830ACCCC)
	// 830ACBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACBFC: 480FB56D  bl 0x831a8168
	ctx.lr = 0x830ACC00;
	sub_831A8130(ctx, base);
	// 830ACC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830ACC08: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830ACC0C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830ACC10: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830ACC14: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ACC18: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 830ACC1C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830ACC20: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830ACC24: 41980008  blt cr6, 0x830acc2c
	if ctx.cr[6].lt {
	pc = 0x830ACC2C; continue 'dispatch;
	}
	// 830ACC28: 4BFBBDC1  bl 0x830689e8
	ctx.lr = 0x830ACC2C;
	sub_830689E8(ctx, base);
	// 830ACC2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830ACC30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830ACC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830ACC38: 4BF4D829  bl 0x82ffa460
	ctx.lr = 0x830ACC3C;
	sub_82FFA460(ctx, base);
	// 830ACC3C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830ACC40: 41820030  beq 0x830acc70
	if ctx.cr[0].eq {
	pc = 0x830ACC70; continue 'dispatch;
	}
	// 830ACC44: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830ACC48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACC4C: 41820018  beq 0x830acc64
	if ctx.cr[0].eq {
	pc = 0x830ACC64; continue 'dispatch;
	}
	// 830ACC50: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACC54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACC58: 4182000C  beq 0x830acc64
	if ctx.cr[0].eq {
	pc = 0x830ACC64; continue 'dispatch;
	}
	// 830ACC5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830ACC60: 4BFFFD79  bl 0x830ac9d8
	ctx.lr = 0x830ACC64;
	sub_830AC9D8(ctx, base);
	// 830ACC64: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830ACC68: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830ACC6C: 48000058  b 0x830accc4
	pc = 0x830ACCC4; continue 'dispatch;
	// 830ACC70: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830ACC74: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACC78: 4BF2B621  bl 0x82fd8298
	ctx.lr = 0x830ACC7C;
	sub_82FD8298(ctx, base);
	// 830ACC7C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ACC80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACC84: 41820024  beq 0x830acca8
	if ctx.cr[0].eq {
	pc = 0x830ACCA8; continue 'dispatch;
	}
	// 830ACC88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACC8C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830ACC90: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830ACC94: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830ACC98: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830ACC9C: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830ACCA0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830ACCA4: 48000008  b 0x830accac
	pc = 0x830ACCAC; continue 'dispatch;
	// 830ACCA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830ACCAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830ACCB0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830ACCB4: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830ACCB8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830ACCBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830ACCC0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830ACCC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830ACCC8: 480FB4F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ACCD0 size=8
    let mut pc: u32 = 0x830ACCD0;
    'dispatch: loop {
        match pc {
            0x830ACCD0 => {
    //   block [0x830ACCD0..0x830ACCD8)
	// 830ACCD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ACCD4: 82180A68  lwz r16, 0xa68(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACCD8 size=164
    let mut pc: u32 = 0x830ACCD8;
    'dispatch: loop {
        match pc {
            0x830ACCD8 => {
    //   block [0x830ACCD8..0x830ACD7C)
	// 830ACCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACCE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830ACCE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830ACCE8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830ACCEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACCF0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830ACCF4: 807EBC4C  lwz r3, -0x43b4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17332 as u32) ) } as u64;
	// 830ACCF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830ACCFC: 409A0068  bne cr6, 0x830acd64
	if !ctx.cr[6].eq {
	pc = 0x830ACD64; continue 'dispatch;
	}
	// 830ACD00: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830ACD04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACD08: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830ACD0C: 4BF48ACD  bl 0x82ff57d8
	ctx.lr = 0x830ACD10;
	sub_82FF57D8(ctx, base);
	// 830ACD10: 817EBC4C  lwz r11, -0x43b4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17332 as u32) ) } as u64;
	// 830ACD14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ACD18: 409A0040  bne cr6, 0x830acd58
	if !ctx.cr[6].eq {
	pc = 0x830ACD58; continue 'dispatch;
	}
	// 830ACD1C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830ACD20: 4BF2B529  bl 0x82fd8248
	ctx.lr = 0x830ACD24;
	sub_82FD8248(ctx, base);
	// 830ACD24: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830ACD28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACD2C: 41820010  beq 0x830acd3c
	if ctx.cr[0].eq {
	pc = 0x830ACD3C; continue 'dispatch;
	}
	// 830ACD30: 4BF48A19  bl 0x82ff5748
	ctx.lr = 0x830ACD34;
	sub_82FF5748(ctx, base);
	// 830ACD34: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830ACD38: 48000008  b 0x830acd40
	pc = 0x830ACD40; continue 'dispatch;
	// 830ACD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830ACD40: 3D60830B  lis r11, -0x7cf5
	ctx.r[11].s64 = -2096431104;
	// 830ACD44: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830ACD48: 913EBC4C  stw r9, -0x43b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17332 as u32), ctx.r[9].u32 ) };
	// 830ACD4C: 388BCAE0  addi r4, r11, -0x3520
	ctx.r[4].s64 = ctx.r[11].s64 + -13600;
	// 830ACD50: 386ABC58  addi r3, r10, -0x43a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17320;
	// 830ACD54: 4BF4ADE5  bl 0x82ff7b38
	ctx.lr = 0x830ACD58;
	sub_82FF7B38(ctx, base);
	// 830ACD58: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACD5C: 4BF48AB5  bl 0x82ff5810
	ctx.lr = 0x830ACD60;
	sub_82FF5810(ctx, base);
	// 830ACD60: 807EBC4C  lwz r3, -0x43b4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17332 as u32) ) } as u64;
	// 830ACD64: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830ACD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACD70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830ACD74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830ACD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACD7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACD7C size=40
    let mut pc: u32 = 0x830ACD7C;
    'dispatch: loop {
        match pc {
            0x830ACD7C => {
    //   block [0x830ACD7C..0x830ACDA4)
	// 830ACD7C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830ACD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACD8C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACD90: 4BF48A81  bl 0x82ff5810
	ctx.lr = 0x830ACD94;
	sub_82FF5810(ctx, base);
	// 830ACD94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACDA4 size=40
    let mut pc: u32 = 0x830ACDA4;
    'dispatch: loop {
        match pc {
            0x830ACDA4 => {
    //   block [0x830ACDA4..0x830ACDCC)
	// 830ACDA4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830ACDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACDB4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830ACDB8: 4BF2B529  bl 0x82fd82e0
	ctx.lr = 0x830ACDBC;
	sub_82FD82E0(ctx, base);
	// 830ACDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ACDD0 size=8
    let mut pc: u32 = 0x830ACDD0;
    'dispatch: loop {
        match pc {
            0x830ACDD0 => {
    //   block [0x830ACDD0..0x830ACDD8)
	// 830ACDD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ACDD4: 82180AF4  lwz r16, 0xaf4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2804 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACDD8 size=196
    let mut pc: u32 = 0x830ACDD8;
    'dispatch: loop {
        match pc {
            0x830ACDD8 => {
    //   block [0x830ACDD8..0x830ACE9C)
	// 830ACDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACDDC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830ACDE0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830ACDE4: 480FB389  bl 0x831a816c
	ctx.lr = 0x830ACDE8;
	sub_831A8130(ctx, base);
	// 830ACDE8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830ACDEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACDF0: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 830ACDF4: 807DBC50  lwz r3, -0x43b0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-17328 as u32) ) } as u64;
	// 830ACDF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830ACDFC: 409A0088  bne cr6, 0x830ace84
	if !ctx.cr[6].eq {
	pc = 0x830ACE84; continue 'dispatch;
	}
	// 830ACE00: 4BFFFED9  bl 0x830accd8
	ctx.lr = 0x830ACE04;
	sub_830ACCD8(ctx, base);
	// 830ACE04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830ACE08: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACE0C: 4BF489CD  bl 0x82ff57d8
	ctx.lr = 0x830ACE10;
	sub_82FF57D8(ctx, base);
	// 830ACE10: 817DBC50  lwz r11, -0x43b0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-17328 as u32) ) } as u64;
	// 830ACE14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830ACE18: 409A0060  bne cr6, 0x830ace78
	if !ctx.cr[6].eq {
	pc = 0x830ACE78; continue 'dispatch;
	}
	// 830ACE1C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830ACE20: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830ACE24: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830ACE28: 4BF2B471  bl 0x82fd8298
	ctx.lr = 0x830ACE2C;
	sub_82FD8298(ctx, base);
	// 830ACE2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ACE30: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830ACE34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ACE38: 41820024  beq 0x830ace5c
	if ctx.cr[0].eq {
	pc = 0x830ACE5C; continue 'dispatch;
	}
	// 830ACE3C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830ACE40: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830ACE44: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 830ACE48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830ACE4C: 388B9630  addi r4, r11, -0x69d0
	ctx.r[4].s64 = ctx.r[11].s64 + -27088;
	// 830ACE50: 4BFDD299  bl 0x8308a0e8
	ctx.lr = 0x830ACE54;
	sub_8308A0E8(ctx, base);
	// 830ACE54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ACE58: 48000008  b 0x830ace60
	pc = 0x830ACE60; continue 'dispatch;
	// 830ACE5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ACE60: 907DBC50  stw r3, -0x43b0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-17328 as u32), ctx.r[3].u32 ) };
	// 830ACE64: 3D60830B  lis r11, -0x7cf5
	ctx.r[11].s64 = -2096431104;
	// 830ACE68: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830ACE6C: 388BCB38  addi r4, r11, -0x34c8
	ctx.r[4].s64 = ctx.r[11].s64 + -13512;
	// 830ACE70: 386ABC58  addi r3, r10, -0x43a8
	ctx.r[3].s64 = ctx.r[10].s64 + -17320;
	// 830ACE74: 4BF4ACC5  bl 0x82ff7b38
	ctx.lr = 0x830ACE78;
	sub_82FF7B38(ctx, base);
	// 830ACE78: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACE7C: 4BF48995  bl 0x82ff5810
	ctx.lr = 0x830ACE80;
	sub_82FF5810(ctx, base);
	// 830ACE80: 807DBC50  lwz r3, -0x43b0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-17328 as u32) ) } as u64;
	// 830ACE84: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830ACE88: 480FB334  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830ACE8C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACE90: 4BF48981  bl 0x82ff5810
	ctx.lr = 0x830ACE94;
	sub_82FF5810(ctx, base);
	// 830ACE94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ACE98: 4BFFFFEC  b 0x830ace84
	pc = 0x830ACE84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACE9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ACE9C size=8
    let mut pc: u32 = 0x830ACE9C;
    'dispatch: loop {
        match pc {
            0x830ACE9C => {
    //   block [0x830ACE9C..0x830ACEA4)
	// 830ACE9C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ACEA0: 82180AF4  lwz r16, 0xaf4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2804 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACEA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACEA4 size=20
    let mut pc: u32 = 0x830ACEA4;
    'dispatch: loop {
        match pc {
            0x830ACEA4 => {
    //   block [0x830ACEA4..0x830ACEB8)
	// 830ACEA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACEA8: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830ACEAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACEB0: 3863CE8C  addi r3, r3, -0x3174
	ctx.r[3].s64 = ctx.r[3].s64 + -12660;
	// 830ACEB4: 480FB308  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACEB8 size=40
    let mut pc: u32 = 0x830ACEB8;
    'dispatch: loop {
        match pc {
            0x830ACEB8 => {
    //   block [0x830ACEB8..0x830ACEE0)
	// 830ACEB8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830ACEBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACEC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACEC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACEC8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830ACECC: 4BF48945  bl 0x82ff5810
	ctx.lr = 0x830ACED0;
	sub_82FF5810(ctx, base);
	// 830ACED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACEE0 size=52
    let mut pc: u32 = 0x830ACEE0;
    'dispatch: loop {
        match pc {
            0x830ACEE0 => {
    //   block [0x830ACEE0..0x830ACF14)
	// 830ACEE0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830ACEE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACEE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ACEEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACEF0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830ACEF4: 396BB7E8  addi r11, r11, -0x4818
	ctx.r[11].s64 = ctx.r[11].s64 + -18456;
	// 830ACEF8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ACEFC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830ACF00: 4BF2B3E1  bl 0x82fd82e0
	ctx.lr = 0x830ACF04;
	sub_82FD82E0(ctx, base);
	// 830ACF04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ACF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ACF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ACF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ACF18 size=8
    let mut pc: u32 = 0x830ACF18;
    'dispatch: loop {
        match pc {
            0x830ACF18 => {
    //   block [0x830ACF18..0x830ACF20)
	// 830ACF18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ACF1C: 82180BB4  lwz r16, 0xbb4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2996 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ACF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ACF20 size=2064
    let mut pc: u32 = 0x830ACF20;
    'dispatch: loop {
        match pc {
            0x830ACF20 => {
    //   block [0x830ACF20..0x830AD730)
	// 830ACF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ACF24: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830ACF28: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830ACF2C: 480FB229  bl 0x831a8154
	ctx.lr = 0x830ACF30;
	sub_831A8130(ctx, base);
	// 830ACF30: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 830ACF34: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ACF38: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 830ACF3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830ACF40: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 830ACF44: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830ACF48: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 830ACF4C: 92FF00E4  stw r23, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[23].u32 ) };
	// 830ACF50: 2B18001E  cmplwi cr6, r24, 0x1e
	ctx.cr[6].compare_u32(ctx.r[24].u32, 30 as u32, &mut ctx.xer);
	// 830ACF54: 419907D0  bgt cr6, 0x830ad724
	if ctx.cr[6].gt {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830ACF58: 3D808218  lis r12, -0x7de8
	ctx.r[12].s64 = -2112356352;
	// 830ACF5C: 398C0970  addi r12, r12, 0x970
	ctx.r[12].s64 = ctx.r[12].s64 + 2416;
	// 830ACF60: 5700083C  slwi r0, r24, 1
	ctx.r[0].u32 = ctx.r[24].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 830ACF64: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 830ACF68: 3D80830B  lis r12, -0x7cf5
	ctx.r[12].s64 = -2096431104;
	// 830ACF6C: 398CCF80  addi r12, r12, -0x3080
	ctx.r[12].s64 = ctx.r[12].s64 + -12416;
	// 830ACF70: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830ACF74: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830ACF78: 60000000  nop
	// 830ACF7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 830ACF80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830ACF84: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830ACF88: 3B8B9678  addi r28, r11, -0x6988
	ctx.r[28].s64 = ctx.r[11].s64 + -27016;
	// 830ACF8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830ACF90: 836B9778  lwz r27, -0x6888(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26760 as u32) ) } as u64;
	// 830ACF94: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830ACF98: 40980028  bge cr6, 0x830acfc0
	if !ctx.cr[6].lt {
	pc = 0x830ACFC0; continue 'dispatch;
	}
	// 830ACF9C: 57AB3032  slwi r11, r29, 6
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830ACFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830ACFA4: 7C8BE214  add r4, r11, r28
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830ACFA8: 4BF26C99  bl 0x82fd3c40
	ctx.lr = 0x830ACFAC;
	sub_82FD3C40(ctx, base);
	// 830ACFAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ACFB0: 4082000C  bne 0x830acfbc
	if !ctx.cr[0].eq {
	pc = 0x830ACFBC; continue 'dispatch;
	}
	// 830ACFB4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830ACFB8: 4BFFFFDC  b 0x830acf94
	pc = 0x830ACF94; continue 'dispatch;
	// 830ACFBC: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830ACFC0: 409A075C  bne cr6, 0x830ad71c
	if !ctx.cr[6].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830ACFC4: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830ACFC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ACFCC: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ACFD0: 48000758  b 0x830ad728
	pc = 0x830AD728; continue 'dispatch;
	// 830ACFD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830ACFD8: 4BFE6C11  bl 0x83093be8
	ctx.lr = 0x830ACFDC;
	sub_83093BE8(ctx, base);
	// 830ACFDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ACFE0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830ACFE4: 409A0738  bne cr6, 0x830ad71c
	if !ctx.cr[6].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830ACFE8: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830ACFEC: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ACFF0: 48000734  b 0x830ad724
	pc = 0x830AD724; continue 'dispatch;
	// 830ACFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830ACFF8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830ACFFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD000: 4BFE6A21  bl 0x83093a20
	ctx.lr = 0x830AD004;
	sub_83093A20(ctx, base);
	// 830AD004: 4BFFFFD8  b 0x830acfdc
	pc = 0x830ACFDC; continue 'dispatch;
	// 830AD008: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AD00C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830AD010: 4BF2F7D9  bl 0x82fdc7e8
	ctx.lr = 0x830AD014;
	sub_82FDC7E8(ctx, base);
	// 830AD014: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD018: 48000710  b 0x830ad728
	pc = 0x830AD728; continue 'dispatch;
	// 830AD01C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD020: 409A004C  bne cr6, 0x830ad06c
	if !ctx.cr[6].eq {
	pc = 0x830AD06C; continue 'dispatch;
	}
	// 830AD024: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD028: 419A0034  beq cr6, 0x830ad05c
	if ctx.cr[6].eq {
	pc = 0x830AD05C; continue 'dispatch;
	}
	// 830AD02C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD030: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD034: 41820028  beq 0x830ad05c
	if ctx.cr[0].eq {
	pc = 0x830AD05C; continue 'dispatch;
	}
	// 830AD038: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD03C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD040: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD044: 4182000C  beq 0x830ad050
	if ctx.cr[0].eq {
	pc = 0x830AD050; continue 'dispatch;
	}
	// 830AD048: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD04C: 4BFFFFF0  b 0x830ad03c
	pc = 0x830AD03C; continue 'dispatch;
	// 830AD050: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD054: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD058: 48000008  b 0x830ad060
	pc = 0x830AD060; continue 'dispatch;
	// 830AD05C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD064: 4BF2C6E5  bl 0x82fd9748
	ctx.lr = 0x830AD068;
	sub_82FD9748(ctx, base);
	// 830AD068: 4BFFFFAC  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD06C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD070: 419A0034  beq cr6, 0x830ad0a4
	if ctx.cr[6].eq {
	pc = 0x830AD0A4; continue 'dispatch;
	}
	// 830AD074: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD078: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD07C: 41820028  beq 0x830ad0a4
	if ctx.cr[0].eq {
	pc = 0x830AD0A4; continue 'dispatch;
	}
	// 830AD080: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD084: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD088: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD08C: 4182000C  beq 0x830ad098
	if ctx.cr[0].eq {
	pc = 0x830AD098; continue 'dispatch;
	}
	// 830AD090: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD094: 4BFFFFF0  b 0x830ad084
	pc = 0x830AD084; continue 'dispatch;
	// 830AD098: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD09C: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD0A0: 48000008  b 0x830ad0a8
	pc = 0x830AD0A8; continue 'dispatch;
	// 830AD0A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD0AC: 4BF2C9DD  bl 0x82fd9a88
	ctx.lr = 0x830AD0B0;
	sub_82FD9A88(ctx, base);
	// 830AD0B0: 4BFFFF64  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD0B4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830AD0B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD0BC: 4BF25AFD  bl 0x82fd2bb8
	ctx.lr = 0x830AD0C0;
	sub_82FD2BB8(ctx, base);
	// 830AD0C0: 4BFFFF54  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD0C4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD0C8: 409A0028  bne cr6, 0x830ad0f0
	if !ctx.cr[6].eq {
	pc = 0x830AD0F0; continue 'dispatch;
	}
	// 830AD0CC: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD0D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD0D4: 41820648  beq 0x830ad71c
	if ctx.cr[0].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830AD0D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD0DC: 4BFFF5ED  bl 0x830ac6c8
	ctx.lr = 0x830AD0E0;
	sub_830AC6C8(ctx, base);
	// 830AD0E0: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 830AD0E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD0E8: 4082FFE4  bne 0x830ad0cc
	if !ctx.cr[0].eq {
	pc = 0x830AD0CC; continue 'dispatch;
	}
	// 830AD0EC: 48000638  b 0x830ad724
	pc = 0x830AD724; continue 'dispatch;
	// 830AD0F0: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD0F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD0F8: 41820624  beq 0x830ad71c
	if ctx.cr[0].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830AD0FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD100: 4BFFF619  bl 0x830ac718
	ctx.lr = 0x830AD104;
	sub_830AC718(ctx, base);
	// 830AD104: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 830AD108: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD10C: 4082FFE4  bne 0x830ad0f0
	if !ctx.cr[0].eq {
	pc = 0x830AD0F0; continue 'dispatch;
	}
	// 830AD110: 48000614  b 0x830ad724
	pc = 0x830AD724; continue 'dispatch;
	// 830AD114: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD118: 409A0040  bne cr6, 0x830ad158
	if !ctx.cr[6].eq {
	pc = 0x830AD158; continue 'dispatch;
	}
	// 830AD11C: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD120: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD124: 418205F8  beq 0x830ad71c
	if ctx.cr[0].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830AD128: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD12C: 4BFFF59D  bl 0x830ac6c8
	ctx.lr = 0x830AD130;
	sub_830AC6C8(ctx, base);
	// 830AD130: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD134: 418205F0  beq 0x830ad724
	if ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD138: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 830AD13C: 419A05E8  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD140: 2B1D000A  cmplwi cr6, r29, 0xa
	ctx.cr[6].compare_u32(ctx.r[29].u32, 10 as u32, &mut ctx.xer);
	// 830AD144: 419A05E0  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD148: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 830AD14C: 419A05D8  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD150: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 830AD154: 4BFFFFC8  b 0x830ad11c
	pc = 0x830AD11C; continue 'dispatch;
	// 830AD158: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD15C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD160: 418205BC  beq 0x830ad71c
	if ctx.cr[0].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830AD164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD168: 4BFFF5B1  bl 0x830ac718
	ctx.lr = 0x830AD16C;
	sub_830AC718(ctx, base);
	// 830AD16C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD170: 418205B4  beq 0x830ad724
	if ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD174: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 830AD178: 419A05AC  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD17C: 2B1D000A  cmplwi cr6, r29, 0xa
	ctx.cr[6].compare_u32(ctx.r[29].u32, 10 as u32, &mut ctx.xer);
	// 830AD180: 419A05A4  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD184: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 830AD188: 419A059C  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD18C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 830AD190: 4BFFFFC8  b 0x830ad158
	pc = 0x830AD158; continue 'dispatch;
	// 830AD194: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD198: 419A0034  beq cr6, 0x830ad1cc
	if ctx.cr[6].eq {
	pc = 0x830AD1CC; continue 'dispatch;
	}
	// 830AD19C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD1A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD1A4: 41820028  beq 0x830ad1cc
	if ctx.cr[0].eq {
	pc = 0x830AD1CC; continue 'dispatch;
	}
	// 830AD1A8: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD1AC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD1B0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD1B4: 4182000C  beq 0x830ad1c0
	if ctx.cr[0].eq {
	pc = 0x830AD1C0; continue 'dispatch;
	}
	// 830AD1B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD1BC: 4BFFFFF0  b 0x830ad1ac
	pc = 0x830AD1AC; continue 'dispatch;
	// 830AD1C0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD1C4: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD1C8: 48000008  b 0x830ad1d0
	pc = 0x830AD1D0; continue 'dispatch;
	// 830AD1CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AD1D0: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 830AD1D4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830AD1D8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD1DC: 409A008C  bne cr6, 0x830ad268
	if !ctx.cr[6].eq {
	pc = 0x830AD268; continue 'dispatch;
	}
	// 830AD1E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 830AD1E4: 3B8B2DD8  addi r28, r11, 0x2dd8
	ctx.r[28].s64 = ctx.r[11].s64 + 11736;
	// 830AD1E8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD1EC: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD1F0: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD1F4: 40820530  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD1F8: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AD1FC: 7D6BF22E  lhzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830AD200: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD204: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD208: 4082051C  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD20C: A07A0000  lhz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD210: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD214: 418200E4  beq 0x830ad2f8
	if ctx.cr[0].eq {
	pc = 0x830AD2F8; continue 'dispatch;
	}
	// 830AD218: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD21C: 4BFFF4AD  bl 0x830ac6c8
	ctx.lr = 0x830AD220;
	sub_830AC6C8(ctx, base);
	// 830AD220: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD224: 41820500  beq 0x830ad724
	if ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD228: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 830AD22C: 419A04F8  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD230: 2B1D000A  cmplwi cr6, r29, 0xa
	ctx.cr[6].compare_u32(ctx.r[29].u32, 10 as u32, &mut ctx.xer);
	// 830AD234: 419A04F0  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD238: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 830AD23C: 419A04E8  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD240: 7D7DE0AE  lbzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD244: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD248: 41820014  beq 0x830ad25c
	if ctx.cr[0].eq {
	pc = 0x830AD25C; continue 'dispatch;
	}
	// 830AD24C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD250: 408204D4  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD254: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 830AD258: 48000008  b 0x830ad260
	pc = 0x830AD260; continue 'dispatch;
	// 830AD25C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830AD260: 3B5A0002  addi r26, r26, 2
	ctx.r[26].s64 = ctx.r[26].s64 + 2;
	// 830AD264: 4BFFFFA8  b 0x830ad20c
	pc = 0x830AD20C; continue 'dispatch;
	// 830AD268: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 830AD26C: 3B8B2DD8  addi r28, r11, 0x2dd8
	ctx.r[28].s64 = ctx.r[11].s64 + 11736;
	// 830AD270: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD274: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD278: 556BC9FE  srwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AD27C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD280: 408204A4  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD284: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AD288: 7D6BF22E  lhzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 830AD28C: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD290: 556BC9FE  srwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AD294: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD298: 4082048C  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD29C: A07A0000  lhz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD2A0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD2A4: 41820054  beq 0x830ad2f8
	if ctx.cr[0].eq {
	pc = 0x830AD2F8; continue 'dispatch;
	}
	// 830AD2A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD2AC: 4BFFF46D  bl 0x830ac718
	ctx.lr = 0x830AD2B0;
	sub_830AC718(ctx, base);
	// 830AD2B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD2B4: 41820470  beq 0x830ad724
	if ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD2B8: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 830AD2BC: 419A0468  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD2C0: 2B1D000A  cmplwi cr6, r29, 0xa
	ctx.cr[6].compare_u32(ctx.r[29].u32, 10 as u32, &mut ctx.xer);
	// 830AD2C4: 419A0460  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD2C8: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 830AD2CC: 419A0458  beq cr6, 0x830ad724
	if ctx.cr[6].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD2D0: 7D7DE0AE  lbzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830AD2D4: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD2D8: 41820014  beq 0x830ad2ec
	if ctx.cr[0].eq {
	pc = 0x830AD2EC; continue 'dispatch;
	}
	// 830AD2DC: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD2E0: 40820444  bne 0x830ad724
	if !ctx.cr[0].eq {
	pc = 0x830AD724; continue 'dispatch;
	}
	// 830AD2E4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 830AD2E8: 48000008  b 0x830ad2f0
	pc = 0x830AD2F0; continue 'dispatch;
	// 830AD2EC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830AD2F0: 3B5A0002  addi r26, r26, 2
	ctx.r[26].s64 = ctx.r[26].s64 + 2;
	// 830AD2F4: 4BFFFFA8  b 0x830ad29c
	pc = 0x830AD29C; continue 'dispatch;
	// 830AD2F8: 2F180015  cmpwi cr6, r24, 0x15
	ctx.cr[6].compare_i32(ctx.r[24].s32, 21, &mut ctx.xer);
	// 830AD2FC: 409A0420  bne cr6, 0x830ad71c
	if !ctx.cr[6].eq {
	pc = 0x830AD71C; continue 'dispatch;
	}
	// 830AD300: 4BFFFAD9  bl 0x830acdd8
	ctx.lr = 0x830AD304;
	sub_830ACDD8(ctx, base);
	// 830AD304: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD308: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD30C: 4082000C  bne 0x830ad318
	if !ctx.cr[0].eq {
	pc = 0x830AD318; continue 'dispatch;
	}
	// 830AD310: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 830AD314: 4BFFFCD8  b 0x830acfec
	pc = 0x830ACFEC; continue 'dispatch;
	// 830AD318: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830AD31C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AD320: 4BFDD3F1  bl 0x8308a710
	ctx.lr = 0x830AD324;
	sub_8308A710(ctx, base);
	// 830AD324: 4BFFFCF0  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD328: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD32C: 409A004C  bne cr6, 0x830ad378
	if !ctx.cr[6].eq {
	pc = 0x830AD378; continue 'dispatch;
	}
	// 830AD330: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD334: 419A0034  beq cr6, 0x830ad368
	if ctx.cr[6].eq {
	pc = 0x830AD368; continue 'dispatch;
	}
	// 830AD338: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD33C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD340: 41820028  beq 0x830ad368
	if ctx.cr[0].eq {
	pc = 0x830AD368; continue 'dispatch;
	}
	// 830AD344: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD348: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD34C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD350: 4182000C  beq 0x830ad35c
	if ctx.cr[0].eq {
	pc = 0x830AD35C; continue 'dispatch;
	}
	// 830AD354: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD358: 4BFFFFF0  b 0x830ad348
	pc = 0x830AD348; continue 'dispatch;
	// 830AD35C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD360: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD364: 48000008  b 0x830ad36c
	pc = 0x830AD36C; continue 'dispatch;
	// 830AD368: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD36C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD370: 4BF2C341  bl 0x82fd96b0
	ctx.lr = 0x830AD374;
	sub_82FD96B0(ctx, base);
	// 830AD374: 4BFFFCA0  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD378: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD37C: 419A0034  beq cr6, 0x830ad3b0
	if ctx.cr[6].eq {
	pc = 0x830AD3B0; continue 'dispatch;
	}
	// 830AD380: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD384: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD388: 41820028  beq 0x830ad3b0
	if ctx.cr[0].eq {
	pc = 0x830AD3B0; continue 'dispatch;
	}
	// 830AD38C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD390: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD394: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD398: 4182000C  beq 0x830ad3a4
	if ctx.cr[0].eq {
	pc = 0x830AD3A4; continue 'dispatch;
	}
	// 830AD39C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD3A0: 4BFFFFF0  b 0x830ad390
	pc = 0x830AD390; continue 'dispatch;
	// 830AD3A4: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD3A8: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD3AC: 48000008  b 0x830ad3b4
	pc = 0x830AD3B4; continue 'dispatch;
	// 830AD3B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD3B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD3B8: 4BF2C571  bl 0x82fd9928
	ctx.lr = 0x830AD3BC;
	sub_82FD9928(ctx, base);
	// 830AD3BC: 4BFFFC58  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD3C0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AD3C4: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 830AD3C8: 396B0944  addi r11, r11, 0x944
	ctx.r[11].s64 = ctx.r[11].s64 + 2372;
	// 830AD3CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AD3D0: 38ABFFFC  addi r5, r11, -4
	ctx.r[5].s64 = ctx.r[11].s64 + -4;
	// 830AD3D4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD3D8: 4BF87291  bl 0x83034668
	ctx.lr = 0x830AD3DC;
	sub_83034668(ctx, base);
	// 830AD3DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD3E0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD3E4: 409A0080  bne cr6, 0x830ad464
	if !ctx.cr[6].eq {
	pc = 0x830AD464; continue 'dispatch;
	}
	// 830AD3E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD3EC: 4BF8709D  bl 0x83034488
	ctx.lr = 0x830AD3F0;
	sub_83034488(ctx, base);
	// 830AD3F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD3F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD3F8: 418200DC  beq 0x830ad4d4
	if ctx.cr[0].eq {
	pc = 0x830AD4D4; continue 'dispatch;
	}
	// 830AD3FC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD400: 4BF87409  bl 0x83034808
	ctx.lr = 0x830AD404;
	sub_83034808(ctx, base);
	// 830AD404: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD408: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD40C: 41820034  beq 0x830ad440
	if ctx.cr[0].eq {
	pc = 0x830AD440; continue 'dispatch;
	}
	// 830AD410: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD414: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD418: 41820028  beq 0x830ad440
	if ctx.cr[0].eq {
	pc = 0x830AD440; continue 'dispatch;
	}
	// 830AD41C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830AD420: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD424: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD428: 4182000C  beq 0x830ad434
	if ctx.cr[0].eq {
	pc = 0x830AD434; continue 'dispatch;
	}
	// 830AD42C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD430: 4BFFFFF0  b 0x830ad420
	pc = 0x830AD420; continue 'dispatch;
	// 830AD434: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830AD438: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD43C: 48000008  b 0x830ad444
	pc = 0x830AD444; continue 'dispatch;
	// 830AD440: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD444: 4BF2C26D  bl 0x82fd96b0
	ctx.lr = 0x830AD448;
	sub_82FD96B0(ctx, base);
	// 830AD448: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD44C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD450: 4082FF98  bne 0x830ad3e8
	if !ctx.cr[0].eq {
	pc = 0x830AD3E8; continue 'dispatch;
	}
	// 830AD454: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD458: 4BF87029  bl 0x83034480
	ctx.lr = 0x830AD45C;
	sub_83034480(ctx, base);
	// 830AD45C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD460: 480002C4  b 0x830ad724
	pc = 0x830AD724; continue 'dispatch;
	// 830AD464: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD468: 4BF87021  bl 0x83034488
	ctx.lr = 0x830AD46C;
	sub_83034488(ctx, base);
	// 830AD46C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD470: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD474: 41820060  beq 0x830ad4d4
	if ctx.cr[0].eq {
	pc = 0x830AD4D4; continue 'dispatch;
	}
	// 830AD478: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD47C: 4BF8738D  bl 0x83034808
	ctx.lr = 0x830AD480;
	sub_83034808(ctx, base);
	// 830AD480: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD484: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD488: 41820034  beq 0x830ad4bc
	if ctx.cr[0].eq {
	pc = 0x830AD4BC; continue 'dispatch;
	}
	// 830AD48C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD490: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD494: 41820028  beq 0x830ad4bc
	if ctx.cr[0].eq {
	pc = 0x830AD4BC; continue 'dispatch;
	}
	// 830AD498: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830AD49C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD4A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD4A4: 4182000C  beq 0x830ad4b0
	if ctx.cr[0].eq {
	pc = 0x830AD4B0; continue 'dispatch;
	}
	// 830AD4A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD4AC: 4BFFFFF0  b 0x830ad49c
	pc = 0x830AD49C; continue 'dispatch;
	// 830AD4B0: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830AD4B4: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD4B8: 48000008  b 0x830ad4c0
	pc = 0x830AD4C0; continue 'dispatch;
	// 830AD4BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD4C0: 4BF2C469  bl 0x82fd9928
	ctx.lr = 0x830AD4C4;
	sub_82FD9928(ctx, base);
	// 830AD4C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD4C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD4CC: 4082FF98  bne 0x830ad464
	if !ctx.cr[0].eq {
	pc = 0x830AD464; continue 'dispatch;
	}
	// 830AD4D0: 4BFFFF84  b 0x830ad454
	pc = 0x830AD454; continue 'dispatch;
	// 830AD4D4: 4800023C  b 0x830ad710
	pc = 0x830AD710; continue 'dispatch;
	// 830AD4D8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD4DC: 409A004C  bne cr6, 0x830ad528
	if !ctx.cr[6].eq {
	pc = 0x830AD528; continue 'dispatch;
	}
	// 830AD4E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD4E4: 419A0034  beq cr6, 0x830ad518
	if ctx.cr[6].eq {
	pc = 0x830AD518; continue 'dispatch;
	}
	// 830AD4E8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD4EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD4F0: 41820028  beq 0x830ad518
	if ctx.cr[0].eq {
	pc = 0x830AD518; continue 'dispatch;
	}
	// 830AD4F4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD4F8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD4FC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD500: 4182000C  beq 0x830ad50c
	if ctx.cr[0].eq {
	pc = 0x830AD50C; continue 'dispatch;
	}
	// 830AD504: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD508: 4BFFFFF0  b 0x830ad4f8
	pc = 0x830AD4F8; continue 'dispatch;
	// 830AD50C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD510: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD514: 48000008  b 0x830ad51c
	pc = 0x830AD51C; continue 'dispatch;
	// 830AD518: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD520: 4BF2C1D9  bl 0x82fd96f8
	ctx.lr = 0x830AD524;
	sub_82FD96F8(ctx, base);
	// 830AD524: 4BFFFAF0  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD528: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD52C: 419A0034  beq cr6, 0x830ad560
	if ctx.cr[6].eq {
	pc = 0x830AD560; continue 'dispatch;
	}
	// 830AD530: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD534: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD538: 41820028  beq 0x830ad560
	if ctx.cr[0].eq {
	pc = 0x830AD560; continue 'dispatch;
	}
	// 830AD53C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD540: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD544: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD548: 4182000C  beq 0x830ad554
	if ctx.cr[0].eq {
	pc = 0x830AD554; continue 'dispatch;
	}
	// 830AD54C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD550: 4BFFFFF0  b 0x830ad540
	pc = 0x830AD540; continue 'dispatch;
	// 830AD554: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD558: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD55C: 48000008  b 0x830ad564
	pc = 0x830AD564; continue 'dispatch;
	// 830AD560: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD568: 4BF2C459  bl 0x82fd99c0
	ctx.lr = 0x830AD56C;
	sub_82FD99C0(ctx, base);
	// 830AD56C: 4BFFFAA8  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD570: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD574: 409A004C  bne cr6, 0x830ad5c0
	if !ctx.cr[6].eq {
	pc = 0x830AD5C0; continue 'dispatch;
	}
	// 830AD578: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD57C: 419A0034  beq cr6, 0x830ad5b0
	if ctx.cr[6].eq {
	pc = 0x830AD5B0; continue 'dispatch;
	}
	// 830AD580: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD584: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD588: 41820028  beq 0x830ad5b0
	if ctx.cr[0].eq {
	pc = 0x830AD5B0; continue 'dispatch;
	}
	// 830AD58C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD590: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD594: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD598: 4182000C  beq 0x830ad5a4
	if ctx.cr[0].eq {
	pc = 0x830AD5A4; continue 'dispatch;
	}
	// 830AD59C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD5A0: 4BFFFFF0  b 0x830ad590
	pc = 0x830AD590; continue 'dispatch;
	// 830AD5A4: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD5A8: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD5AC: 48000008  b 0x830ad5b4
	pc = 0x830AD5B4; continue 'dispatch;
	// 830AD5B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD5B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD5B8: 4BF2C099  bl 0x82fd9650
	ctx.lr = 0x830AD5BC;
	sub_82FD9650(ctx, base);
	// 830AD5BC: 4BFFFA58  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD5C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830AD5C4: 419A0034  beq cr6, 0x830ad5f8
	if ctx.cr[6].eq {
	pc = 0x830AD5F8; continue 'dispatch;
	}
	// 830AD5C8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD5CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD5D0: 41820028  beq 0x830ad5f8
	if ctx.cr[0].eq {
	pc = 0x830AD5F8; continue 'dispatch;
	}
	// 830AD5D4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830AD5D8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD5DC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD5E0: 4182000C  beq 0x830ad5ec
	if ctx.cr[0].eq {
	pc = 0x830AD5EC; continue 'dispatch;
	}
	// 830AD5E4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD5E8: 4BFFFFF0  b 0x830ad5d8
	pc = 0x830AD5D8; continue 'dispatch;
	// 830AD5EC: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830AD5F0: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD5F4: 48000008  b 0x830ad5fc
	pc = 0x830AD5FC; continue 'dispatch;
	// 830AD5F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD5FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD600: 4BF2C251  bl 0x82fd9850
	ctx.lr = 0x830AD604;
	sub_82FD9850(ctx, base);
	// 830AD604: 4BFFFA10  b 0x830ad014
	pc = 0x830AD014; continue 'dispatch;
	// 830AD608: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AD60C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 830AD610: 38AB0944  addi r5, r11, 0x944
	ctx.r[5].s64 = ctx.r[11].s64 + 2372;
	// 830AD614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AD618: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD61C: 4BF8704D  bl 0x83034668
	ctx.lr = 0x830AD620;
	sub_83034668(ctx, base);
	// 830AD620: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD624: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AD628: 409A0078  bne cr6, 0x830ad6a0
	if !ctx.cr[6].eq {
	pc = 0x830AD6A0; continue 'dispatch;
	}
	// 830AD62C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD630: 4BF86E59  bl 0x83034488
	ctx.lr = 0x830AD634;
	sub_83034488(ctx, base);
	// 830AD634: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD638: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD63C: 418200D4  beq 0x830ad710
	if ctx.cr[0].eq {
	pc = 0x830AD710; continue 'dispatch;
	}
	// 830AD640: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD644: 4BF871C5  bl 0x83034808
	ctx.lr = 0x830AD648;
	sub_83034808(ctx, base);
	// 830AD648: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD64C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD650: 41820034  beq 0x830ad684
	if ctx.cr[0].eq {
	pc = 0x830AD684; continue 'dispatch;
	}
	// 830AD654: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD658: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD65C: 41820028  beq 0x830ad684
	if ctx.cr[0].eq {
	pc = 0x830AD684; continue 'dispatch;
	}
	// 830AD660: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830AD664: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD668: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD66C: 4182000C  beq 0x830ad678
	if ctx.cr[0].eq {
	pc = 0x830AD678; continue 'dispatch;
	}
	// 830AD670: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD674: 4BFFFFF0  b 0x830ad664
	pc = 0x830AD664; continue 'dispatch;
	// 830AD678: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830AD67C: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD680: 48000008  b 0x830ad688
	pc = 0x830AD688; continue 'dispatch;
	// 830AD684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD688: 4BF2BFC9  bl 0x82fd9650
	ctx.lr = 0x830AD68C;
	sub_82FD9650(ctx, base);
	// 830AD68C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD690: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD694: 4082FF98  bne 0x830ad62c
	if !ctx.cr[0].eq {
	pc = 0x830AD62C; continue 'dispatch;
	}
	// 830AD698: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD69C: 4BFFFDBC  b 0x830ad458
	pc = 0x830AD458; continue 'dispatch;
	// 830AD6A0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD6A4: 4BF86DE5  bl 0x83034488
	ctx.lr = 0x830AD6A8;
	sub_83034488(ctx, base);
	// 830AD6A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD6AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD6B0: 41820060  beq 0x830ad710
	if ctx.cr[0].eq {
	pc = 0x830AD710; continue 'dispatch;
	}
	// 830AD6B4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD6B8: 4BF87151  bl 0x83034808
	ctx.lr = 0x830AD6BC;
	sub_83034808(ctx, base);
	// 830AD6BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD6C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD6C4: 41820034  beq 0x830ad6f8
	if ctx.cr[0].eq {
	pc = 0x830AD6F8; continue 'dispatch;
	}
	// 830AD6C8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD6CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD6D0: 41820028  beq 0x830ad6f8
	if ctx.cr[0].eq {
	pc = 0x830AD6F8; continue 'dispatch;
	}
	// 830AD6D4: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830AD6D8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AD6DC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD6E0: 4182000C  beq 0x830ad6ec
	if ctx.cr[0].eq {
	pc = 0x830AD6EC; continue 'dispatch;
	}
	// 830AD6E4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830AD6E8: 4BFFFFF0  b 0x830ad6d8
	pc = 0x830AD6D8; continue 'dispatch;
	// 830AD6EC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830AD6F0: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830AD6F4: 48000008  b 0x830ad6fc
	pc = 0x830AD6FC; continue 'dispatch;
	// 830AD6F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AD6FC: 4BF2C155  bl 0x82fd9850
	ctx.lr = 0x830AD700;
	sub_82FD9850(ctx, base);
	// 830AD700: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD704: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD708: 4082FF98  bne 0x830ad6a0
	if !ctx.cr[0].eq {
	pc = 0x830AD6A0; continue 'dispatch;
	}
	// 830AD70C: 4BFFFF8C  b 0x830ad698
	pc = 0x830AD698; continue 'dispatch;
	// 830AD710: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD714: 4BF86D6D  bl 0x83034480
	ctx.lr = 0x830AD718;
	sub_83034480(ctx, base);
	// 830AD718: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD71C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830AD720: 48000008  b 0x830ad728
	pc = 0x830AD728; continue 'dispatch;
	// 830AD724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AD728: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 830AD72C: 480FAA78  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AD730 size=8
    let mut pc: u32 = 0x830AD730;
    'dispatch: loop {
        match pc {
            0x830AD730 => {
    //   block [0x830AD730..0x830AD738)
	// 830AD730: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AD734: 82180BB4  lwz r16, 0xbb4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(2996 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AD738 size=36
    let mut pc: u32 = 0x830AD738;
    'dispatch: loop {
        match pc {
            0x830AD738 => {
    //   block [0x830AD738..0x830AD75C)
	// 830AD738: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830AD73C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AD740: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830AD744: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830AD748: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830AD74C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AD750: 3863D724  addi r3, r3, -0x28dc
	ctx.r[3].s64 = ctx.r[3].s64 + -10460;
	// 830AD754: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AD758: 480FAA4C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD75C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AD75C size=40
    let mut pc: u32 = 0x830AD75C;
    'dispatch: loop {
        match pc {
            0x830AD75C => {
    //   block [0x830AD75C..0x830AD784)
	// 830AD75C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830AD760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AD764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AD768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AD76C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD770: 4BF86D11  bl 0x83034480
	ctx.lr = 0x830AD774;
	sub_83034480(ctx, base);
	// 830AD774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AD778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AD77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AD780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AD784 size=40
    let mut pc: u32 = 0x830AD784;
    'dispatch: loop {
        match pc {
            0x830AD784 => {
    //   block [0x830AD784..0x830AD7AC)
	// 830AD784: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830AD788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AD78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AD790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AD794: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AD798: 4BF86CE9  bl 0x83034480
	ctx.lr = 0x830AD79C;
	sub_83034480(ctx, base);
	// 830AD79C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AD7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AD7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AD7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AD7B0 size=8
    let mut pc: u32 = 0x830AD7B0;
    'dispatch: loop {
        match pc {
            0x830AD7B0 => {
    //   block [0x830AD7B0..0x830AD7B8)
	// 830AD7B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AD7B4: 82180D24  lwz r16, 0xd24(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3364 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AD7B8 size=568
    let mut pc: u32 = 0x830AD7B8;
    'dispatch: loop {
        match pc {
            0x830AD7B8 => {
    //   block [0x830AD7B8..0x830AD9F0)
	// 830AD7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AD7BC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830AD7C0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830AD7C4: 480FA9A1  bl 0x831a8164
	ctx.lr = 0x830AD7C8;
	sub_831A8130(ctx, base);
	// 830AD7C8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830AD7CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AD7D0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830AD7D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AD7D8: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 830AD7DC: 937F00B4  stw r27, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 830AD7E0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830AD7E4: 419A01D0  beq cr6, 0x830ad9b4
	if ctx.cr[6].eq {
	pc = 0x830AD9B4; continue 'dispatch;
	}
	// 830AD7E8: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 830AD7EC: 419A00F4  beq cr6, 0x830ad8e0
	if ctx.cr[6].eq {
	pc = 0x830AD8E0; continue 'dispatch;
	}
	// 830AD7F0: 2F04000E  cmpwi cr6, r4, 0xe
	ctx.cr[6].compare_i32(ctx.r[4].s32, 14, &mut ctx.xer);
	// 830AD7F4: 419A008C  beq cr6, 0x830ad880
	if ctx.cr[6].eq {
	pc = 0x830AD880; continue 'dispatch;
	}
	// 830AD7F8: 2F04000F  cmpwi cr6, r4, 0xf
	ctx.cr[6].compare_i32(ctx.r[4].s32, 15, &mut ctx.xer);
	// 830AD7FC: 419A0018  beq cr6, 0x830ad814
	if ctx.cr[6].eq {
	pc = 0x830AD814; continue 'dispatch;
	}
	// 830AD800: 4099000C  ble cr6, 0x830ad80c
	if !ctx.cr[6].gt {
	pc = 0x830AD80C; continue 'dispatch;
	}
	// 830AD804: 2F04001E  cmpwi cr6, r4, 0x1e
	ctx.cr[6].compare_i32(ctx.r[4].s32, 30, &mut ctx.xer);
	// 830AD808: 409901AC  ble cr6, 0x830ad9b4
	if !ctx.cr[6].gt {
	pc = 0x830AD9B4; continue 'dispatch;
	}
	// 830AD80C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AD810: 480001D4  b 0x830ad9e4
	pc = 0x830AD9E4; continue 'dispatch;
	// 830AD814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD818: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830AD81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AD820: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AD824: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830AD828: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830AD82C: 4BFE6015  bl 0x83093840
	ctx.lr = 0x830AD830;
	sub_83093840(ctx, base);
	// 830AD830: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830AD834: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD838: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD83C: 4082000C  bne 0x830ad848
	if !ctx.cr[0].eq {
	pc = 0x830AD848; continue 'dispatch;
	}
	// 830AD840: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830AD844: 48000198  b 0x830ad9dc
	pc = 0x830AD9DC; continue 'dispatch;
	// 830AD848: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AD84C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AD850: 4BF2AA49  bl 0x82fd8298
	ctx.lr = 0x830AD854;
	sub_82FD8298(ctx, base);
	// 830AD854: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD858: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD85C: 41820018  beq 0x830ad874
	if ctx.cr[0].eq {
	pc = 0x830AD874; continue 'dispatch;
	}
	// 830AD860: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 830AD864: 9BC30028  stb r30, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 830AD868: 93A3002C  stw r29, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830AD86C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AD870: 48000008  b 0x830ad878
	pc = 0x830AD878; continue 'dispatch;
	// 830AD874: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD878: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830AD87C: 48000058  b 0x830ad8d4
	pc = 0x830AD8D4; continue 'dispatch;
	// 830AD880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AD884: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD888: 4BFE63E1  bl 0x83093c68
	ctx.lr = 0x830AD88C;
	sub_83093C68(ctx, base);
	// 830AD88C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AD890: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD894: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD898: 4182FFA8  beq 0x830ad840
	if ctx.cr[0].eq {
	pc = 0x830AD840; continue 'dispatch;
	}
	// 830AD89C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AD8A0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AD8A4: 4BF2A9F5  bl 0x82fd8298
	ctx.lr = 0x830AD8A8;
	sub_82FD8298(ctx, base);
	// 830AD8A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD8AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD8B0: 4182001C  beq 0x830ad8cc
	if ctx.cr[0].eq {
	pc = 0x830AD8CC; continue 'dispatch;
	}
	// 830AD8B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AD8B8: 93A3002C  stw r29, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830AD8BC: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 830AD8C0: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AD8C4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AD8C8: 48000008  b 0x830ad8d0
	pc = 0x830AD8D0; continue 'dispatch;
	// 830AD8CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AD8D0: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830AD8D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AD8D8: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AD8DC: 48000108  b 0x830ad9e4
	pc = 0x830AD9E4; continue 'dispatch;
	// 830AD8E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AD8E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD8E8: 3B8B9678  addi r28, r11, -0x6988
	ctx.r[28].s64 = ctx.r[11].s64 + -27016;
	// 830AD8EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AD8F0: 4BF26351  bl 0x82fd3c40
	ctx.lr = 0x830AD8F4;
	sub_82FD3C40(ctx, base);
	// 830AD8F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD8F8: 4082007C  bne 0x830ad974
	if !ctx.cr[0].eq {
	pc = 0x830AD974; continue 'dispatch;
	}
	// 830AD8FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD900: 389C0080  addi r4, r28, 0x80
	ctx.r[4].s64 = ctx.r[28].s64 + 128;
	// 830AD904: 4BF2633D  bl 0x82fd3c40
	ctx.lr = 0x830AD908;
	sub_82FD3C40(ctx, base);
	// 830AD908: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD90C: 40820068  bne 0x830ad974
	if !ctx.cr[0].eq {
	pc = 0x830AD974; continue 'dispatch;
	}
	// 830AD910: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD914: 389C0040  addi r4, r28, 0x40
	ctx.r[4].s64 = ctx.r[28].s64 + 64;
	// 830AD918: 4BF26329  bl 0x82fd3c40
	ctx.lr = 0x830AD91C;
	sub_82FD3C40(ctx, base);
	// 830AD91C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD920: 40820018  bne 0x830ad938
	if !ctx.cr[0].eq {
	pc = 0x830AD938; continue 'dispatch;
	}
	// 830AD924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD928: 389C00C0  addi r4, r28, 0xc0
	ctx.r[4].s64 = ctx.r[28].s64 + 192;
	// 830AD92C: 4BF26315  bl 0x82fd3c40
	ctx.lr = 0x830AD930;
	sub_82FD3C40(ctx, base);
	// 830AD930: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD934: 4182FF0C  beq 0x830ad840
	if ctx.cr[0].eq {
	pc = 0x830AD840; continue 'dispatch;
	}
	// 830AD938: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AD93C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AD940: 4BF2A959  bl 0x82fd8298
	ctx.lr = 0x830AD944;
	sub_82FD8298(ctx, base);
	// 830AD944: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD94C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830AD950: 41820018  beq 0x830ad968
	if ctx.cr[0].eq {
	pc = 0x830AD968; continue 'dispatch;
	}
	// 830AD954: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AD958: 9BC30028  stb r30, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 830AD95C: 93A3002C  stw r29, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830AD960: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AD964: 48000008  b 0x830ad96c
	pc = 0x830AD96C; continue 'dispatch;
	// 830AD968: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD96C: 9BC30008  stb r30, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 830AD970: 48000074  b 0x830ad9e4
	pc = 0x830AD9E4; continue 'dispatch;
	// 830AD974: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AD978: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AD97C: 4BF2A91D  bl 0x82fd8298
	ctx.lr = 0x830AD980;
	sub_82FD8298(ctx, base);
	// 830AD980: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD984: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AD988: 4182001C  beq 0x830ad9a4
	if ctx.cr[0].eq {
	pc = 0x830AD9A4; continue 'dispatch;
	}
	// 830AD98C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AD990: 93A3002C  stw r29, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830AD994: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AD998: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AD99C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AD9A0: 48000008  b 0x830ad9a8
	pc = 0x830AD9A8; continue 'dispatch;
	// 830AD9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AD9A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AD9AC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 830AD9B0: 48000034  b 0x830ad9e4
	pc = 0x830AD9E4; continue 'dispatch;
	// 830AD9B4: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD9B8: 41820020  beq 0x830ad9d8
	if ctx.cr[0].eq {
	pc = 0x830AD9D8; continue 'dispatch;
	}
	// 830AD9BC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830AD9C0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830AD9C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AD9C8: 4BFFF559  bl 0x830acf20
	ctx.lr = 0x830AD9CC;
	sub_830ACF20(ctx, base);
	// 830AD9CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AD9D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AD9D4: 4182FE6C  beq 0x830ad840
	if ctx.cr[0].eq {
	pc = 0x830AD840; continue 'dispatch;
	}
	// 830AD9D8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830AD9DC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AD9E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AD9E4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830AD9E8: 480FA7CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830AD9EC: 4BFFFFF4  b 0x830ad9e0
	pc = 0x830AD9E0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AD9F0 size=8
    let mut pc: u32 = 0x830AD9F0;
    'dispatch: loop {
        match pc {
            0x830AD9F0 => {
    //   block [0x830AD9F0..0x830AD9F8)
	// 830AD9F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AD9F4: 82180D24  lwz r16, 0xd24(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3364 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AD9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AD9F8 size=36
    let mut pc: u32 = 0x830AD9F8;
    'dispatch: loop {
        match pc {
            0x830AD9F8 => {
    //   block [0x830AD9F8..0x830ADA1C)
	// 830AD9F8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830AD9FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ADA00: 815F00B4  lwz r10, 0xb4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 830ADA04: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830ADA08: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830ADA0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ADA10: 3863D9EC  addi r3, r3, -0x2614
	ctx.r[3].s64 = ctx.r[3].s64 + -9748;
	// 830ADA14: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADA18: 480FA79C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ADA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ADA20 size=8
    let mut pc: u32 = 0x830ADA20;
    'dispatch: loop {
        match pc {
            0x830ADA20 => {
    //   block [0x830ADA20..0x830ADA28)
	// 830ADA20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ADA24: 82180D58  lwz r16, 0xd58(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ADA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ADA28 size=308
    let mut pc: u32 = 0x830ADA28;
    'dispatch: loop {
        match pc {
            0x830ADA28 => {
    //   block [0x830ADA28..0x830ADB5C)
	// 830ADA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ADA2C: 480FA729  bl 0x831a8154
	ctx.lr = 0x830ADA30;
	sub_831A8130(ctx, base);
	// 830ADA30: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830ADA34: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ADA38: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 830ADA3C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 830ADA40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADA44: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830ADA48: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830ADA4C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 830ADA50: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 830ADA54: 4BF23975  bl 0x82fd13c8
	ctx.lr = 0x830ADA58;
	sub_82FD13C8(ctx, base);
	// 830ADA58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830ADA5C: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 830ADA60: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 830ADA64: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830ADA68: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 830ADA6C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ADA70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830ADA74: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 830ADA78: 409AFFF4  bne cr6, 0x830ada6c
	if !ctx.cr[6].eq {
	pc = 0x830ADA6C; continue 'dispatch;
	}
	// 830ADA7C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830ADA80: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830ADA84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830ADA88: 5578003E  slwi r24, r11, 0
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 830ADA8C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830ADA90: 481032D1  bl 0x831b0d60
	ctx.lr = 0x830ADA94;
	sub_831B0D60(ctx, base);
	// 830ADA94: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 830ADA98: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830ADA9C: 409A0018  bne cr6, 0x830adab4
	if !ctx.cr[6].eq {
	pc = 0x830ADAB4; continue 'dispatch;
	}
	// 830ADAA0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830ADAA4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830ADAA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ADAAC: 480FCA65  bl 0x831aa510
	ctx.lr = 0x830ADAB0;
	sub_831AA510(ctx, base);
	// 830ADAB0: 48000050  b 0x830adb00
	pc = 0x830ADB00; continue 'dispatch;
	// 830ADAB4: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 830ADAB8: 409A004C  bne cr6, 0x830adb04
	if !ctx.cr[6].eq {
	pc = 0x830ADB04; continue 'dispatch;
	}
	// 830ADABC: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 830ADAC0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830ADAC4: 4BF242ED  bl 0x82fd1db0
	ctx.lr = 0x830ADAC8;
	sub_82FD1DB0(ctx, base);
	// 830ADAC8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830ADACC: 419A0024  beq cr6, 0x830adaf0
	if ctx.cr[6].eq {
	pc = 0x830ADAF0; continue 'dispatch;
	}
	// 830ADAD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830ADAD4: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADAD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ADADC: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830ADAE0: 4BF24FE1  bl 0x82fd2ac0
	ctx.lr = 0x830ADAE4;
	sub_82FD2AC0(ctx, base);
	// 830ADAE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830ADAE8: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 830ADAEC: 480FA6B8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 830ADAF0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830ADAF4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830ADAF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ADAFC: 480FCA35  bl 0x831aa530
	ctx.lr = 0x830ADB00;
	sub_831AA530(ctx, base);
	// 830ADB00: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830ADB04: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ADB08: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 830ADB0C: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 830ADB10: 419A0018  beq cr6, 0x830adb28
	if ctx.cr[6].eq {
	pc = 0x830ADB28; continue 'dispatch;
	}
	// 830ADB14: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830ADB18: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADB1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ADB20: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830ADB24: 4BFFFFBC  b 0x830adae0
	pc = 0x830ADAE0; continue 'dispatch;
	// 830ADB28: 48103239  bl 0x831b0d60
	ctx.lr = 0x830ADB2C;
	sub_831B0D60(ctx, base);
	// 830ADB2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ADB30: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 830ADB34: 409A0020  bne cr6, 0x830adb54
	if !ctx.cr[6].eq {
	pc = 0x830ADB54; continue 'dispatch;
	}
	// 830ADB38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830ADB3C: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADB40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830ADB44: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830ADB48: 4BF24F79  bl 0x82fd2ac0
	ctx.lr = 0x830ADB4C;
	sub_82FD2AC0(ctx, base);
	// 830ADB4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830ADB50: 4BFFFF98  b 0x830adae8
	pc = 0x830ADAE8; continue 'dispatch;
	// 830ADB54: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 830ADB58: 4BFFFFE8  b 0x830adb40
	pc = 0x830ADB40; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ADB5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830ADB5C size=40
    let mut pc: u32 = 0x830ADB5C;
    'dispatch: loop {
        match pc {
            0x830ADB5C => {
    //   block [0x830ADB5C..0x830ADB84)
	// 830ADB5C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830ADB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ADB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830ADB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ADB6C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 830ADB70: 4BF252E9  bl 0x82fd2e58
	ctx.lr = 0x830ADB74;
	sub_82FD2E58(ctx, base);
	// 830ADB74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830ADB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830ADB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830ADB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ADB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830ADB88 size=8
    let mut pc: u32 = 0x830ADB88;
    'dispatch: loop {
        match pc {
            0x830ADB88 => {
    //   block [0x830ADB88..0x830ADB90)
	// 830ADB88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830ADB8C: 82180DF4  lwz r16, 0xdf4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3572 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830ADB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830ADB90 size=1772
    let mut pc: u32 = 0x830ADB90;
    'dispatch: loop {
        match pc {
            0x830ADB90 => {
    //   block [0x830ADB90..0x830AE27C)
	// 830ADB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830ADB94: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830ADB98: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830ADB9C: 480FA5BD  bl 0x831a8158
	ctx.lr = 0x830ADBA0;
	sub_831A8130(ctx, base);
	// 830ADBA0: 3BE1FEB0  addi r31, r1, -0x150
	ctx.r[31].s64 = ctx.r[1].s64 + -336;
	// 830ADBA4: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830ADBA8: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 830ADBAC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830ADBB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830ADBB4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830ADBB8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 830ADBBC: 933F0174  stw r25, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[25].u32 ) };
	// 830ADBC0: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 830ADBC4: 409A019C  bne cr6, 0x830add60
	if !ctx.cr[6].eq {
	pc = 0x830ADD60; continue 'dispatch;
	}
	// 830ADBC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830ADBCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830ADBD0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830ADBD4: 4BFE65AD  bl 0x83094180
	ctx.lr = 0x830ADBD8;
	sub_83094180(ctx, base);
	// 830ADBD8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADBDC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830ADBE0: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830ADBE4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830ADBE8: 83BF0078  lwz r29, 0x78(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830ADBEC: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 830ADBF0: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830ADBF4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830ADBF8: 5503083C  slwi r3, r8, 1
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830ADBFC: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 830ADC00: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830ADC04: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830ADC08: 7C63EA14  add r3, r3, r29
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 830ADC0C: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 830ADC10: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 830ADC14: 4BFFFE15  bl 0x830ada28
	ctx.lr = 0x830ADC18;
	sub_830ADA28(ctx, base);
	// 830ADC18: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADC1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ADC20: 40820028  bne 0x830adc48
	if !ctx.cr[0].eq {
	pc = 0x830ADC48; continue 'dispatch;
	}
	// 830ADC24: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ADC28: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830ADC2C: 419A000C  beq cr6, 0x830adc38
	if ctx.cr[6].eq {
	pc = 0x830ADC38; continue 'dispatch;
	}
	// 830ADC30: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830ADC34: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADC38: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830ADC3C: 4BFE6725  bl 0x83094360
	ctx.lr = 0x830ADC40;
	sub_83094360(ctx, base);
	// 830ADC40: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADC44: 4800062C  b 0x830ae270
	pc = 0x830AE270; continue 'dispatch;
	// 830ADC48: 7D7AD850  subf r11, r26, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[26].s64;
	// 830ADC4C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830ADC50: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830ADC54: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830ADC58: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 830ADC5C: 38FF0054  addi r7, r31, 0x54
	ctx.r[7].s64 = ctx.r[31].s64 + 84;
	// 830ADC60: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830ADC64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830ADC68: 7F6BEB2E  sthx r27, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[27].u16) };
	// 830ADC6C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830ADC70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ADC74: 4BFFFDB5  bl 0x830ada28
	ctx.lr = 0x830ADC78;
	sub_830ADA28(ctx, base);
	// 830ADC78: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADC7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ADC80: 4082001C  bne 0x830adc9c
	if !ctx.cr[0].eq {
	pc = 0x830ADC9C; continue 'dispatch;
	}
	// 830ADC84: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830ADC88: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830ADC8C: 419A000C  beq cr6, 0x830adc98
	if ctx.cr[6].eq {
	pc = 0x830ADC98; continue 'dispatch;
	}
	// 830ADC90: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830ADC94: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADC98: 4BFFFFA0  b 0x830adc38
	pc = 0x830ADC38; continue 'dispatch;
	// 830ADC9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830ADCA0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830ADCA4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830ADCA8: 4BFE5209  bl 0x83092eb0
	ctx.lr = 0x830ADCAC;
	sub_83092EB0(ctx, base);
	// 830ADCAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADCB0: 897F0094  lbz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830ADCB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADCB8: 4182002C  beq 0x830adce4
	if ctx.cr[0].eq {
	pc = 0x830ADCE4; continue 'dispatch;
	}
	// 830ADCBC: 897F0095  lbz r11, 0x95(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(149 as u32) ) } as u64;
	// 830ADCC0: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 830ADCC4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 830ADCC8: 556B077E  clrlwi r11, r11, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 830ADCCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830ADCD0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADCD4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830ADCD8: 4BFE5249  bl 0x83092f20
	ctx.lr = 0x830ADCDC;
	sub_83092F20(ctx, base);
	// 830ADCDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADCE0: 4BFFFF58  b 0x830adc38
	pc = 0x830ADC38; continue 'dispatch;
	// 830ADCE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADCE8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADCEC: 4BF2A5AD  bl 0x82fd8298
	ctx.lr = 0x830ADCF0;
	sub_82FD8298(ctx, base);
	// 830ADCF0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADCF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADCF8: 4182001C  beq 0x830add14
	if ctx.cr[0].eq {
	pc = 0x830ADD14; continue 'dispatch;
	}
	// 830ADCFC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830ADD00: 9B630028  stb r27, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[27].u8 ) };
	// 830ADD04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830ADD08: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830ADD0C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADD10: 48000008  b 0x830add18
	pc = 0x830ADD18; continue 'dispatch;
	// 830ADD14: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 830ADD18: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830ADD1C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ADD20: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830ADD24: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830ADD28: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830ADD2C: 915D0014  stw r10, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830ADD30: 913D0010  stw r9, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830ADD34: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830ADD38: C81F0088  lfd f0, 0x88(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) };
	// 830ADD3C: D81D0018  stfd f0, 0x18(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.f[0].u64 ) };
	// 830ADD40: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830ADD44: 4BFE51DD  bl 0x83092f20
	ctx.lr = 0x830ADD48;
	sub_83092F20(ctx, base);
	// 830ADD48: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADD4C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830ADD50: 4BFE6611  bl 0x83094360
	ctx.lr = 0x830ADD54;
	sub_83094360(ctx, base);
	// 830ADD54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADD58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830ADD5C: 48000518  b 0x830ae274
	pc = 0x830AE274; continue 'dispatch;
	// 830ADD60: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 830ADD64: 409A0088  bne cr6, 0x830addec
	if !ctx.cr[6].eq {
	pc = 0x830ADDEC; continue 'dispatch;
	}
	// 830ADD68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830ADD6C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830ADD70: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830ADD74: 4BFE4DCD  bl 0x83092b40
	ctx.lr = 0x830ADD78;
	sub_83092B40(ctx, base);
	// 830ADD78: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADD7C: 897F00C4  lbz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 830ADD80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADD84: 41820018  beq 0x830add9c
	if ctx.cr[0].eq {
	pc = 0x830ADD9C; continue 'dispatch;
	}
	// 830ADD88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830ADD8C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADD90: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830ADD94: 4BFE4E1D  bl 0x83092bb0
	ctx.lr = 0x830ADD98;
	sub_83092BB0(ctx, base);
	// 830ADD98: 4BFFFEA8  b 0x830adc40
	pc = 0x830ADC40; continue 'dispatch;
	// 830ADD9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADDA0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADDA4: 4BF2A4F5  bl 0x82fd8298
	ctx.lr = 0x830ADDA8;
	sub_82FD8298(ctx, base);
	// 830ADDA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADDAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADDB0: 41820020  beq 0x830addd0
	if ctx.cr[0].eq {
	pc = 0x830ADDD0; continue 'dispatch;
	}
	// 830ADDB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ADDB8: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830ADDBC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830ADDC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830ADDC4: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830ADDC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830ADDCC: 48000008  b 0x830addd4
	pc = 0x830ADDD4; continue 'dispatch;
	// 830ADDD0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830ADDD4: C81F00B8  lfd f0, 0xb8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) };
	// 830ADDD8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 830ADDDC: D01D0008  stfs f0, 8(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 830ADDE0: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830ADDE4: 4BFE4DCD  bl 0x83092bb0
	ctx.lr = 0x830ADDE8;
	sub_83092BB0(ctx, base);
	// 830ADDE8: 4BFFFF6C  b 0x830add54
	pc = 0x830ADD54; continue 'dispatch;
	// 830ADDEC: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 830ADDF0: 409A0084  bne cr6, 0x830ade74
	if !ctx.cr[6].eq {
	pc = 0x830ADE74; continue 'dispatch;
	}
	// 830ADDF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830ADDF8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830ADDFC: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830ADE00: 4BFE50B1  bl 0x83092eb0
	ctx.lr = 0x830ADE04;
	sub_83092EB0(ctx, base);
	// 830ADE04: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADE08: 897F00F4  lbz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 830ADE0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADE10: 41820018  beq 0x830ade28
	if ctx.cr[0].eq {
	pc = 0x830ADE28; continue 'dispatch;
	}
	// 830ADE14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830ADE18: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADE1C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830ADE20: 4BFE5101  bl 0x83092f20
	ctx.lr = 0x830ADE24;
	sub_83092F20(ctx, base);
	// 830ADE24: 4BFFFE1C  b 0x830adc40
	pc = 0x830ADC40; continue 'dispatch;
	// 830ADE28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADE2C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADE30: 4BF2A469  bl 0x82fd8298
	ctx.lr = 0x830ADE34;
	sub_82FD8298(ctx, base);
	// 830ADE34: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADE38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADE3C: 41820020  beq 0x830ade5c
	if ctx.cr[0].eq {
	pc = 0x830ADE5C; continue 'dispatch;
	}
	// 830ADE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830ADE44: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830ADE48: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830ADE4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830ADE50: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830ADE54: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830ADE58: 48000008  b 0x830ade60
	pc = 0x830ADE60; continue 'dispatch;
	// 830ADE5C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830ADE60: C81F00E8  lfd f0, 0xe8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	// 830ADE64: D81D0008  stfd f0, 8(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 830ADE68: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830ADE6C: 4BFE50B5  bl 0x83092f20
	ctx.lr = 0x830ADE70;
	sub_83092F20(ctx, base);
	// 830ADE70: 4BFFFEE4  b 0x830add54
	pc = 0x830ADD54; continue 'dispatch;
	// 830ADE74: 2F1D001F  cmpwi cr6, r29, 0x1f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 31, &mut ctx.xer);
	// 830ADE78: 419A0284  beq cr6, 0x830ae0fc
	if ctx.cr[6].eq {
	pc = 0x830AE0FC; continue 'dispatch;
	}
	// 830ADE7C: 2F1D0021  cmpwi cr6, r29, 0x21
	ctx.cr[6].compare_i32(ctx.r[29].s32, 33, &mut ctx.xer);
	// 830ADE80: 419A027C  beq cr6, 0x830ae0fc
	if ctx.cr[6].eq {
	pc = 0x830AE0FC; continue 'dispatch;
	}
	// 830ADE84: 2F1D0020  cmpwi cr6, r29, 0x20
	ctx.cr[6].compare_i32(ctx.r[29].s32, 32, &mut ctx.xer);
	// 830ADE88: 419A0274  beq cr6, 0x830ae0fc
	if ctx.cr[6].eq {
	pc = 0x830AE0FC; continue 'dispatch;
	}
	// 830ADE8C: 2F1D0026  cmpwi cr6, r29, 0x26
	ctx.cr[6].compare_i32(ctx.r[29].s32, 38, &mut ctx.xer);
	// 830ADE90: 419A026C  beq cr6, 0x830ae0fc
	if ctx.cr[6].eq {
	pc = 0x830AE0FC; continue 'dispatch;
	}
	// 830ADE94: 2F1D002B  cmpwi cr6, r29, 0x2b
	ctx.cr[6].compare_i32(ctx.r[29].s32, 43, &mut ctx.xer);
	// 830ADE98: 419A0264  beq cr6, 0x830ae0fc
	if ctx.cr[6].eq {
	pc = 0x830AE0FC; continue 'dispatch;
	}
	// 830ADE9C: 2F1D0022  cmpwi cr6, r29, 0x22
	ctx.cr[6].compare_i32(ctx.r[29].s32, 34, &mut ctx.xer);
	// 830ADEA0: 419A011C  beq cr6, 0x830adfbc
	if ctx.cr[6].eq {
	pc = 0x830ADFBC; continue 'dispatch;
	}
	// 830ADEA4: 2F1D0023  cmpwi cr6, r29, 0x23
	ctx.cr[6].compare_i32(ctx.r[29].s32, 35, &mut ctx.xer);
	// 830ADEA8: 419A0114  beq cr6, 0x830adfbc
	if ctx.cr[6].eq {
	pc = 0x830ADFBC; continue 'dispatch;
	}
	// 830ADEAC: 2F1D0024  cmpwi cr6, r29, 0x24
	ctx.cr[6].compare_i32(ctx.r[29].s32, 36, &mut ctx.xer);
	// 830ADEB0: 419A010C  beq cr6, 0x830adfbc
	if ctx.cr[6].eq {
	pc = 0x830ADFBC; continue 'dispatch;
	}
	// 830ADEB4: 2F1D0025  cmpwi cr6, r29, 0x25
	ctx.cr[6].compare_i32(ctx.r[29].s32, 37, &mut ctx.xer);
	// 830ADEB8: 419A0104  beq cr6, 0x830adfbc
	if ctx.cr[6].eq {
	pc = 0x830ADFBC; continue 'dispatch;
	}
	// 830ADEBC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830ADEC0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 830ADEC4: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 830ADEC8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830ADECC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830ADED0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830ADED4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830ADED8: 4BFFFB51  bl 0x830ada28
	ctx.lr = 0x830ADEDC;
	sub_830ADA28(ctx, base);
	// 830ADEDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADEE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ADEE4: 4182038C  beq 0x830ae270
	if ctx.cr[0].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830ADEE8: 2F1D0027  cmpwi cr6, r29, 0x27
	ctx.cr[6].compare_i32(ctx.r[29].s32, 39, &mut ctx.xer);
	// 830ADEEC: 419A00B0  beq cr6, 0x830adf9c
	if ctx.cr[6].eq {
	pc = 0x830ADF9C; continue 'dispatch;
	}
	// 830ADEF0: 2F1D0028  cmpwi cr6, r29, 0x28
	ctx.cr[6].compare_i32(ctx.r[29].s32, 40, &mut ctx.xer);
	// 830ADEF4: 419A0078  beq cr6, 0x830adf6c
	if ctx.cr[6].eq {
	pc = 0x830ADF6C; continue 'dispatch;
	}
	// 830ADEF8: 2F1D0029  cmpwi cr6, r29, 0x29
	ctx.cr[6].compare_i32(ctx.r[29].s32, 41, &mut ctx.xer);
	// 830ADEFC: 419A0044  beq cr6, 0x830adf40
	if ctx.cr[6].eq {
	pc = 0x830ADF40; continue 'dispatch;
	}
	// 830ADF00: 2F1D002A  cmpwi cr6, r29, 0x2a
	ctx.cr[6].compare_i32(ctx.r[29].s32, 42, &mut ctx.xer);
	// 830ADF04: 409A036C  bne cr6, 0x830ae270
	if !ctx.cr[6].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830ADF08: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ADF0C: 2B1D00FF  cmplwi cr6, r29, 0xff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 255 as u32, &mut ctx.xer);
	// 830ADF10: 40990010  ble cr6, 0x830adf20
	if !ctx.cr[6].gt {
	pc = 0x830ADF20; continue 'dispatch;
	}
	// 830ADF14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830ADF18: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830ADF1C: 48000354  b 0x830ae270
	pc = 0x830AE270; continue 'dispatch;
	// 830ADF20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADF24: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADF28: 4BF2A371  bl 0x82fd8298
	ctx.lr = 0x830ADF2C;
	sub_82FD8298(ctx, base);
	// 830ADF2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADF30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADF34: 41820114  beq 0x830ae048
	if ctx.cr[0].eq {
	pc = 0x830AE048; continue 'dispatch;
	}
	// 830ADF38: 3940002A  li r10, 0x2a
	ctx.r[10].s64 = 42;
	// 830ADF3C: 480000F8  b 0x830ae034
	pc = 0x830AE034; continue 'dispatch;
	// 830ADF40: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ADF44: 2B1DFFFF  cmplwi cr6, r29, 0xffff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 65535 as u32, &mut ctx.xer);
	// 830ADF48: 4199FFCC  bgt cr6, 0x830adf14
	if ctx.cr[6].gt {
	pc = 0x830ADF14; continue 'dispatch;
	}
	// 830ADF4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADF50: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADF54: 4BF2A345  bl 0x82fd8298
	ctx.lr = 0x830ADF58;
	sub_82FD8298(ctx, base);
	// 830ADF58: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADF5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADF60: 41820138  beq 0x830ae098
	if ctx.cr[0].eq {
	pc = 0x830AE098; continue 'dispatch;
	}
	// 830ADF64: 39400029  li r10, 0x29
	ctx.r[10].s64 = 41;
	// 830ADF68: 4800011C  b 0x830ae084
	pc = 0x830AE084; continue 'dispatch;
	// 830ADF6C: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830ADF70: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830ADF74: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830ADF78: 4199FF9C  bgt cr6, 0x830adf14
	if ctx.cr[6].gt {
	pc = 0x830ADF14; continue 'dispatch;
	}
	// 830ADF7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADF80: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADF84: 4BF2A315  bl 0x82fd8298
	ctx.lr = 0x830ADF88;
	sub_82FD8298(ctx, base);
	// 830ADF88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADF8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADF90: 41820268  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830ADF94: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 830ADF98: 4800024C  b 0x830ae1e4
	pc = 0x830AE1E4; continue 'dispatch;
	// 830ADF9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830ADFA0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830ADFA4: 4BF2A2F5  bl 0x82fd8298
	ctx.lr = 0x830ADFA8;
	sub_82FD8298(ctx, base);
	// 830ADFA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADFAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830ADFB0: 418202B0  beq 0x830ae260
	if ctx.cr[0].eq {
	pc = 0x830AE260; continue 'dispatch;
	}
	// 830ADFB4: 39400027  li r10, 0x27
	ctx.r[10].s64 = 39;
	// 830ADFB8: 48000294  b 0x830ae24c
	pc = 0x830AE24C; continue 'dispatch;
	// 830ADFBC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830ADFC0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 830ADFC4: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 830ADFC8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830ADFCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830ADFD0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830ADFD4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830ADFD8: 4BFFFA51  bl 0x830ada28
	ctx.lr = 0x830ADFDC;
	sub_830ADA28(ctx, base);
	// 830ADFDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830ADFE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830ADFE4: 4182028C  beq 0x830ae270
	if ctx.cr[0].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830ADFE8: 2F1D0022  cmpwi cr6, r29, 0x22
	ctx.cr[6].compare_i32(ctx.r[29].s32, 34, &mut ctx.xer);
	// 830ADFEC: 419A00F0  beq cr6, 0x830ae0dc
	if ctx.cr[6].eq {
	pc = 0x830AE0DC; continue 'dispatch;
	}
	// 830ADFF0: 2F1D0023  cmpwi cr6, r29, 0x23
	ctx.cr[6].compare_i32(ctx.r[29].s32, 35, &mut ctx.xer);
	// 830ADFF4: 419A00B0  beq cr6, 0x830ae0a4
	if ctx.cr[6].eq {
	pc = 0x830AE0A4; continue 'dispatch;
	}
	// 830ADFF8: 2F1D0024  cmpwi cr6, r29, 0x24
	ctx.cr[6].compare_i32(ctx.r[29].s32, 36, &mut ctx.xer);
	// 830ADFFC: 419A0058  beq cr6, 0x830ae054
	if ctx.cr[6].eq {
	pc = 0x830AE054; continue 'dispatch;
	}
	// 830AE000: 2F1D0025  cmpwi cr6, r29, 0x25
	ctx.cr[6].compare_i32(ctx.r[29].s32, 37, &mut ctx.xer);
	// 830AE004: 409A026C  bne cr6, 0x830ae270
	if !ctx.cr[6].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE008: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE00C: 397D0080  addi r11, r29, 0x80
	ctx.r[11].s64 = ctx.r[29].s64 + 128;
	// 830AE010: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 830AE014: 4199FF00  bgt cr6, 0x830adf14
	if ctx.cr[6].gt {
	pc = 0x830ADF14; continue 'dispatch;
	}
	// 830AE018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE01C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE020: 4BF2A279  bl 0x82fd8298
	ctx.lr = 0x830AE024;
	sub_82FD8298(ctx, base);
	// 830AE024: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE028: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE02C: 4182001C  beq 0x830ae048
	if ctx.cr[0].eq {
	pc = 0x830AE048; continue 'dispatch;
	}
	// 830AE030: 39400025  li r10, 0x25
	ctx.r[10].s64 = 37;
	// 830AE034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AE038: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AE03C: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830AE040: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE044: 48000008  b 0x830ae04c
	pc = 0x830AE04C; continue 'dispatch;
	// 830AE048: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AE04C: 9BA30008  stb r29, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 830AE050: 48000224  b 0x830ae274
	pc = 0x830AE274; continue 'dispatch;
	// 830AE054: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE058: 3D7D0001  addis r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 65536;
	// 830AE05C: 396B8000  addi r11, r11, -0x8000
	ctx.r[11].s64 = ctx.r[11].s64 + -32768;
	// 830AE060: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 830AE064: 4199FEB0  bgt cr6, 0x830adf14
	if ctx.cr[6].gt {
	pc = 0x830ADF14; continue 'dispatch;
	}
	// 830AE068: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE06C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE070: 4BF2A229  bl 0x82fd8298
	ctx.lr = 0x830AE074;
	sub_82FD8298(ctx, base);
	// 830AE074: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE078: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE07C: 4182001C  beq 0x830ae098
	if ctx.cr[0].eq {
	pc = 0x830AE098; continue 'dispatch;
	}
	// 830AE080: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 830AE084: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AE088: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AE08C: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830AE090: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE094: 48000008  b 0x830ae09c
	pc = 0x830AE09C; continue 'dispatch;
	// 830AE098: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AE09C: B3A30008  sth r29, 8(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u16 ) };
	// 830AE0A0: 480001D4  b 0x830ae274
	pc = 0x830AE274; continue 'dispatch;
	// 830AE0A4: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE0A8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 830AE0AC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830AE0B0: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 830AE0B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830AE0B8: 4199FE5C  bgt cr6, 0x830adf14
	if ctx.cr[6].gt {
	pc = 0x830ADF14; continue 'dispatch;
	}
	// 830AE0BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE0C0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE0C4: 4BF2A1D5  bl 0x82fd8298
	ctx.lr = 0x830AE0C8;
	sub_82FD8298(ctx, base);
	// 830AE0C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE0CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE0D0: 41820128  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830AE0D4: 39400023  li r10, 0x23
	ctx.r[10].s64 = 35;
	// 830AE0D8: 4800010C  b 0x830ae1e4
	pc = 0x830AE1E4; continue 'dispatch;
	// 830AE0DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE0E0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE0E4: 4BF2A1B5  bl 0x82fd8298
	ctx.lr = 0x830AE0E8;
	sub_82FD8298(ctx, base);
	// 830AE0E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE0EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE0F0: 41820170  beq 0x830ae260
	if ctx.cr[0].eq {
	pc = 0x830AE260; continue 'dispatch;
	}
	// 830AE0F4: 39400022  li r10, 0x22
	ctx.r[10].s64 = 34;
	// 830AE0F8: 48000154  b 0x830ae24c
	pc = 0x830AE24C; continue 'dispatch;
	// 830AE0FC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830AE100: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 830AE104: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 830AE108: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830AE10C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830AE110: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830AE114: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830AE118: 4BFFF911  bl 0x830ada28
	ctx.lr = 0x830AE11C;
	sub_830ADA28(ctx, base);
	// 830AE11C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE120: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AE124: 40820018  bne 0x830ae13c
	if !ctx.cr[0].eq {
	pc = 0x830AE13C; continue 'dispatch;
	}
	// 830AE128: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE12C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830AE130: 419A0140  beq cr6, 0x830ae270
	if ctx.cr[6].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE134: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 830AE138: 4BFFFDE0  b 0x830adf18
	pc = 0x830ADF18; continue 'dispatch;
	// 830AE13C: 2F1D001F  cmpwi cr6, r29, 0x1f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 31, &mut ctx.xer);
	// 830AE140: 419A00F0  beq cr6, 0x830ae230
	if ctx.cr[6].eq {
	pc = 0x830AE230; continue 'dispatch;
	}
	// 830AE144: 2F1D0020  cmpwi cr6, r29, 0x20
	ctx.cr[6].compare_i32(ctx.r[29].s32, 32, &mut ctx.xer);
	// 830AE148: 419A00BC  beq cr6, 0x830ae204
	if ctx.cr[6].eq {
	pc = 0x830AE204; continue 'dispatch;
	}
	// 830AE14C: 2F1D0021  cmpwi cr6, r29, 0x21
	ctx.cr[6].compare_i32(ctx.r[29].s32, 33, &mut ctx.xer);
	// 830AE150: 419A006C  beq cr6, 0x830ae1bc
	if ctx.cr[6].eq {
	pc = 0x830AE1BC; continue 'dispatch;
	}
	// 830AE154: 2F1D0026  cmpwi cr6, r29, 0x26
	ctx.cr[6].compare_i32(ctx.r[29].s32, 38, &mut ctx.xer);
	// 830AE158: 419A0038  beq cr6, 0x830ae190
	if ctx.cr[6].eq {
	pc = 0x830AE190; continue 'dispatch;
	}
	// 830AE15C: 2F1D002B  cmpwi cr6, r29, 0x2b
	ctx.cr[6].compare_i32(ctx.r[29].s32, 43, &mut ctx.xer);
	// 830AE160: 409A0110  bne cr6, 0x830ae270
	if !ctx.cr[6].eq {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE164: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE168: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 830AE16C: 41980104  blt cr6, 0x830ae270
	if ctx.cr[6].lt {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE170: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE174: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE178: 4BF2A121  bl 0x82fd8298
	ctx.lr = 0x830AE17C;
	sub_82FD8298(ctx, base);
	// 830AE17C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE180: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE184: 41820074  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830AE188: 3940002B  li r10, 0x2b
	ctx.r[10].s64 = 43;
	// 830AE18C: 48000058  b 0x830ae1e4
	pc = 0x830AE1E4; continue 'dispatch;
	// 830AE190: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE194: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AE198: 419800D8  blt cr6, 0x830ae270
	if ctx.cr[6].lt {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE19C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE1A0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE1A4: 4BF2A0F5  bl 0x82fd8298
	ctx.lr = 0x830AE1A8;
	sub_82FD8298(ctx, base);
	// 830AE1A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE1AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE1B0: 41820048  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830AE1B4: 39400026  li r10, 0x26
	ctx.r[10].s64 = 38;
	// 830AE1B8: 4800002C  b 0x830ae1e4
	pc = 0x830AE1E4; continue 'dispatch;
	// 830AE1BC: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE1C0: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 830AE1C4: 419900AC  bgt cr6, 0x830ae270
	if ctx.cr[6].gt {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE1C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE1CC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE1D0: 4BF2A0C9  bl 0x82fd8298
	ctx.lr = 0x830AE1D4;
	sub_82FD8298(ctx, base);
	// 830AE1D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE1D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE1DC: 4182001C  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830AE1E0: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 830AE1E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AE1E8: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AE1EC: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830AE1F0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE1F4: 48000008  b 0x830ae1fc
	pc = 0x830AE1FC; continue 'dispatch;
	// 830AE1F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AE1FC: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830AE200: 48000074  b 0x830ae274
	pc = 0x830AE274; continue 'dispatch;
	// 830AE204: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE208: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830AE20C: 41990064  bgt cr6, 0x830ae270
	if ctx.cr[6].gt {
	pc = 0x830AE270; continue 'dispatch;
	}
	// 830AE210: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE214: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE218: 4BF2A081  bl 0x82fd8298
	ctx.lr = 0x830AE21C;
	sub_82FD8298(ctx, base);
	// 830AE21C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE220: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE224: 4182FFD4  beq 0x830ae1f8
	if ctx.cr[0].eq {
	pc = 0x830AE1F8; continue 'dispatch;
	}
	// 830AE228: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 830AE22C: 4BFFFFB8  b 0x830ae1e4
	pc = 0x830AE1E4; continue 'dispatch;
	// 830AE230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AE234: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE238: 4BF2A061  bl 0x82fd8298
	ctx.lr = 0x830AE23C;
	sub_82FD8298(ctx, base);
	// 830AE23C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE240: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE244: 4182001C  beq 0x830ae260
	if ctx.cr[0].eq {
	pc = 0x830AE260; continue 'dispatch;
	}
	// 830AE248: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 830AE24C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AE250: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 830AE254: 93C3002C  stw r30, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830AE258: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE25C: 48000008  b 0x830ae264
	pc = 0x830AE264; continue 'dispatch;
	// 830AE260: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AE264: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AE268: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AE26C: 48000008  b 0x830ae274
	pc = 0x830AE274; continue 'dispatch;
	// 830AE270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AE274: 383F0150  addi r1, r31, 0x150
	ctx.r[1].s64 = ctx.r[31].s64 + 336;
	// 830AE278: 480F9F30  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AE27C size=8
    let mut pc: u32 = 0x830AE27C;
    'dispatch: loop {
        match pc {
            0x830AE27C => {
    //   block [0x830AE27C..0x830AE284)
	// 830AE27C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AE280: 82180DF4  lwz r16, 0xdf4(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3572 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE284 size=36
    let mut pc: u32 = 0x830AE284;
    'dispatch: loop {
        match pc {
            0x830AE284 => {
    //   block [0x830AE284..0x830AE2A8)
	// 830AE284: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 830AE288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE28C: 815F0174  lwz r10, 0x174(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 830AE290: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 830AE294: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830AE298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AE29C: 3863E270  addi r3, r3, -0x1d90
	ctx.r[3].s64 = ctx.r[3].s64 + -7568;
	// 830AE2A0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AE2A4: 480F9F04  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE2A8 size=40
    let mut pc: u32 = 0x830AE2A8;
    'dispatch: loop {
        match pc {
            0x830AE2A8 => {
    //   block [0x830AE2A8..0x830AE2D0)
	// 830AE2A8: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 830AE2AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE2B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AE2B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE2B8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830AE2BC: 4BFE60A5  bl 0x83094360
	ctx.lr = 0x830AE2C0;
	sub_83094360(ctx, base);
	// 830AE2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AE2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AE2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AE2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE2D0 size=40
    let mut pc: u32 = 0x830AE2D0;
    'dispatch: loop {
        match pc {
            0x830AE2D0 => {
    //   block [0x830AE2D0..0x830AE2F8)
	// 830AE2D0: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 830AE2D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE2D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AE2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE2E0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830AE2E4: 4BFE4C3D  bl 0x83092f20
	ctx.lr = 0x830AE2E8;
	sub_83092F20(ctx, base);
	// 830AE2E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AE2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AE2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AE2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE2F8 size=40
    let mut pc: u32 = 0x830AE2F8;
    'dispatch: loop {
        match pc {
            0x830AE2F8 => {
    //   block [0x830AE2F8..0x830AE320)
	// 830AE2F8: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 830AE2FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE300: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AE304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE308: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 830AE30C: 4BFE48A5  bl 0x83092bb0
	ctx.lr = 0x830AE310;
	sub_83092BB0(ctx, base);
	// 830AE310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AE314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AE318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AE31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE320 size=40
    let mut pc: u32 = 0x830AE320;
    'dispatch: loop {
        match pc {
            0x830AE320 => {
    //   block [0x830AE320..0x830AE348)
	// 830AE320: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 830AE324: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE328: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AE32C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE330: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 830AE334: 4BFE4BED  bl 0x83092f20
	ctx.lr = 0x830AE338;
	sub_83092F20(ctx, base);
	// 830AE338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AE33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AE340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AE344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE348 size=84
    let mut pc: u32 = 0x830AE348;
    'dispatch: loop {
        match pc {
            0x830AE348 => {
    //   block [0x830AE348..0x830AE39C)
	// 830AE348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AE350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AE354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AE358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE35C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830AE360: 83FEBC54  lwz r31, -0x43ac(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE364: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830AE368: 419A0014  beq cr6, 0x830ae37c
	if ctx.cr[6].eq {
	pc = 0x830AE37C; continue 'dispatch;
	}
	// 830AE36C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AE370: 4BFFE821  bl 0x830acb90
	ctx.lr = 0x830AE374;
	sub_830ACB90(ctx, base);
	// 830AE374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AE378: 4BF29F69  bl 0x82fd82e0
	ctx.lr = 0x830AE37C;
	sub_82FD82E0(ctx, base);
	// 830AE37C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AE380: 917EBC54  stw r11, -0x43ac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17324 as u32), ctx.r[11].u32 ) };
	// 830AE384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AE388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AE38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AE390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AE394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AE398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AE3A0 size=8
    let mut pc: u32 = 0x830AE3A0;
    'dispatch: loop {
        match pc {
            0x830AE3A0 => {
    //   block [0x830AE3A0..0x830AE3A8)
	// 830AE3A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AE3A4: 82180F44  lwz r16, 0xf44(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3908 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AE3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AE3A8 size=3464
    let mut pc: u32 = 0x830AE3A8;
    'dispatch: loop {
        match pc {
            0x830AE3A8 => {
    //   block [0x830AE3A8..0x830AF130)
	// 830AE3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AE3AC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830AE3B0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830AE3B4: 480F9DAD  bl 0x831a8160
	ctx.lr = 0x830AE3B8;
	sub_831A8130(ctx, base);
	// 830AE3B8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830AE3BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AE3C0: 3F608339  lis r27, -0x7cc7
	ctx.r[27].s64 = -2093416448;
	// 830AE3C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830AE3C8: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE3CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830AE3D0: 409A0D24  bne cr6, 0x830af0f4
	if !ctx.cr[6].eq {
	pc = 0x830AF0F4; continue 'dispatch;
	}
	// 830AE3D4: 4BFFE905  bl 0x830accd8
	ctx.lr = 0x830AE3D8;
	sub_830ACCD8(ctx, base);
	// 830AE3D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AE3DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AE3E0: 4BF473F9  bl 0x82ff57d8
	ctx.lr = 0x830AE3E4;
	sub_82FF57D8(ctx, base);
	// 830AE3E4: 817BBC54  lwz r11, -0x43ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE3E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AE3EC: 409A0CFC  bne cr6, 0x830af0e8
	if !ctx.cr[6].eq {
	pc = 0x830AF0E8; continue 'dispatch;
	}
	// 830AE3F0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830AE3F4: 4BF29E55  bl 0x82fd8248
	ctx.lr = 0x830AE3F8;
	sub_82FD8248(ctx, base);
	// 830AE3F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE3FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830AE400: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830AE404: 41820060  beq 0x830ae464
	if ctx.cr[0].eq {
	pc = 0x830AE464; continue 'dispatch;
	}
	// 830AE408: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830AE40C: 4BF29E3D  bl 0x82fd8248
	ctx.lr = 0x830AE410;
	sub_82FD8248(ctx, base);
	// 830AE410: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE414: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 830AE418: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE41C: 41820018  beq 0x830ae434
	if ctx.cr[0].eq {
	pc = 0x830AE434; continue 'dispatch;
	}
	// 830AE420: 4BF61541  bl 0x8300f960
	ctx.lr = 0x830AE424;
	sub_8300F960(ctx, base);
	// 830AE424: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 830AE428: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE42C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AE430: 4800000C  b 0x830ae43c
	pc = 0x830AE43C; continue 'dispatch;
	// 830AE434: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AE438: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830AE43C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830AE440: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AE444: 3B8BB7E8  addi r28, r11, -0x4818
	ctx.r[28].s64 = ctx.r[11].s64 + -18456;
	// 830AE448: 3880002B  li r4, 0x2b
	ctx.r[4].s64 = 43;
	// 830AE44C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AE450: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE454: 4BF8CC9D  bl 0x8303b0f0
	ctx.lr = 0x830AE458;
	sub_8303B0F0(ctx, base);
	// 830AE458: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830AE45C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE460: 48000014  b 0x830ae474
	pc = 0x830AE474; continue 'dispatch;
	// 830AE464: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830AE468: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830AE46C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830AE470: 3B8AB7E8  addi r28, r10, -0x4818
	ctx.r[28].s64 = ctx.r[10].s64 + -18456;
	// 830AE474: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE478: 917BBC54  stw r11, -0x43ac(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(-17324 as u32), ctx.r[11].u32 ) };
	// 830AE47C: 4BF29DCD  bl 0x82fd8248
	ctx.lr = 0x830AE480;
	sub_82FD8248(ctx, base);
	// 830AE480: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE484: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE488: 4182001C  beq 0x830ae4a4
	if ctx.cr[0].eq {
	pc = 0x830AE4A4; continue 'dispatch;
	}
	// 830AE48C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE490: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE494: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE498: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830AE49C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE4A0: 48000008  b 0x830ae4a8
	pc = 0x830AE4A8; continue 'dispatch;
	// 830AE4A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE4A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE4AC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE4B0: 388BD61C  addi r4, r11, -0x29e4
	ctx.r[4].s64 = ctx.r[11].s64 + -10724;
	// 830AE4B4: 4BFFE745  bl 0x830acbf8
	ctx.lr = 0x830AE4B8;
	sub_830ACBF8(ctx, base);
	// 830AE4B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE4BC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE4C0: 4BF29D89  bl 0x82fd8248
	ctx.lr = 0x830AE4C4;
	sub_82FD8248(ctx, base);
	// 830AE4C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE4C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE4CC: 41820020  beq 0x830ae4ec
	if ctx.cr[0].eq {
	pc = 0x830AE4EC; continue 'dispatch;
	}
	// 830AE4D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE4D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830AE4D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE4DC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE4E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE4E4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE4E8: 48000008  b 0x830ae4f0
	pc = 0x830AE4F0; continue 'dispatch;
	// 830AE4EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE4F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE4F4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE4F8: 388BD688  addi r4, r11, -0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + -10616;
	// 830AE4FC: 4BFFE6FD  bl 0x830acbf8
	ctx.lr = 0x830AE500;
	sub_830ACBF8(ctx, base);
	// 830AE500: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE504: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE508: 4BF29D41  bl 0x82fd8248
	ctx.lr = 0x830AE50C;
	sub_82FD8248(ctx, base);
	// 830AE50C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE510: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE514: 41820020  beq 0x830ae534
	if ctx.cr[0].eq {
	pc = 0x830AE534; continue 'dispatch;
	}
	// 830AE518: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE51C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830AE520: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE524: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE528: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE52C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE530: 48000008  b 0x830ae538
	pc = 0x830AE538; continue 'dispatch;
	// 830AE534: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE538: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE53C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE540: 388BD678  addi r4, r11, -0x2988
	ctx.r[4].s64 = ctx.r[11].s64 + -10632;
	// 830AE544: 4BFFE6B5  bl 0x830acbf8
	ctx.lr = 0x830AE548;
	sub_830ACBF8(ctx, base);
	// 830AE548: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE54C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE550: 4BF29CF9  bl 0x82fd8248
	ctx.lr = 0x830AE554;
	sub_82FD8248(ctx, base);
	// 830AE554: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE558: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE55C: 41820020  beq 0x830ae57c
	if ctx.cr[0].eq {
	pc = 0x830AE57C; continue 'dispatch;
	}
	// 830AE560: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE564: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830AE568: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE56C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE570: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE574: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE578: 48000008  b 0x830ae580
	pc = 0x830AE580; continue 'dispatch;
	// 830AE57C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE580: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE584: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE588: 388BD884  addi r4, r11, -0x277c
	ctx.r[4].s64 = ctx.r[11].s64 + -10108;
	// 830AE58C: 4BFFE66D  bl 0x830acbf8
	ctx.lr = 0x830AE590;
	sub_830ACBF8(ctx, base);
	// 830AE590: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE594: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE598: 4BF29CB1  bl 0x82fd8248
	ctx.lr = 0x830AE59C;
	sub_82FD8248(ctx, base);
	// 830AE59C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE5A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE5A4: 41820020  beq 0x830ae5c4
	if ctx.cr[0].eq {
	pc = 0x830AE5C4; continue 'dispatch;
	}
	// 830AE5A8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE5AC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 830AE5B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE5B4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE5B8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE5BC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE5C0: 48000008  b 0x830ae5c8
	pc = 0x830AE5C8; continue 'dispatch;
	// 830AE5C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE5C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE5CC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE5D0: 388BD890  addi r4, r11, -0x2770
	ctx.r[4].s64 = ctx.r[11].s64 + -10096;
	// 830AE5D4: 4BFFE625  bl 0x830acbf8
	ctx.lr = 0x830AE5D8;
	sub_830ACBF8(ctx, base);
	// 830AE5D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE5DC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE5E0: 4BF29C69  bl 0x82fd8248
	ctx.lr = 0x830AE5E4;
	sub_82FD8248(ctx, base);
	// 830AE5E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE5E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE5EC: 41820020  beq 0x830ae60c
	if ctx.cr[0].eq {
	pc = 0x830AE60C; continue 'dispatch;
	}
	// 830AE5F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE5F4: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 830AE5F8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE5FC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE600: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE604: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE608: 48000008  b 0x830ae610
	pc = 0x830AE610; continue 'dispatch;
	// 830AE60C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE610: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE614: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE618: 388BD7EC  addi r4, r11, -0x2814
	ctx.r[4].s64 = ctx.r[11].s64 + -10260;
	// 830AE61C: 4BFFE5DD  bl 0x830acbf8
	ctx.lr = 0x830AE620;
	sub_830ACBF8(ctx, base);
	// 830AE620: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE624: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE628: 4BF29C21  bl 0x82fd8248
	ctx.lr = 0x830AE62C;
	sub_82FD8248(ctx, base);
	// 830AE62C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE630: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE634: 41820020  beq 0x830ae654
	if ctx.cr[0].eq {
	pc = 0x830AE654; continue 'dispatch;
	}
	// 830AE638: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE63C: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 830AE640: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE644: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE648: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE64C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE650: 48000008  b 0x830ae658
	pc = 0x830AE658; continue 'dispatch;
	// 830AE654: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE658: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE65C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE660: 388BD7C0  addi r4, r11, -0x2840
	ctx.r[4].s64 = ctx.r[11].s64 + -10304;
	// 830AE664: 4BFFE595  bl 0x830acbf8
	ctx.lr = 0x830AE668;
	sub_830ACBF8(ctx, base);
	// 830AE668: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE66C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE670: 4BF29BD9  bl 0x82fd8248
	ctx.lr = 0x830AE674;
	sub_82FD8248(ctx, base);
	// 830AE674: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE678: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE67C: 41820020  beq 0x830ae69c
	if ctx.cr[0].eq {
	pc = 0x830AE69C; continue 'dispatch;
	}
	// 830AE680: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE684: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 830AE688: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE68C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE690: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE694: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE698: 48000008  b 0x830ae6a0
	pc = 0x830AE6A0; continue 'dispatch;
	// 830AE69C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE6A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE6A4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE6A8: 388BD7E0  addi r4, r11, -0x2820
	ctx.r[4].s64 = ctx.r[11].s64 + -10272;
	// 830AE6AC: 4BFFE54D  bl 0x830acbf8
	ctx.lr = 0x830AE6B0;
	sub_830ACBF8(ctx, base);
	// 830AE6B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE6B4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE6B8: 4BF29B91  bl 0x82fd8248
	ctx.lr = 0x830AE6BC;
	sub_82FD8248(ctx, base);
	// 830AE6BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE6C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE6C4: 41820020  beq 0x830ae6e4
	if ctx.cr[0].eq {
	pc = 0x830AE6E4; continue 'dispatch;
	}
	// 830AE6C8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE6CC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 830AE6D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE6D4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE6D8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE6DC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE6E0: 48000008  b 0x830ae6e8
	pc = 0x830AE6E8; continue 'dispatch;
	// 830AE6E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE6E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE6EC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE6F0: 388BD7D4  addi r4, r11, -0x282c
	ctx.r[4].s64 = ctx.r[11].s64 + -10284;
	// 830AE6F4: 4BFFE505  bl 0x830acbf8
	ctx.lr = 0x830AE6F8;
	sub_830ACBF8(ctx, base);
	// 830AE6F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE6FC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE700: 4BF29B49  bl 0x82fd8248
	ctx.lr = 0x830AE704;
	sub_82FD8248(ctx, base);
	// 830AE704: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE708: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE70C: 41820020  beq 0x830ae72c
	if ctx.cr[0].eq {
	pc = 0x830AE72C; continue 'dispatch;
	}
	// 830AE710: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE714: 39400009  li r10, 9
	ctx.r[10].s64 = 9;
	// 830AE718: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE71C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE720: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE724: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE728: 48000008  b 0x830ae730
	pc = 0x830AE730; continue 'dispatch;
	// 830AE72C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE730: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE734: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE738: 388BD83C  addi r4, r11, -0x27c4
	ctx.r[4].s64 = ctx.r[11].s64 + -10180;
	// 830AE73C: 4BFFE4BD  bl 0x830acbf8
	ctx.lr = 0x830AE740;
	sub_830ACBF8(ctx, base);
	// 830AE740: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE744: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE748: 4BF29B01  bl 0x82fd8248
	ctx.lr = 0x830AE74C;
	sub_82FD8248(ctx, base);
	// 830AE74C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE750: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE754: 41820020  beq 0x830ae774
	if ctx.cr[0].eq {
	pc = 0x830AE774; continue 'dispatch;
	}
	// 830AE758: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE75C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 830AE760: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE764: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE768: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE76C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE770: 48000008  b 0x830ae778
	pc = 0x830AE778; continue 'dispatch;
	// 830AE774: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE778: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE77C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE780: 388BD830  addi r4, r11, -0x27d0
	ctx.r[4].s64 = ctx.r[11].s64 + -10192;
	// 830AE784: 4BFFE475  bl 0x830acbf8
	ctx.lr = 0x830AE788;
	sub_830ACBF8(ctx, base);
	// 830AE788: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE78C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE790: 4BF29AB9  bl 0x82fd8248
	ctx.lr = 0x830AE794;
	sub_82FD8248(ctx, base);
	// 830AE794: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE798: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE79C: 41820020  beq 0x830ae7bc
	if ctx.cr[0].eq {
	pc = 0x830AE7BC; continue 'dispatch;
	}
	// 830AE7A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE7A4: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 830AE7A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE7AC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE7B0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE7B4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE7B8: 48000008  b 0x830ae7c0
	pc = 0x830AE7C0; continue 'dispatch;
	// 830AE7BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE7C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE7C4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE7C8: 388BD81C  addi r4, r11, -0x27e4
	ctx.r[4].s64 = ctx.r[11].s64 + -10212;
	// 830AE7CC: 4BFFE42D  bl 0x830acbf8
	ctx.lr = 0x830AE7D0;
	sub_830ACBF8(ctx, base);
	// 830AE7D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE7D4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE7D8: 4BF29A71  bl 0x82fd8248
	ctx.lr = 0x830AE7DC;
	sub_82FD8248(ctx, base);
	// 830AE7DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE7E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE7E4: 41820020  beq 0x830ae804
	if ctx.cr[0].eq {
	pc = 0x830AE804; continue 'dispatch;
	}
	// 830AE7E8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE7EC: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 830AE7F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE7F4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE7F8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE7FC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE800: 48000008  b 0x830ae808
	pc = 0x830AE808; continue 'dispatch;
	// 830AE804: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE808: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE80C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE810: 388BD800  addi r4, r11, -0x2800
	ctx.r[4].s64 = ctx.r[11].s64 + -10240;
	// 830AE814: 4BFFE3E5  bl 0x830acbf8
	ctx.lr = 0x830AE818;
	sub_830ACBF8(ctx, base);
	// 830AE818: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE81C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE820: 4BF29A29  bl 0x82fd8248
	ctx.lr = 0x830AE824;
	sub_82FD8248(ctx, base);
	// 830AE824: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE828: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE82C: 41820020  beq 0x830ae84c
	if ctx.cr[0].eq {
	pc = 0x830AE84C; continue 'dispatch;
	}
	// 830AE830: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE834: 3940000D  li r10, 0xd
	ctx.r[10].s64 = 13;
	// 830AE838: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE83C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE840: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE844: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE848: 48000008  b 0x830ae850
	pc = 0x830AE850; continue 'dispatch;
	// 830AE84C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE850: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE854: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE858: 388BD80C  addi r4, r11, -0x27f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10228;
	// 830AE85C: 4BFFE39D  bl 0x830acbf8
	ctx.lr = 0x830AE860;
	sub_830ACBF8(ctx, base);
	// 830AE860: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE864: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE868: 4BF299E1  bl 0x82fd8248
	ctx.lr = 0x830AE86C;
	sub_82FD8248(ctx, base);
	// 830AE86C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE870: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE874: 41820020  beq 0x830ae894
	if ctx.cr[0].eq {
	pc = 0x830AE894; continue 'dispatch;
	}
	// 830AE878: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE87C: 3940000E  li r10, 0xe
	ctx.r[10].s64 = 14;
	// 830AE880: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE884: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE888: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE88C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE890: 48000008  b 0x830ae898
	pc = 0x830AE898; continue 'dispatch;
	// 830AE894: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE898: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE89C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE8A0: 388BD870  addi r4, r11, -0x2790
	ctx.r[4].s64 = ctx.r[11].s64 + -10128;
	// 830AE8A4: 4BFFE355  bl 0x830acbf8
	ctx.lr = 0x830AE8A8;
	sub_830ACBF8(ctx, base);
	// 830AE8A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE8AC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE8B0: 4BF29999  bl 0x82fd8248
	ctx.lr = 0x830AE8B4;
	sub_82FD8248(ctx, base);
	// 830AE8B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE8B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE8BC: 41820020  beq 0x830ae8dc
	if ctx.cr[0].eq {
	pc = 0x830AE8DC; continue 'dispatch;
	}
	// 830AE8C0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE8C4: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 830AE8C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE8CC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE8D0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE8D4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE8D8: 48000008  b 0x830ae8e0
	pc = 0x830AE8E0; continue 'dispatch;
	// 830AE8DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE8E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE8E4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE8E8: 388BD854  addi r4, r11, -0x27ac
	ctx.r[4].s64 = ctx.r[11].s64 + -10156;
	// 830AE8EC: 4BFFE30D  bl 0x830acbf8
	ctx.lr = 0x830AE8F0;
	sub_830ACBF8(ctx, base);
	// 830AE8F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE8F4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE8F8: 4BF29951  bl 0x82fd8248
	ctx.lr = 0x830AE8FC;
	sub_82FD8248(ctx, base);
	// 830AE8FC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE900: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE904: 41820020  beq 0x830ae924
	if ctx.cr[0].eq {
	pc = 0x830AE924; continue 'dispatch;
	}
	// 830AE908: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE90C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 830AE910: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE914: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE918: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE91C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE920: 48000008  b 0x830ae928
	pc = 0x830AE928; continue 'dispatch;
	// 830AE924: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE928: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE92C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE930: 388BD8BC  addi r4, r11, -0x2744
	ctx.r[4].s64 = ctx.r[11].s64 + -10052;
	// 830AE934: 4BFFE2C5  bl 0x830acbf8
	ctx.lr = 0x830AE938;
	sub_830ACBF8(ctx, base);
	// 830AE938: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE93C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE940: 4BF29909  bl 0x82fd8248
	ctx.lr = 0x830AE944;
	sub_82FD8248(ctx, base);
	// 830AE944: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE94C: 41820020  beq 0x830ae96c
	if ctx.cr[0].eq {
	pc = 0x830AE96C; continue 'dispatch;
	}
	// 830AE950: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE954: 39400011  li r10, 0x11
	ctx.r[10].s64 = 17;
	// 830AE958: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE95C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE960: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE964: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE968: 48000008  b 0x830ae970
	pc = 0x830AE970; continue 'dispatch;
	// 830AE96C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE970: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AE974: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE978: 388BD8CC  addi r4, r11, -0x2734
	ctx.r[4].s64 = ctx.r[11].s64 + -10036;
	// 830AE97C: 4BFFE27D  bl 0x830acbf8
	ctx.lr = 0x830AE980;
	sub_830ACBF8(ctx, base);
	// 830AE980: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE984: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE988: 4BF298C1  bl 0x82fd8248
	ctx.lr = 0x830AE98C;
	sub_82FD8248(ctx, base);
	// 830AE98C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE990: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE994: 41820020  beq 0x830ae9b4
	if ctx.cr[0].eq {
	pc = 0x830AE9B4; continue 'dispatch;
	}
	// 830AE998: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE99C: 39400012  li r10, 0x12
	ctx.r[10].s64 = 18;
	// 830AE9A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE9A4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE9A8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE9AC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE9B0: 48000008  b 0x830ae9b8
	pc = 0x830AE9B8; continue 'dispatch;
	// 830AE9B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AE9B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AE9BC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AE9C0: 388B7C10  addi r4, r11, 0x7c10
	ctx.r[4].s64 = ctx.r[11].s64 + 31760;
	// 830AE9C4: 4BFFE235  bl 0x830acbf8
	ctx.lr = 0x830AE9C8;
	sub_830ACBF8(ctx, base);
	// 830AE9C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE9CC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AE9D0: 4BF29879  bl 0x82fd8248
	ctx.lr = 0x830AE9D4;
	sub_82FD8248(ctx, base);
	// 830AE9D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AE9D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AE9DC: 41820020  beq 0x830ae9fc
	if ctx.cr[0].eq {
	pc = 0x830AE9FC; continue 'dispatch;
	}
	// 830AE9E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AE9E4: 39400013  li r10, 0x13
	ctx.r[10].s64 = 19;
	// 830AE9E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AE9EC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AE9F0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AE9F4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AE9F8: 48000008  b 0x830aea00
	pc = 0x830AEA00; continue 'dispatch;
	// 830AE9FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEA00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEA04: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEA08: 388BD8D8  addi r4, r11, -0x2728
	ctx.r[4].s64 = ctx.r[11].s64 + -10024;
	// 830AEA0C: 4BFFE1ED  bl 0x830acbf8
	ctx.lr = 0x830AEA10;
	sub_830ACBF8(ctx, base);
	// 830AEA10: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEA14: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEA18: 4BF29831  bl 0x82fd8248
	ctx.lr = 0x830AEA1C;
	sub_82FD8248(ctx, base);
	// 830AEA1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEA20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEA24: 41820020  beq 0x830aea44
	if ctx.cr[0].eq {
	pc = 0x830AEA44; continue 'dispatch;
	}
	// 830AEA28: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEA2C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 830AEA30: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEA34: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEA38: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEA3C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEA40: 48000008  b 0x830aea48
	pc = 0x830AEA48; continue 'dispatch;
	// 830AEA44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEA48: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEA4C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEA50: 388BD62C  addi r4, r11, -0x29d4
	ctx.r[4].s64 = ctx.r[11].s64 + -10708;
	// 830AEA54: 4BFFE1A5  bl 0x830acbf8
	ctx.lr = 0x830AEA58;
	sub_830ACBF8(ctx, base);
	// 830AEA58: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEA5C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEA60: 4BF297E9  bl 0x82fd8248
	ctx.lr = 0x830AEA64;
	sub_82FD8248(ctx, base);
	// 830AEA64: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEA68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEA6C: 41820020  beq 0x830aea8c
	if ctx.cr[0].eq {
	pc = 0x830AEA8C; continue 'dispatch;
	}
	// 830AEA70: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEA74: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 830AEA78: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEA7C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEA80: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEA84: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEA88: 48000008  b 0x830aea90
	pc = 0x830AEA90; continue 'dispatch;
	// 830AEA8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEA90: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEA94: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEA98: 388BD638  addi r4, r11, -0x29c8
	ctx.r[4].s64 = ctx.r[11].s64 + -10696;
	// 830AEA9C: 4BFFE15D  bl 0x830acbf8
	ctx.lr = 0x830AEAA0;
	sub_830ACBF8(ctx, base);
	// 830AEAA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEAA4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEAA8: 4BF297A1  bl 0x82fd8248
	ctx.lr = 0x830AEAAC;
	sub_82FD8248(ctx, base);
	// 830AEAAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEAB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEAB4: 41820020  beq 0x830aead4
	if ctx.cr[0].eq {
	pc = 0x830AEAD4; continue 'dispatch;
	}
	// 830AEAB8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEABC: 39400016  li r10, 0x16
	ctx.r[10].s64 = 22;
	// 830AEAC0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEAC4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEAC8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEACC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEAD0: 48000008  b 0x830aead8
	pc = 0x830AEAD8; continue 'dispatch;
	// 830AEAD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEAD8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AEADC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEAE0: 388B7C30  addi r4, r11, 0x7c30
	ctx.r[4].s64 = ctx.r[11].s64 + 31792;
	// 830AEAE4: 4BFFE115  bl 0x830acbf8
	ctx.lr = 0x830AEAE8;
	sub_830ACBF8(ctx, base);
	// 830AEAE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEAEC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEAF0: 4BF29759  bl 0x82fd8248
	ctx.lr = 0x830AEAF4;
	sub_82FD8248(ctx, base);
	// 830AEAF4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEAF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEAFC: 41820020  beq 0x830aeb1c
	if ctx.cr[0].eq {
	pc = 0x830AEB1C; continue 'dispatch;
	}
	// 830AEB00: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEB04: 39400017  li r10, 0x17
	ctx.r[10].s64 = 23;
	// 830AEB08: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEB0C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEB10: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEB14: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEB18: 48000008  b 0x830aeb20
	pc = 0x830AEB20; continue 'dispatch;
	// 830AEB1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEB20: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AEB24: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEB28: 388B7C40  addi r4, r11, 0x7c40
	ctx.r[4].s64 = ctx.r[11].s64 + 31808;
	// 830AEB2C: 4BFFE0CD  bl 0x830acbf8
	ctx.lr = 0x830AEB30;
	sub_830ACBF8(ctx, base);
	// 830AEB30: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEB34: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEB38: 4BF29711  bl 0x82fd8248
	ctx.lr = 0x830AEB3C;
	sub_82FD8248(ctx, base);
	// 830AEB3C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEB40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEB44: 41820020  beq 0x830aeb64
	if ctx.cr[0].eq {
	pc = 0x830AEB64; continue 'dispatch;
	}
	// 830AEB48: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEB4C: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 830AEB50: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEB54: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEB58: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEB5C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEB60: 48000008  b 0x830aeb68
	pc = 0x830AEB68; continue 'dispatch;
	// 830AEB64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEB68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEB6C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEB70: 388BD64C  addi r4, r11, -0x29b4
	ctx.r[4].s64 = ctx.r[11].s64 + -10676;
	// 830AEB74: 4BFFE085  bl 0x830acbf8
	ctx.lr = 0x830AEB78;
	sub_830ACBF8(ctx, base);
	// 830AEB78: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEB7C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEB80: 4BF296C9  bl 0x82fd8248
	ctx.lr = 0x830AEB84;
	sub_82FD8248(ctx, base);
	// 830AEB84: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEB88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEB8C: 41820020  beq 0x830aebac
	if ctx.cr[0].eq {
	pc = 0x830AEBAC; continue 'dispatch;
	}
	// 830AEB90: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEB94: 39400019  li r10, 0x19
	ctx.r[10].s64 = 25;
	// 830AEB98: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEB9C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEBA0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEBA4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEBA8: 48000008  b 0x830aebb0
	pc = 0x830AEBB0; continue 'dispatch;
	// 830AEBAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEBB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEBB4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEBB8: 388BD658  addi r4, r11, -0x29a8
	ctx.r[4].s64 = ctx.r[11].s64 + -10664;
	// 830AEBBC: 4BFFE03D  bl 0x830acbf8
	ctx.lr = 0x830AEBC0;
	sub_830ACBF8(ctx, base);
	// 830AEBC0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEBC4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEBC8: 4BF29681  bl 0x82fd8248
	ctx.lr = 0x830AEBCC;
	sub_82FD8248(ctx, base);
	// 830AEBCC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEBD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEBD4: 41820020  beq 0x830aebf4
	if ctx.cr[0].eq {
	pc = 0x830AEBF4; continue 'dispatch;
	}
	// 830AEBD8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEBDC: 3940001A  li r10, 0x1a
	ctx.r[10].s64 = 26;
	// 830AEBE0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEBE4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEBE8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEBEC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEBF0: 48000008  b 0x830aebf8
	pc = 0x830AEBF8; continue 'dispatch;
	// 830AEBF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEBF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AEBFC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEC00: 388B7AC4  addi r4, r11, 0x7ac4
	ctx.r[4].s64 = ctx.r[11].s64 + 31428;
	// 830AEC04: 4BFFDFF5  bl 0x830acbf8
	ctx.lr = 0x830AEC08;
	sub_830ACBF8(ctx, base);
	// 830AEC08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEC0C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEC10: 4BF29639  bl 0x82fd8248
	ctx.lr = 0x830AEC14;
	sub_82FD8248(ctx, base);
	// 830AEC14: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEC18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEC1C: 41820020  beq 0x830aec3c
	if ctx.cr[0].eq {
	pc = 0x830AEC3C; continue 'dispatch;
	}
	// 830AEC20: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEC24: 3940001B  li r10, 0x1b
	ctx.r[10].s64 = 27;
	// 830AEC28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEC2C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEC30: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEC34: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEC38: 48000008  b 0x830aec40
	pc = 0x830AEC40; continue 'dispatch;
	// 830AEC3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEC40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AEC44: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEC48: 388B7ACC  addi r4, r11, 0x7acc
	ctx.r[4].s64 = ctx.r[11].s64 + 31436;
	// 830AEC4C: 4BFFDFAD  bl 0x830acbf8
	ctx.lr = 0x830AEC50;
	sub_830ACBF8(ctx, base);
	// 830AEC50: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEC54: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEC58: 4BF295F1  bl 0x82fd8248
	ctx.lr = 0x830AEC5C;
	sub_82FD8248(ctx, base);
	// 830AEC5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEC60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEC64: 41820020  beq 0x830aec84
	if ctx.cr[0].eq {
	pc = 0x830AEC84; continue 'dispatch;
	}
	// 830AEC68: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEC6C: 3940001C  li r10, 0x1c
	ctx.r[10].s64 = 28;
	// 830AEC70: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEC74: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEC78: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEC7C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEC80: 48000008  b 0x830aec88
	pc = 0x830AEC88; continue 'dispatch;
	// 830AEC84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEC88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AEC8C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEC90: 388B7AD8  addi r4, r11, 0x7ad8
	ctx.r[4].s64 = ctx.r[11].s64 + 31448;
	// 830AEC94: 4BFFDF65  bl 0x830acbf8
	ctx.lr = 0x830AEC98;
	sub_830ACBF8(ctx, base);
	// 830AEC98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEC9C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AECA0: 4BF295A9  bl 0x82fd8248
	ctx.lr = 0x830AECA4;
	sub_82FD8248(ctx, base);
	// 830AECA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AECA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AECAC: 41820020  beq 0x830aeccc
	if ctx.cr[0].eq {
	pc = 0x830AECCC; continue 'dispatch;
	}
	// 830AECB0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AECB4: 3940001D  li r10, 0x1d
	ctx.r[10].s64 = 29;
	// 830AECB8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AECBC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AECC0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AECC4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AECC8: 48000008  b 0x830aecd0
	pc = 0x830AECD0; continue 'dispatch;
	// 830AECCC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AECD0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AECD4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AECD8: 388B797C  addi r4, r11, 0x797c
	ctx.r[4].s64 = ctx.r[11].s64 + 31100;
	// 830AECDC: 4BFFDF1D  bl 0x830acbf8
	ctx.lr = 0x830AECE0;
	sub_830ACBF8(ctx, base);
	// 830AECE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AECE4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AECE8: 4BF29561  bl 0x82fd8248
	ctx.lr = 0x830AECEC;
	sub_82FD8248(ctx, base);
	// 830AECEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AECF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AECF4: 41820020  beq 0x830aed14
	if ctx.cr[0].eq {
	pc = 0x830AED14; continue 'dispatch;
	}
	// 830AECF8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AECFC: 3940001E  li r10, 0x1e
	ctx.r[10].s64 = 30;
	// 830AED00: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AED04: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AED08: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AED0C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AED10: 48000008  b 0x830aed18
	pc = 0x830AED18; continue 'dispatch;
	// 830AED14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AED18: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AED1C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AED20: 388B798C  addi r4, r11, 0x798c
	ctx.r[4].s64 = ctx.r[11].s64 + 31116;
	// 830AED24: 4BFFDED5  bl 0x830acbf8
	ctx.lr = 0x830AED28;
	sub_830ACBF8(ctx, base);
	// 830AED28: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AED2C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AED30: 4BF29519  bl 0x82fd8248
	ctx.lr = 0x830AED34;
	sub_82FD8248(ctx, base);
	// 830AED34: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AED38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AED3C: 41820020  beq 0x830aed5c
	if ctx.cr[0].eq {
	pc = 0x830AED5C; continue 'dispatch;
	}
	// 830AED40: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AED44: 3940001F  li r10, 0x1f
	ctx.r[10].s64 = 31;
	// 830AED48: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AED4C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AED50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AED54: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AED58: 48000008  b 0x830aed60
	pc = 0x830AED60; continue 'dispatch;
	// 830AED5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AED60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AED64: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AED68: 388BD668  addi r4, r11, -0x2998
	ctx.r[4].s64 = ctx.r[11].s64 + -10648;
	// 830AED6C: 4BFFDE8D  bl 0x830acbf8
	ctx.lr = 0x830AED70;
	sub_830ACBF8(ctx, base);
	// 830AED70: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AED74: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AED78: 4BF294D1  bl 0x82fd8248
	ctx.lr = 0x830AED7C;
	sub_82FD8248(ctx, base);
	// 830AED7C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AED80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AED84: 41820020  beq 0x830aeda4
	if ctx.cr[0].eq {
	pc = 0x830AEDA4; continue 'dispatch;
	}
	// 830AED88: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AED8C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 830AED90: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AED94: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AED98: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AED9C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEDA0: 48000008  b 0x830aeda8
	pc = 0x830AEDA8; continue 'dispatch;
	// 830AEDA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEDA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEDAC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEDB0: 388BD698  addi r4, r11, -0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + -10600;
	// 830AEDB4: 4BFFDE45  bl 0x830acbf8
	ctx.lr = 0x830AEDB8;
	sub_830ACBF8(ctx, base);
	// 830AEDB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEDBC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEDC0: 4BF29489  bl 0x82fd8248
	ctx.lr = 0x830AEDC4;
	sub_82FD8248(ctx, base);
	// 830AEDC4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEDC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEDCC: 41820020  beq 0x830aedec
	if ctx.cr[0].eq {
	pc = 0x830AEDEC; continue 'dispatch;
	}
	// 830AEDD0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEDD4: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 830AEDD8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEDDC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEDE0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEDE4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEDE8: 48000008  b 0x830aedf0
	pc = 0x830AEDF0; continue 'dispatch;
	// 830AEDEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEDF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEDF4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEDF8: 388BD6C0  addi r4, r11, -0x2940
	ctx.r[4].s64 = ctx.r[11].s64 + -10560;
	// 830AEDFC: 4BFFDDFD  bl 0x830acbf8
	ctx.lr = 0x830AEE00;
	sub_830ACBF8(ctx, base);
	// 830AEE00: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEE04: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEE08: 4BF29441  bl 0x82fd8248
	ctx.lr = 0x830AEE0C;
	sub_82FD8248(ctx, base);
	// 830AEE0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEE10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEE14: 41820020  beq 0x830aee34
	if ctx.cr[0].eq {
	pc = 0x830AEE34; continue 'dispatch;
	}
	// 830AEE18: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEE1C: 39400022  li r10, 0x22
	ctx.r[10].s64 = 34;
	// 830AEE20: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEE24: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEE28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEE2C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEE30: 48000008  b 0x830aee38
	pc = 0x830AEE38; continue 'dispatch;
	// 830AEE34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEE38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEE3C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEE40: 388BD6E0  addi r4, r11, -0x2920
	ctx.r[4].s64 = ctx.r[11].s64 + -10528;
	// 830AEE44: 4BFFDDB5  bl 0x830acbf8
	ctx.lr = 0x830AEE48;
	sub_830ACBF8(ctx, base);
	// 830AEE48: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEE4C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEE50: 4BF293F9  bl 0x82fd8248
	ctx.lr = 0x830AEE54;
	sub_82FD8248(ctx, base);
	// 830AEE54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEE58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEE5C: 41820020  beq 0x830aee7c
	if ctx.cr[0].eq {
	pc = 0x830AEE7C; continue 'dispatch;
	}
	// 830AEE60: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEE64: 39400023  li r10, 0x23
	ctx.r[10].s64 = 35;
	// 830AEE68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEE6C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEE70: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEE74: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEE78: 48000008  b 0x830aee80
	pc = 0x830AEE80; continue 'dispatch;
	// 830AEE7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEE80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEE84: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEE88: 388BD6EC  addi r4, r11, -0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + -10516;
	// 830AEE8C: 4BFFDD6D  bl 0x830acbf8
	ctx.lr = 0x830AEE90;
	sub_830ACBF8(ctx, base);
	// 830AEE90: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEE94: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEE98: 4BF293B1  bl 0x82fd8248
	ctx.lr = 0x830AEE9C;
	sub_82FD8248(ctx, base);
	// 830AEE9C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEEA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEEA4: 41820020  beq 0x830aeec4
	if ctx.cr[0].eq {
	pc = 0x830AEEC4; continue 'dispatch;
	}
	// 830AEEA8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEEAC: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 830AEEB0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEEB4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEEB8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEEBC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEEC0: 48000008  b 0x830aeec8
	pc = 0x830AEEC8; continue 'dispatch;
	// 830AEEC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEEC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEECC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEED0: 388BD6F4  addi r4, r11, -0x290c
	ctx.r[4].s64 = ctx.r[11].s64 + -10508;
	// 830AEED4: 4BFFDD25  bl 0x830acbf8
	ctx.lr = 0x830AEED8;
	sub_830ACBF8(ctx, base);
	// 830AEED8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEEDC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEEE0: 4BF29369  bl 0x82fd8248
	ctx.lr = 0x830AEEE4;
	sub_82FD8248(ctx, base);
	// 830AEEE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEEE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEEEC: 41820020  beq 0x830aef0c
	if ctx.cr[0].eq {
	pc = 0x830AEF0C; continue 'dispatch;
	}
	// 830AEEF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEEF4: 39400025  li r10, 0x25
	ctx.r[10].s64 = 37;
	// 830AEEF8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEEFC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEF00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEF04: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEF08: 48000008  b 0x830aef10
	pc = 0x830AEF10; continue 'dispatch;
	// 830AEF0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEF10: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEF14: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEF18: 388BD700  addi r4, r11, -0x2900
	ctx.r[4].s64 = ctx.r[11].s64 + -10496;
	// 830AEF1C: 4BFFDCDD  bl 0x830acbf8
	ctx.lr = 0x830AEF20;
	sub_830ACBF8(ctx, base);
	// 830AEF20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEF24: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEF28: 4BF29321  bl 0x82fd8248
	ctx.lr = 0x830AEF2C;
	sub_82FD8248(ctx, base);
	// 830AEF2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEF30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEF34: 41820020  beq 0x830aef54
	if ctx.cr[0].eq {
	pc = 0x830AEF54; continue 'dispatch;
	}
	// 830AEF38: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEF3C: 39400026  li r10, 0x26
	ctx.r[10].s64 = 38;
	// 830AEF40: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEF44: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEF48: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEF4C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEF50: 48000008  b 0x830aef58
	pc = 0x830AEF58; continue 'dispatch;
	// 830AEF54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEF58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEF5C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEF60: 388BD70C  addi r4, r11, -0x28f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10484;
	// 830AEF64: 4BFFDC95  bl 0x830acbf8
	ctx.lr = 0x830AEF68;
	sub_830ACBF8(ctx, base);
	// 830AEF68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEF6C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEF70: 4BF292D9  bl 0x82fd8248
	ctx.lr = 0x830AEF74;
	sub_82FD8248(ctx, base);
	// 830AEF74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEF78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEF7C: 41820020  beq 0x830aef9c
	if ctx.cr[0].eq {
	pc = 0x830AEF9C; continue 'dispatch;
	}
	// 830AEF80: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEF84: 39400027  li r10, 0x27
	ctx.r[10].s64 = 39;
	// 830AEF88: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEF8C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEF90: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEF94: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEF98: 48000008  b 0x830aefa0
	pc = 0x830AEFA0; continue 'dispatch;
	// 830AEF9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEFA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEFA4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEFA8: 388BD734  addi r4, r11, -0x28cc
	ctx.r[4].s64 = ctx.r[11].s64 + -10444;
	// 830AEFAC: 4BFFDC4D  bl 0x830acbf8
	ctx.lr = 0x830AEFB0;
	sub_830ACBF8(ctx, base);
	// 830AEFB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEFB4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AEFB8: 4BF29291  bl 0x82fd8248
	ctx.lr = 0x830AEFBC;
	sub_82FD8248(ctx, base);
	// 830AEFBC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEFC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AEFC4: 41820020  beq 0x830aefe4
	if ctx.cr[0].eq {
	pc = 0x830AEFE4; continue 'dispatch;
	}
	// 830AEFC8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AEFCC: 39400028  li r10, 0x28
	ctx.r[10].s64 = 40;
	// 830AEFD0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AEFD4: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AEFD8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AEFDC: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AEFE0: 48000008  b 0x830aefe8
	pc = 0x830AEFE8; continue 'dispatch;
	// 830AEFE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AEFE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AEFEC: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AEFF0: 388BD750  addi r4, r11, -0x28b0
	ctx.r[4].s64 = ctx.r[11].s64 + -10416;
	// 830AEFF4: 4BFFDC05  bl 0x830acbf8
	ctx.lr = 0x830AEFF8;
	sub_830ACBF8(ctx, base);
	// 830AEFF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AEFFC: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AF000: 4BF29249  bl 0x82fd8248
	ctx.lr = 0x830AF004;
	sub_82FD8248(ctx, base);
	// 830AF004: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF008: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF00C: 41820020  beq 0x830af02c
	if ctx.cr[0].eq {
	pc = 0x830AF02C; continue 'dispatch;
	}
	// 830AF010: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF014: 39400029  li r10, 0x29
	ctx.r[10].s64 = 41;
	// 830AF018: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AF01C: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AF020: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AF024: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AF028: 48000008  b 0x830af030
	pc = 0x830AF030; continue 'dispatch;
	// 830AF02C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AF030: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AF034: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AF038: 388BD768  addi r4, r11, -0x2898
	ctx.r[4].s64 = ctx.r[11].s64 + -10392;
	// 830AF03C: 4BFFDBBD  bl 0x830acbf8
	ctx.lr = 0x830AF040;
	sub_830ACBF8(ctx, base);
	// 830AF040: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF044: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AF048: 4BF29201  bl 0x82fd8248
	ctx.lr = 0x830AF04C;
	sub_82FD8248(ctx, base);
	// 830AF04C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF050: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF054: 41820020  beq 0x830af074
	if ctx.cr[0].eq {
	pc = 0x830AF074; continue 'dispatch;
	}
	// 830AF058: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF05C: 3940002A  li r10, 0x2a
	ctx.r[10].s64 = 42;
	// 830AF060: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AF064: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AF068: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AF06C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AF070: 48000008  b 0x830af078
	pc = 0x830AF078; continue 'dispatch;
	// 830AF074: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AF078: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AF07C: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AF080: 388BD784  addi r4, r11, -0x287c
	ctx.r[4].s64 = ctx.r[11].s64 + -10364;
	// 830AF084: 4BFFDB75  bl 0x830acbf8
	ctx.lr = 0x830AF088;
	sub_830ACBF8(ctx, base);
	// 830AF088: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF08C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 830AF090: 4BF291B9  bl 0x82fd8248
	ctx.lr = 0x830AF094;
	sub_82FD8248(ctx, base);
	// 830AF094: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF098: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF09C: 41820020  beq 0x830af0bc
	if ctx.cr[0].eq {
	pc = 0x830AF0BC; continue 'dispatch;
	}
	// 830AF0A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF0A4: 3940002B  li r10, 0x2b
	ctx.r[10].s64 = 43;
	// 830AF0A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AF0AC: 9BA30028  stb r29, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830AF0B0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AF0B4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830AF0B8: 48000008  b 0x830af0c0
	pc = 0x830AF0C0; continue 'dispatch;
	// 830AF0BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830AF0C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830AF0C4: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AF0C8: 388BD7A0  addi r4, r11, -0x2860
	ctx.r[4].s64 = ctx.r[11].s64 + -10336;
	// 830AF0CC: 4BFFDB2D  bl 0x830acbf8
	ctx.lr = 0x830AF0D0;
	sub_830ACBF8(ctx, base);
	// 830AF0D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF0D4: 3D60830B  lis r11, -0x7cf5
	ctx.r[11].s64 = -2096431104;
	// 830AF0D8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830AF0DC: 388BE348  addi r4, r11, -0x1cb8
	ctx.r[4].s64 = ctx.r[11].s64 + -7352;
	// 830AF0E0: 386ABC70  addi r3, r10, -0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + -17296;
	// 830AF0E4: 4BF48A55  bl 0x82ff7b38
	ctx.lr = 0x830AF0E8;
	sub_82FF7B38(ctx, base);
	// 830AF0E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AF0EC: 4BF46725  bl 0x82ff5810
	ctx.lr = 0x830AF0F0;
	sub_82FF5810(ctx, base);
	// 830AF0F0: 807BBC54  lwz r3, -0x43ac(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-17324 as u32) ) } as u64;
	// 830AF0F4: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 830AF0F8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830AF0FC: 4BF4B365  bl 0x82ffa460
	ctx.lr = 0x830AF100;
	sub_82FFA460(ctx, base);
	// 830AF100: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF104: 41820020  beq 0x830af124
	if ctx.cr[0].eq {
	pc = 0x830AF124; continue 'dispatch;
	}
	// 830AF108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF10C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF110: 41820014  beq 0x830af124
	if ctx.cr[0].eq {
	pc = 0x830AF124; continue 'dispatch;
	}
	// 830AF114: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF118: 48000010  b 0x830af128
	pc = 0x830AF128; continue 'dispatch;
	// 830AF11C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AF120: 4BF466F1  bl 0x82ff5810
	ctx.lr = 0x830AF124;
	sub_82FF5810(ctx, base);
	// 830AF124: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830AF128: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830AF12C: 480F9084  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF130 size=8
    let mut pc: u32 = 0x830AF130;
    'dispatch: loop {
        match pc {
            0x830AF130 => {
    //   block [0x830AF130..0x830AF138)
	// 830AF130: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AF134: 82180F44  lwz r16, 0xf44(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(3908 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF138 size=20
    let mut pc: u32 = 0x830AF138;
    'dispatch: loop {
        match pc {
            0x830AF138 => {
    //   block [0x830AF138..0x830AF14C)
	// 830AF138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF13C: 3C60830B  lis r3, -0x7cf5
	ctx.r[3].s64 = -2096431104;
	// 830AF140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF144: 3863F11C  addi r3, r3, -0xee4
	ctx.r[3].s64 = ctx.r[3].s64 + -3812;
	// 830AF148: 480F9068  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF14C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF14C size=40
    let mut pc: u32 = 0x830AF14C;
    'dispatch: loop {
        match pc {
            0x830AF14C => {
    //   block [0x830AF14C..0x830AF174)
	// 830AF14C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830AF150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF15C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830AF160: 4BF466B1  bl 0x82ff5810
	ctx.lr = 0x830AF164;
	sub_82FF5810(ctx, base);
	// 830AF164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF174 size=40
    let mut pc: u32 = 0x830AF174;
    'dispatch: loop {
        match pc {
            0x830AF174 => {
    //   block [0x830AF174..0x830AF19C)
	// 830AF174: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830AF178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF184: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830AF188: 4BF29159  bl 0x82fd82e0
	ctx.lr = 0x830AF18C;
	sub_82FD82E0(ctx, base);
	// 830AF18C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF19C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF19C size=40
    let mut pc: u32 = 0x830AF19C;
    'dispatch: loop {
        match pc {
            0x830AF19C => {
    //   block [0x830AF19C..0x830AF1C4)
	// 830AF19C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830AF1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF1AC: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830AF1B0: 4BF29131  bl 0x82fd82e0
	ctx.lr = 0x830AF1B4;
	sub_82FD82E0(ctx, base);
	// 830AF1B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF1C8 size=64
    let mut pc: u32 = 0x830AF1C8;
    'dispatch: loop {
        match pc {
            0x830AF1C8 => {
    //   block [0x830AF1C8..0x830AF208)
	// 830AF1C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830AF1CC: 419A0050  beq cr6, 0x830af21c
	if ctx.cr[6].eq {
		sub_830AF21C(ctx, base);
		return;
	}
	// 830AF1D0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF1D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF1D8: 41820044  beq 0x830af21c
	if ctx.cr[0].eq {
		sub_830AF21C(ctx, base);
		return;
	}
	// 830AF1DC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF1E0: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830AF1E4: 396B0890  addi r11, r11, 0x890
	ctx.r[11].s64 = ctx.r[11].s64 + 2192;
	// 830AF1E8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AF1EC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830AF1F0: 41980024  blt cr6, 0x830af214
	if ctx.cr[6].lt {
		sub_830AF214(ctx, base);
		return;
	}
	// 830AF1F4: 419A0018  beq cr6, 0x830af20c
	if ctx.cr[6].eq {
		sub_830AF20C(ctx, base);
		return;
	}
	// 830AF1F8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830AF1FC: 4198000C  blt cr6, 0x830af208
	if ctx.cr[6].lt {
		sub_830AF208(ctx, base);
		return;
	}
	// 830AF200: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 830AF204: 4800001C  b 0x830af220
	sub_830AF21C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF208 size=4
    let mut pc: u32 = 0x830AF208;
    'dispatch: loop {
        match pc {
            0x830AF208 => {
    //   block [0x830AF208..0x830AF20C)
	// 830AF208: 4BFFE5B0  b 0x830ad7b8
	sub_830AD7B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF20C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF20C size=8
    let mut pc: u32 = 0x830AF20C;
    'dispatch: loop {
        match pc {
            0x830AF20C => {
    //   block [0x830AF20C..0x830AF214)
	// 830AF20C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 830AF210: 4BFFD560  b 0x830ac770
	sub_830AC770(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF214 size=8
    let mut pc: u32 = 0x830AF214;
    'dispatch: loop {
        match pc {
            0x830AF214 => {
    //   block [0x830AF214..0x830AF21C)
	// 830AF214: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 830AF218: 4BFFE978  b 0x830adb90
	sub_830ADB90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF21C size=16
    let mut pc: u32 = 0x830AF21C;
    'dispatch: loop {
        match pc {
            0x830AF21C => {
    //   block [0x830AF21C..0x830AF22C)
	// 830AF21C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830AF220: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF224: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AF228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF230 size=24
    let mut pc: u32 = 0x830AF230;
    'dispatch: loop {
        match pc {
            0x830AF230 => {
    //   block [0x830AF230..0x830AF248)
	// 830AF230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AF234: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 830AF238: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF23C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830AF240: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AF244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF248 size=196
    let mut pc: u32 = 0x830AF248;
    'dispatch: loop {
        match pc {
            0x830AF248 => {
    //   block [0x830AF248..0x830AF30C)
	// 830AF248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AF254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AF258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF25C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AF260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AF268: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF26C: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830AF270: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF274: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830AF278: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF27C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830AF280: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AF284: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830AF288: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AF28C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830AF290: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830AF294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF298: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF29C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF2A0: 4E800421  bctrl
	ctx.lr = 0x830AF2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF2A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF2A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AF2AC: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830AF2B0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 830AF2B4: 480F8F2D  bl 0x831a81e0
	ctx.lr = 0x830AF2B8;
	sub_831A81E0(ctx, base);
	// 830AF2B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF2BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AF2C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AF2C4: 4099002C  ble cr6, 0x830af2f0
	if !ctx.cr[6].gt {
	pc = 0x830AF2F0; continue 'dispatch;
	}
	// 830AF2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830AF2CC: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF2D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830AF2D4: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF2D8: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830AF2DC: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 830AF2E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830AF2E4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF2E8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830AF2EC: 4198FFE0  blt cr6, 0x830af2cc
	if ctx.cr[6].lt {
	pc = 0x830AF2CC; continue 'dispatch;
	}
	// 830AF2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF2F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AF2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AF304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AF308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF310 size=112
    let mut pc: u32 = 0x830AF310;
    'dispatch: loop {
        match pc {
            0x830AF310 => {
    //   block [0x830AF310..0x830AF380)
	// 830AF310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF318: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AF31C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF324: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF328: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF32C: 4182000C  beq 0x830af338
	if ctx.cr[0].eq {
	pc = 0x830AF338; continue 'dispatch;
	}
	// 830AF330: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF334: 4BFEB3E5  bl 0x8309a718
	ctx.lr = 0x830AF338;
	sub_8309A718(ctx, base);
	// 830AF338: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF33C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF340: 4182000C  beq 0x830af34c
	if ctx.cr[0].eq {
	pc = 0x830AF34C; continue 'dispatch;
	}
	// 830AF344: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF348: 4BFEB3D1  bl 0x8309a718
	ctx.lr = 0x830AF34C;
	sub_8309A718(ctx, base);
	// 830AF34C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF350: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF354: 41820018  beq 0x830af36c
	if ctx.cr[0].eq {
	pc = 0x830AF36C; continue 'dispatch;
	}
	// 830AF358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF35C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF360: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF368: 4E800421  bctrl
	ctx.lr = 0x830AF36C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF36C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AF37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF380 size=104
    let mut pc: u32 = 0x830AF380;
    'dispatch: loop {
        match pc {
            0x830AF380 => {
    //   block [0x830AF380..0x830AF3E8)
	// 830AF380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF384: 480F8DE5  bl 0x831a8168
	ctx.lr = 0x830AF388;
	sub_831A8130(ctx, base);
	// 830AF388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF38C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AF390: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830AF394: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF398: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF39C: 41820038  beq 0x830af3d4
	if ctx.cr[0].eq {
	pc = 0x830AF3D4; continue 'dispatch;
	}
	// 830AF3A0: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF3A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830AF3A8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF3AC: 41820028  beq 0x830af3d4
	if ctx.cr[0].eq {
	pc = 0x830AF3D4; continue 'dispatch;
	}
	// 830AF3B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AF3B4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF3B8: 4BF48D19  bl 0x82ff80d0
	ctx.lr = 0x830AF3BC;
	sub_82FF80D0(ctx, base);
	// 830AF3BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF3C0: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830AF3C4: 419A001C  beq cr6, 0x830af3e0
	if ctx.cr[6].eq {
	pc = 0x830AF3E0; continue 'dispatch;
	}
	// 830AF3C8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830AF3CC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830AF3D0: 4198FFE0  blt cr6, 0x830af3b0
	if ctx.cr[6].lt {
	pc = 0x830AF3B0; continue 'dispatch;
	}
	// 830AF3D4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830AF3D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AF3DC: 480F8DDC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830AF3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF3E4: 4BFFFFF4  b 0x830af3d8
	pc = 0x830AF3D8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF3E8 size=8
    let mut pc: u32 = 0x830AF3E8;
    'dispatch: loop {
        match pc {
            0x830AF3E8 => {
    //   block [0x830AF3E8..0x830AF3F0)
	// 830AF3E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AF3EC: 821815FC  lwz r16, 0x15fc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(5628 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF3F0 size=384
    let mut pc: u32 = 0x830AF3F0;
    'dispatch: loop {
        match pc {
            0x830AF3F0 => {
    //   block [0x830AF3F0..0x830AF570)
	// 830AF3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF3F4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830AF3F8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830AF3FC: 480F8D65  bl 0x831a8160
	ctx.lr = 0x830AF400;
	sub_831A8130(ctx, base);
	// 830AF400: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830AF404: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF408: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830AF40C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830AF410: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830AF414: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 830AF418: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 830AF41C: 937D0004  stw r27, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 830AF420: 937D0008  stw r27, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 830AF424: 809C000C  lwz r4, 0xc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF428: 909D000C  stw r4, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 830AF42C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830AF434: 419A0130  beq cr6, 0x830af564
	if ctx.cr[6].eq {
	pc = 0x830AF564; continue 'dispatch;
	}
	// 830AF438: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF43C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830AF440: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF444: 4BF28E55  bl 0x82fd8298
	ctx.lr = 0x830AF448;
	sub_82FD8298(ctx, base);
	// 830AF448: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF44C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF450: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830AF454: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF458: 4182001C  beq 0x830af474
	if ctx.cr[0].eq {
	pc = 0x830AF474; continue 'dispatch;
	}
	// 830AF45C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF460: 4BFFFDE9  bl 0x830af248
	ctx.lr = 0x830AF464;
	sub_830AF248(ctx, base);
	// 830AF464: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830AF468: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF46C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF470: 48000008  b 0x830af478
	pc = 0x830AF478; continue 'dispatch;
	// 830AF474: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830AF478: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830AF47C: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF480: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF484: 4BF28E15  bl 0x82fd8298
	ctx.lr = 0x830AF488;
	sub_82FD8298(ctx, base);
	// 830AF488: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF48C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF490: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830AF494: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF498: 4182001C  beq 0x830af4b4
	if ctx.cr[0].eq {
	pc = 0x830AF4B4; continue 'dispatch;
	}
	// 830AF49C: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF4A0: 4BFFFDA9  bl 0x830af248
	ctx.lr = 0x830AF4A4;
	sub_830AF248(ctx, base);
	// 830AF4A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830AF4A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4B0: 48000008  b 0x830af4b8
	pc = 0x830AF4B8; continue 'dispatch;
	// 830AF4B4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830AF4B8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830AF4BC: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF4C0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830AF4C4: 4BF28DD5  bl 0x82fd8298
	ctx.lr = 0x830AF4C8;
	sub_82FD8298(ctx, base);
	// 830AF4C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4D0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830AF4D4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830AF4D8: 41820038  beq 0x830af510
	if ctx.cr[0].eq {
	pc = 0x830AF510; continue 'dispatch;
	}
	// 830AF4DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF4E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830AF4E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF4E8: 80DD000C  lwz r6, 0xc(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF4EC: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF4F0: 4BF23551  bl 0x82fd2a40
	ctx.lr = 0x830AF4F4;
	sub_82FD2A40(ctx, base);
	// 830AF4F4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF4FC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830AF500: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 830AF504: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830AF508: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830AF50C: 48000008  b 0x830af514
	pc = 0x830AF514; continue 'dispatch;
	// 830AF510: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 830AF514: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 830AF518: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830AF51C: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 830AF520: 40980044  bge cr6, 0x830af564
	if !ctx.cr[6].lt {
	pc = 0x830AF564; continue 'dispatch;
	}
	// 830AF524: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AF528: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF52C: 4BF7D345  bl 0x8302c870
	ctx.lr = 0x830AF530;
	sub_8302C870(ctx, base);
	// 830AF530: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF534: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF538: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF53C: 4BF21645  bl 0x82fd0b80
	ctx.lr = 0x830AF540;
	sub_82FD0B80(ctx, base);
	// 830AF540: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830AF544: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF548: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF54C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF550: 4BF8BC01  bl 0x8303b150
	ctx.lr = 0x830AF554;
	sub_8303B150(ctx, base);
	// 830AF554: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF558: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830AF55C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830AF560: 4BFFFFBC  b 0x830af51c
	pc = 0x830AF51C; continue 'dispatch;
	// 830AF564: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AF568: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830AF56C: 480F8C44  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AF570 size=8
    let mut pc: u32 = 0x830AF570;
    'dispatch: loop {
        match pc {
            0x830AF570 => {
    //   block [0x830AF570..0x830AF578)
	// 830AF570: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830AF574: 821815FC  lwz r16, 0x15fc(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(5628 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF578 size=24
    let mut pc: u32 = 0x830AF578;
    'dispatch: loop {
        match pc {
            0x830AF578 => {
    //   block [0x830AF578..0x830AF590)
	// 830AF578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AF588: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AF58C: 4810169D  bl 0x831b0c28
	ctx.lr = 0x830AF590;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF598 size=104
    let mut pc: u32 = 0x830AF598;
    'dispatch: loop {
        match pc {
            0x830AF598 => {
    //   block [0x830AF598..0x830AF600)
	// 830AF598: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830AF59C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF5A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF5A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF5A8: 83DF00A4  lwz r30, 0xa4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830AF5AC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF5B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF5B4: 4182000C  beq 0x830af5c0
	if ctx.cr[0].eq {
	pc = 0x830AF5C0; continue 'dispatch;
	}
	// 830AF5B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF5BC: 4BFEB15D  bl 0x8309a718
	ctx.lr = 0x830AF5C0;
	sub_8309A718(ctx, base);
	// 830AF5C0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF5C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF5C8: 4182000C  beq 0x830af5d4
	if ctx.cr[0].eq {
	pc = 0x830AF5D4; continue 'dispatch;
	}
	// 830AF5CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF5D0: 4BFEB149  bl 0x8309a718
	ctx.lr = 0x830AF5D4;
	sub_8309A718(ctx, base);
	// 830AF5D4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830AF5D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF5DC: 41820018  beq 0x830af5f4
	if ctx.cr[0].eq {
	pc = 0x830AF5F4; continue 'dispatch;
	}
	// 830AF5E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF5E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830AF5E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF5EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF5F0: 4E800421  bctrl
	ctx.lr = 0x830AF5F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF5F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AF5F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AF5FC: 4810162D  bl 0x831b0c28
	ctx.lr = 0x830AF600;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF600 size=48
    let mut pc: u32 = 0x830AF600;
    'dispatch: loop {
        match pc {
            0x830AF600 => {
    //   block [0x830AF600..0x830AF630)
	// 830AF600: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830AF604: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF608: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF60C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF610: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830AF614: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF618: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AF61C: 4BF28CC5  bl 0x82fd82e0
	ctx.lr = 0x830AF620;
	sub_82FD82E0(ctx, base);
	// 830AF620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF630 size=48
    let mut pc: u32 = 0x830AF630;
    'dispatch: loop {
        match pc {
            0x830AF630 => {
    //   block [0x830AF630..0x830AF660)
	// 830AF630: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830AF634: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF638: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF63C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF640: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830AF644: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF648: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AF64C: 4BF28C95  bl 0x82fd82e0
	ctx.lr = 0x830AF650;
	sub_82FD82E0(ctx, base);
	// 830AF650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF660 size=48
    let mut pc: u32 = 0x830AF660;
    'dispatch: loop {
        match pc {
            0x830AF660 => {
    //   block [0x830AF660..0x830AF690)
	// 830AF660: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830AF664: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF668: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF66C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF670: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830AF674: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830AF678: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AF67C: 4BF28C65  bl 0x82fd82e0
	ctx.lr = 0x830AF680;
	sub_82FD82E0(ctx, base);
	// 830AF680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF690 size=72
    let mut pc: u32 = 0x830AF690;
    'dispatch: loop {
        match pc {
            0x830AF690 => {
    //   block [0x830AF690..0x830AF6D8)
	// 830AF690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AF69C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF6A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF6A4: 4BF4A715  bl 0x82ff9db8
	ctx.lr = 0x830AF6A8;
	sub_82FF9DB8(ctx, base);
	// 830AF6A8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF6AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AF6B0: 396B5D54  addi r11, r11, 0x5d54
	ctx.r[11].s64 = ctx.r[11].s64 + 23892;
	// 830AF6B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF6B8: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830AF6BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF6C0: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 830AF6C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF6D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AF6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF6D8 size=356
    let mut pc: u32 = 0x830AF6D8;
    'dispatch: loop {
        match pc {
            0x830AF6D8 => {
    //   block [0x830AF6D8..0x830AF83C)
	// 830AF6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF6DC: 480F8A79  bl 0x831a8154
	ctx.lr = 0x830AF6E0;
	sub_831A8130(ctx, base);
	// 830AF6E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF6E4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 830AF6E8: 89770004  lbz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF6EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF6F0: 40820144  bne 0x830af834
	if !ctx.cr[0].eq {
	pc = 0x830AF834; continue 'dispatch;
	}
	// 830AF6F4: 89770005  lbz r11, 5(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(5 as u32) ) } as u64;
	// 830AF6F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF6FC: 40820014  bne 0x830af710
	if !ctx.cr[0].eq {
	pc = 0x830AF710; continue 'dispatch;
	}
	// 830AF700: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF704: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF70C: 4E800421  bctrl
	ctx.lr = 0x830AF710;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF710: 480013A1  bl 0x830b0ab0
	ctx.lr = 0x830AF714;
	sub_830B0AB0(ctx, base);
	// 830AF714: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830AF718: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF71C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830AF720: 3BCB16B4  addi r30, r11, 0x16b4
	ctx.r[30].s64 = ctx.r[11].s64 + 5812;
	// 830AF724: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830AF728: 831B0010  lwz r24, 0x10(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AF72C: 3B9E001C  addi r28, r30, 0x1c
	ctx.r[28].s64 = ctx.r[30].s64 + 28;
	// 830AF730: 3BBE43B4  addi r29, r30, 0x43b4
	ctx.r[29].s64 = ctx.r[30].s64 + 17332;
	// 830AF734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AF738: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830AF73C: 4BFF4035  bl 0x830a3770
	ctx.lr = 0x830AF740;
	sub_830A3770(ctx, base);
	// 830AF740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF744: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AF748: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF74C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF750: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AF754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF758: 4E800421  bctrl
	ctx.lr = 0x830AF75C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF75C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AF760: 40820040  bne 0x830af7a0
	if !ctx.cr[0].eq {
	pc = 0x830AF7A0; continue 'dispatch;
	}
	// 830AF764: 389EFFE8  addi r4, r30, -0x18
	ctx.r[4].s64 = ctx.r[30].s64 + -24;
	// 830AF768: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AF76C: 4BF244D5  bl 0x82fd3c40
	ctx.lr = 0x830AF770;
	sub_82FD3C40(ctx, base);
	// 830AF770: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AF774: 4182002C  beq 0x830af7a0
	if ctx.cr[0].eq {
	pc = 0x830AF7A0; continue 'dispatch;
	}
	// 830AF778: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF77C: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 830AF780: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 830AF784: 60A5FFFD  ori r5, r5, 0xfffd
	ctx.r[5].u64 = ctx.r[5].u64 | 65533;
	// 830AF788: 6084FFF0  ori r4, r4, 0xfff0
	ctx.r[4].u64 = ctx.r[4].u64 | 65520;
	// 830AF78C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF790: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AF794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF798: 4E800421  bctrl
	ctx.lr = 0x830AF79C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF79C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 830AF7A0: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AF7A4: 4082005C  bne 0x830af800
	if !ctx.cr[0].eq {
	pc = 0x830AF800; continue 'dispatch;
	}
	// 830AF7A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AF7AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AF7B0: 4BF24491  bl 0x82fd3c40
	ctx.lr = 0x830AF7B4;
	sub_82FD3C40(ctx, base);
	// 830AF7B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AF7B8: 41820048  beq 0x830af800
	if ctx.cr[0].eq {
	pc = 0x830AF800; continue 'dispatch;
	}
	// 830AF7BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF7C0: 3CA0000F  lis r5, 0xf
	ctx.r[5].s64 = 983040;
	// 830AF7C4: 3C80000F  lis r4, 0xf
	ctx.r[4].s64 = 983040;
	// 830AF7C8: 60A5FFFD  ori r5, r5, 0xfffd
	ctx.r[5].u64 = ctx.r[5].u64 | 65533;
	// 830AF7CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF7D0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AF7D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF7D8: 4E800421  bctrl
	ctx.lr = 0x830AF7DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF7DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AF7E0: 3CA00010  lis r5, 0x10
	ctx.r[5].s64 = 1048576;
	// 830AF7E4: 3C800010  lis r4, 0x10
	ctx.r[4].s64 = 1048576;
	// 830AF7E8: 60A5FFFD  ori r5, r5, 0xfffd
	ctx.r[5].u64 = ctx.r[5].u64 | 65533;
	// 830AF7EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF7F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AF7F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AF7F8: 4E800421  bctrl
	ctx.lr = 0x830AF7FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AF7FC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 830AF800: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AF804: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF808: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AF80C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AF810: 48000EA9  bl 0x830b06b8
	ctx.lr = 0x830AF814;
	sub_830B06B8(ctx, base);
	// 830AF814: 397E43B4  addi r11, r30, 0x43b4
	ctx.r[11].s64 = ctx.r[30].s64 + 17332;
	// 830AF818: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 830AF81C: 396B02E8  addi r11, r11, 0x2e8
	ctx.r[11].s64 = ctx.r[11].s64 + 744;
	// 830AF820: 3B9C00BA  addi r28, r28, 0xba
	ctx.r[28].s64 = ctx.r[28].s64 + 186;
	// 830AF824: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AF828: 4198FF0C  blt cr6, 0x830af734
	if ctx.cr[6].lt {
	pc = 0x830AF734; continue 'dispatch;
	}
	// 830AF82C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AF830: 99770004  stb r11, 4(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830AF834: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 830AF838: 480F896C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF840 size=100
    let mut pc: u32 = 0x830AF840;
    'dispatch: loop {
        match pc {
            0x830AF840 => {
    //   block [0x830AF840..0x830AF8A4)
	// 830AF840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF844: 480F8925  bl 0x831a8168
	ctx.lr = 0x830AF848;
	sub_831A8130(ctx, base);
	// 830AF848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF84C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830AF850: 897C0005  lbz r11, 5(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(5 as u32) ) } as u64;
	// 830AF854: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF858: 40820044  bne 0x830af89c
	if !ctx.cr[0].eq {
	pc = 0x830AF89C; continue 'dispatch;
	}
	// 830AF85C: 48001255  bl 0x830b0ab0
	ctx.lr = 0x830AF860;
	sub_830B0AB0(ctx, base);
	// 830AF860: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF864: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830AF868: 3BCB1690  addi r30, r11, 0x1690
	ctx.r[30].s64 = ctx.r[11].s64 + 5776;
	// 830AF86C: 3BFE0040  addi r31, r30, 0x40
	ctx.r[31].s64 = ctx.r[30].s64 + 64;
	// 830AF870: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AF874: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AF878: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AF87C: 48000D0D  bl 0x830b0588
	ctx.lr = 0x830AF880;
	sub_830B0588(ctx, base);
	// 830AF880: 397E0040  addi r11, r30, 0x40
	ctx.r[11].s64 = ctx.r[30].s64 + 64;
	// 830AF884: 3BFF00BA  addi r31, r31, 0xba
	ctx.r[31].s64 = ctx.r[31].s64 + 186;
	// 830AF888: 396B4392  addi r11, r11, 0x4392
	ctx.r[11].s64 = ctx.r[11].s64 + 17298;
	// 830AF88C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AF890: 4198FFE0  blt cr6, 0x830af870
	if ctx.cr[6].lt {
	pc = 0x830AF870; continue 'dispatch;
	}
	// 830AF894: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AF898: 997C0005  stb r11, 5(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 830AF89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AF8A0: 480F8918  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF8A8 size=88
    let mut pc: u32 = 0x830AF8A8;
    'dispatch: loop {
        match pc {
            0x830AF8A8 => {
    //   block [0x830AF8A8..0x830AF900)
	// 830AF8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF8B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AF8B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AF8B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF8BC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF8C4: 396B5D54  addi r11, r11, 0x5d54
	ctx.r[11].s64 = ctx.r[11].s64 + 23892;
	// 830AF8C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AF8CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF8D0: 4BF4A4E9  bl 0x82ff9db8
	ctx.lr = 0x830AF8D4;
	sub_82FF9DB8(ctx, base);
	// 830AF8D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AF8D8: 4182000C  beq 0x830af8e4
	if ctx.cr[0].eq {
	pc = 0x830AF8E4; continue 'dispatch;
	}
	// 830AF8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF8E0: 4BF28A01  bl 0x82fd82e0
	ctx.lr = 0x830AF8E4;
	sub_82FD82E0(ctx, base);
	// 830AF8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF8E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AF8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF8F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AF8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AF8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF900 size=72
    let mut pc: u32 = 0x830AF900;
    'dispatch: loop {
        match pc {
            0x830AF900 => {
    //   block [0x830AF900..0x830AF948)
	// 830AF900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AF908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AF90C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AF914: 4BF4A4A5  bl 0x82ff9db8
	ctx.lr = 0x830AF918;
	sub_82FF9DB8(ctx, base);
	// 830AF918: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AF920: 396B6898  addi r11, r11, 0x6898
	ctx.r[11].s64 = ctx.r[11].s64 + 26776;
	// 830AF924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AF928: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830AF92C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AF930: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 830AF934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AF938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AF93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AF940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AF944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AF948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AF948 size=180
    let mut pc: u32 = 0x830AF948;
    'dispatch: loop {
        match pc {
            0x830AF948 => {
    //   block [0x830AF948..0x830AF9FC)
	// 830AF948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AF94C: 480F881D  bl 0x831a8168
	ctx.lr = 0x830AF950;
	sub_831A8130(ctx, base);
	// 830AF950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AF954: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830AF958: 897C0005  lbz r11, 5(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(5 as u32) ) } as u64;
	// 830AF95C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AF960: 40820094  bne 0x830af9f4
	if !ctx.cr[0].eq {
	pc = 0x830AF9F4; continue 'dispatch;
	}
	// 830AF964: 4800114D  bl 0x830b0ab0
	ctx.lr = 0x830AF968;
	sub_830B0AB0(ctx, base);
	// 830AF968: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AF96C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AF970: 3BEB5D60  addi r31, r11, 0x5d60
	ctx.r[31].s64 = ctx.r[11].s64 + 23904;
	// 830AF974: 3BBF0060  addi r29, r31, 0x60
	ctx.r[29].s64 = ctx.r[31].s64 + 96;
	// 830AF978: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AF97C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF984: 48000C05  bl 0x830b0588
	ctx.lr = 0x830AF988;
	sub_830B0588(ctx, base);
	// 830AF988: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 830AF98C: 3BBD004A  addi r29, r29, 0x4a
	ctx.r[29].s64 = ctx.r[29].s64 + 74;
	// 830AF990: 396B0AB2  addi r11, r11, 0xab2
	ctx.r[11].s64 = ctx.r[11].s64 + 2738;
	// 830AF994: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AF998: 4198FFE0  blt cr6, 0x830af978
	if ctx.cr[6].lt {
	pc = 0x830AF978; continue 'dispatch;
	}
	// 830AF99C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 830AF9A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF9A8: 48000BE1  bl 0x830b0588
	ctx.lr = 0x830AF9AC;
	sub_830B0588(ctx, base);
	// 830AF9AC: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 830AF9B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF9B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF9B8: 48000BD1  bl 0x830b0588
	ctx.lr = 0x830AF9BC;
	sub_830B0588(ctx, base);
	// 830AF9BC: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 830AF9C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF9C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF9C8: 48000BC1  bl 0x830b0588
	ctx.lr = 0x830AF9CC;
	sub_830B0588(ctx, base);
	// 830AF9CC: 389F0038  addi r4, r31, 0x38
	ctx.r[4].s64 = ctx.r[31].s64 + 56;
	// 830AF9D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF9D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF9D8: 48000BB1  bl 0x830b0588
	ctx.lr = 0x830AF9DC;
	sub_830B0588(ctx, base);
	// 830AF9DC: 389F0048  addi r4, r31, 0x48
	ctx.r[4].s64 = ctx.r[31].s64 + 72;
	// 830AF9E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AF9E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AF9E8: 48000BA1  bl 0x830b0588
	ctx.lr = 0x830AF9EC;
	sub_830B0588(ctx, base);
	// 830AF9EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AF9F0: 997C0005  stb r11, 5(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 830AF9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830AF9F8: 480F87C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA00 size=52
    //   switch @ 0x830AFA24: r11 with 30 label(s)
    //       case  0 → 0x830AFA54
    //       case  1 → 0x830AFA34
    //       case  2 → 0x830AFA34
    //       case  3 → 0x830AFA34
    //       case  4 → 0x830AFA34
    //       case  5 → 0x830AFA34
    //       case  6 → 0x830AFA3C
    //       case  7 → 0x830AFA3C
    //       case  8 → 0x830AFA3C
    //       case  9 → 0x830AFA44
    //       case 10 → 0x830AFA44
    //       case 11 → 0x830AFA44
    //       case 12 → 0x830AFA4C
    //       case 13 → 0x830AFA4C
    //       case 14 → 0x830AFA4C
    //       case 15 → 0x830AFA54
    //       case 16 → 0x830AFA54
    //       case 17 → 0x830AFA54
    //       case 18 → 0x830AFA54
    //       case 19 → 0x830AFA5C
    //       case 20 → 0x830AFA5C
    //       case 21 → 0x830AFA5C
    //       case 22 → 0x830AFA5C
    //       case 23 → 0x830AFA5C
    //       case 24 → 0x830AFA64
    //       case 25 → 0x830AFA64
    //       case 26 → 0x830AFA64
    //       case 27 → 0x830AFA64
    //       case 28 → 0x830AFA5C
    //       case 29 → 0x830AFA5C
    let mut pc: u32 = 0x830AFA00;
    'dispatch: loop {
        match pc {
            0x830AFA00 => {
    //   block [0x830AFA00..0x830AFA34)
	// 830AFA00: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 830AFA04: 2B0B001D  cmplwi cr6, r11, 0x1d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 29 as u32, &mut ctx.xer);
	// 830AFA08: 41990064  bgt cr6, 0x830afa6c
	if ctx.cr[6].gt {
		sub_830AFA6C(ctx, base);
		return;
	}
	// 830AFA0C: 3D808218  lis r12, -0x7de8
	ctx.r[12].s64 = -2112356352;
	// 830AFA10: 398C6878  addi r12, r12, 0x6878
	ctx.r[12].s64 = ctx.r[12].s64 + 26744;
	// 830AFA14: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AFA18: 3D80830B  lis r12, -0x7cf5
	ctx.r[12].s64 = -2096431104;
	// 830AFA1C: 398CFA34  addi r12, r12, -0x5cc
	ctx.r[12].s64 = ctx.r[12].s64 + -1484;
	// 830AFA20: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830AFA24: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 830AFA28: 60000000  nop
	// 830AFA2C: 60000000  nop
	// 830AFA30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA34 size=8
    let mut pc: u32 = 0x830AFA34;
    'dispatch: loop {
        match pc {
            0x830AFA34 => {
    //   block [0x830AFA34..0x830AFA3C)
	// 830AFA34: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 830AFA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA3C size=8
    let mut pc: u32 = 0x830AFA3C;
    'dispatch: loop {
        match pc {
            0x830AFA3C => {
    //   block [0x830AFA3C..0x830AFA44)
	// 830AFA3C: 3860001F  li r3, 0x1f
	ctx.r[3].s64 = 31;
	// 830AFA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA44 size=8
    let mut pc: u32 = 0x830AFA44;
    'dispatch: loop {
        match pc {
            0x830AFA44 => {
    //   block [0x830AFA44..0x830AFA4C)
	// 830AFA44: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 830AFA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA4C size=8
    let mut pc: u32 = 0x830AFA4C;
    'dispatch: loop {
        match pc {
            0x830AFA4C => {
    //   block [0x830AFA4C..0x830AFA54)
	// 830AFA4C: 38600021  li r3, 0x21
	ctx.r[3].s64 = 33;
	// 830AFA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA54 size=8
    let mut pc: u32 = 0x830AFA54;
    'dispatch: loop {
        match pc {
            0x830AFA54 => {
    //   block [0x830AFA54..0x830AFA5C)
	// 830AFA54: 38600022  li r3, 0x22
	ctx.r[3].s64 = 34;
	// 830AFA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA5C size=8
    let mut pc: u32 = 0x830AFA5C;
    'dispatch: loop {
        match pc {
            0x830AFA5C => {
    //   block [0x830AFA5C..0x830AFA64)
	// 830AFA5C: 38600023  li r3, 0x23
	ctx.r[3].s64 = 35;
	// 830AFA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA64 size=8
    let mut pc: u32 = 0x830AFA64;
    'dispatch: loop {
        match pc {
            0x830AFA64 => {
    //   block [0x830AFA64..0x830AFA6C)
	// 830AFA64: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830AFA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830AFA6C size=8
    let mut pc: u32 = 0x830AFA6C;
    'dispatch: loop {
        match pc {
            0x830AFA6C => {
    //   block [0x830AFA6C..0x830AFA74)
	// 830AFA6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830AFA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AFA78 size=88
    let mut pc: u32 = 0x830AFA78;
    'dispatch: loop {
        match pc {
            0x830AFA78 => {
    //   block [0x830AFA78..0x830AFAD0)
	// 830AFA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AFA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AFA80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830AFA84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AFA88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AFA8C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AFA90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFA94: 396B6898  addi r11, r11, 0x6898
	ctx.r[11].s64 = ctx.r[11].s64 + 26776;
	// 830AFA98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830AFA9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AFAA0: 4BF4A319  bl 0x82ff9db8
	ctx.lr = 0x830AFAA4;
	sub_82FF9DB8(ctx, base);
	// 830AFAA4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830AFAA8: 4182000C  beq 0x830afab4
	if ctx.cr[0].eq {
	pc = 0x830AFAB4; continue 'dispatch;
	}
	// 830AFAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFAB0: 4BF28831  bl 0x82fd82e0
	ctx.lr = 0x830AFAB4;
	sub_82FD82E0(ctx, base);
	// 830AFAB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFAB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830AFABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AFAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AFAC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830AFAC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AFACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AFAD0 size=696
    let mut pc: u32 = 0x830AFAD0;
    'dispatch: loop {
        match pc {
            0x830AFAD0 => {
    //   block [0x830AFAD0..0x830AFD88)
	// 830AFAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AFAD4: 480F8685  bl 0x831a8158
	ctx.lr = 0x830AFAD8;
	sub_831A8130(ctx, base);
	// 830AFAD8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AFADC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 830AFAE0: 89780004  lbz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AFAE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AFAE8: 40820298  bne 0x830afd80
	if !ctx.cr[0].eq {
	pc = 0x830AFD80; continue 'dispatch;
	}
	// 830AFAEC: 89780005  lbz r11, 5(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(5 as u32) ) } as u64;
	// 830AFAF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AFAF4: 40820014  bne 0x830afb08
	if !ctx.cr[0].eq {
	pc = 0x830AFB08; continue 'dispatch;
	}
	// 830AFAF8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFAFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AFB00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFB04: 4E800421  bctrl
	ctx.lr = 0x830AFB08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFB08: 48000FA9  bl 0x830b0ab0
	ctx.lr = 0x830AFB0C;
	sub_830B0AB0(ctx, base);
	// 830AFB0C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830AFB10: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 830AFB14: 3BE00025  li r31, 0x25
	ctx.r[31].s64 = 37;
	// 830AFB18: 839B0010  lwz r28, 0x10(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AFB1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFB20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AFB24: 4BFF3C4D  bl 0x830a3770
	ctx.lr = 0x830AFB28;
	sub_830A3770(ctx, base);
	// 830AFB28: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830AFB2C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830AFB30: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830AFB34: 4082FFE8  bne 0x830afb1c
	if !ctx.cr[0].eq {
	pc = 0x830AFB1C; continue 'dispatch;
	}
	// 830AFB38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830AFB3C: 57E3043E  clrlwi r3, r31, 0x10
	ctx.r[3].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 830AFB40: 4BFF7471  bl 0x830a6fb0
	ctx.lr = 0x830AFB44;
	sub_830A6FB0(ctx, base);
	// 830AFB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AFB48: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830AFB4C: 57CA13BA  rlwinm r10, r30, 2, 0xe, 0x1d
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x3FFFFFFFu64;
	// 830AFB50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AFB58: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AFB5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFB60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFB64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFB68: 4E800421  bctrl
	ctx.lr = 0x830AFB6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFB6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AFB70: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830AFB74: 4BFFFE8D  bl 0x830afa00
	ctx.lr = 0x830AFB78;
	sub_830AFA00(ctx, base);
	// 830AFB78: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 830AFB7C: 546A13BA  rlwinm r10, r3, 2, 0xe, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x3FFFFFFFu64;
	// 830AFB80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFB84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AFB88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830AFB8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFB90: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFB94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFB98: 4E800421  bctrl
	ctx.lr = 0x830AFB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFB9C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830AFBA0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 830AFBA4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AFBA8: 4198FF94  blt cr6, 0x830afb3c
	if ctx.cr[6].lt {
	pc = 0x830AFB3C; continue 'dispatch;
	}
	// 830AFBAC: 83410050  lwz r26, 0x50(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830AFBB0: 3F208217  lis r25, -0x7de9
	ctx.r[25].s64 = -2112421888;
	// 830AFBB4: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 830AFBB8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830AFBBC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFBC0: 80B9FD68  lwz r5, -0x298(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830AFBC4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFBC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFBCC: 4E800421  bctrl
	ctx.lr = 0x830AFBD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFBD0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AFBD4: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 830AFBD8: 3BAB5DA8  addi r29, r11, 0x5da8
	ctx.r[29].s64 = ctx.r[11].s64 + 23976;
	// 830AFBDC: 3BFD0018  addi r31, r29, 0x18
	ctx.r[31].s64 = ctx.r[29].s64 + 24;
	// 830AFBE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFBE4: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFBE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AFBEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFBF0: 48000AC9  bl 0x830b06b8
	ctx.lr = 0x830AFBF4;
	sub_830B06B8(ctx, base);
	// 830AFBF4: 397D0018  addi r11, r29, 0x18
	ctx.r[11].s64 = ctx.r[29].s64 + 24;
	// 830AFBF8: 3BFF004A  addi r31, r31, 0x4a
	ctx.r[31].s64 = ctx.r[31].s64 + 74;
	// 830AFBFC: 396B0AB2  addi r11, r11, 0xab2
	ctx.r[11].s64 = ctx.r[11].s64 + 2738;
	// 830AFC00: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 830AFC04: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830AFC08: 4198FFD8  blt cr6, 0x830afbe0
	if ctx.cr[6].lt {
	pc = 0x830AFBE0; continue 'dispatch;
	}
	// 830AFC0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFC10: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AFC14: 4BFF3B5D  bl 0x830a3770
	ctx.lr = 0x830AFC18;
	sub_830A3770(ctx, base);
	// 830AFC18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFC1C: 80B9FD68  lwz r5, -0x298(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-664 as u32) ) } as u64;
	// 830AFC20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFC24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFC28: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFC2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFC30: 4E800421  bctrl
	ctx.lr = 0x830AFC34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFC34: 389DFFC8  addi r4, r29, -0x38
	ctx.r[4].s64 = ctx.r[29].s64 + -56;
	// 830AFC38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFC3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFC40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFC44: 48000A75  bl 0x830b06b8
	ctx.lr = 0x830AFC48;
	sub_830B06B8(ctx, base);
	// 830AFC48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFC4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AFC50: 4BFF3B21  bl 0x830a3770
	ctx.lr = 0x830AFC54;
	sub_830A3770(ctx, base);
	// 830AFC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFC58: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830AFC5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFC60: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFC64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFC68: 4E800421  bctrl
	ctx.lr = 0x830AFC6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFC6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFC70: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830AFC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFC78: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFC7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFC80: 4E800421  bctrl
	ctx.lr = 0x830AFC84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFC84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFC88: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 830AFC8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFC90: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFC94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFC98: 4E800421  bctrl
	ctx.lr = 0x830AFC9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFC9C: 389DFFD0  addi r4, r29, -0x30
	ctx.r[4].s64 = ctx.r[29].s64 + -48;
	// 830AFCA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFCA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFCA8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFCAC: 48000A0D  bl 0x830b06b8
	ctx.lr = 0x830AFCB0;
	sub_830B06B8(ctx, base);
	// 830AFCB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFCB4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AFCB8: 4BFF3AB9  bl 0x830a3770
	ctx.lr = 0x830AFCBC;
	sub_830A3770(ctx, base);
	// 830AFCBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AFCC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830AFCC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFCC8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFCCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFCD0: 4E800421  bctrl
	ctx.lr = 0x830AFCD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFCD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFCD8: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 830AFCDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AFCE0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFCE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFCE8: 4E800421  bctrl
	ctx.lr = 0x830AFCEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFCEC: 389DFFE0  addi r4, r29, -0x20
	ctx.r[4].s64 = ctx.r[29].s64 + -32;
	// 830AFCF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFCF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830AFCF8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFCFC: 480009BD  bl 0x830b06b8
	ctx.lr = 0x830AFD00;
	sub_830B06B8(ctx, base);
	// 830AFD00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFD04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830AFD08: 4BFF3A69  bl 0x830a3770
	ctx.lr = 0x830AFD0C;
	sub_830A3770(ctx, base);
	// 830AFD0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFD10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830AFD14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFD18: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830AFD1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFD20: 4E800421  bctrl
	ctx.lr = 0x830AFD24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFD24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFD28: 38A0005F  li r5, 0x5f
	ctx.r[5].s64 = 95;
	// 830AFD2C: 3880005F  li r4, 0x5f
	ctx.r[4].s64 = 95;
	// 830AFD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFD34: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFD38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFD3C: 4E800421  bctrl
	ctx.lr = 0x830AFD40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFD40: 389DFFF0  addi r4, r29, -0x10
	ctx.r[4].s64 = ctx.r[29].s64 + -16;
	// 830AFD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFD48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFD4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFD50: 48000969  bl 0x830b06b8
	ctx.lr = 0x830AFD54;
	sub_830B06B8(ctx, base);
	// 830AFD54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830AFD58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830AFD5C: 80BC003C  lwz r5, 0x3c(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 830AFD60: 4BFF6C49  bl 0x830a69a8
	ctx.lr = 0x830AFD64;
	sub_830A69A8(ctx, base);
	// 830AFD64: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830AFD68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830AFD6C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830AFD70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFD74: 48000945  bl 0x830b06b8
	ctx.lr = 0x830AFD78;
	sub_830B06B8(ctx, base);
	// 830AFD78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830AFD7C: 99780004  stb r11, 4(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830AFD80: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 830AFD84: 480F8424  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AFD88 size=72
    let mut pc: u32 = 0x830AFD88;
    'dispatch: loop {
        match pc {
            0x830AFD88 => {
    //   block [0x830AFD88..0x830AFDD0)
	// 830AFD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AFD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830AFD90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830AFD94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AFD98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFD9C: 4BF4A01D  bl 0x82ff9db8
	ctx.lr = 0x830AFDA0;
	sub_82FF9DB8(ctx, base);
	// 830AFDA0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AFDA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830AFDA8: 396B6940  addi r11, r11, 0x6940
	ctx.r[11].s64 = ctx.r[11].s64 + 26944;
	// 830AFDAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFDB0: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830AFDB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830AFDB8: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 830AFDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830AFDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830AFDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830AFDC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830AFDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830AFDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830AFDD0 size=644
    let mut pc: u32 = 0x830AFDD0;
    'dispatch: loop {
        match pc {
            0x830AFDD0 => {
    //   block [0x830AFDD0..0x830B0054)
	// 830AFDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830AFDD4: 480F8391  bl 0x831a8164
	ctx.lr = 0x830AFDD8;
	sub_831A8130(ctx, base);
	// 830AFDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830AFDDC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830AFDE0: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AFDE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AFDE8: 40820264  bne 0x830b004c
	if !ctx.cr[0].eq {
	pc = 0x830B004C; continue 'dispatch;
	}
	// 830AFDEC: 897B0005  lbz r11, 5(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(5 as u32) ) } as u64;
	// 830AFDF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830AFDF4: 40820014  bne 0x830afe08
	if !ctx.cr[0].eq {
	pc = 0x830AFE08; continue 'dispatch;
	}
	// 830AFDF8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFDFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830AFE00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFE04: 4E800421  bctrl
	ctx.lr = 0x830AFE08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFE08: 48000CA9  bl 0x830b0ab0
	ctx.lr = 0x830AFE0C;
	sub_830B0AB0(ctx, base);
	// 830AFE0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830AFE10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFE14: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830AFE18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AFE1C: 4BFF3955  bl 0x830a3770
	ctx.lr = 0x830AFE20;
	sub_830A3770(ctx, base);
	// 830AFE20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFE24: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 830AFE28: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 830AFE2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFE30: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFE34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFE38: 4E800421  bctrl
	ctx.lr = 0x830AFE3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFE3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFE40: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 830AFE44: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 830AFE48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFE4C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFE50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFE54: 4E800421  bctrl
	ctx.lr = 0x830AFE58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFE58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFE5C: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 830AFE60: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 830AFE64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFE68: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFE70: 4E800421  bctrl
	ctx.lr = 0x830AFE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFE74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFE78: 38A0000D  li r5, 0xd
	ctx.r[5].s64 = 13;
	// 830AFE7C: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 830AFE80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFE84: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFE88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFE8C: 4E800421  bctrl
	ctx.lr = 0x830AFE90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFE90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFE94: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 830AFE98: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 830AFE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFEA0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFEA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFEA8: 4E800421  bctrl
	ctx.lr = 0x830AFEAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFEAC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830AFEB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFEB4: 3B8B68B0  addi r28, r11, 0x68b0
	ctx.r[28].s64 = ctx.r[11].s64 + 26800;
	// 830AFEB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFEBC: 389C0054  addi r4, r28, 0x54
	ctx.r[4].s64 = ctx.r[28].s64 + 84;
	// 830AFEC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AFEC4: 480007F5  bl 0x830b06b8
	ctx.lr = 0x830AFEC8;
	sub_830B06B8(ctx, base);
	// 830AFEC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFECC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AFED0: 4BFF38A1  bl 0x830a3770
	ctx.lr = 0x830AFED4;
	sub_830A3770(ctx, base);
	// 830AFED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFED8: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 830AFEDC: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 830AFEE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFEE4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFEE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFEEC: 4E800421  bctrl
	ctx.lr = 0x830AFEF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFEF0: 389C001C  addi r4, r28, 0x1c
	ctx.r[4].s64 = ctx.r[28].s64 + 28;
	// 830AFEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFEF8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFEFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AFF00: 480007B9  bl 0x830b06b8
	ctx.lr = 0x830AFF04;
	sub_830B06B8(ctx, base);
	// 830AFF04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFF08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AFF0C: 4BFF3865  bl 0x830a3770
	ctx.lr = 0x830AFF10;
	sub_830A3770(ctx, base);
	// 830AFF10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFF14: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 830AFF18: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 830AFF1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFF20: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFF28: 4E800421  bctrl
	ctx.lr = 0x830AFF2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFF2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFF30: 38A0005A  li r5, 0x5a
	ctx.r[5].s64 = 90;
	// 830AFF34: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 830AFF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFF3C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFF40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFF44: 4E800421  bctrl
	ctx.lr = 0x830AFF48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFF48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFF4C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFF50: 38A0005F  li r5, 0x5f
	ctx.r[5].s64 = 95;
	// 830AFF54: 3880005F  li r4, 0x5f
	ctx.r[4].s64 = 95;
	// 830AFF58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFF5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFF60: 4E800421  bctrl
	ctx.lr = 0x830AFF64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFF64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFF68: 38A0007A  li r5, 0x7a
	ctx.r[5].s64 = 122;
	// 830AFF6C: 38800061  li r4, 0x61
	ctx.r[4].s64 = 97;
	// 830AFF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFF74: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFF78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFF7C: 4E800421  bctrl
	ctx.lr = 0x830AFF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFF80: 389C0038  addi r4, r28, 0x38
	ctx.r[4].s64 = ctx.r[28].s64 + 56;
	// 830AFF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFF88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830AFF8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830AFF90: 48000729  bl 0x830b06b8
	ctx.lr = 0x830AFF94;
	sub_830B06B8(ctx, base);
	// 830AFF94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830AFF98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830AFF9C: 4BFF37D5  bl 0x830a3770
	ctx.lr = 0x830AFFA0;
	sub_830A3770(ctx, base);
	// 830AFFA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830AFFA4: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 830AFFA8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 830AFFAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFFB0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFFB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFFB8: 4E800421  bctrl
	ctx.lr = 0x830AFFBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFFBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFFC0: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 830AFFC4: 38800041  li r4, 0x41
	ctx.r[4].s64 = 65;
	// 830AFFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFFCC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFFD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFFD4: 4E800421  bctrl
	ctx.lr = 0x830AFFD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFFD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830AFFDC: 38A00061  li r5, 0x61
	ctx.r[5].s64 = 97;
	// 830AFFE0: 38800061  li r4, 0x61
	ctx.r[4].s64 = 97;
	// 830AFFE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830AFFE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830AFFEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830AFFF0: 4E800421  bctrl
	ctx.lr = 0x830AFFF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830AFFF4: 389C0070  addi r4, r28, 0x70
	ctx.r[4].s64 = ctx.r[28].s64 + 112;
	// 830AFFF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830AFFFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B0000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0004: 480006B5  bl 0x830b06b8
	ctx.lr = 0x830B0008;
	sub_830B06B8(ctx, base);
	// 830B0008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B000C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B0010: 4BFF3761  bl 0x830a3770
	ctx.lr = 0x830B0014;
	sub_830A3770(ctx, base);
	// 830B0014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B0018: 38A0007F  li r5, 0x7f
	ctx.r[5].s64 = 127;
	// 830B001C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B0020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0024: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830B0028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B002C: 4E800421  bctrl
	ctx.lr = 0x830B0030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0030: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B0034: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B0038: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B003C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0040: 48000679  bl 0x830b06b8
	ctx.lr = 0x830B0044;
	sub_830B06B8(ctx, base);
	// 830B0044: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B0048: 997B0004  stb r11, 4(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830B004C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B0050: 480F8164  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0058 size=136
    let mut pc: u32 = 0x830B0058;
    'dispatch: loop {
        match pc {
            0x830B0058 => {
    //   block [0x830B0058..0x830B00E0)
	// 830B0058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B005C: 480F8111  bl 0x831a816c
	ctx.lr = 0x830B0060;
	sub_831A8130(ctx, base);
	// 830B0060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0064: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B0068: 897D0005  lbz r11, 5(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(5 as u32) ) } as u64;
	// 830B006C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0070: 40820068  bne 0x830b00d8
	if !ctx.cr[0].eq {
	pc = 0x830B00D8; continue 'dispatch;
	}
	// 830B0074: 48000A3D  bl 0x830b0ab0
	ctx.lr = 0x830B0078;
	sub_830B0AB0(ctx, base);
	// 830B0078: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B007C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B0080: 3BEB68A4  addi r31, r11, 0x68a4
	ctx.r[31].s64 = ctx.r[11].s64 + 26788;
	// 830B0084: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 830B0088: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B008C: 480004FD  bl 0x830b0588
	ctx.lr = 0x830B0090;
	sub_830B0588(ctx, base);
	// 830B0090: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 830B0094: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B0098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B009C: 480004ED  bl 0x830b0588
	ctx.lr = 0x830B00A0;
	sub_830B0588(ctx, base);
	// 830B00A0: 389F0044  addi r4, r31, 0x44
	ctx.r[4].s64 = ctx.r[31].s64 + 68;
	// 830B00A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B00A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B00AC: 480004DD  bl 0x830b0588
	ctx.lr = 0x830B00B0;
	sub_830B0588(ctx, base);
	// 830B00B0: 389F007C  addi r4, r31, 0x7c
	ctx.r[4].s64 = ctx.r[31].s64 + 124;
	// 830B00B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B00B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B00BC: 480004CD  bl 0x830b0588
	ctx.lr = 0x830B00C0;
	sub_830B0588(ctx, base);
	// 830B00C0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 830B00C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830B00C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B00CC: 480004BD  bl 0x830b0588
	ctx.lr = 0x830B00D0;
	sub_830B0588(ctx, base);
	// 830B00D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B00D4: 997D0005  stb r11, 5(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 830B00D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B00DC: 480F80E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B00E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B00E0 size=88
    let mut pc: u32 = 0x830B00E0;
    'dispatch: loop {
        match pc {
            0x830B00E0 => {
    //   block [0x830B00E0..0x830B0138)
	// 830B00E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B00E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B00E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B00EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B00F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B00F4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B00F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B00FC: 396B6940  addi r11, r11, 0x6940
	ctx.r[11].s64 = ctx.r[11].s64 + 26944;
	// 830B0100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B0104: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B0108: 4BF49CB1  bl 0x82ff9db8
	ctx.lr = 0x830B010C;
	sub_82FF9DB8(ctx, base);
	// 830B010C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B0110: 4182000C  beq 0x830b011c
	if ctx.cr[0].eq {
	pc = 0x830B011C; continue 'dispatch;
	}
	// 830B0114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0118: 4BF281C9  bl 0x82fd82e0
	ctx.lr = 0x830B011C;
	sub_82FD82E0(ctx, base);
	// 830B011C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0120: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B0124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B012C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B0130: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0138 size=76
    let mut pc: u32 = 0x830B0138;
    'dispatch: loop {
        match pc {
            0x830B0138 => {
    //   block [0x830B0138..0x830B0184)
	// 830B0138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B014C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0150: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 830B0154: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830B0158: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B015C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B0160: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830B0164: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830B0168: 4BF455E1  bl 0x82ff5748
	ctx.lr = 0x830B016C;
	sub_82FF5748(ctx, base);
	// 830B016C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B017C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0188 size=12
    let mut pc: u32 = 0x830B0188;
    'dispatch: loop {
        match pc {
            0x830B0188 => {
    //   block [0x830B0188..0x830B0194)
	// 830B0188: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B018C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B0190: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0194 size=20
    let mut pc: u32 = 0x830B0194;
    'dispatch: loop {
        match pc {
            0x830B0194 => {
    //   block [0x830B0194..0x830B01A8)
	// 830B0194: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830B0198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B019C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B01A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B01A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B01A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B01A8 size=4
    let mut pc: u32 = 0x830B01A8;
    'dispatch: loop {
        match pc {
            0x830B01A8 => {
    //   block [0x830B01A8..0x830B01AC)
	// 830B01A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B01B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B01B0 size=84
    let mut pc: u32 = 0x830B01B0;
    'dispatch: loop {
        match pc {
            0x830B01B0 => {
    //   block [0x830B01B0..0x830B0204)
	// 830B01B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B01B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B01B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B01BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B01C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B01C4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830B01C8: 83FEBC7C  lwz r31, -0x4384(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17284 as u32) ) } as u64;
	// 830B01CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830B01D0: 419A0014  beq cr6, 0x830b01e4
	if ctx.cr[6].eq {
	pc = 0x830B01E4; continue 'dispatch;
	}
	// 830B01D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B01D8: 4BF455B1  bl 0x82ff5788
	ctx.lr = 0x830B01DC;
	sub_82FF5788(ctx, base);
	// 830B01DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B01E0: 4BF28101  bl 0x82fd82e0
	ctx.lr = 0x830B01E4;
	sub_82FD82E0(ctx, base);
	// 830B01E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B01E8: 917EBC7C  stw r11, -0x4384(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17284 as u32), ctx.r[11].u32 ) };
	// 830B01EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B01F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B01F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B01F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B01FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0208 size=8
    let mut pc: u32 = 0x830B0208;
    'dispatch: loop {
        match pc {
            0x830B0208 => {
    //   block [0x830B0208..0x830B0210)
	// 830B0208: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B020C: 82186960  lwz r16, 0x6960(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(26976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0210 size=164
    let mut pc: u32 = 0x830B0210;
    'dispatch: loop {
        match pc {
            0x830B0210 => {
    //   block [0x830B0210..0x830B02B4)
	// 830B0210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B021C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0220: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830B0224: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0228: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830B022C: 807EBC7C  lwz r3, -0x4384(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17284 as u32) ) } as u64;
	// 830B0230: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B0234: 409A0068  bne cr6, 0x830b029c
	if !ctx.cr[6].eq {
	pc = 0x830B029C; continue 'dispatch;
	}
	// 830B0238: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830B023C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0240: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830B0244: 4BF45595  bl 0x82ff57d8
	ctx.lr = 0x830B0248;
	sub_82FF57D8(ctx, base);
	// 830B0248: 817EBC7C  lwz r11, -0x4384(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17284 as u32) ) } as u64;
	// 830B024C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B0250: 409A0040  bne cr6, 0x830b0290
	if !ctx.cr[6].eq {
	pc = 0x830B0290; continue 'dispatch;
	}
	// 830B0254: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830B0258: 4BF27FF1  bl 0x82fd8248
	ctx.lr = 0x830B025C;
	sub_82FD8248(ctx, base);
	// 830B025C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B0260: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0264: 41820010  beq 0x830b0274
	if ctx.cr[0].eq {
	pc = 0x830B0274; continue 'dispatch;
	}
	// 830B0268: 4BF454E1  bl 0x82ff5748
	ctx.lr = 0x830B026C;
	sub_82FF5748(ctx, base);
	// 830B026C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830B0270: 48000008  b 0x830b0278
	pc = 0x830B0278; continue 'dispatch;
	// 830B0274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B0278: 3D60830B  lis r11, -0x7cf5
	ctx.r[11].s64 = -2096431104;
	// 830B027C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830B0280: 913EBC7C  stw r9, -0x4384(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17284 as u32), ctx.r[9].u32 ) };
	// 830B0284: 388B01B0  addi r4, r11, 0x1b0
	ctx.r[4].s64 = ctx.r[11].s64 + 432;
	// 830B0288: 386ABC90  addi r3, r10, -0x4370
	ctx.r[3].s64 = ctx.r[10].s64 + -17264;
	// 830B028C: 4BF478AD  bl 0x82ff7b38
	ctx.lr = 0x830B0290;
	sub_82FF7B38(ctx, base);
	// 830B0290: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0294: 4BF4557D  bl 0x82ff5810
	ctx.lr = 0x830B0298;
	sub_82FF5810(ctx, base);
	// 830B0298: 807EBC7C  lwz r3, -0x4384(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17284 as u32) ) } as u64;
	// 830B029C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830B02A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B02A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B02A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B02AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B02B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B02B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B02B4 size=40
    let mut pc: u32 = 0x830B02B4;
    'dispatch: loop {
        match pc {
            0x830B02B4 => {
    //   block [0x830B02B4..0x830B02DC)
	// 830B02B4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B02B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B02BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B02C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B02C4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B02C8: 4BF45549  bl 0x82ff5810
	ctx.lr = 0x830B02CC;
	sub_82FF5810(ctx, base);
	// 830B02CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B02D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B02D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B02D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B02DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B02DC size=40
    let mut pc: u32 = 0x830B02DC;
    'dispatch: loop {
        match pc {
            0x830B02DC => {
    //   block [0x830B02DC..0x830B0304)
	// 830B02DC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B02E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B02E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B02E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B02EC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B02F0: 4BF27FF1  bl 0x82fd82e0
	ctx.lr = 0x830B02F4;
	sub_82FD82E0(ctx, base);
	// 830B02F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B02F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B02FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0308 size=208
    let mut pc: u32 = 0x830B0308;
    'dispatch: loop {
        match pc {
            0x830B0308 => {
    //   block [0x830B0308..0x830B03D8)
	// 830B0308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B030C: 480F7E5D  bl 0x831a8168
	ctx.lr = 0x830B0310;
	sub_831A8130(ctx, base);
	// 830B0310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B0318: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830B031C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B0320: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B0324: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B0328: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 830B032C: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0330: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830B0334: 41980008  blt cr6, 0x830b033c
	if ctx.cr[6].lt {
	pc = 0x830B033C; continue 'dispatch;
	}
	// 830B0338: 4BFB86B1  bl 0x830689e8
	ctx.lr = 0x830B033C;
	sub_830689E8(ctx, base);
	// 830B033C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830B0340: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B0344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0348: 4BF4A119  bl 0x82ffa460
	ctx.lr = 0x830B034C;
	sub_82FFA460(ctx, base);
	// 830B034C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830B0350: 4182002C  beq 0x830b037c
	if ctx.cr[0].eq {
	pc = 0x830B037C; continue 'dispatch;
	}
	// 830B0354: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0358: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B035C: 41820014  beq 0x830b0370
	if ctx.cr[0].eq {
	pc = 0x830B0370; continue 'dispatch;
	}
	// 830B0360: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0364: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0368: 41820008  beq 0x830b0370
	if ctx.cr[0].eq {
	pc = 0x830B0370; continue 'dispatch;
	}
	// 830B036C: 4BF27F75  bl 0x82fd82e0
	ctx.lr = 0x830B0370;
	sub_82FD82E0(ctx, base);
	// 830B0370: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B0374: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830B0378: 48000058  b 0x830b03d0
	pc = 0x830B03D0; continue 'dispatch;
	// 830B037C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830B0380: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0384: 4BF27F15  bl 0x82fd8298
	ctx.lr = 0x830B0388;
	sub_82FD8298(ctx, base);
	// 830B0388: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B038C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0390: 41820024  beq 0x830b03b4
	if ctx.cr[0].eq {
	pc = 0x830B03B4; continue 'dispatch;
	}
	// 830B0394: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B0398: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830B039C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830B03A0: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830B03A4: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830B03A8: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830B03AC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B03B0: 48000008  b 0x830b03b8
	pc = 0x830B03B8; continue 'dispatch;
	// 830B03B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B03B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B03BC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830B03C0: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 830B03C4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B03C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B03CC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830B03D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830B03D4: 480F7DE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B03D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B03D8 size=8
    let mut pc: u32 = 0x830B03D8;
    'dispatch: loop {
        match pc {
            0x830B03D8 => {
    //   block [0x830B03D8..0x830B03E0)
	// 830B03D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B03DC: 821869B0  lwz r16, 0x69b0(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(27056 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B03E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B03E0 size=360
    let mut pc: u32 = 0x830B03E0;
    'dispatch: loop {
        match pc {
            0x830B03E0 => {
    //   block [0x830B03E0..0x830B0548)
	// 830B03E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B03E4: 480F7D81  bl 0x831a8164
	ctx.lr = 0x830B03E8;
	sub_831A8130(ctx, base);
	// 830B03E8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830B03EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B03F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B03F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B03F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B03FC: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0400: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0404: 418200F0  beq 0x830b04f4
	if ctx.cr[0].eq {
	pc = 0x830B04F4; continue 'dispatch;
	}
	// 830B0408: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B040C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B0410: 419A00E4  beq cr6, 0x830b04f4
	if ctx.cr[6].eq {
	pc = 0x830B04F4; continue 'dispatch;
	}
	// 830B0414: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B0418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B041C: 419A00D8  beq cr6, 0x830b04f4
	if ctx.cr[6].eq {
	pc = 0x830B04F4; continue 'dispatch;
	}
	// 830B0420: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 830B0424: 4BF36335  bl 0x82fe6758
	ctx.lr = 0x830B0428;
	sub_82FE6758(ctx, base);
	// 830B0428: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830B042C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830B0430: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830B0434: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0438: 418200BC  beq 0x830b04f4
	if ctx.cr[0].eq {
	pc = 0x830B04F4; continue 'dispatch;
	}
	// 830B043C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 830B0440: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0444: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B0448: 4BF4A019  bl 0x82ffa460
	ctx.lr = 0x830B044C;
	sub_82FFA460(ctx, base);
	// 830B044C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0450: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830B0454: 41820008  beq 0x830b045c
	if ctx.cr[0].eq {
	pc = 0x830B045C; continue 'dispatch;
	}
	// 830B0458: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B045C: 577B063F  clrlwi. r27, r27, 0x18
	ctx.r[27].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 830B0460: 4182000C  beq 0x830b046c
	if ctx.cr[0].eq {
	pc = 0x830B046C; continue 'dispatch;
	}
	// 830B0464: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B0468: 48000008  b 0x830b0470
	pc = 0x830B0470; continue 'dispatch;
	// 830B046C: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0470: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B0474: 409A00CC  bne cr6, 0x830b0540
	if !ctx.cr[6].eq {
	pc = 0x830B0540; continue 'dispatch;
	}
	// 830B0478: 389C0014  addi r4, r28, 0x14
	ctx.r[4].s64 = ctx.r[28].s64 + 20;
	// 830B047C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0480: 4BF45359  bl 0x82ff57d8
	ctx.lr = 0x830B0484;
	sub_82FF57D8(ctx, base);
	// 830B0484: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830B0488: 419A000C  beq cr6, 0x830b0494
	if ctx.cr[6].eq {
	pc = 0x830B0494; continue 'dispatch;
	}
	// 830B048C: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B0490: 48000008  b 0x830b0498
	pc = 0x830B0498; continue 'dispatch;
	// 830B0494: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0498: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B049C: 409A009C  bne cr6, 0x830b0538
	if !ctx.cr[6].eq {
	pc = 0x830B0538; continue 'dispatch;
	}
	// 830B04A0: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B04A4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B04A8: 4082006C  bne 0x830b0514
	if !ctx.cr[0].eq {
	pc = 0x830B0514; continue 'dispatch;
	}
	// 830B04AC: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B04B0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B04B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B04B8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 830B04BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B04C0: 4E800421  bctrl
	ctx.lr = 0x830B04C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B04C4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B04C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B04CC: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 830B04D0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B04D4: 4BF49F8D  bl 0x82ffa460
	ctx.lr = 0x830B04D8;
	sub_82FFA460(ctx, base);
	// 830B04D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B04DC: 41820010  beq 0x830b04ec
	if ctx.cr[0].eq {
	pc = 0x830B04EC; continue 'dispatch;
	}
	// 830B04E0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B04E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B04E8: 40820018  bne 0x830b0500
	if !ctx.cr[0].eq {
	pc = 0x830B0500; continue 'dispatch;
	}
	// 830B04EC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B04F0: 4BF45321  bl 0x82ff5810
	ctx.lr = 0x830B04F4;
	sub_82FF5810(ctx, base);
	// 830B04F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B04F8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830B04FC: 480F7CB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830B0500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0504: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B0508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B050C: 4E800421  bctrl
	ctx.lr = 0x830B0510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0510: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0514: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830B0518: 419A0020  beq cr6, 0x830b0538
	if ctx.cr[6].eq {
	pc = 0x830B0538; continue 'dispatch;
	}
	// 830B051C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0524: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B0528: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B052C: 4BFF647D  bl 0x830a69a8
	ctx.lr = 0x830B0530;
	sub_830A69A8(ctx, base);
	// 830B0530: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B0534: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830B0538: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B053C: 4BF452D5  bl 0x82ff5810
	ctx.lr = 0x830B0540;
	sub_82FF5810(ctx, base);
	// 830B0540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0544: 4BFFFFB4  b 0x830b04f8
	pc = 0x830B04F8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0548 size=40
    let mut pc: u32 = 0x830B0548;
    'dispatch: loop {
        match pc {
            0x830B0548 => {
    //   block [0x830B0548..0x830B0570)
	// 830B0548: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830B054C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0550: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0558: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B055C: 4BF452B5  bl 0x82ff5810
	ctx.lr = 0x830B0560;
	sub_82FF5810(ctx, base);
	// 830B0560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B056C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0570 size=12
    let mut pc: u32 = 0x830B0570;
    'dispatch: loop {
        match pc {
            0x830B0570 => {
    //   block [0x830B0570..0x830B057C)
	// 830B0570: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B0574: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0578: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B057C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B057C size=8
    let mut pc: u32 = 0x830B057C;
    'dispatch: loop {
        match pc {
            0x830B057C => {
    //   block [0x830B057C..0x830B0584)
	// 830B057C: 4BF7C8CC  b 0x8302ce48
	sub_8302CE48(ctx, base);
	return;
	// 830B0580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0588 size=300
    let mut pc: u32 = 0x830B0588;
    'dispatch: loop {
        match pc {
            0x830B0588 => {
    //   block [0x830B0588..0x830B06B4)
	// 830B0588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B058C: 480F7BDD  bl 0x831a8168
	ctx.lr = 0x830B0590;
	sub_831A8130(ctx, base);
	// 830B0590: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B0598: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830B059C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B05A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B05A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B05A8: 419A0104  beq cr6, 0x830b06ac
	if ctx.cr[6].eq {
	pc = 0x830B06AC; continue 'dispatch;
	}
	// 830B05AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B05B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B05B4: 419A00F8  beq cr6, 0x830b06ac
	if ctx.cr[6].eq {
	pc = 0x830B06AC; continue 'dispatch;
	}
	// 830B05B8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B05BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B05C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B05C4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830B05C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B05CC: 4E800421  bctrl
	ctx.lr = 0x830B05D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B05D0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830B05D4: 40820048  bne 0x830b061c
	if !ctx.cr[0].eq {
	pc = 0x830B061C; continue 'dispatch;
	}
	// 830B05D8: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B05DC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B05E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B05E4: 388B6A08  addi r4, r11, 0x6a08
	ctx.r[4].s64 = ctx.r[11].s64 + 27144;
	// 830B05E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B05EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B05F0: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B05F4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 830B05F8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830B05FC: 38A000FB  li r5, 0xfb
	ctx.r[5].s64 = 251;
	// 830B0600: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B0604: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830B0608: 4BF67179  bl 0x83017780
	ctx.lr = 0x830B060C;
	sub_83017780(ctx, base);
	// 830B060C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830B0610: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B0614: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830B0618: 48100611  bl 0x831b0c28
	ctx.lr = 0x830B061C;
	sub_831B0C28(ctx, base);
	// 830B061C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B0620: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0624: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B0628: 4BF36131  bl 0x82fe6758
	ctx.lr = 0x830B062C;
	sub_82FE6758(ctx, base);
	// 830B062C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830B0630: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830B0634: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830B0638: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B063C: 41820038  beq 0x830b0674
	if ctx.cr[0].eq {
	pc = 0x830B0674; continue 'dispatch;
	}
	// 830B0640: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B0644: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0648: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B064C: 4BF49E15  bl 0x82ffa460
	ctx.lr = 0x830B0650;
	sub_82FFA460(ctx, base);
	// 830B0650: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0654: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0658: 41820008  beq 0x830b0660
	if ctx.cr[0].eq {
	pc = 0x830B0660; continue 'dispatch;
	}
	// 830B065C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0660: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0664: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830B0668: 419A0044  beq cr6, 0x830b06ac
	if ctx.cr[6].eq {
	pc = 0x830B06AC; continue 'dispatch;
	}
	// 830B066C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B0670: 4800003C  b 0x830b06ac
	pc = 0x830B06AC; continue 'dispatch;
	// 830B0674: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830B0678: 4BF27BD1  bl 0x82fd8248
	ctx.lr = 0x830B067C;
	sub_82FD8248(ctx, base);
	// 830B067C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0680: 4182001C  beq 0x830b069c
	if ctx.cr[0].eq {
	pc = 0x830B069C; continue 'dispatch;
	}
	// 830B0684: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0688: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 830B068C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830B0690: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B0694: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B0698: 48000008  b 0x830b06a0
	pc = 0x830B06A0; continue 'dispatch;
	// 830B069C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B06A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B06A4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B06A8: 4BFFFC61  bl 0x830b0308
	ctx.lr = 0x830B06AC;
	sub_830B0308(ctx, base);
	// 830B06AC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830B06B0: 480F7B08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B06B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B06B8 size=196
    let mut pc: u32 = 0x830B06B8;
    'dispatch: loop {
        match pc {
            0x830B06B8 => {
    //   block [0x830B06B8..0x830B077C)
	// 830B06B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B06BC: 480F7AAD  bl 0x831a8168
	ctx.lr = 0x830B06C0;
	sub_831A8130(ctx, base);
	// 830B06C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B06C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B06C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B06CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B06D0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830B06D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B06D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B06DC: 4182004C  beq 0x830b0728
	if ctx.cr[0].eq {
	pc = 0x830B0728; continue 'dispatch;
	}
	// 830B06E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B06E4: 4BF36075  bl 0x82fe6758
	ctx.lr = 0x830B06E8;
	sub_82FE6758(ctx, base);
	// 830B06E8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830B06EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830B06F0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830B06F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B06F8: 41820040  beq 0x830b0738
	if ctx.cr[0].eq {
	pc = 0x830B0738; continue 'dispatch;
	}
	// 830B06FC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 830B0700: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0704: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B0708: 4BF49D59  bl 0x82ffa460
	ctx.lr = 0x830B070C;
	sub_82FFA460(ctx, base);
	// 830B070C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0714: 41820008  beq 0x830b071c
	if ctx.cr[0].eq {
	pc = 0x830B071C; continue 'dispatch;
	}
	// 830B0718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B071C: 578A063F  clrlwi. r10, r28, 0x18
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830B0720: 41820010  beq 0x830b0730
	if ctx.cr[0].eq {
	pc = 0x830B0730; continue 'dispatch;
	}
	// 830B0724: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830B0728: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830B072C: 480F7A8C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830B0730: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830B0734: 4BFFFFF4  b 0x830b0728
	pc = 0x830B0728; continue 'dispatch;
	// 830B0738: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B073C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B0740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B0744: 388B6A08  addi r4, r11, 0x6a08
	ctx.r[4].s64 = ctx.r[11].s64 + 27144;
	// 830B0748: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B074C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830B0750: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0754: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830B0758: 38C00121  li r6, 0x121
	ctx.r[6].s64 = 289;
	// 830B075C: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 830B0760: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B0764: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830B0768: 4BF67019  bl 0x83017780
	ctx.lr = 0x830B076C;
	sub_83017780(ctx, base);
	// 830B076C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830B0770: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830B0774: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 830B0778: 481004B1  bl 0x831b0c28
	ctx.lr = 0x830B077C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0780 size=8
    let mut pc: u32 = 0x830B0780;
    'dispatch: loop {
        match pc {
            0x830B0780 => {
    //   block [0x830B0780..0x830B0788)
	// 830B0780: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B0784: 82186A68  lwz r16, 0x6a68(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(27240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0788 size=268
    let mut pc: u32 = 0x830B0788;
    'dispatch: loop {
        match pc {
            0x830B0788 => {
    //   block [0x830B0788..0x830B0894)
	// 830B0788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B078C: 480F79E1  bl 0x831a816c
	ctx.lr = 0x830B0790;
	sub_831A8130(ctx, base);
	// 830B0790: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830B0794: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0798: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B079C: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B07A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B07A4: 408200E8  bne 0x830b088c
	if !ctx.cr[0].eq {
	pc = 0x830B088C; continue 'dispatch;
	}
	// 830B07A8: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 830B07AC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B07B0: 4BF45029  bl 0x82ff57d8
	ctx.lr = 0x830B07B4;
	sub_82FF57D8(ctx, base);
	// 830B07B4: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B07B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B07BC: 408200C8  bne 0x830b0884
	if !ctx.cr[0].eq {
	pc = 0x830B0884; continue 'dispatch;
	}
	// 830B07C0: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830B07C4: 4BF27A85  bl 0x82fd8248
	ctx.lr = 0x830B07C8;
	sub_82FD8248(ctx, base);
	// 830B07C8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B07CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B07D0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830B07D4: 41820014  beq 0x830b07e8
	if ctx.cr[0].eq {
	pc = 0x830B07E8; continue 'dispatch;
	}
	// 830B07D8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B07DC: 4BFF29E5  bl 0x830a31c0
	ctx.lr = 0x830B07E0;
	sub_830A31C0(ctx, base);
	// 830B07E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B07E4: 48000008  b 0x830b07ec
	pc = 0x830B07EC; continue 'dispatch;
	// 830B07E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B07EC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830B07F0: 917D0010  stw r11, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830B07F4: 4BF27A55  bl 0x82fd8248
	ctx.lr = 0x830B07F8;
	sub_82FD8248(ctx, base);
	// 830B07F8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B07FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0800: 41820018  beq 0x830b0818
	if ctx.cr[0].eq {
	pc = 0x830B0818; continue 'dispatch;
	}
	// 830B0804: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 830B0808: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B080C: 4BFB9DFD  bl 0x8306a608
	ctx.lr = 0x830B0810;
	sub_8306A608(ctx, base);
	// 830B0810: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B0814: 48000008  b 0x830b081c
	pc = 0x830B081C; continue 'dispatch;
	// 830B0818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B081C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830B0820: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830B0824: 4BF27A25  bl 0x82fd8248
	ctx.lr = 0x830B0828;
	sub_82FD8248(ctx, base);
	// 830B0828: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B082C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0830: 41820018  beq 0x830b0848
	if ctx.cr[0].eq {
	pc = 0x830B0848; continue 'dispatch;
	}
	// 830B0834: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 830B0838: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B083C: 4BFB9DCD  bl 0x8306a608
	ctx.lr = 0x830B0840;
	sub_8306A608(ctx, base);
	// 830B0840: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B0844: 48000008  b 0x830b084c
	pc = 0x830B084C; continue 'dispatch;
	// 830B0848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B084C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830B0850: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830B0854: 4BF279F5  bl 0x82fd8248
	ctx.lr = 0x830B0858;
	sub_82FD8248(ctx, base);
	// 830B0858: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B085C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0860: 41820014  beq 0x830b0874
	if ctx.cr[0].eq {
	pc = 0x830B0874; continue 'dispatch;
	}
	// 830B0864: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 830B0868: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B086C: 4BF4D27D  bl 0x82ffdae8
	ctx.lr = 0x830B0870;
	sub_82FFDAE8(ctx, base);
	// 830B0870: 48000008  b 0x830b0878
	pc = 0x830B0878; continue 'dispatch;
	// 830B0874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B0878: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B087C: 907D000C  stw r3, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830B0880: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830B0884: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0888: 4BF44F89  bl 0x82ff5810
	ctx.lr = 0x830B088C;
	sub_82FF5810(ctx, base);
	// 830B088C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830B0890: 480F792C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0894(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0894 size=40
    let mut pc: u32 = 0x830B0894;
    'dispatch: loop {
        match pc {
            0x830B0894 => {
    //   block [0x830B0894..0x830B08BC)
	// 830B0894: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B0898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B089C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B08A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B08A4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B08A8: 4BF44F69  bl 0x82ff5810
	ctx.lr = 0x830B08AC;
	sub_82FF5810(ctx, base);
	// 830B08AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B08B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B08B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B08B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B08BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B08BC size=40
    let mut pc: u32 = 0x830B08BC;
    'dispatch: loop {
        match pc {
            0x830B08BC => {
    //   block [0x830B08BC..0x830B08E4)
	// 830B08BC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B08C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B08C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B08C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B08CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B08D0: 4BF27A11  bl 0x82fd82e0
	ctx.lr = 0x830B08D4;
	sub_82FD82E0(ctx, base);
	// 830B08D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B08D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B08DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B08E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B08E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B08E4 size=40
    let mut pc: u32 = 0x830B08E4;
    'dispatch: loop {
        match pc {
            0x830B08E4 => {
    //   block [0x830B08E4..0x830B090C)
	// 830B08E4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B08E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B08EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B08F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B08F4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B08F8: 4BF279E9  bl 0x82fd82e0
	ctx.lr = 0x830B08FC;
	sub_82FD82E0(ctx, base);
	// 830B08FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B090C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B090C size=40
    let mut pc: u32 = 0x830B090C;
    'dispatch: loop {
        match pc {
            0x830B090C => {
    //   block [0x830B090C..0x830B0934)
	// 830B090C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B0910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B091C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B0920: 4BF279C1  bl 0x82fd82e0
	ctx.lr = 0x830B0924;
	sub_82FD82E0(ctx, base);
	// 830B0924: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B092C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0934 size=40
    let mut pc: u32 = 0x830B0934;
    'dispatch: loop {
        match pc {
            0x830B0934 => {
    //   block [0x830B0934..0x830B095C)
	// 830B0934: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B0938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B093C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0944: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B0948: 4BF27999  bl 0x82fd82e0
	ctx.lr = 0x830B094C;
	sub_82FD82E0(ctx, base);
	// 830B094C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0960 size=8
    let mut pc: u32 = 0x830B0960;
    'dispatch: loop {
        match pc {
            0x830B0960 => {
    //   block [0x830B0960..0x830B0968)
	// 830B0960: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B0964: 82186AE8  lwz r16, 0x6ae8(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(27368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0968 size=176
    let mut pc: u32 = 0x830B0968;
    'dispatch: loop {
        match pc {
            0x830B0968 => {
    //   block [0x830B0968..0x830B0A18)
	// 830B0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B096C: 480F77FD  bl 0x831a8168
	ctx.lr = 0x830B0970;
	sub_831A8130(ctx, base);
	// 830B0970: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830B0974: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0978: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B097C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830B0980: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0984: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0988: 41820014  beq 0x830b099c
	if ctx.cr[0].eq {
	pc = 0x830B099C; continue 'dispatch;
	}
	// 830B098C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B0990: 4BFD4A11  bl 0x830853a0
	ctx.lr = 0x830B0994;
	sub_830853A0(ctx, base);
	// 830B0994: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B0998: 4BF27949  bl 0x82fd82e0
	ctx.lr = 0x830B099C;
	sub_82FD82E0(ctx, base);
	// 830B099C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830B09A0: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B09A4: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B09A8: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830B09AC: 41820014  beq 0x830b09c0
	if ctx.cr[0].eq {
	pc = 0x830B09C0; continue 'dispatch;
	}
	// 830B09B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B09B4: 4BF362B5  bl 0x82fe6c68
	ctx.lr = 0x830B09B8;
	sub_82FE6C68(ctx, base);
	// 830B09B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B09BC: 4BF27925  bl 0x82fd82e0
	ctx.lr = 0x830B09C0;
	sub_82FD82E0(ctx, base);
	// 830B09C0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B09C4: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830B09C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B09CC: 41820018  beq 0x830b09e4
	if ctx.cr[0].eq {
	pc = 0x830B09E4; continue 'dispatch;
	}
	// 830B09D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B09D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B09D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B09DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B09E0: 4E800421  bctrl
	ctx.lr = 0x830B09E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B09E4: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B09E8: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830B09EC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B09F0: 41820014  beq 0x830b0a04
	if ctx.cr[0].eq {
	pc = 0x830B0A04; continue 'dispatch;
	}
	// 830B09F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B09F8: 4BFF2341  bl 0x830a2d38
	ctx.lr = 0x830B09FC;
	sub_830A2D38(ctx, base);
	// 830B09FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B0A00: 4BF278E1  bl 0x82fd82e0
	ctx.lr = 0x830B0A04;
	sub_82FD82E0(ctx, base);
	// 830B0A04: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 830B0A08: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 830B0A0C: 4BF44D7D  bl 0x82ff5788
	ctx.lr = 0x830B0A10;
	sub_82FF5788(ctx, base);
	// 830B0A10: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830B0A14: 480F77A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0A18 size=44
    let mut pc: u32 = 0x830B0A18;
    'dispatch: loop {
        match pc {
            0x830B0A18 => {
    //   block [0x830B0A18..0x830B0A44)
	// 830B0A18: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B0A1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0A20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0A28: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830B0A2C: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 830B0A30: 4BF44D59  bl 0x82ff5788
	ctx.lr = 0x830B0A34;
	sub_82FF5788(ctx, base);
	// 830B0A34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0A48 size=92
    let mut pc: u32 = 0x830B0A48;
    'dispatch: loop {
        match pc {
            0x830B0A48 => {
    //   block [0x830B0A48..0x830B0AA4)
	// 830B0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B0A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0A5C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830B0A60: 83DFBC80  lwz r30, -0x4380(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-17280 as u32) ) } as u64;
	// 830B0A64: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B0A68: 419A0014  beq cr6, 0x830b0a7c
	if ctx.cr[6].eq {
	pc = 0x830B0A7C; continue 'dispatch;
	}
	// 830B0A6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0A70: 4BFFFEF9  bl 0x830b0968
	ctx.lr = 0x830B0A74;
	sub_830B0968(ctx, base);
	// 830B0A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0A78: 4BF27869  bl 0x82fd82e0
	ctx.lr = 0x830B0A7C;
	sub_82FD82E0(ctx, base);
	// 830B0A7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0A80: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830B0A84: 917FBC80  stw r11, -0x4380(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-17280 as u32), ctx.r[11].u32 ) };
	// 830B0A88: 996ABC38  stb r11, -0x43c8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-17352 as u32), ctx.r[11].u8 ) };
	// 830B0A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B0A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B0A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0AA8 size=8
    let mut pc: u32 = 0x830B0AA8;
    'dispatch: loop {
        match pc {
            0x830B0AA8 => {
    //   block [0x830B0AA8..0x830B0AB0)
	// 830B0AA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B0AAC: 82186B38  lwz r16, 0x6b38(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(27448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0AB0 size=164
    let mut pc: u32 = 0x830B0AB0;
    'dispatch: loop {
        match pc {
            0x830B0AB0 => {
    //   block [0x830B0AB0..0x830B0B54)
	// 830B0AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0AB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B0ABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0AC0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830B0AC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0AC8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830B0ACC: 807EBC80  lwz r3, -0x4380(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17280 as u32) ) } as u64;
	// 830B0AD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B0AD4: 409A0068  bne cr6, 0x830b0b3c
	if !ctx.cr[6].eq {
	pc = 0x830B0B3C; continue 'dispatch;
	}
	// 830B0AD8: 4BFFF739  bl 0x830b0210
	ctx.lr = 0x830B0ADC;
	sub_830B0210(ctx, base);
	// 830B0ADC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B0AE0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0AE4: 4BF44CF5  bl 0x82ff57d8
	ctx.lr = 0x830B0AE8;
	sub_82FF57D8(ctx, base);
	// 830B0AE8: 817EBC80  lwz r11, -0x4380(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17280 as u32) ) } as u64;
	// 830B0AEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B0AF0: 409A0040  bne cr6, 0x830b0b30
	if !ctx.cr[6].eq {
	pc = 0x830B0B30; continue 'dispatch;
	}
	// 830B0AF4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830B0AF8: 4BF27751  bl 0x82fd8248
	ctx.lr = 0x830B0AFC;
	sub_82FD8248(ctx, base);
	// 830B0AFC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830B0B00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0B04: 41820010  beq 0x830b0b14
	if ctx.cr[0].eq {
	pc = 0x830B0B14; continue 'dispatch;
	}
	// 830B0B08: 4BFFF631  bl 0x830b0138
	ctx.lr = 0x830B0B0C;
	sub_830B0138(ctx, base);
	// 830B0B0C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830B0B10: 48000008  b 0x830b0b18
	pc = 0x830B0B18; continue 'dispatch;
	// 830B0B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B0B18: 3D60830B  lis r11, -0x7cf5
	ctx.r[11].s64 = -2096431104;
	// 830B0B1C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830B0B20: 913EBC80  stw r9, -0x4380(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-17280 as u32), ctx.r[9].u32 ) };
	// 830B0B24: 388B0A48  addi r4, r11, 0xa48
	ctx.r[4].s64 = ctx.r[11].s64 + 2632;
	// 830B0B28: 386ABC84  addi r3, r10, -0x437c
	ctx.r[3].s64 = ctx.r[10].s64 + -17276;
	// 830B0B2C: 4BF4700D  bl 0x82ff7b38
	ctx.lr = 0x830B0B30;
	sub_82FF7B38(ctx, base);
	// 830B0B30: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0B34: 4BF44CDD  bl 0x82ff5810
	ctx.lr = 0x830B0B38;
	sub_82FF5810(ctx, base);
	// 830B0B38: 807EBC80  lwz r3, -0x4380(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-17280 as u32) ) } as u64;
	// 830B0B3C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830B0B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0B48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B0B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0B54 size=40
    let mut pc: u32 = 0x830B0B54;
    'dispatch: loop {
        match pc {
            0x830B0B54 => {
    //   block [0x830B0B54..0x830B0B7C)
	// 830B0B54: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B0B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0B64: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B0B68: 4BF44CA9  bl 0x82ff5810
	ctx.lr = 0x830B0B6C;
	sub_82FF5810(ctx, base);
	// 830B0B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0B7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0B7C size=40
    let mut pc: u32 = 0x830B0B7C;
    'dispatch: loop {
        match pc {
            0x830B0B7C => {
    //   block [0x830B0B7C..0x830B0BA4)
	// 830B0B7C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B0B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0B8C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B0B90: 4BF27751  bl 0x82fd82e0
	ctx.lr = 0x830B0B94;
	sub_82FD82E0(ctx, base);
	// 830B0B94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0BA8 size=64
    let mut pc: u32 = 0x830B0BA8;
    'dispatch: loop {
        match pc {
            0x830B0BA8 => {
    //   block [0x830B0BA8..0x830B0BE8)
	// 830B0BA8: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0BAC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0BB0: 41820028  beq 0x830b0bd8
	if ctx.cr[0].eq {
	pc = 0x830B0BD8; continue 'dispatch;
	}
	// 830B0BB4: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0BB8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 830B0BBC: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 830B0BC0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830B0BC4: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 830B0BC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830B0BCC: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0BD0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0BD4: 4082FFE8  bne 0x830b0bbc
	if !ctx.cr[0].eq {
	pc = 0x830B0BBC; continue 'dispatch;
	}
	// 830B0BD8: 39440002  addi r10, r4, 2
	ctx.r[10].s64 = ctx.r[4].s64 + 2;
	// 830B0BDC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0BE0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0BE4: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0BE8 size=48
    let mut pc: u32 = 0x830B0BE8;
    'dispatch: loop {
        match pc {
            0x830B0BE8 => {
    //   block [0x830B0BE8..0x830B0C18)
	// 830B0BE8: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0BEC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 830B0BF0: 5529043E  clrlwi r9, r9, 0x10
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 830B0BF4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830B0BF8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830B0BFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830B0C00: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830B0C04: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 830B0C08: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0C0C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0C10: 4082FFE0  bne 0x830b0bf0
	if !ctx.cr[0].eq {
	pc = 0x830B0BF0; continue 'dispatch;
	}
	// 830B0C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0C18 size=28
    let mut pc: u32 = 0x830B0C18;
    'dispatch: loop {
        match pc {
            0x830B0C18 => {
    //   block [0x830B0C18..0x830B0C34)
	// 830B0C18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830B0C1C: 419A0034  beq cr6, 0x830b0c50
	if ctx.cr[6].eq {
		sub_830B0C50(ctx, base);
		return;
	}
	// 830B0C20: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0C24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0C28: 41820028  beq 0x830b0c50
	if ctx.cr[0].eq {
		sub_830B0C50(ctx, base);
		return;
	}
	// 830B0C2C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830B0C30: 48000008  b 0x830b0c38
	sub_830B0C34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0C34 size=28
    let mut pc: u32 = 0x830B0C34;
    'dispatch: loop {
        match pc {
            0x830B0C34 => {
    //   block [0x830B0C34..0x830B0C50)
	// 830B0C34: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830B0C38: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0C3C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0C40: 4082FFF4  bne 0x830b0c34
	if !ctx.cr[0].eq {
	pc = 0x830B0C34; continue 'dispatch;
	}
	// 830B0C44: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830B0C48: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830B0C4C: 48000008  b 0x830b0c54
	sub_830B0C50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0C50 size=40
    let mut pc: u32 = 0x830B0C50;
    'dispatch: loop {
        match pc {
            0x830B0C50 => {
    //   block [0x830B0C50..0x830B0C78)
	// 830B0C50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B0C54: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 830B0C58: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0C5C: 7D6B1A15  add. r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B0C60: 41820034  beq 0x830b0c94
	if ctx.cr[0].eq {
		sub_830B0C94(ctx, base);
		return;
	}
	// 830B0C64: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0C68: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0C6C: 41820028  beq 0x830b0c94
	if ctx.cr[0].eq {
		sub_830B0C94(ctx, base);
		return;
	}
	// 830B0C70: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830B0C74: 48000008  b 0x830b0c7c
	sub_830B0C78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0C78 size=28
    let mut pc: u32 = 0x830B0C78;
    'dispatch: loop {
        match pc {
            0x830B0C78 => {
    //   block [0x830B0C78..0x830B0C94)
	// 830B0C78: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830B0C7C: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0C80: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0C84: 4082FFF4  bne 0x830b0c78
	if !ctx.cr[0].eq {
	pc = 0x830B0C78; continue 'dispatch;
	}
	// 830B0C88: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830B0C8C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830B0C90: 48000008  b 0x830b0c98
	sub_830B0C94(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0C94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0C94 size=16
    let mut pc: u32 = 0x830B0C94;
    'dispatch: loop {
        match pc {
            0x830B0C94 => {
    //   block [0x830B0C94..0x830B0CA4)
	// 830B0C94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830B0C98: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0C9C: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830B0CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0CA8 size=72
    let mut pc: u32 = 0x830B0CA8;
    'dispatch: loop {
        match pc {
            0x830B0CA8 => {
    //   block [0x830B0CA8..0x830B0CF0)
	// 830B0CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B0CBC: 4BF490FD  bl 0x82ff9db8
	ctx.lr = 0x830B0CC0;
	sub_82FF9DB8(ctx, base);
	// 830B0CC0: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B0CC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B0CC8: 396B70A4  addi r11, r11, 0x70a4
	ctx.r[11].s64 = ctx.r[11].s64 + 28836;
	// 830B0CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0CD0: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830B0CD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B0CD8: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 830B0CDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B0CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0CF0 size=136
    let mut pc: u32 = 0x830B0CF0;
    'dispatch: loop {
        match pc {
            0x830B0CF0 => {
    //   block [0x830B0CF0..0x830B0D78)
	// 830B0CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0CF4: 480F7479  bl 0x831a816c
	ctx.lr = 0x830B0CF8;
	sub_831A8130(ctx, base);
	// 830B0CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0CFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B0D00: 897D0005  lbz r11, 5(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(5 as u32) ) } as u64;
	// 830B0D04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0D08: 40820068  bne 0x830b0d70
	if !ctx.cr[0].eq {
	pc = 0x830B0D70; continue 'dispatch;
	}
	// 830B0D0C: 4BFFFDA5  bl 0x830b0ab0
	ctx.lr = 0x830B0D10;
	sub_830B0AB0(ctx, base);
	// 830B0D10: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B0D14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B0D18: 3BEB7078  addi r31, r11, 0x7078
	ctx.r[31].s64 = ctx.r[11].s64 + 28792;
	// 830B0D1C: 38BFFF90  addi r5, r31, -0x70
	ctx.r[5].s64 = ctx.r[31].s64 + -112;
	// 830B0D20: 389FFF98  addi r4, r31, -0x68
	ctx.r[4].s64 = ctx.r[31].s64 + -104;
	// 830B0D24: 4BFFF865  bl 0x830b0588
	ctx.lr = 0x830B0D28;
	sub_830B0588(ctx, base);
	// 830B0D28: 38BFFF90  addi r5, r31, -0x70
	ctx.r[5].s64 = ctx.r[31].s64 + -112;
	// 830B0D2C: 389FFFB0  addi r4, r31, -0x50
	ctx.r[4].s64 = ctx.r[31].s64 + -80;
	// 830B0D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0D34: 4BFFF855  bl 0x830b0588
	ctx.lr = 0x830B0D38;
	sub_830B0588(ctx, base);
	// 830B0D38: 38BFFF90  addi r5, r31, -0x70
	ctx.r[5].s64 = ctx.r[31].s64 + -112;
	// 830B0D3C: 389FFFC8  addi r4, r31, -0x38
	ctx.r[4].s64 = ctx.r[31].s64 + -56;
	// 830B0D40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0D44: 4BFFF845  bl 0x830b0588
	ctx.lr = 0x830B0D48;
	sub_830B0588(ctx, base);
	// 830B0D48: 38BFFF90  addi r5, r31, -0x70
	ctx.r[5].s64 = ctx.r[31].s64 + -112;
	// 830B0D4C: 389FFFE0  addi r4, r31, -0x20
	ctx.r[4].s64 = ctx.r[31].s64 + -32;
	// 830B0D50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0D54: 4BFFF835  bl 0x830b0588
	ctx.lr = 0x830B0D58;
	sub_830B0588(ctx, base);
	// 830B0D58: 38BFFF90  addi r5, r31, -0x70
	ctx.r[5].s64 = ctx.r[31].s64 + -112;
	// 830B0D5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830B0D60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0D64: 4BFFF825  bl 0x830b0588
	ctx.lr = 0x830B0D68;
	sub_830B0588(ctx, base);
	// 830B0D68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B0D6C: 997D0005  stb r11, 5(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 830B0D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B0D74: 480F7448  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0D78 size=88
    let mut pc: u32 = 0x830B0D78;
    'dispatch: loop {
        match pc {
            0x830B0D78 => {
    //   block [0x830B0D78..0x830B0DD0)
	// 830B0D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B0D80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B0D84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B0D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0D8C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B0D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B0D94: 396B70A4  addi r11, r11, 0x70a4
	ctx.r[11].s64 = ctx.r[11].s64 + 28836;
	// 830B0D98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B0D9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B0DA0: 4BF49019  bl 0x82ff9db8
	ctx.lr = 0x830B0DA4;
	sub_82FF9DB8(ctx, base);
	// 830B0DA4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B0DA8: 4182000C  beq 0x830b0db4
	if ctx.cr[0].eq {
	pc = 0x830B0DB4; continue 'dispatch;
	}
	// 830B0DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0DB0: 4BF27531  bl 0x82fd82e0
	ctx.lr = 0x830B0DB4;
	sub_82FD82E0(ctx, base);
	// 830B0DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B0DB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B0DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B0DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B0DC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B0DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B0DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B0DD0 size=8
    let mut pc: u32 = 0x830B0DD0;
    'dispatch: loop {
        match pc {
            0x830B0DD0 => {
    //   block [0x830B0DD0..0x830B0DD8)
	// 830B0DD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B0DD4: 821870B8  lwz r16, 0x70b8(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28856 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B0DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B0DD8 size=1064
    let mut pc: u32 = 0x830B0DD8;
    'dispatch: loop {
        match pc {
            0x830B0DD8 => {
    //   block [0x830B0DD8..0x830B1200)
	// 830B0DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B0DDC: 480F7361  bl 0x831a813c
	ctx.lr = 0x830B0DE0;
	sub_831A8130(ctx, base);
	// 830B0DE0: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 830B0DE4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B0DE8: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 830B0DEC: 89710004  lbz r11, 4(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[17].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0DF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0DF4: 40820404  bne 0x830b11f8
	if !ctx.cr[0].eq {
	pc = 0x830B11F8; continue 'dispatch;
	}
	// 830B0DF8: 89710005  lbz r11, 5(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[17].u32.wrapping_add(5 as u32) ) } as u64;
	// 830B0DFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B0E00: 40820014  bne 0x830b0e14
	if !ctx.cr[0].eq {
	pc = 0x830B0E14; continue 'dispatch;
	}
	// 830B0E04: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0E08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0E0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B0E10: 4E800421  bctrl
	ctx.lr = 0x830B0E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0E14: 4BFFFC9D  bl 0x830b0ab0
	ctx.lr = 0x830B0E18;
	sub_830B0AB0(ctx, base);
	// 830B0E18: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B0E1C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830B0E20: 3BCB6B80  addi r30, r11, 0x6b80
	ctx.r[30].s64 = ctx.r[11].s64 + 27520;
	// 830B0E24: 387E02C4  addi r3, r30, 0x2c4
	ctx.r[3].s64 = ctx.r[30].s64 + 708;
	// 830B0E28: 831B0010  lwz r24, 0x10(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B0E2C: 4BFFFDED  bl 0x830b0c18
	ctx.lr = 0x830B0E30;
	sub_830B0C18(ctx, base);
	// 830B0E30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B0E34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B0E38: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B0E3C: 4BFF2935  bl 0x830a3770
	ctx.lr = 0x830B0E40;
	sub_830A3770(ctx, base);
	// 830B0E40: 3F208339  lis r25, -0x7cc7
	ctx.r[25].s64 = -2093416448;
	// 830B0E44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B0E48: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830B0E4C: 8179B7E8  lwz r11, -0x4818(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B0E50: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B0E54: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0E58: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0E5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B0E60: 4E800421  bctrl
	ctx.lr = 0x830B0E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0E64: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B0E68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B0E6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B0E70: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B0E74: 4BFF51BD  bl 0x830a6030
	ctx.lr = 0x830B0E78;
	sub_830A6030(ctx, base);
	// 830B0E78: 389E02C4  addi r4, r30, 0x2c4
	ctx.r[4].s64 = ctx.r[30].s64 + 708;
	// 830B0E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B0E80: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B0E84: 4BFFFD25  bl 0x830b0ba8
	ctx.lr = 0x830B0E88;
	sub_830B0BA8(ctx, base);
	// 830B0E88: 389E0490  addi r4, r30, 0x490
	ctx.r[4].s64 = ctx.r[30].s64 + 1168;
	// 830B0E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B0E90: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830B0E94: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B0E98: 4BFFF821  bl 0x830b06b8
	ctx.lr = 0x830B0E9C;
	sub_830B06B8(ctx, base);
	// 830B0E9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B0EA0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B0EA4: 4BFF28CD  bl 0x830a3770
	ctx.lr = 0x830B0EA8;
	sub_830A3770(ctx, base);
	// 830B0EA8: 397E0418  addi r11, r30, 0x418
	ctx.r[11].s64 = ctx.r[30].s64 + 1048;
	// 830B0EAC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B0EB0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B0EB4: 4BFFFD65  bl 0x830b0c18
	ctx.lr = 0x830B0EB8;
	sub_830B0C18(ctx, base);
	// 830B0EB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B0EBC: 8079B7E8  lwz r3, -0x4818(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B0EC0: 57B5103A  slwi r21, r29, 2
	ctx.r[21].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 830B0EC4: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 830B0EC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0ECC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B0ED4: 4E800421  bctrl
	ctx.lr = 0x830B0ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0ED8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B0EDC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B0EE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B0EE4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B0EE8: 4BFF5149  bl 0x830a6030
	ctx.lr = 0x830B0EEC;
	sub_830A6030(ctx, base);
	// 830B0EEC: 389E0418  addi r4, r30, 0x418
	ctx.r[4].s64 = ctx.r[30].s64 + 1048;
	// 830B0EF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B0EF4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B0EF8: 4BFFFCB1  bl 0x830b0ba8
	ctx.lr = 0x830B0EFC;
	sub_830B0BA8(ctx, base);
	// 830B0EFC: 389E04A8  addi r4, r30, 0x4a8
	ctx.r[4].s64 = ctx.r[30].s64 + 1192;
	// 830B0F00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B0F04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830B0F08: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B0F0C: 4BFFF7AD  bl 0x830b06b8
	ctx.lr = 0x830B0F10;
	sub_830B06B8(ctx, base);
	// 830B0F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B0F14: 4BFFFD05  bl 0x830b0c18
	ctx.lr = 0x830B0F18;
	sub_830B0C18(ctx, base);
	// 830B0F18: 397E0458  addi r11, r30, 0x458
	ctx.r[11].s64 = ctx.r[30].s64 + 1112;
	// 830B0F1C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 830B0F20: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B0F24: 4BFFFCF5  bl 0x830b0c18
	ctx.lr = 0x830B0F28;
	sub_830B0C18(ctx, base);
	// 830B0F28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B0F2C: 8079B7E8  lwz r3, -0x4818(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B0F30: 7EEBA214  add r23, r11, r20
	ctx.r[23].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 830B0F34: 7ED7EA14  add r22, r23, r29
	ctx.r[22].u64 = ctx.r[23].u64 + ctx.r[29].u64;
	// 830B0F38: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0F3C: 56D3103A  slwi r19, r22, 2
	ctx.r[19].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[19].u64 = ctx.r[19].u32 as u64;
	// 830B0F40: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 830B0F44: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0F48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830B0F4C: 4E800421  bctrl
	ctx.lr = 0x830B0F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0F50: 8179B7E8  lwz r11, -0x4818(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B0F54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830B0F58: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830B0F5C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830B0F60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B0F64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B0F68: 4BFFFC41  bl 0x830b0ba8
	ctx.lr = 0x830B0F6C;
	sub_830B0BA8(ctx, base);
	// 830B0F6C: 389E0458  addi r4, r30, 0x458
	ctx.r[4].s64 = ctx.r[30].s64 + 1112;
	// 830B0F70: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 830B0F74: 4BFFFC35  bl 0x830b0ba8
	ctx.lr = 0x830B0F78;
	sub_830B0BA8(ctx, base);
	// 830B0F78: 56EB103A  slwi r11, r23, 2
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B0F7C: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 830B0F80: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B0F84: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830B0F88: 480F7589  bl 0x831a8510
	ctx.lr = 0x830B0F8C;
	sub_831A8510(ctx, base);
	// 830B0F8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B0F90: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B0F94: 4BFF27DD  bl 0x830a3770
	ctx.lr = 0x830B0F98;
	sub_830A3770(ctx, base);
	// 830B0F98: 397E02D0  addi r11, r30, 0x2d0
	ctx.r[11].s64 = ctx.r[30].s64 + 720;
	// 830B0F9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B0FA0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B0FA4: 4BFFFC75  bl 0x830b0c18
	ctx.lr = 0x830B0FA8;
	sub_830B0C18(ctx, base);
	// 830B0FA8: 397E0468  addi r11, r30, 0x468
	ctx.r[11].s64 = ctx.r[30].s64 + 1128;
	// 830B0FAC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 830B0FB0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B0FB4: 4BFFFC65  bl 0x830b0c18
	ctx.lr = 0x830B0FB8;
	sub_830B0C18(ctx, base);
	// 830B0FB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830B0FBC: 8079B7E8  lwz r3, -0x4818(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B0FC0: 7D6BA214  add r11, r11, r20
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 830B0FC4: 7EABB214  add r21, r11, r22
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 830B0FC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B0FCC: 3A550008  addi r18, r21, 8
	ctx.r[18].s64 = ctx.r[21].s64 + 8;
	// 830B0FD0: 5644103A  slwi r4, r18, 2
	ctx.r[4].u32 = ctx.r[18].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830B0FD4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B0FD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 830B0FDC: 4E800421  bctrl
	ctx.lr = 0x830B0FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B0FE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B0FE4: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830B0FE8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B0FEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B0FF0: 4BFF5041  bl 0x830a6030
	ctx.lr = 0x830B0FF4;
	sub_830A6030(ctx, base);
	// 830B0FF4: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 830B0FF8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B0FFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B1000: 480F7511  bl 0x831a8510
	ctx.lr = 0x830B1004;
	sub_831A8510(ctx, base);
	// 830B1004: 389E02D0  addi r4, r30, 0x2d0
	ctx.r[4].s64 = ctx.r[30].s64 + 720;
	// 830B1008: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 830B100C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B1010: 4BFFFB99  bl 0x830b0ba8
	ctx.lr = 0x830B1014;
	sub_830B0BA8(ctx, base);
	// 830B1014: 389E0468  addi r4, r30, 0x468
	ctx.r[4].s64 = ctx.r[30].s64 + 1128;
	// 830B1018: 7CB4B214  add r5, r20, r22
	ctx.r[5].u64 = ctx.r[20].u64 + ctx.r[22].u64;
	// 830B101C: 4BFFFB8D  bl 0x830b0ba8
	ctx.lr = 0x830B1020;
	sub_830B0BA8(ctx, base);
	// 830B1020: 56A8103A  slwi r8, r21, 2
	ctx.r[8].u32 = ctx.r[21].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830B1024: 3920002D  li r9, 0x2d
	ctx.r[9].s64 = 45;
	// 830B1028: 39750001  addi r11, r21, 1
	ctx.r[11].s64 = ctx.r[21].s64 + 1;
	// 830B102C: 3A80003A  li r20, 0x3a
	ctx.r[20].s64 = 58;
	// 830B1030: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 830B1034: 3AA0005F  li r21, 0x5f
	ctx.r[21].s64 = 95;
	// 830B1038: 7D28E92E  stwx r9, r8, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 830B103C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830B1040: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1044: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B1048: 7D28E92E  stwx r9, r8, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 830B104C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830B1050: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1054: 7E89E92E  stwx r20, r9, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32), ctx.r[20].u32) };
	// 830B1058: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830B105C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1060: 7E89E92E  stwx r20, r9, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32), ctx.r[20].u32) };
	// 830B1064: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830B1068: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B106C: 7D49E92E  stwx r10, r9, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 830B1070: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830B1074: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1078: 7D49E92E  stwx r10, r9, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 830B107C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830B1080: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1084: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B1088: 7EAAE92E  stwx r21, r10, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32), ctx.r[21].u32) };
	// 830B108C: 7EABE92E  stwx r21, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[21].u32) };
	// 830B1090: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1094: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B1098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B109C: 4E800421  bctrl
	ctx.lr = 0x830B10A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B10A0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B10A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B10A8: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B10AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B10B0: 4E800421  bctrl
	ctx.lr = 0x830B10B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B10B4: 389E04D8  addi r4, r30, 0x4d8
	ctx.r[4].s64 = ctx.r[30].s64 + 1240;
	// 830B10B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B10BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830B10C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B10C4: 4BFFF5F5  bl 0x830b06b8
	ctx.lr = 0x830B10C8;
	sub_830B06B8(ctx, base);
	// 830B10C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B10CC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B10D0: 4BFF26A1  bl 0x830a3770
	ctx.lr = 0x830B10D4;
	sub_830A3770(ctx, base);
	// 830B10D4: 8179B7E8  lwz r11, -0x4818(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830B10D8: 3B370004  addi r25, r23, 4
	ctx.r[25].s64 = ctx.r[23].s64 + 4;
	// 830B10DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B10E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B10E4: 5724103A  slwi r4, r25, 2
	ctx.r[4].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830B10E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B10EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B10F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B10F4: 4E800421  bctrl
	ctx.lr = 0x830B10F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B10F8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B10FC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830B1100: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B1104: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830B1108: 4BFF4F29  bl 0x830a6030
	ctx.lr = 0x830B110C;
	sub_830A6030(ctx, base);
	// 830B110C: 56F9103A  slwi r25, r23, 2
	ctx.r[25].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 830B1110: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B1114: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B1118: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 830B111C: 480F73F5  bl 0x831a8510
	ctx.lr = 0x830B1120;
	sub_831A8510(ctx, base);
	// 830B1120: 39770001  addi r11, r23, 1
	ctx.r[11].s64 = ctx.r[23].s64 + 1;
	// 830B1124: 7E99D12E  stwx r20, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[20].u32) };
	// 830B1128: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B112C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830B1130: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1134: 7E8AD12E  stwx r20, r10, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32), ctx.r[20].u32) };
	// 830B1138: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830B113C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830B1140: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B1144: 7EAAD12E  stwx r21, r10, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32), ctx.r[21].u32) };
	// 830B1148: 7EABD12E  stwx r21, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[21].u32) };
	// 830B114C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1150: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B1154: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1158: 4E800421  bctrl
	ctx.lr = 0x830B115C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B115C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1160: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B1164: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B1168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B116C: 4E800421  bctrl
	ctx.lr = 0x830B1170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1170: 389E04F8  addi r4, r30, 0x4f8
	ctx.r[4].s64 = ctx.r[30].s64 + 1272;
	// 830B1174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B1178: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B117C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B1180: 4BFFF539  bl 0x830b06b8
	ctx.lr = 0x830B1184;
	sub_830B06B8(ctx, base);
	// 830B1184: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B1188: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B118C: 4BFF25E5  bl 0x830a3770
	ctx.lr = 0x830B1190;
	sub_830A3770(ctx, base);
	// 830B1190: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 830B1194: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B1198: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B119C: 4BFF4E95  bl 0x830a6030
	ctx.lr = 0x830B11A0;
	sub_830A6030(ctx, base);
	// 830B11A0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B11A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B11A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B11AC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830B11B0: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830B11B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B11B8: 4E800421  bctrl
	ctx.lr = 0x830B11BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B11BC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B11C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B11C4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830B11C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B11CC: 4E800421  bctrl
	ctx.lr = 0x830B11D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B11D0: 389E04C0  addi r4, r30, 0x4c0
	ctx.r[4].s64 = ctx.r[30].s64 + 1216;
	// 830B11D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830B11D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B11DC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B11E0: 4BFFF4D9  bl 0x830b06b8
	ctx.lr = 0x830B11E4;
	sub_830B06B8(ctx, base);
	// 830B11E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830B11E8: 99710004  stb r11, 4(r17)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[17].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830B11EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B11F0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B11F4: 4BF218CD  bl 0x82fd2ac0
	ctx.lr = 0x830B11F8;
	sub_82FD2AC0(ctx, base);
	// 830B11F8: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830B11FC: 480F6F90  b 0x831a818c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1200 size=40
    let mut pc: u32 = 0x830B1200;
    'dispatch: loop {
        match pc {
            0x830B1200 => {
    //   block [0x830B1200..0x830B1228)
	// 830B1200: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830B1204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B120C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1210: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830B1214: 4BF21C45  bl 0x82fd2e58
	ctx.lr = 0x830B1218;
	sub_82FD2E58(ctx, base);
	// 830B1218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B121C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1228 size=64
    let mut pc: u32 = 0x830B1228;
    'dispatch: loop {
        match pc {
            0x830B1228 => {
    //   block [0x830B1228..0x830B1268)
	// 830B1228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B122C: 480F6F41  bl 0x831a816c
	ctx.lr = 0x830B1230;
	sub_831A8130(ctx, base);
	// 830B1230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1234: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B1238: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 830B123C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1240: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830B1244: 4BFF4285  bl 0x830a54c8
	ctx.lr = 0x830B1248;
	sub_830A54C8(ctx, base);
	// 830B1248: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B124C: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830B1250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1254: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 830B1258: 396B70F8  addi r11, r11, 0x70f8
	ctx.r[11].s64 = ctx.r[11].s64 + 28920;
	// 830B125C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1260: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1264: 480F6F58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1268 size=88
    let mut pc: u32 = 0x830B1268;
    'dispatch: loop {
        match pc {
            0x830B1268 => {
    //   block [0x830B1268..0x830B12C0)
	// 830B1268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B126C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B127C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1284: 396B70F8  addi r11, r11, 0x70f8
	ctx.r[11].s64 = ctx.r[11].s64 + 28920;
	// 830B1288: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B128C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1290: 4BFF43D9  bl 0x830a5668
	ctx.lr = 0x830B1294;
	sub_830A5668(ctx, base);
	// 830B1294: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1298: 4182000C  beq 0x830b12a4
	if ctx.cr[0].eq {
	pc = 0x830B12A4; continue 'dispatch;
	}
	// 830B129C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B12A0: 4BF27041  bl 0x82fd82e0
	ctx.lr = 0x830B12A4;
	sub_82FD82E0(ctx, base);
	// 830B12A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B12A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B12AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B12B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B12B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B12B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B12BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B12C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B12C0 size=92
    let mut pc: u32 = 0x830B12C0;
    'dispatch: loop {
        match pc {
            0x830B12C0 => {
    //   block [0x830B12C0..0x830B131C)
	// 830B12C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B12C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B12C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B12CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B12D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B12D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B12D8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830B12DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B12E0: 4BFF41E9  bl 0x830a54c8
	ctx.lr = 0x830B12E4;
	sub_830A54C8(ctx, base);
	// 830B12E4: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B12E8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830B12EC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830B12F0: 396B7140  addi r11, r11, 0x7140
	ctx.r[11].s64 = ctx.r[11].s64 + 28992;
	// 830B12F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B12F8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B12FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1300: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830B1304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B130C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1320 size=88
    let mut pc: u32 = 0x830B1320;
    'dispatch: loop {
        match pc {
            0x830B1320 => {
    //   block [0x830B1320..0x830B1378)
	// 830B1320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B132C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1334: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B133C: 396B7140  addi r11, r11, 0x7140
	ctx.r[11].s64 = ctx.r[11].s64 + 28992;
	// 830B1340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1344: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1348: 4BFF4321  bl 0x830a5668
	ctx.lr = 0x830B134C;
	sub_830A5668(ctx, base);
	// 830B134C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1350: 4182000C  beq 0x830b135c
	if ctx.cr[0].eq {
	pc = 0x830B135C; continue 'dispatch;
	}
	// 830B1354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1358: 4BF26F89  bl 0x82fd82e0
	ctx.lr = 0x830B135C;
	sub_82FD82E0(ctx, base);
	// 830B135C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B136C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1378 size=68
    let mut pc: u32 = 0x830B1378;
    'dispatch: loop {
        match pc {
            0x830B1378 => {
    //   block [0x830B1378..0x830B13BC)
	// 830B1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B137C: 480F6DF1  bl 0x831a816c
	ctx.lr = 0x830B1380;
	sub_831A8130(ctx, base);
	// 830B1380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1384: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B1388: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830B138C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830B1390: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B1394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1398: 4BFF4131  bl 0x830a54c8
	ctx.lr = 0x830B139C;
	sub_830A54C8(ctx, base);
	// 830B139C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B13A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B13A4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B13A8: 396B7188  addi r11, r11, 0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + 29064;
	// 830B13AC: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830B13B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B13B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B13B8: 480F6E04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B13C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B13C0 size=16
    let mut pc: u32 = 0x830B13C0;
    'dispatch: loop {
        match pc {
            0x830B13C0 => {
    //   block [0x830B13C0..0x830B13D0)
	// 830B13C0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 830B13C4: 409A000C  bne cr6, 0x830b13d0
	if !ctx.cr[6].eq {
		sub_830B13D0(ctx, base);
		return;
	}
	// 830B13C8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B13CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B13D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B13D0 size=8
    let mut pc: u32 = 0x830B13D0;
    'dispatch: loop {
        match pc {
            0x830B13D0 => {
    //   block [0x830B13D0..0x830B13D8)
	// 830B13D0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B13D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B13D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B13D8 size=88
    let mut pc: u32 = 0x830B13D8;
    'dispatch: loop {
        match pc {
            0x830B13D8 => {
    //   block [0x830B13D8..0x830B1430)
	// 830B13D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B13DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B13E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B13E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B13E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B13EC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B13F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B13F4: 396B7188  addi r11, r11, 0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + 29064;
	// 830B13F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B13FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1400: 4BFF4269  bl 0x830a5668
	ctx.lr = 0x830B1404;
	sub_830A5668(ctx, base);
	// 830B1404: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B1408: 4182000C  beq 0x830b1414
	if ctx.cr[0].eq {
	pc = 0x830B1414; continue 'dispatch;
	}
	// 830B140C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1410: 4BF26ED1  bl 0x82fd82e0
	ctx.lr = 0x830B1414;
	sub_82FD82E0(ctx, base);
	// 830B1414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B141C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B142C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1430 size=92
    let mut pc: u32 = 0x830B1430;
    'dispatch: loop {
        match pc {
            0x830B1430 => {
    //   block [0x830B1430..0x830B148C)
	// 830B1430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B143C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B144C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B1450: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830B1454: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1458: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B145C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1460: 4E800421  bctrl
	ctx.lr = 0x830B1464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B1468: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830B146C: 4BF1F715  bl 0x82fd0b80
	ctx.lr = 0x830B1470;
	sub_82FD0B80(ctx, base);
	// 830B1470: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830B1474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B147C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1490 size=68
    let mut pc: u32 = 0x830B1490;
    'dispatch: loop {
        match pc {
            0x830B1490 => {
    //   block [0x830B1490..0x830B14D4)
	// 830B1490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1498: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B149C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B14A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B14A4: 4BFF4025  bl 0x830a54c8
	ctx.lr = 0x830B14A8;
	sub_830A54C8(ctx, base);
	// 830B14A8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B14AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B14B0: 396B71D8  addi r11, r11, 0x71d8
	ctx.r[11].s64 = ctx.r[11].s64 + 29144;
	// 830B14B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B14B8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B14BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B14C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B14C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B14C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B14CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B14D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B14D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B14D8 size=8
    let mut pc: u32 = 0x830B14D8;
    'dispatch: loop {
        match pc {
            0x830B14D8 => {
    //   block [0x830B14D8..0x830B14E0)
	// 830B14D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B14DC: 82187228  lwz r16, 0x7228(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(29224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B14E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B14E0 size=108
    let mut pc: u32 = 0x830B14E0;
    'dispatch: loop {
        match pc {
            0x830B14E0 => {
    //   block [0x830B14E0..0x830B154C)
	// 830B14E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B14E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B14E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B14EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B14F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830B14F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B14F8: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B14FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B1500: 396B71D8  addi r11, r11, 0x71d8
	ctx.r[11].s64 = ctx.r[11].s64 + 29144;
	// 830B1504: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830B1508: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B150C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1510: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B1514: 41820018  beq 0x830b152c
	if ctx.cr[0].eq {
	pc = 0x830B152C; continue 'dispatch;
	}
	// 830B1518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B151C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830B1520: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1528: 4E800421  bctrl
	ctx.lr = 0x830B152C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B152C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B1530: 4BFF4139  bl 0x830a5668
	ctx.lr = 0x830B1534;
	sub_830A5668(ctx, base);
	// 830B1534: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830B1538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B153C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B154C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B154C size=40
    let mut pc: u32 = 0x830B154C;
    'dispatch: loop {
        match pc {
            0x830B154C => {
    //   block [0x830B154C..0x830B1574)
	// 830B154C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830B1550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B155C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830B1560: 4BFF4109  bl 0x830a5668
	ctx.lr = 0x830B1564;
	sub_830A5668(ctx, base);
	// 830B1564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B156C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1578 size=16
    let mut pc: u32 = 0x830B1578;
    'dispatch: loop {
        match pc {
            0x830B1578 => {
    //   block [0x830B1578..0x830B1588)
	// 830B1578: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B157C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830B1580: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B1584: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1588 size=8
    let mut pc: u32 = 0x830B1588;
    'dispatch: loop {
        match pc {
            0x830B1588 => {
    //   block [0x830B1588..0x830B1590)
	// 830B1588: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1590 size=8
    let mut pc: u32 = 0x830B1590;
    'dispatch: loop {
        match pc {
            0x830B1590 => {
    //   block [0x830B1590..0x830B1598)
	// 830B1590: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1594: 4BF7B2DC  b 0x8302c870
	sub_8302C870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1598 size=76
    let mut pc: u32 = 0x830B1598;
    'dispatch: loop {
        match pc {
            0x830B1598 => {
    //   block [0x830B1598..0x830B15E4)
	// 830B1598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B15A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B15A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B15A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B15AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B15B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B15B4: 4BFFFF2D  bl 0x830b14e0
	ctx.lr = 0x830B15B8;
	sub_830B14E0(ctx, base);
	// 830B15B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B15BC: 4182000C  beq 0x830b15c8
	if ctx.cr[0].eq {
	pc = 0x830B15C8; continue 'dispatch;
	}
	// 830B15C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B15C4: 4BF26D1D  bl 0x82fd82e0
	ctx.lr = 0x830B15C8;
	sub_82FD82E0(ctx, base);
	// 830B15C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B15CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B15D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B15D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B15D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B15DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B15E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B15E8 size=8
    let mut pc: u32 = 0x830B15E8;
    'dispatch: loop {
        match pc {
            0x830B15E8 => {
    //   block [0x830B15E8..0x830B15F0)
	// 830B15E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B15EC: 82187270  lwz r16, 0x7270(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(29296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B15F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B15F0 size=768
    let mut pc: u32 = 0x830B15F0;
    'dispatch: loop {
        match pc {
            0x830B15F0 => {
    //   block [0x830B15F0..0x830B18F0)
	// 830B15F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B15F4: 480F6B61  bl 0x831a8154
	ctx.lr = 0x830B15F8;
	sub_831A8130(ctx, base);
	// 830B15F8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 830B15FC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1600: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 830B1604: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 830B1608: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830B160C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830B1610: 419A02D8  beq cr6, 0x830b18e8
	if ctx.cr[6].eq {
	pc = 0x830B18E8; continue 'dispatch;
	}
	// 830B1614: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830B161C: 409A0050  bne cr6, 0x830b166c
	if !ctx.cr[6].eq {
	pc = 0x830B166C; continue 'dispatch;
	}
	// 830B1620: 809B003C  lwz r4, 0x3c(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B1624: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830B1628: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830B162C: 4BF26C6D  bl 0x82fd8298
	ctx.lr = 0x830B1630;
	sub_82FD8298(ctx, base);
	// 830B1630: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830B1634: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830B1638: 4182002C  beq 0x830b1664
	if ctx.cr[0].eq {
	pc = 0x830B1664; continue 'dispatch;
	}
	// 830B163C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B1640: 80DB003C  lwz r6, 0x3c(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B1644: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830B1648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B164C: 4BF9B1A5  bl 0x8304c7f0
	ctx.lr = 0x830B1650;
	sub_8304C7F0(ctx, base);
	// 830B1650: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830B1654: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830B1658: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 830B165C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1660: 48000008  b 0x830b1668
	pc = 0x830B1668; continue 'dispatch;
	// 830B1664: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830B1668: 915A000C  stw r10, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830B166C: A17A0004  lhz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1670: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830B1674: 409A0014  bne cr6, 0x830b1688
	if !ctx.cr[6].eq {
	pc = 0x830B1688; continue 'dispatch;
	}
	// 830B1678: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B167C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830B1680: 4BF89AD1  bl 0x8303b150
	ctx.lr = 0x830B1684;
	sub_8303B150(ctx, base);
	// 830B1684: 48000264  b 0x830b18e8
	pc = 0x830B18E8; continue 'dispatch;
	// 830B1688: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B168C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B1690: A2F80004  lhz r23, 4(r24)
	ctx.r[23].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1694: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B169C: 4E800421  bctrl
	ctx.lr = 0x830B16A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B16A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B16A4: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 830B16A8: 409A0054  bne cr6, 0x830b16fc
	if !ctx.cr[6].eq {
	pc = 0x830B16FC; continue 'dispatch;
	}
	// 830B16AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830B16B0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830B16B4: 419A0234  beq cr6, 0x830b18e8
	if ctx.cr[6].eq {
	pc = 0x830B18E8; continue 'dispatch;
	}
	// 830B16B8: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B16BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B16C0: 839A0000  lwz r28, 0(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B16C4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B16C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B16CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B16D0: 4E800421  bctrl
	ctx.lr = 0x830B16D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B16D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B16D8: 817C0044  lwz r11, 0x44(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 830B16DC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830B16E0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830B16E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B16E8: 4E800421  bctrl
	ctx.lr = 0x830B16EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B16EC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830B16F0: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830B16F4: 4198FFC4  blt cr6, 0x830b16b8
	if ctx.cr[6].lt {
	pc = 0x830B16B8; continue 'dispatch;
	}
	// 830B16F8: 480001F0  b 0x830b18e8
	pc = 0x830B18E8; continue 'dispatch;
	// 830B16FC: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B1700: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1704: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B1708: 4082000C  bne 0x830b1714
	if !ctx.cr[0].eq {
	pc = 0x830B1714; continue 'dispatch;
	}
	// 830B170C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830B1710: 4BFFFF70  b 0x830b1680
	pc = 0x830B1680; continue 'dispatch;
	// 830B1714: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 830B1718: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830B171C: 4BF7B155  bl 0x8302c870
	ctx.lr = 0x830B1720;
	sub_8302C870(ctx, base);
	// 830B1720: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B1724: A3DD0004  lhz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B1728: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830B172C: 4182000C  beq 0x830b1738
	if ctx.cr[0].eq {
	pc = 0x830B1738; continue 'dispatch;
	}
	// 830B1730: 2B1E000A  cmplwi cr6, r30, 0xa
	ctx.cr[6].compare_u32(ctx.r[30].u32, 10 as u32, &mut ctx.xer);
	// 830B1734: 409A0014  bne cr6, 0x830b1748
	if !ctx.cr[6].eq {
	pc = 0x830B1748; continue 'dispatch;
	}
	// 830B1738: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830B173C: 419A0018  beq cr6, 0x830b1754
	if ctx.cr[6].eq {
	pc = 0x830B1754; continue 'dispatch;
	}
	// 830B1740: 2B17000A  cmplwi cr6, r23, 0xa
	ctx.cr[6].compare_u32(ctx.r[23].u32, 10 as u32, &mut ctx.xer);
	// 830B1744: 419A0010  beq cr6, 0x830b1754
	if ctx.cr[6].eq {
	pc = 0x830B1754; continue 'dispatch;
	}
	// 830B1748: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B174C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830B1750: 4BFFFF30  b 0x830b1680
	pc = 0x830B1680; continue 'dispatch;
	// 830B1754: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 830B1758: 80BB003C  lwz r5, 0x3c(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B175C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B1760: 4BF2D6F9  bl 0x82fdee58
	ctx.lr = 0x830B1764;
	sub_82FDEE58(ctx, base);
	// 830B1764: 3F200001  lis r25, 1
	ctx.r[25].s64 = 65536;
	// 830B1768: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830B176C: 409A0098  bne cr6, 0x830b1804
	if !ctx.cr[6].eq {
	pc = 0x830B1804; continue 'dispatch;
	}
	// 830B1770: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1774: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B1778: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830B177C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1780: 4E800421  bctrl
	ctx.lr = 0x830B1784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1784: 7F03C800  cmpw cr6, r3, r25
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830B1788: 4198003C  blt cr6, 0x830b17c4
	if ctx.cr[6].lt {
	pc = 0x830B17C4; continue 'dispatch;
	}
	// 830B178C: 809B003C  lwz r4, 0x3c(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B1790: 4BFF5839  bl 0x830a6fc8
	ctx.lr = 0x830B1794;
	sub_830A6FC8(ctx, base);
	// 830B1794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B1798: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B179C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B17A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B17A4: 4BF27DCD  bl 0x82fd9570
	ctx.lr = 0x830B17A8;
	sub_82FD9570(ctx, base);
	// 830B17A8: 807B003C  lwz r3, 0x3c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B17AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B17B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B17B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B17B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B17BC: 4E800421  bctrl
	ctx.lr = 0x830B17C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B17C0: 48000010  b 0x830b17d0
	pc = 0x830B17D0; continue 'dispatch;
	// 830B17C4: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830B17C8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B17CC: 4BF1F34D  bl 0x82fd0b18
	ctx.lr = 0x830B17D0;
	sub_82FD0B18(ctx, base);
	// 830B17D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830B17D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830B17D8: 4BFF2259  bl 0x830a3a30
	ctx.lr = 0x830B17DC;
	sub_830A3A30(ctx, base);
	// 830B17DC: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 830B17E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830B17E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830B17E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830B17EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B17F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B17F4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830B17F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B17FC: 4E800421  bctrl
	ctx.lr = 0x830B1800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1800: 48000028  b 0x830b1828
	pc = 0x830B1828; continue 'dispatch;
	// 830B1804: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B180C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B1810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1814: 4E800421  bctrl
	ctx.lr = 0x830B1818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1818: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B181C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B1820: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B1824: 4BF27D4D  bl 0x82fd9570
	ctx.lr = 0x830B1828;
	sub_82FD9570(ctx, base);
	// 830B1828: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B182C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 830B1830: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830B1834: 409A0060  bne cr6, 0x830b1894
	if !ctx.cr[6].eq {
	pc = 0x830B1894; continue 'dispatch;
	}
	// 830B1838: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830B183C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B1840: 4E800421  bctrl
	ctx.lr = 0x830B1844;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1844: 7F03C800  cmpw cr6, r3, r25
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[25].s32, &mut ctx.xer);
	// 830B1848: 4198003C  blt cr6, 0x830b1884
	if ctx.cr[6].lt {
	pc = 0x830B1884; continue 'dispatch;
	}
	// 830B184C: 809B003C  lwz r4, 0x3c(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B1850: 4BFF5779  bl 0x830a6fc8
	ctx.lr = 0x830B1854;
	sub_830A6FC8(ctx, base);
	// 830B1854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B1858: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B185C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B1860: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B1864: 4BF27D0D  bl 0x82fd9570
	ctx.lr = 0x830B1868;
	sub_82FD9570(ctx, base);
	// 830B1868: 807B003C  lwz r3, 0x3c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) } as u64;
	// 830B186C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830B1870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B1874: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B1878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B187C: 4E800421  bctrl
	ctx.lr = 0x830B1880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B1880: 48000030  b 0x830b18b0
	pc = 0x830B18B0; continue 'dispatch;
	// 830B1884: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830B1888: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B188C: 4BF1F28D  bl 0x82fd0b18
	ctx.lr = 0x830B1890;
	sub_82FD0B18(ctx, base);
	// 830B1890: 48000020  b 0x830b18b0
	pc = 0x830B18B0; continue 'dispatch;
	// 830B1894: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830B1898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B189C: 4E800421  bctrl
	ctx.lr = 0x830B18A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B18A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830B18A4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B18A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830B18AC: 4BF27CC5  bl 0x82fd9570
	ctx.lr = 0x830B18B0;
	sub_82FD9570(ctx, base);
	// 830B18B0: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830B18B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830B18B8: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B18BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830B18C0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830B18C4: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 830B18C8: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B18CC: 4BFFFB65  bl 0x830b1430
	ctx.lr = 0x830B18D0;
	sub_830B1430(ctx, base);
	// 830B18D0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830B18D4: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830B18D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830B18DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830B18E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830B18E4: 4E800421  bctrl
	ctx.lr = 0x830B18E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830B18E8: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 830B18EC: 480F68B8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B18F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B18F0 size=44
    let mut pc: u32 = 0x830B18F0;
    'dispatch: loop {
        match pc {
            0x830B18F0 => {
    //   block [0x830B18F0..0x830B191C)
	// 830B18F0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830B18F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B18F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B18FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1900: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830B1904: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830B1908: 4BF269D9  bl 0x82fd82e0
	ctx.lr = 0x830B190C;
	sub_82FD82E0(ctx, base);
	// 830B190C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B191C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B191C size=40
    let mut pc: u32 = 0x830B191C;
    'dispatch: loop {
        match pc {
            0x830B191C => {
    //   block [0x830B191C..0x830B1944)
	// 830B191C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 830B1920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B192C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830B1930: 4BF2D5A9  bl 0x82fdeed8
	ctx.lr = 0x830B1934;
	sub_82FDEED8(ctx, base);
	// 830B1934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1948 size=80
    let mut pc: u32 = 0x830B1948;
    'dispatch: loop {
        match pc {
            0x830B1948 => {
    //   block [0x830B1948..0x830B1998)
	// 830B1948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B1954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B1958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B195C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830B1960: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 830B1964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B1968: 4BFF3B61  bl 0x830a54c8
	ctx.lr = 0x830B196C;
	sub_830A54C8(ctx, base);
	// 830B196C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B1974: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830B1978: 396B72C0  addi r11, r11, 0x72c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29376;
	// 830B197C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B1984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B198C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B1990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B1994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1998 size=88
    let mut pc: u32 = 0x830B1998;
    'dispatch: loop {
        match pc {
            0x830B1998 => {
    //   block [0x830B1998..0x830B19F0)
	// 830B1998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B19A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830B19A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830B19A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B19AC: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B19B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830B19B4: 396B72C0  addi r11, r11, 0x72c0
	ctx.r[11].s64 = ctx.r[11].s64 + 29376;
	// 830B19B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830B19BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B19C0: 4BFF3CA9  bl 0x830a5668
	ctx.lr = 0x830B19C4;
	sub_830A5668(ctx, base);
	// 830B19C4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830B19C8: 4182000C  beq 0x830b19d4
	if ctx.cr[0].eq {
	pc = 0x830B19D4; continue 'dispatch;
	}
	// 830B19CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B19D0: 4BF26911  bl 0x82fd82e0
	ctx.lr = 0x830B19D4;
	sub_82FD82E0(ctx, base);
	// 830B19D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830B19D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830B19DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B19E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B19E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830B19E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830B19EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B19F0 size=8
    let mut pc: u32 = 0x830B19F0;
    'dispatch: loop {
        match pc {
            0x830B19F0 => {
    //   block [0x830B19F0..0x830B19F8)
	// 830B19F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B19F4: 82187358  lwz r16, 0x7358(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(29528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B19F8 size=92
    let mut pc: u32 = 0x830B19F8;
    'dispatch: loop {
        match pc {
            0x830B19F8 => {
    //   block [0x830B19F8..0x830B1A54)
	// 830B19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B19FC: 480F6769  bl 0x831a8164
	ctx.lr = 0x830B1A00;
	sub_831A8130(ctx, base);
	// 830B1A00: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830B1A04: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1A08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830B1A0C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 830B1A10: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830B1A14: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830B1A18: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830B1A1C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830B1A20: 4BFF3AA9  bl 0x830a54c8
	ctx.lr = 0x830B1A24;
	sub_830A54C8(ctx, base);
	// 830B1A24: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830B1A28: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830B1A2C: 937E000C  stw r27, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 830B1A30: 396B7308  addi r11, r11, 0x7308
	ctx.r[11].s64 = ctx.r[11].s64 + 29448;
	// 830B1A34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830B1A38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830B1A3C: 4BF1F145  bl 0x82fd0b80
	ctx.lr = 0x830B1A40;
	sub_82FD0B80(ctx, base);
	// 830B1A40: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830B1A44: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830B1A48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830B1A4C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830B1A50: 480F6764  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830B1A54 size=40
    let mut pc: u32 = 0x830B1A54;
    'dispatch: loop {
        match pc {
            0x830B1A54 => {
    //   block [0x830B1A54..0x830B1A7C)
	// 830B1A54: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830B1A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830B1A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830B1A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830B1A64: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830B1A68: 4BFF3C01  bl 0x830a5668
	ctx.lr = 0x830B1A6C;
	sub_830A5668(ctx, base);
	// 830B1A6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830B1A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830B1A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830B1A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830B1A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830B1A80 size=8
    let mut pc: u32 = 0x830B1A80;
    'dispatch: loop {
        match pc {
            0x830B1A80 => {
    //   block [0x830B1A80..0x830B1A88)
	// 830B1A80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830B1A84: 82187390  lwz r16, 0x7390(r24)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(29584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


