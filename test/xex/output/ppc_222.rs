pub fn sub_8327D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D830 size=56
    let mut pc: u32 = 0x8327D830;
    'dispatch: loop {
        match pc {
            0x8327D830 => {
    //   block [0x8327D830..0x8327D868)
	// 8327D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327D840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327D844: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327D848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327D84C: 4AF7650D  bl 0x821f3d58
	ctx.lr = 0x8327D850;
	sub_821F3D58(ctx, base);
	// 8327D850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D854: 906ADC48  stw r3, -0x23b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9144 as u32), ctx.r[3].u32 ) };
	// 8327D858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D868 size=64
    let mut pc: u32 = 0x8327D868;
    'dispatch: loop {
        match pc {
            0x8327D868 => {
    //   block [0x8327D868..0x8327D8A8)
	// 8327D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D87C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327D880: 386ADC4C  addi r3, r10, -0x23b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9140;
	// 8327D884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D888: 4AFAF649  bl 0x8222ced0
	ctx.lr = 0x8327D88C;
	sub_8222CED0(ctx, base);
	// 8327D88C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D890: 386906E8  addi r3, r9, 0x6e8
	ctx.r[3].s64 = ctx.r[9].s64 + 1768;
	// 8327D894: 4BA2C68D  bl 0x82ca9f20
	ctx.lr = 0x8327D898;
	sub_82CA9F20(ctx, base);
	// 8327D898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D8A8 size=64
    let mut pc: u32 = 0x8327D8A8;
    'dispatch: loop {
        match pc {
            0x8327D8A8 => {
    //   block [0x8327D8A8..0x8327D8E8)
	// 8327D8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D8B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D8B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D8BC: 388BEB38  addi r4, r11, -0x14c8
	ctx.r[4].s64 = ctx.r[11].s64 + -5320;
	// 8327D8C0: 386ADC50  addi r3, r10, -0x23b0
	ctx.r[3].s64 = ctx.r[10].s64 + -9136;
	// 8327D8C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D8C8: 4AFAF609  bl 0x8222ced0
	ctx.lr = 0x8327D8CC;
	sub_8222CED0(ctx, base);
	// 8327D8CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D8D0: 386906F8  addi r3, r9, 0x6f8
	ctx.r[3].s64 = ctx.r[9].s64 + 1784;
	// 8327D8D4: 4BA2C64D  bl 0x82ca9f20
	ctx.lr = 0x8327D8D8;
	sub_82CA9F20(ctx, base);
	// 8327D8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D8E8 size=64
    let mut pc: u32 = 0x8327D8E8;
    'dispatch: loop {
        match pc {
            0x8327D8E8 => {
    //   block [0x8327D8E8..0x8327D928)
	// 8327D8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D8F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D8F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D8FC: 388BEB70  addi r4, r11, -0x1490
	ctx.r[4].s64 = ctx.r[11].s64 + -5264;
	// 8327D900: 386ADC54  addi r3, r10, -0x23ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9132;
	// 8327D904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D908: 4AFAF5C9  bl 0x8222ced0
	ctx.lr = 0x8327D90C;
	sub_8222CED0(ctx, base);
	// 8327D90C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D910: 38690708  addi r3, r9, 0x708
	ctx.r[3].s64 = ctx.r[9].s64 + 1800;
	// 8327D914: 4BA2C60D  bl 0x82ca9f20
	ctx.lr = 0x8327D918;
	sub_82CA9F20(ctx, base);
	// 8327D918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D928 size=64
    let mut pc: u32 = 0x8327D928;
    'dispatch: loop {
        match pc {
            0x8327D928 => {
    //   block [0x8327D928..0x8327D968)
	// 8327D928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D934: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D93C: 388BEBAC  addi r4, r11, -0x1454
	ctx.r[4].s64 = ctx.r[11].s64 + -5204;
	// 8327D940: 386ADC58  addi r3, r10, -0x23a8
	ctx.r[3].s64 = ctx.r[10].s64 + -9128;
	// 8327D944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D948: 4AFAF589  bl 0x8222ced0
	ctx.lr = 0x8327D94C;
	sub_8222CED0(ctx, base);
	// 8327D94C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D950: 38690718  addi r3, r9, 0x718
	ctx.r[3].s64 = ctx.r[9].s64 + 1816;
	// 8327D954: 4BA2C5CD  bl 0x82ca9f20
	ctx.lr = 0x8327D958;
	sub_82CA9F20(ctx, base);
	// 8327D958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D968 size=64
    let mut pc: u32 = 0x8327D968;
    'dispatch: loop {
        match pc {
            0x8327D968 => {
    //   block [0x8327D968..0x8327D9A8)
	// 8327D968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D974: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D97C: 388BEBC0  addi r4, r11, -0x1440
	ctx.r[4].s64 = ctx.r[11].s64 + -5184;
	// 8327D980: 386ADC5C  addi r3, r10, -0x23a4
	ctx.r[3].s64 = ctx.r[10].s64 + -9124;
	// 8327D984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D988: 4AFAF549  bl 0x8222ced0
	ctx.lr = 0x8327D98C;
	sub_8222CED0(ctx, base);
	// 8327D98C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D990: 38690728  addi r3, r9, 0x728
	ctx.r[3].s64 = ctx.r[9].s64 + 1832;
	// 8327D994: 4BA2C58D  bl 0x82ca9f20
	ctx.lr = 0x8327D998;
	sub_82CA9F20(ctx, base);
	// 8327D998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D9A8 size=64
    let mut pc: u32 = 0x8327D9A8;
    'dispatch: loop {
        match pc {
            0x8327D9A8 => {
    //   block [0x8327D9A8..0x8327D9E8)
	// 8327D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D9B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D9B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D9B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D9BC: 388BEBD0  addi r4, r11, -0x1430
	ctx.r[4].s64 = ctx.r[11].s64 + -5168;
	// 8327D9C0: 386ADC60  addi r3, r10, -0x23a0
	ctx.r[3].s64 = ctx.r[10].s64 + -9120;
	// 8327D9C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327D9C8: 4AFAF509  bl 0x8222ced0
	ctx.lr = 0x8327D9CC;
	sub_8222CED0(ctx, base);
	// 8327D9CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327D9D0: 38690738  addi r3, r9, 0x738
	ctx.r[3].s64 = ctx.r[9].s64 + 1848;
	// 8327D9D4: 4BA2C54D  bl 0x82ca9f20
	ctx.lr = 0x8327D9D8;
	sub_82CA9F20(ctx, base);
	// 8327D9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327D9E8 size=64
    let mut pc: u32 = 0x8327D9E8;
    'dispatch: loop {
        match pc {
            0x8327D9E8 => {
    //   block [0x8327D9E8..0x8327DA28)
	// 8327D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327D9F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327D9F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327D9F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327D9FC: 388BEBE0  addi r4, r11, -0x1420
	ctx.r[4].s64 = ctx.r[11].s64 + -5152;
	// 8327DA00: 386ADC64  addi r3, r10, -0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + -9116;
	// 8327DA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA08: 4AFAF4C9  bl 0x8222ced0
	ctx.lr = 0x8327DA0C;
	sub_8222CED0(ctx, base);
	// 8327DA0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA10: 38690748  addi r3, r9, 0x748
	ctx.r[3].s64 = ctx.r[9].s64 + 1864;
	// 8327DA14: 4BA2C50D  bl 0x82ca9f20
	ctx.lr = 0x8327DA18;
	sub_82CA9F20(ctx, base);
	// 8327DA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DA28 size=64
    let mut pc: u32 = 0x8327DA28;
    'dispatch: loop {
        match pc {
            0x8327DA28 => {
    //   block [0x8327DA28..0x8327DA68)
	// 8327DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DA34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DA38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DA3C: 388BEBF0  addi r4, r11, -0x1410
	ctx.r[4].s64 = ctx.r[11].s64 + -5136;
	// 8327DA40: 386ADC68  addi r3, r10, -0x2398
	ctx.r[3].s64 = ctx.r[10].s64 + -9112;
	// 8327DA44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA48: 4AFAF489  bl 0x8222ced0
	ctx.lr = 0x8327DA4C;
	sub_8222CED0(ctx, base);
	// 8327DA4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA50: 38690758  addi r3, r9, 0x758
	ctx.r[3].s64 = ctx.r[9].s64 + 1880;
	// 8327DA54: 4BA2C4CD  bl 0x82ca9f20
	ctx.lr = 0x8327DA58;
	sub_82CA9F20(ctx, base);
	// 8327DA58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DA68 size=64
    let mut pc: u32 = 0x8327DA68;
    'dispatch: loop {
        match pc {
            0x8327DA68 => {
    //   block [0x8327DA68..0x8327DAA8)
	// 8327DA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DA70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DA74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DA78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DA7C: 388BEC00  addi r4, r11, -0x1400
	ctx.r[4].s64 = ctx.r[11].s64 + -5120;
	// 8327DA80: 386ADC6C  addi r3, r10, -0x2394
	ctx.r[3].s64 = ctx.r[10].s64 + -9108;
	// 8327DA84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DA88: 4AFAF449  bl 0x8222ced0
	ctx.lr = 0x8327DA8C;
	sub_8222CED0(ctx, base);
	// 8327DA8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DA90: 38690768  addi r3, r9, 0x768
	ctx.r[3].s64 = ctx.r[9].s64 + 1896;
	// 8327DA94: 4BA2C48D  bl 0x82ca9f20
	ctx.lr = 0x8327DA98;
	sub_82CA9F20(ctx, base);
	// 8327DA98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DAA8 size=64
    let mut pc: u32 = 0x8327DAA8;
    'dispatch: loop {
        match pc {
            0x8327DAA8 => {
    //   block [0x8327DAA8..0x8327DAE8)
	// 8327DAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DAB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DAB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DABC: 388BEC10  addi r4, r11, -0x13f0
	ctx.r[4].s64 = ctx.r[11].s64 + -5104;
	// 8327DAC0: 386ADC70  addi r3, r10, -0x2390
	ctx.r[3].s64 = ctx.r[10].s64 + -9104;
	// 8327DAC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DAC8: 4AFAF409  bl 0x8222ced0
	ctx.lr = 0x8327DACC;
	sub_8222CED0(ctx, base);
	// 8327DACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DAD0: 38690778  addi r3, r9, 0x778
	ctx.r[3].s64 = ctx.r[9].s64 + 1912;
	// 8327DAD4: 4BA2C44D  bl 0x82ca9f20
	ctx.lr = 0x8327DAD8;
	sub_82CA9F20(ctx, base);
	// 8327DAD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DAE8 size=64
    let mut pc: u32 = 0x8327DAE8;
    'dispatch: loop {
        match pc {
            0x8327DAE8 => {
    //   block [0x8327DAE8..0x8327DB28)
	// 8327DAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DAF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DAF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DAFC: 388BEC20  addi r4, r11, -0x13e0
	ctx.r[4].s64 = ctx.r[11].s64 + -5088;
	// 8327DB00: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 8327DB04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB08: 4AFAF3C9  bl 0x8222ced0
	ctx.lr = 0x8327DB0C;
	sub_8222CED0(ctx, base);
	// 8327DB0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB10: 38690788  addi r3, r9, 0x788
	ctx.r[3].s64 = ctx.r[9].s64 + 1928;
	// 8327DB14: 4BA2C40D  bl 0x82ca9f20
	ctx.lr = 0x8327DB18;
	sub_82CA9F20(ctx, base);
	// 8327DB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DB28 size=64
    let mut pc: u32 = 0x8327DB28;
    'dispatch: loop {
        match pc {
            0x8327DB28 => {
    //   block [0x8327DB28..0x8327DB68)
	// 8327DB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DB34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DB38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DB3C: 388BEC2C  addi r4, r11, -0x13d4
	ctx.r[4].s64 = ctx.r[11].s64 + -5076;
	// 8327DB40: 386ADC78  addi r3, r10, -0x2388
	ctx.r[3].s64 = ctx.r[10].s64 + -9096;
	// 8327DB44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB48: 4AFAF389  bl 0x8222ced0
	ctx.lr = 0x8327DB4C;
	sub_8222CED0(ctx, base);
	// 8327DB4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB50: 38690798  addi r3, r9, 0x798
	ctx.r[3].s64 = ctx.r[9].s64 + 1944;
	// 8327DB54: 4BA2C3CD  bl 0x82ca9f20
	ctx.lr = 0x8327DB58;
	sub_82CA9F20(ctx, base);
	// 8327DB58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DB68 size=64
    let mut pc: u32 = 0x8327DB68;
    'dispatch: loop {
        match pc {
            0x8327DB68 => {
    //   block [0x8327DB68..0x8327DBA8)
	// 8327DB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DB70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DB74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DB7C: 388BEC38  addi r4, r11, -0x13c8
	ctx.r[4].s64 = ctx.r[11].s64 + -5064;
	// 8327DB80: 386ADC7C  addi r3, r10, -0x2384
	ctx.r[3].s64 = ctx.r[10].s64 + -9092;
	// 8327DB84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DB88: 4AFAF349  bl 0x8222ced0
	ctx.lr = 0x8327DB8C;
	sub_8222CED0(ctx, base);
	// 8327DB8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DB90: 386907A8  addi r3, r9, 0x7a8
	ctx.r[3].s64 = ctx.r[9].s64 + 1960;
	// 8327DB94: 4BA2C38D  bl 0x82ca9f20
	ctx.lr = 0x8327DB98;
	sub_82CA9F20(ctx, base);
	// 8327DB98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DBA8 size=64
    let mut pc: u32 = 0x8327DBA8;
    'dispatch: loop {
        match pc {
            0x8327DBA8 => {
    //   block [0x8327DBA8..0x8327DBE8)
	// 8327DBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DBB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DBBC: 388BEC48  addi r4, r11, -0x13b8
	ctx.r[4].s64 = ctx.r[11].s64 + -5048;
	// 8327DBC0: 386ADC80  addi r3, r10, -0x2380
	ctx.r[3].s64 = ctx.r[10].s64 + -9088;
	// 8327DBC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DBC8: 4AFAF309  bl 0x8222ced0
	ctx.lr = 0x8327DBCC;
	sub_8222CED0(ctx, base);
	// 8327DBCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DBD0: 386907B8  addi r3, r9, 0x7b8
	ctx.r[3].s64 = ctx.r[9].s64 + 1976;
	// 8327DBD4: 4BA2C34D  bl 0x82ca9f20
	ctx.lr = 0x8327DBD8;
	sub_82CA9F20(ctx, base);
	// 8327DBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DBE8 size=64
    let mut pc: u32 = 0x8327DBE8;
    'dispatch: loop {
        match pc {
            0x8327DBE8 => {
    //   block [0x8327DBE8..0x8327DC28)
	// 8327DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DBF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DBF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DBFC: 388BEC58  addi r4, r11, -0x13a8
	ctx.r[4].s64 = ctx.r[11].s64 + -5032;
	// 8327DC00: 386ADC84  addi r3, r10, -0x237c
	ctx.r[3].s64 = ctx.r[10].s64 + -9084;
	// 8327DC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC08: 4AFAF2C9  bl 0x8222ced0
	ctx.lr = 0x8327DC0C;
	sub_8222CED0(ctx, base);
	// 8327DC0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC10: 386907C8  addi r3, r9, 0x7c8
	ctx.r[3].s64 = ctx.r[9].s64 + 1992;
	// 8327DC14: 4BA2C30D  bl 0x82ca9f20
	ctx.lr = 0x8327DC18;
	sub_82CA9F20(ctx, base);
	// 8327DC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DC28 size=64
    let mut pc: u32 = 0x8327DC28;
    'dispatch: loop {
        match pc {
            0x8327DC28 => {
    //   block [0x8327DC28..0x8327DC68)
	// 8327DC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DC34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DC38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DC3C: 388BEC68  addi r4, r11, -0x1398
	ctx.r[4].s64 = ctx.r[11].s64 + -5016;
	// 8327DC40: 386ADC88  addi r3, r10, -0x2378
	ctx.r[3].s64 = ctx.r[10].s64 + -9080;
	// 8327DC44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC48: 4AFAF289  bl 0x8222ced0
	ctx.lr = 0x8327DC4C;
	sub_8222CED0(ctx, base);
	// 8327DC4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC50: 386907D8  addi r3, r9, 0x7d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2008;
	// 8327DC54: 4BA2C2CD  bl 0x82ca9f20
	ctx.lr = 0x8327DC58;
	sub_82CA9F20(ctx, base);
	// 8327DC58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DC68 size=64
    let mut pc: u32 = 0x8327DC68;
    'dispatch: loop {
        match pc {
            0x8327DC68 => {
    //   block [0x8327DC68..0x8327DCA8)
	// 8327DC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DC74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DC7C: 388BECA4  addi r4, r11, -0x135c
	ctx.r[4].s64 = ctx.r[11].s64 + -4956;
	// 8327DC80: 386ADC8C  addi r3, r10, -0x2374
	ctx.r[3].s64 = ctx.r[10].s64 + -9076;
	// 8327DC84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DC88: 4AFAF249  bl 0x8222ced0
	ctx.lr = 0x8327DC8C;
	sub_8222CED0(ctx, base);
	// 8327DC8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DC90: 386907E8  addi r3, r9, 0x7e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2024;
	// 8327DC94: 4BA2C28D  bl 0x82ca9f20
	ctx.lr = 0x8327DC98;
	sub_82CA9F20(ctx, base);
	// 8327DC98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DCA8 size=64
    let mut pc: u32 = 0x8327DCA8;
    'dispatch: loop {
        match pc {
            0x8327DCA8 => {
    //   block [0x8327DCA8..0x8327DCE8)
	// 8327DCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DCB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DCB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DCB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DCBC: 388BECB4  addi r4, r11, -0x134c
	ctx.r[4].s64 = ctx.r[11].s64 + -4940;
	// 8327DCC0: 386ADC90  addi r3, r10, -0x2370
	ctx.r[3].s64 = ctx.r[10].s64 + -9072;
	// 8327DCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DCC8: 4AFAF209  bl 0x8222ced0
	ctx.lr = 0x8327DCCC;
	sub_8222CED0(ctx, base);
	// 8327DCCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DCD0: 386907F8  addi r3, r9, 0x7f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2040;
	// 8327DCD4: 4BA2C24D  bl 0x82ca9f20
	ctx.lr = 0x8327DCD8;
	sub_82CA9F20(ctx, base);
	// 8327DCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DCE8 size=64
    let mut pc: u32 = 0x8327DCE8;
    'dispatch: loop {
        match pc {
            0x8327DCE8 => {
    //   block [0x8327DCE8..0x8327DD28)
	// 8327DCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DCF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DCF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DCFC: 388BECC4  addi r4, r11, -0x133c
	ctx.r[4].s64 = ctx.r[11].s64 + -4924;
	// 8327DD00: 386ADC94  addi r3, r10, -0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + -9068;
	// 8327DD04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD08: 4AFAF1C9  bl 0x8222ced0
	ctx.lr = 0x8327DD0C;
	sub_8222CED0(ctx, base);
	// 8327DD0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD10: 38690808  addi r3, r9, 0x808
	ctx.r[3].s64 = ctx.r[9].s64 + 2056;
	// 8327DD14: 4BA2C20D  bl 0x82ca9f20
	ctx.lr = 0x8327DD18;
	sub_82CA9F20(ctx, base);
	// 8327DD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DD28 size=64
    let mut pc: u32 = 0x8327DD28;
    'dispatch: loop {
        match pc {
            0x8327DD28 => {
    //   block [0x8327DD28..0x8327DD68)
	// 8327DD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DD34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DD38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DD3C: 388BECD4  addi r4, r11, -0x132c
	ctx.r[4].s64 = ctx.r[11].s64 + -4908;
	// 8327DD40: 386ADC98  addi r3, r10, -0x2368
	ctx.r[3].s64 = ctx.r[10].s64 + -9064;
	// 8327DD44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD48: 4AFAF189  bl 0x8222ced0
	ctx.lr = 0x8327DD4C;
	sub_8222CED0(ctx, base);
	// 8327DD4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD50: 38690818  addi r3, r9, 0x818
	ctx.r[3].s64 = ctx.r[9].s64 + 2072;
	// 8327DD54: 4BA2C1CD  bl 0x82ca9f20
	ctx.lr = 0x8327DD58;
	sub_82CA9F20(ctx, base);
	// 8327DD58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DD68 size=64
    let mut pc: u32 = 0x8327DD68;
    'dispatch: loop {
        match pc {
            0x8327DD68 => {
    //   block [0x8327DD68..0x8327DDA8)
	// 8327DD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DD70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DD74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DD7C: 388BECE4  addi r4, r11, -0x131c
	ctx.r[4].s64 = ctx.r[11].s64 + -4892;
	// 8327DD80: 386ADC9C  addi r3, r10, -0x2364
	ctx.r[3].s64 = ctx.r[10].s64 + -9060;
	// 8327DD84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DD88: 4AFAF149  bl 0x8222ced0
	ctx.lr = 0x8327DD8C;
	sub_8222CED0(ctx, base);
	// 8327DD8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DD90: 38690828  addi r3, r9, 0x828
	ctx.r[3].s64 = ctx.r[9].s64 + 2088;
	// 8327DD94: 4BA2C18D  bl 0x82ca9f20
	ctx.lr = 0x8327DD98;
	sub_82CA9F20(ctx, base);
	// 8327DD98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DDA8 size=64
    let mut pc: u32 = 0x8327DDA8;
    'dispatch: loop {
        match pc {
            0x8327DDA8 => {
    //   block [0x8327DDA8..0x8327DDE8)
	// 8327DDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DDB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DDB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327DDB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DDBC: 388BECF8  addi r4, r11, -0x1308
	ctx.r[4].s64 = ctx.r[11].s64 + -4872;
	// 8327DDC0: 386ADCA0  addi r3, r10, -0x2360
	ctx.r[3].s64 = ctx.r[10].s64 + -9056;
	// 8327DDC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327DDC8: 4AFAF109  bl 0x8222ced0
	ctx.lr = 0x8327DDCC;
	sub_8222CED0(ctx, base);
	// 8327DDCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327DDD0: 38690838  addi r3, r9, 0x838
	ctx.r[3].s64 = ctx.r[9].s64 + 2104;
	// 8327DDD4: 4BA2C14D  bl 0x82ca9f20
	ctx.lr = 0x8327DDD8;
	sub_82CA9F20(ctx, base);
	// 8327DDD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DDE8 size=56
    let mut pc: u32 = 0x8327DDE8;
    'dispatch: loop {
        match pc {
            0x8327DDE8 => {
    //   block [0x8327DDE8..0x8327DE20)
	// 8327DDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DDF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DDF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DDF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DDFC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327DE00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE04: 4AF75F55  bl 0x821f3d58
	ctx.lr = 0x8327DE08;
	sub_821F3D58(ctx, base);
	// 8327DE08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE0C: 906ADCA4  stw r3, -0x235c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9052 as u32), ctx.r[3].u32 ) };
	// 8327DE10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE20 size=56
    let mut pc: u32 = 0x8327DE20;
    'dispatch: loop {
        match pc {
            0x8327DE20 => {
    //   block [0x8327DE20..0x8327DE58)
	// 8327DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DE30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DE34: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327DE38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE3C: 4AF75F1D  bl 0x821f3d58
	ctx.lr = 0x8327DE40;
	sub_821F3D58(ctx, base);
	// 8327DE40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE44: 906ADCA8  stw r3, -0x2358(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9048 as u32), ctx.r[3].u32 ) };
	// 8327DE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE58 size=56
    let mut pc: u32 = 0x8327DE58;
    'dispatch: loop {
        match pc {
            0x8327DE58 => {
    //   block [0x8327DE58..0x8327DE90)
	// 8327DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DE68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DE6C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327DE70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DE74: 4AF75EE5  bl 0x821f3d58
	ctx.lr = 0x8327DE78;
	sub_821F3D58(ctx, base);
	// 8327DE78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DE7C: 906ADCAC  stw r3, -0x2354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9044 as u32), ctx.r[3].u32 ) };
	// 8327DE80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DE90 size=56
    let mut pc: u32 = 0x8327DE90;
    'dispatch: loop {
        match pc {
            0x8327DE90 => {
    //   block [0x8327DE90..0x8327DEC8)
	// 8327DE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DE9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DEA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DEA4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327DEA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DEAC: 4AF75EAD  bl 0x821f3d58
	ctx.lr = 0x8327DEB0;
	sub_821F3D58(ctx, base);
	// 8327DEB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DEB4: 906ADCB0  stw r3, -0x2350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9040 as u32), ctx.r[3].u32 ) };
	// 8327DEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DEC8 size=56
    let mut pc: u32 = 0x8327DEC8;
    'dispatch: loop {
        match pc {
            0x8327DEC8 => {
    //   block [0x8327DEC8..0x8327DF00)
	// 8327DEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DEDC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327DEE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DEE4: 4AF75E75  bl 0x821f3d58
	ctx.lr = 0x8327DEE8;
	sub_821F3D58(ctx, base);
	// 8327DEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DEEC: 906ADCB4  stw r3, -0x234c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9036 as u32), ctx.r[3].u32 ) };
	// 8327DEF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF00 size=56
    let mut pc: u32 = 0x8327DF00;
    'dispatch: loop {
        match pc {
            0x8327DF00 => {
    //   block [0x8327DF00..0x8327DF38)
	// 8327DF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF14: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327DF18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF1C: 4AF75E3D  bl 0x821f3d58
	ctx.lr = 0x8327DF20;
	sub_821F3D58(ctx, base);
	// 8327DF20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF24: 906ADCB8  stw r3, -0x2348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9032 as u32), ctx.r[3].u32 ) };
	// 8327DF28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF38 size=56
    let mut pc: u32 = 0x8327DF38;
    'dispatch: loop {
        match pc {
            0x8327DF38 => {
    //   block [0x8327DF38..0x8327DF70)
	// 8327DF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF4C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327DF50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF54: 4AF75E05  bl 0x821f3d58
	ctx.lr = 0x8327DF58;
	sub_821F3D58(ctx, base);
	// 8327DF58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF5C: 906ADCBC  stw r3, -0x2344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9028 as u32), ctx.r[3].u32 ) };
	// 8327DF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DF70 size=56
    let mut pc: u32 = 0x8327DF70;
    'dispatch: loop {
        match pc {
            0x8327DF70 => {
    //   block [0x8327DF70..0x8327DFA8)
	// 8327DF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DF7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DF80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DF84: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327DF88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DF8C: 4AF75DCD  bl 0x821f3d58
	ctx.lr = 0x8327DF90;
	sub_821F3D58(ctx, base);
	// 8327DF90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DF94: 906ADCC0  stw r3, -0x2340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9024 as u32), ctx.r[3].u32 ) };
	// 8327DF98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DFA8 size=56
    let mut pc: u32 = 0x8327DFA8;
    'dispatch: loop {
        match pc {
            0x8327DFA8 => {
    //   block [0x8327DFA8..0x8327DFE0)
	// 8327DFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DFB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DFB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DFB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DFBC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327DFC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DFC4: 4AF75D95  bl 0x821f3d58
	ctx.lr = 0x8327DFC8;
	sub_821F3D58(ctx, base);
	// 8327DFC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327DFCC: 906ADCC4  stw r3, -0x233c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9020 as u32), ctx.r[3].u32 ) };
	// 8327DFD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327DFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327DFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327DFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327DFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327DFE0 size=56
    let mut pc: u32 = 0x8327DFE0;
    'dispatch: loop {
        match pc {
            0x8327DFE0 => {
    //   block [0x8327DFE0..0x8327E018)
	// 8327DFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327DFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327DFE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327DFEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327DFF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327DFF4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327DFF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327DFFC: 4AF75D5D  bl 0x821f3d58
	ctx.lr = 0x8327E000;
	sub_821F3D58(ctx, base);
	// 8327E000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E004: 906ADCC8  stw r3, -0x2338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9016 as u32), ctx.r[3].u32 ) };
	// 8327E008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E018 size=56
    let mut pc: u32 = 0x8327E018;
    'dispatch: loop {
        match pc {
            0x8327E018 => {
    //   block [0x8327E018..0x8327E050)
	// 8327E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E024: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E028: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E02C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327E030: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E034: 4AF75D25  bl 0x821f3d58
	ctx.lr = 0x8327E038;
	sub_821F3D58(ctx, base);
	// 8327E038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E03C: 906ADCCC  stw r3, -0x2334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9012 as u32), ctx.r[3].u32 ) };
	// 8327E040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E050 size=56
    let mut pc: u32 = 0x8327E050;
    'dispatch: loop {
        match pc {
            0x8327E050 => {
    //   block [0x8327E050..0x8327E088)
	// 8327E050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E05C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E060: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E064: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327E068: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E06C: 4AF75CED  bl 0x821f3d58
	ctx.lr = 0x8327E070;
	sub_821F3D58(ctx, base);
	// 8327E070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E074: 906ADCD0  stw r3, -0x2330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9008 as u32), ctx.r[3].u32 ) };
	// 8327E078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E088 size=56
    let mut pc: u32 = 0x8327E088;
    'dispatch: loop {
        match pc {
            0x8327E088 => {
    //   block [0x8327E088..0x8327E0C0)
	// 8327E088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E094: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E098: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E09C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327E0A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E0A4: 4AF75CB5  bl 0x821f3d58
	ctx.lr = 0x8327E0A8;
	sub_821F3D58(ctx, base);
	// 8327E0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E0AC: 906ADCD4  stw r3, -0x232c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9004 as u32), ctx.r[3].u32 ) };
	// 8327E0B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E0C0 size=56
    let mut pc: u32 = 0x8327E0C0;
    'dispatch: loop {
        match pc {
            0x8327E0C0 => {
    //   block [0x8327E0C0..0x8327E0F8)
	// 8327E0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E0C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E0CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E0D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E0D4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327E0D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E0DC: 4AF75C7D  bl 0x821f3d58
	ctx.lr = 0x8327E0E0;
	sub_821F3D58(ctx, base);
	// 8327E0E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E0E4: 906ADCD8  stw r3, -0x2328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9000 as u32), ctx.r[3].u32 ) };
	// 8327E0E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E0F8 size=56
    let mut pc: u32 = 0x8327E0F8;
    'dispatch: loop {
        match pc {
            0x8327E0F8 => {
    //   block [0x8327E0F8..0x8327E130)
	// 8327E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E108: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E10C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327E110: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E114: 4AF75C45  bl 0x821f3d58
	ctx.lr = 0x8327E118;
	sub_821F3D58(ctx, base);
	// 8327E118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E11C: 906ADCDC  stw r3, -0x2324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8996 as u32), ctx.r[3].u32 ) };
	// 8327E120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E130 size=56
    let mut pc: u32 = 0x8327E130;
    'dispatch: loop {
        match pc {
            0x8327E130 => {
    //   block [0x8327E130..0x8327E168)
	// 8327E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E13C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E140: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E144: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327E148: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E14C: 4AF75C0D  bl 0x821f3d58
	ctx.lr = 0x8327E150;
	sub_821F3D58(ctx, base);
	// 8327E150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E154: 906ADCE0  stw r3, -0x2320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8992 as u32), ctx.r[3].u32 ) };
	// 8327E158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E168 size=56
    let mut pc: u32 = 0x8327E168;
    'dispatch: loop {
        match pc {
            0x8327E168 => {
    //   block [0x8327E168..0x8327E1A0)
	// 8327E168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E174: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E178: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E17C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327E180: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E184: 4AF75BD5  bl 0x821f3d58
	ctx.lr = 0x8327E188;
	sub_821F3D58(ctx, base);
	// 8327E188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E18C: 906ADCE4  stw r3, -0x231c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8988 as u32), ctx.r[3].u32 ) };
	// 8327E190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E1A0 size=56
    let mut pc: u32 = 0x8327E1A0;
    'dispatch: loop {
        match pc {
            0x8327E1A0 => {
    //   block [0x8327E1A0..0x8327E1D8)
	// 8327E1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E1AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E1B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E1B4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327E1B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E1BC: 4AF75B9D  bl 0x821f3d58
	ctx.lr = 0x8327E1C0;
	sub_821F3D58(ctx, base);
	// 8327E1C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E1C4: 906ADCE8  stw r3, -0x2318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8984 as u32), ctx.r[3].u32 ) };
	// 8327E1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E1D8 size=56
    let mut pc: u32 = 0x8327E1D8;
    'dispatch: loop {
        match pc {
            0x8327E1D8 => {
    //   block [0x8327E1D8..0x8327E210)
	// 8327E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E1E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E1E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E1EC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327E1F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E1F4: 4AF75B65  bl 0x821f3d58
	ctx.lr = 0x8327E1F8;
	sub_821F3D58(ctx, base);
	// 8327E1F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E1FC: 906ADCEC  stw r3, -0x2314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8980 as u32), ctx.r[3].u32 ) };
	// 8327E200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E210 size=56
    let mut pc: u32 = 0x8327E210;
    'dispatch: loop {
        match pc {
            0x8327E210 => {
    //   block [0x8327E210..0x8327E248)
	// 8327E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E21C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E220: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E224: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327E228: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E22C: 4AF75B2D  bl 0x821f3d58
	ctx.lr = 0x8327E230;
	sub_821F3D58(ctx, base);
	// 8327E230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E234: 906ADCF0  stw r3, -0x2310(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8976 as u32), ctx.r[3].u32 ) };
	// 8327E238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E248 size=56
    let mut pc: u32 = 0x8327E248;
    'dispatch: loop {
        match pc {
            0x8327E248 => {
    //   block [0x8327E248..0x8327E280)
	// 8327E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E254: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E25C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327E260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E264: 4AF75AF5  bl 0x821f3d58
	ctx.lr = 0x8327E268;
	sub_821F3D58(ctx, base);
	// 8327E268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E26C: 906ADCF4  stw r3, -0x230c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8972 as u32), ctx.r[3].u32 ) };
	// 8327E270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E280 size=64
    let mut pc: u32 = 0x8327E280;
    'dispatch: loop {
        match pc {
            0x8327E280 => {
    //   block [0x8327E280..0x8327E2C0)
	// 8327E280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E28C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E294: 388BED78  addi r4, r11, -0x1288
	ctx.r[4].s64 = ctx.r[11].s64 + -4744;
	// 8327E298: 386ADCF8  addi r3, r10, -0x2308
	ctx.r[3].s64 = ctx.r[10].s64 + -8968;
	// 8327E29C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E2A0: 4AFAEC31  bl 0x8222ced0
	ctx.lr = 0x8327E2A4;
	sub_8222CED0(ctx, base);
	// 8327E2A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E2A8: 38690848  addi r3, r9, 0x848
	ctx.r[3].s64 = ctx.r[9].s64 + 2120;
	// 8327E2AC: 4BA2BC75  bl 0x82ca9f20
	ctx.lr = 0x8327E2B0;
	sub_82CA9F20(ctx, base);
	// 8327E2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E2C0 size=64
    let mut pc: u32 = 0x8327E2C0;
    'dispatch: loop {
        match pc {
            0x8327E2C0 => {
    //   block [0x8327E2C0..0x8327E300)
	// 8327E2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E2C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E2CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E2D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E2D4: 388BEDB8  addi r4, r11, -0x1248
	ctx.r[4].s64 = ctx.r[11].s64 + -4680;
	// 8327E2D8: 386ADCFC  addi r3, r10, -0x2304
	ctx.r[3].s64 = ctx.r[10].s64 + -8964;
	// 8327E2DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E2E0: 4AFAEBF1  bl 0x8222ced0
	ctx.lr = 0x8327E2E4;
	sub_8222CED0(ctx, base);
	// 8327E2E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E2E8: 38690858  addi r3, r9, 0x858
	ctx.r[3].s64 = ctx.r[9].s64 + 2136;
	// 8327E2EC: 4BA2BC35  bl 0x82ca9f20
	ctx.lr = 0x8327E2F0;
	sub_82CA9F20(ctx, base);
	// 8327E2F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E300 size=64
    let mut pc: u32 = 0x8327E300;
    'dispatch: loop {
        match pc {
            0x8327E300 => {
    //   block [0x8327E300..0x8327E340)
	// 8327E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E30C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E314: 388BEE00  addi r4, r11, -0x1200
	ctx.r[4].s64 = ctx.r[11].s64 + -4608;
	// 8327E318: 386ADD00  addi r3, r10, -0x2300
	ctx.r[3].s64 = ctx.r[10].s64 + -8960;
	// 8327E31C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E320: 4AFAEBB1  bl 0x8222ced0
	ctx.lr = 0x8327E324;
	sub_8222CED0(ctx, base);
	// 8327E324: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E328: 38690868  addi r3, r9, 0x868
	ctx.r[3].s64 = ctx.r[9].s64 + 2152;
	// 8327E32C: 4BA2BBF5  bl 0x82ca9f20
	ctx.lr = 0x8327E330;
	sub_82CA9F20(ctx, base);
	// 8327E330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E340 size=64
    let mut pc: u32 = 0x8327E340;
    'dispatch: loop {
        match pc {
            0x8327E340 => {
    //   block [0x8327E340..0x8327E380)
	// 8327E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E34C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E354: 388BEE4C  addi r4, r11, -0x11b4
	ctx.r[4].s64 = ctx.r[11].s64 + -4532;
	// 8327E358: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 8327E35C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E360: 4AFAEB71  bl 0x8222ced0
	ctx.lr = 0x8327E364;
	sub_8222CED0(ctx, base);
	// 8327E364: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E368: 38690878  addi r3, r9, 0x878
	ctx.r[3].s64 = ctx.r[9].s64 + 2168;
	// 8327E36C: 4BA2BBB5  bl 0x82ca9f20
	ctx.lr = 0x8327E370;
	sub_82CA9F20(ctx, base);
	// 8327E370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E380 size=64
    let mut pc: u32 = 0x8327E380;
    'dispatch: loop {
        match pc {
            0x8327E380 => {
    //   block [0x8327E380..0x8327E3C0)
	// 8327E380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E38C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E394: 388BEE5C  addi r4, r11, -0x11a4
	ctx.r[4].s64 = ctx.r[11].s64 + -4516;
	// 8327E398: 386ADD08  addi r3, r10, -0x22f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8952;
	// 8327E39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E3A0: 4AFAEB31  bl 0x8222ced0
	ctx.lr = 0x8327E3A4;
	sub_8222CED0(ctx, base);
	// 8327E3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E3A8: 38690888  addi r3, r9, 0x888
	ctx.r[3].s64 = ctx.r[9].s64 + 2184;
	// 8327E3AC: 4BA2BB75  bl 0x82ca9f20
	ctx.lr = 0x8327E3B0;
	sub_82CA9F20(ctx, base);
	// 8327E3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E3C0 size=64
    let mut pc: u32 = 0x8327E3C0;
    'dispatch: loop {
        match pc {
            0x8327E3C0 => {
    //   block [0x8327E3C0..0x8327E400)
	// 8327E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E3CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E3D4: 388BEE68  addi r4, r11, -0x1198
	ctx.r[4].s64 = ctx.r[11].s64 + -4504;
	// 8327E3D8: 386ADD0C  addi r3, r10, -0x22f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8948;
	// 8327E3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E3E0: 4AFAEAF1  bl 0x8222ced0
	ctx.lr = 0x8327E3E4;
	sub_8222CED0(ctx, base);
	// 8327E3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E3E8: 38690898  addi r3, r9, 0x898
	ctx.r[3].s64 = ctx.r[9].s64 + 2200;
	// 8327E3EC: 4BA2BB35  bl 0x82ca9f20
	ctx.lr = 0x8327E3F0;
	sub_82CA9F20(ctx, base);
	// 8327E3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E400 size=64
    let mut pc: u32 = 0x8327E400;
    'dispatch: loop {
        match pc {
            0x8327E400 => {
    //   block [0x8327E400..0x8327E440)
	// 8327E400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E40C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E414: 388BEE74  addi r4, r11, -0x118c
	ctx.r[4].s64 = ctx.r[11].s64 + -4492;
	// 8327E418: 386ADD10  addi r3, r10, -0x22f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8944;
	// 8327E41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E420: 4AFAEAB1  bl 0x8222ced0
	ctx.lr = 0x8327E424;
	sub_8222CED0(ctx, base);
	// 8327E424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E428: 386908A8  addi r3, r9, 0x8a8
	ctx.r[3].s64 = ctx.r[9].s64 + 2216;
	// 8327E42C: 4BA2BAF5  bl 0x82ca9f20
	ctx.lr = 0x8327E430;
	sub_82CA9F20(ctx, base);
	// 8327E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E440 size=64
    let mut pc: u32 = 0x8327E440;
    'dispatch: loop {
        match pc {
            0x8327E440 => {
    //   block [0x8327E440..0x8327E480)
	// 8327E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E44C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E454: 388BEE84  addi r4, r11, -0x117c
	ctx.r[4].s64 = ctx.r[11].s64 + -4476;
	// 8327E458: 386ADD14  addi r3, r10, -0x22ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8940;
	// 8327E45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E460: 4AFAEA71  bl 0x8222ced0
	ctx.lr = 0x8327E464;
	sub_8222CED0(ctx, base);
	// 8327E464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E468: 386908B8  addi r3, r9, 0x8b8
	ctx.r[3].s64 = ctx.r[9].s64 + 2232;
	// 8327E46C: 4BA2BAB5  bl 0x82ca9f20
	ctx.lr = 0x8327E470;
	sub_82CA9F20(ctx, base);
	// 8327E470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E480 size=64
    let mut pc: u32 = 0x8327E480;
    'dispatch: loop {
        match pc {
            0x8327E480 => {
    //   block [0x8327E480..0x8327E4C0)
	// 8327E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E48C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E494: 388BEE90  addi r4, r11, -0x1170
	ctx.r[4].s64 = ctx.r[11].s64 + -4464;
	// 8327E498: 386ADD18  addi r3, r10, -0x22e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8936;
	// 8327E49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E4A0: 4AFAEA31  bl 0x8222ced0
	ctx.lr = 0x8327E4A4;
	sub_8222CED0(ctx, base);
	// 8327E4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E4A8: 386908C8  addi r3, r9, 0x8c8
	ctx.r[3].s64 = ctx.r[9].s64 + 2248;
	// 8327E4AC: 4BA2BA75  bl 0x82ca9f20
	ctx.lr = 0x8327E4B0;
	sub_82CA9F20(ctx, base);
	// 8327E4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E4C0 size=64
    let mut pc: u32 = 0x8327E4C0;
    'dispatch: loop {
        match pc {
            0x8327E4C0 => {
    //   block [0x8327E4C0..0x8327E500)
	// 8327E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E4CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E4D4: 388BEE9C  addi r4, r11, -0x1164
	ctx.r[4].s64 = ctx.r[11].s64 + -4452;
	// 8327E4D8: 386ADD1C  addi r3, r10, -0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8932;
	// 8327E4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E4E0: 4AFAE9F1  bl 0x8222ced0
	ctx.lr = 0x8327E4E4;
	sub_8222CED0(ctx, base);
	// 8327E4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E4E8: 386908D8  addi r3, r9, 0x8d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2264;
	// 8327E4EC: 4BA2BA35  bl 0x82ca9f20
	ctx.lr = 0x8327E4F0;
	sub_82CA9F20(ctx, base);
	// 8327E4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E500 size=64
    let mut pc: u32 = 0x8327E500;
    'dispatch: loop {
        match pc {
            0x8327E500 => {
    //   block [0x8327E500..0x8327E540)
	// 8327E500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E50C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E514: 388BEEB4  addi r4, r11, -0x114c
	ctx.r[4].s64 = ctx.r[11].s64 + -4428;
	// 8327E518: 386ADD20  addi r3, r10, -0x22e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8928;
	// 8327E51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E520: 4AFAE9B1  bl 0x8222ced0
	ctx.lr = 0x8327E524;
	sub_8222CED0(ctx, base);
	// 8327E524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E528: 386908E8  addi r3, r9, 0x8e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2280;
	// 8327E52C: 4BA2B9F5  bl 0x82ca9f20
	ctx.lr = 0x8327E530;
	sub_82CA9F20(ctx, base);
	// 8327E530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E540 size=64
    let mut pc: u32 = 0x8327E540;
    'dispatch: loop {
        match pc {
            0x8327E540 => {
    //   block [0x8327E540..0x8327E580)
	// 8327E540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E54C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E554: 388BEEC0  addi r4, r11, -0x1140
	ctx.r[4].s64 = ctx.r[11].s64 + -4416;
	// 8327E558: 386ADD24  addi r3, r10, -0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8924;
	// 8327E55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E560: 4AFAE971  bl 0x8222ced0
	ctx.lr = 0x8327E564;
	sub_8222CED0(ctx, base);
	// 8327E564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E568: 386908F8  addi r3, r9, 0x8f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2296;
	// 8327E56C: 4BA2B9B5  bl 0x82ca9f20
	ctx.lr = 0x8327E570;
	sub_82CA9F20(ctx, base);
	// 8327E570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E580 size=64
    let mut pc: u32 = 0x8327E580;
    'dispatch: loop {
        match pc {
            0x8327E580 => {
    //   block [0x8327E580..0x8327E5C0)
	// 8327E580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E58C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E594: 388BEED0  addi r4, r11, -0x1130
	ctx.r[4].s64 = ctx.r[11].s64 + -4400;
	// 8327E598: 386ADD28  addi r3, r10, -0x22d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8920;
	// 8327E59C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E5A0: 4AFAE931  bl 0x8222ced0
	ctx.lr = 0x8327E5A4;
	sub_8222CED0(ctx, base);
	// 8327E5A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E5A8: 38690908  addi r3, r9, 0x908
	ctx.r[3].s64 = ctx.r[9].s64 + 2312;
	// 8327E5AC: 4BA2B975  bl 0x82ca9f20
	ctx.lr = 0x8327E5B0;
	sub_82CA9F20(ctx, base);
	// 8327E5B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E5C0 size=64
    let mut pc: u32 = 0x8327E5C0;
    'dispatch: loop {
        match pc {
            0x8327E5C0 => {
    //   block [0x8327E5C0..0x8327E600)
	// 8327E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E5CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327E5D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E5D4: 388BEEDC  addi r4, r11, -0x1124
	ctx.r[4].s64 = ctx.r[11].s64 + -4388;
	// 8327E5D8: 386ADD2C  addi r3, r10, -0x22d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8916;
	// 8327E5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327E5E0: 4AFAE8F1  bl 0x8222ced0
	ctx.lr = 0x8327E5E4;
	sub_8222CED0(ctx, base);
	// 8327E5E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327E5E8: 38690918  addi r3, r9, 0x918
	ctx.r[3].s64 = ctx.r[9].s64 + 2328;
	// 8327E5EC: 4BA2B935  bl 0x82ca9f20
	ctx.lr = 0x8327E5F0;
	sub_82CA9F20(ctx, base);
	// 8327E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E600 size=56
    let mut pc: u32 = 0x8327E600;
    'dispatch: loop {
        match pc {
            0x8327E600 => {
    //   block [0x8327E600..0x8327E638)
	// 8327E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E614: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8327E618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E61C: 4AF7573D  bl 0x821f3d58
	ctx.lr = 0x8327E620;
	sub_821F3D58(ctx, base);
	// 8327E620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E624: 906ADD30  stw r3, -0x22d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8912 as u32), ctx.r[3].u32 ) };
	// 8327E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E638 size=56
    let mut pc: u32 = 0x8327E638;
    'dispatch: loop {
        match pc {
            0x8327E638 => {
    //   block [0x8327E638..0x8327E670)
	// 8327E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E64C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8327E650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E654: 4AF75705  bl 0x821f3d58
	ctx.lr = 0x8327E658;
	sub_821F3D58(ctx, base);
	// 8327E658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E65C: 906ADD34  stw r3, -0x22cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8908 as u32), ctx.r[3].u32 ) };
	// 8327E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E670 size=56
    let mut pc: u32 = 0x8327E670;
    'dispatch: loop {
        match pc {
            0x8327E670 => {
    //   block [0x8327E670..0x8327E6A8)
	// 8327E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E684: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8327E688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E68C: 4AF756CD  bl 0x821f3d58
	ctx.lr = 0x8327E690;
	sub_821F3D58(ctx, base);
	// 8327E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E694: 906ADD38  stw r3, -0x22c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8904 as u32), ctx.r[3].u32 ) };
	// 8327E698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E6A8 size=56
    let mut pc: u32 = 0x8327E6A8;
    'dispatch: loop {
        match pc {
            0x8327E6A8 => {
    //   block [0x8327E6A8..0x8327E6E0)
	// 8327E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E6BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8327E6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E6C4: 4AF75695  bl 0x821f3d58
	ctx.lr = 0x8327E6C8;
	sub_821F3D58(ctx, base);
	// 8327E6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E6CC: 906ADD3C  stw r3, -0x22c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8900 as u32), ctx.r[3].u32 ) };
	// 8327E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E6E0 size=56
    let mut pc: u32 = 0x8327E6E0;
    'dispatch: loop {
        match pc {
            0x8327E6E0 => {
    //   block [0x8327E6E0..0x8327E718)
	// 8327E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E6F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8327E6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E6FC: 4AF7565D  bl 0x821f3d58
	ctx.lr = 0x8327E700;
	sub_821F3D58(ctx, base);
	// 8327E700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E704: 906ADD40  stw r3, -0x22c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8896 as u32), ctx.r[3].u32 ) };
	// 8327E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E718 size=56
    let mut pc: u32 = 0x8327E718;
    'dispatch: loop {
        match pc {
            0x8327E718 => {
    //   block [0x8327E718..0x8327E750)
	// 8327E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E72C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8327E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E734: 4AF75625  bl 0x821f3d58
	ctx.lr = 0x8327E738;
	sub_821F3D58(ctx, base);
	// 8327E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E73C: 906ADD44  stw r3, -0x22bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8892 as u32), ctx.r[3].u32 ) };
	// 8327E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E750 size=56
    let mut pc: u32 = 0x8327E750;
    'dispatch: loop {
        match pc {
            0x8327E750 => {
    //   block [0x8327E750..0x8327E788)
	// 8327E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E764: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8327E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E76C: 4AF755ED  bl 0x821f3d58
	ctx.lr = 0x8327E770;
	sub_821F3D58(ctx, base);
	// 8327E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E774: 906ADD48  stw r3, -0x22b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8888 as u32), ctx.r[3].u32 ) };
	// 8327E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E788 size=56
    let mut pc: u32 = 0x8327E788;
    'dispatch: loop {
        match pc {
            0x8327E788 => {
    //   block [0x8327E788..0x8327E7C0)
	// 8327E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E79C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8327E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E7A4: 4AF755B5  bl 0x821f3d58
	ctx.lr = 0x8327E7A8;
	sub_821F3D58(ctx, base);
	// 8327E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E7AC: 906ADD4C  stw r3, -0x22b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8884 as u32), ctx.r[3].u32 ) };
	// 8327E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E7C0 size=56
    let mut pc: u32 = 0x8327E7C0;
    'dispatch: loop {
        match pc {
            0x8327E7C0 => {
    //   block [0x8327E7C0..0x8327E7F8)
	// 8327E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E7D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8327E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E7DC: 4AF7557D  bl 0x821f3d58
	ctx.lr = 0x8327E7E0;
	sub_821F3D58(ctx, base);
	// 8327E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E7E4: 906ADD50  stw r3, -0x22b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8880 as u32), ctx.r[3].u32 ) };
	// 8327E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E7F8 size=56
    let mut pc: u32 = 0x8327E7F8;
    'dispatch: loop {
        match pc {
            0x8327E7F8 => {
    //   block [0x8327E7F8..0x8327E830)
	// 8327E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E80C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8327E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E814: 4AF75545  bl 0x821f3d58
	ctx.lr = 0x8327E818;
	sub_821F3D58(ctx, base);
	// 8327E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E81C: 906ADD54  stw r3, -0x22ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8876 as u32), ctx.r[3].u32 ) };
	// 8327E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E830 size=56
    let mut pc: u32 = 0x8327E830;
    'dispatch: loop {
        match pc {
            0x8327E830 => {
    //   block [0x8327E830..0x8327E868)
	// 8327E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E844: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8327E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E84C: 4AF7550D  bl 0x821f3d58
	ctx.lr = 0x8327E850;
	sub_821F3D58(ctx, base);
	// 8327E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E854: 906ADD58  stw r3, -0x22a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8872 as u32), ctx.r[3].u32 ) };
	// 8327E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E868 size=56
    let mut pc: u32 = 0x8327E868;
    'dispatch: loop {
        match pc {
            0x8327E868 => {
    //   block [0x8327E868..0x8327E8A0)
	// 8327E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E87C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8327E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E884: 4AF754D5  bl 0x821f3d58
	ctx.lr = 0x8327E888;
	sub_821F3D58(ctx, base);
	// 8327E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E88C: 906ADD5C  stw r3, -0x22a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8868 as u32), ctx.r[3].u32 ) };
	// 8327E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E8A0 size=56
    let mut pc: u32 = 0x8327E8A0;
    'dispatch: loop {
        match pc {
            0x8327E8A0 => {
    //   block [0x8327E8A0..0x8327E8D8)
	// 8327E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E8B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8327E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E8BC: 4AF7549D  bl 0x821f3d58
	ctx.lr = 0x8327E8C0;
	sub_821F3D58(ctx, base);
	// 8327E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E8C4: 906ADD60  stw r3, -0x22a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8864 as u32), ctx.r[3].u32 ) };
	// 8327E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E8D8 size=56
    let mut pc: u32 = 0x8327E8D8;
    'dispatch: loop {
        match pc {
            0x8327E8D8 => {
    //   block [0x8327E8D8..0x8327E910)
	// 8327E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E8EC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8327E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E8F4: 4AF75465  bl 0x821f3d58
	ctx.lr = 0x8327E8F8;
	sub_821F3D58(ctx, base);
	// 8327E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E8FC: 906ADD64  stw r3, -0x229c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8860 as u32), ctx.r[3].u32 ) };
	// 8327E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E910 size=56
    let mut pc: u32 = 0x8327E910;
    'dispatch: loop {
        match pc {
            0x8327E910 => {
    //   block [0x8327E910..0x8327E948)
	// 8327E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E924: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8327E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E92C: 4AF7542D  bl 0x821f3d58
	ctx.lr = 0x8327E930;
	sub_821F3D58(ctx, base);
	// 8327E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E934: 906ADD68  stw r3, -0x2298(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8856 as u32), ctx.r[3].u32 ) };
	// 8327E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E948 size=56
    let mut pc: u32 = 0x8327E948;
    'dispatch: loop {
        match pc {
            0x8327E948 => {
    //   block [0x8327E948..0x8327E980)
	// 8327E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E95C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8327E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E964: 4AF753F5  bl 0x821f3d58
	ctx.lr = 0x8327E968;
	sub_821F3D58(ctx, base);
	// 8327E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E96C: 906ADD6C  stw r3, -0x2294(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8852 as u32), ctx.r[3].u32 ) };
	// 8327E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E980 size=56
    let mut pc: u32 = 0x8327E980;
    'dispatch: loop {
        match pc {
            0x8327E980 => {
    //   block [0x8327E980..0x8327E9B8)
	// 8327E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E98C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E994: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8327E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E99C: 4AF753BD  bl 0x821f3d58
	ctx.lr = 0x8327E9A0;
	sub_821F3D58(ctx, base);
	// 8327E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E9A4: 906ADD70  stw r3, -0x2290(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8848 as u32), ctx.r[3].u32 ) };
	// 8327E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E9B8 size=56
    let mut pc: u32 = 0x8327E9B8;
    'dispatch: loop {
        match pc {
            0x8327E9B8 => {
    //   block [0x8327E9B8..0x8327E9F0)
	// 8327E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E9C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327E9CC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8327E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327E9D4: 4AF75385  bl 0x821f3d58
	ctx.lr = 0x8327E9D8;
	sub_821F3D58(ctx, base);
	// 8327E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327E9DC: 906ADD74  stw r3, -0x228c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8844 as u32), ctx.r[3].u32 ) };
	// 8327E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327E9F0 size=56
    let mut pc: u32 = 0x8327E9F0;
    'dispatch: loop {
        match pc {
            0x8327E9F0 => {
    //   block [0x8327E9F0..0x8327EA28)
	// 8327E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA04: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8327EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA0C: 4AF7534D  bl 0x821f3d58
	ctx.lr = 0x8327EA10;
	sub_821F3D58(ctx, base);
	// 8327EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA14: 906ADD78  stw r3, -0x2288(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8840 as u32), ctx.r[3].u32 ) };
	// 8327EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA28 size=56
    let mut pc: u32 = 0x8327EA28;
    'dispatch: loop {
        match pc {
            0x8327EA28 => {
    //   block [0x8327EA28..0x8327EA60)
	// 8327EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EA34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA3C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8327EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA44: 4AF75315  bl 0x821f3d58
	ctx.lr = 0x8327EA48;
	sub_821F3D58(ctx, base);
	// 8327EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA4C: 906ADD7C  stw r3, -0x2284(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8836 as u32), ctx.r[3].u32 ) };
	// 8327EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA60 size=56
    let mut pc: u32 = 0x8327EA60;
    'dispatch: loop {
        match pc {
            0x8327EA60 => {
    //   block [0x8327EA60..0x8327EA98)
	// 8327EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EA6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8327EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327EA74: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8327EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327EA7C: 4AF752DD  bl 0x821f3d58
	ctx.lr = 0x8327EA80;
	sub_821F3D58(ctx, base);
	// 8327EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EA84: 906ADD80  stw r3, -0x2280(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8832 as u32), ctx.r[3].u32 ) };
	// 8327EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EA98 size=64
    let mut pc: u32 = 0x8327EA98;
    'dispatch: loop {
        match pc {
            0x8327EA98 => {
    //   block [0x8327EA98..0x8327EAD8)
	// 8327EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EAA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EAAC: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 8327EAB0: 386ADD84  addi r3, r10, -0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + -8828;
	// 8327EAB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EAB8: 4AFAE419  bl 0x8222ced0
	ctx.lr = 0x8327EABC;
	sub_8222CED0(ctx, base);
	// 8327EABC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EAC0: 38690928  addi r3, r9, 0x928
	ctx.r[3].s64 = ctx.r[9].s64 + 2344;
	// 8327EAC4: 4BA2B45D  bl 0x82ca9f20
	ctx.lr = 0x8327EAC8;
	sub_82CA9F20(ctx, base);
	// 8327EAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EAD8 size=64
    let mut pc: u32 = 0x8327EAD8;
    'dispatch: loop {
        match pc {
            0x8327EAD8 => {
    //   block [0x8327EAD8..0x8327EB18)
	// 8327EAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EAE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EAE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EAEC: 388BEF10  addi r4, r11, -0x10f0
	ctx.r[4].s64 = ctx.r[11].s64 + -4336;
	// 8327EAF0: 386ADD88  addi r3, r10, -0x2278
	ctx.r[3].s64 = ctx.r[10].s64 + -8824;
	// 8327EAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EAF8: 4AFAE3D9  bl 0x8222ced0
	ctx.lr = 0x8327EAFC;
	sub_8222CED0(ctx, base);
	// 8327EAFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB00: 38690938  addi r3, r9, 0x938
	ctx.r[3].s64 = ctx.r[9].s64 + 2360;
	// 8327EB04: 4BA2B41D  bl 0x82ca9f20
	ctx.lr = 0x8327EB08;
	sub_82CA9F20(ctx, base);
	// 8327EB08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB18 size=64
    let mut pc: u32 = 0x8327EB18;
    'dispatch: loop {
        match pc {
            0x8327EB18 => {
    //   block [0x8327EB18..0x8327EB58)
	// 8327EB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EB20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EB24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EB2C: 388BEF48  addi r4, r11, -0x10b8
	ctx.r[4].s64 = ctx.r[11].s64 + -4280;
	// 8327EB30: 386ADD8C  addi r3, r10, -0x2274
	ctx.r[3].s64 = ctx.r[10].s64 + -8820;
	// 8327EB34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EB38: 4AFAE399  bl 0x8222ced0
	ctx.lr = 0x8327EB3C;
	sub_8222CED0(ctx, base);
	// 8327EB3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB40: 38690948  addi r3, r9, 0x948
	ctx.r[3].s64 = ctx.r[9].s64 + 2376;
	// 8327EB44: 4BA2B3DD  bl 0x82ca9f20
	ctx.lr = 0x8327EB48;
	sub_82CA9F20(ctx, base);
	// 8327EB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB58 size=64
    let mut pc: u32 = 0x8327EB58;
    'dispatch: loop {
        match pc {
            0x8327EB58 => {
    //   block [0x8327EB58..0x8327EB98)
	// 8327EB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EB64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EB6C: 388BEF80  addi r4, r11, -0x1080
	ctx.r[4].s64 = ctx.r[11].s64 + -4224;
	// 8327EB70: 386ADD90  addi r3, r10, -0x2270
	ctx.r[3].s64 = ctx.r[10].s64 + -8816;
	// 8327EB74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EB78: 4AFAE359  bl 0x8222ced0
	ctx.lr = 0x8327EB7C;
	sub_8222CED0(ctx, base);
	// 8327EB7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EB80: 38690958  addi r3, r9, 0x958
	ctx.r[3].s64 = ctx.r[9].s64 + 2392;
	// 8327EB84: 4BA2B39D  bl 0x82ca9f20
	ctx.lr = 0x8327EB88;
	sub_82CA9F20(ctx, base);
	// 8327EB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EB98 size=64
    let mut pc: u32 = 0x8327EB98;
    'dispatch: loop {
        match pc {
            0x8327EB98 => {
    //   block [0x8327EB98..0x8327EBD8)
	// 8327EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EBA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EBA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EBAC: 388BEF88  addi r4, r11, -0x1078
	ctx.r[4].s64 = ctx.r[11].s64 + -4216;
	// 8327EBB0: 386ADD94  addi r3, r10, -0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + -8812;
	// 8327EBB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EBB8: 4AFAE319  bl 0x8222ced0
	ctx.lr = 0x8327EBBC;
	sub_8222CED0(ctx, base);
	// 8327EBBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EBC0: 38690968  addi r3, r9, 0x968
	ctx.r[3].s64 = ctx.r[9].s64 + 2408;
	// 8327EBC4: 4BA2B35D  bl 0x82ca9f20
	ctx.lr = 0x8327EBC8;
	sub_82CA9F20(ctx, base);
	// 8327EBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EBD8 size=64
    let mut pc: u32 = 0x8327EBD8;
    'dispatch: loop {
        match pc {
            0x8327EBD8 => {
    //   block [0x8327EBD8..0x8327EC18)
	// 8327EBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EBE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EBE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EBEC: 388BEF90  addi r4, r11, -0x1070
	ctx.r[4].s64 = ctx.r[11].s64 + -4208;
	// 8327EBF0: 386ADD98  addi r3, r10, -0x2268
	ctx.r[3].s64 = ctx.r[10].s64 + -8808;
	// 8327EBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EBF8: 4AFAE2D9  bl 0x8222ced0
	ctx.lr = 0x8327EBFC;
	sub_8222CED0(ctx, base);
	// 8327EBFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC00: 38690978  addi r3, r9, 0x978
	ctx.r[3].s64 = ctx.r[9].s64 + 2424;
	// 8327EC04: 4BA2B31D  bl 0x82ca9f20
	ctx.lr = 0x8327EC08;
	sub_82CA9F20(ctx, base);
	// 8327EC08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC18 size=64
    let mut pc: u32 = 0x8327EC18;
    'dispatch: loop {
        match pc {
            0x8327EC18 => {
    //   block [0x8327EC18..0x8327EC58)
	// 8327EC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EC24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EC2C: 388BEF98  addi r4, r11, -0x1068
	ctx.r[4].s64 = ctx.r[11].s64 + -4200;
	// 8327EC30: 386ADD9C  addi r3, r10, -0x2264
	ctx.r[3].s64 = ctx.r[10].s64 + -8804;
	// 8327EC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EC38: 4AFAE299  bl 0x8222ced0
	ctx.lr = 0x8327EC3C;
	sub_8222CED0(ctx, base);
	// 8327EC3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC40: 38690988  addi r3, r9, 0x988
	ctx.r[3].s64 = ctx.r[9].s64 + 2440;
	// 8327EC44: 4BA2B2DD  bl 0x82ca9f20
	ctx.lr = 0x8327EC48;
	sub_82CA9F20(ctx, base);
	// 8327EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC58 size=64
    let mut pc: u32 = 0x8327EC58;
    'dispatch: loop {
        match pc {
            0x8327EC58 => {
    //   block [0x8327EC58..0x8327EC98)
	// 8327EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EC68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EC6C: 388BEFA4  addi r4, r11, -0x105c
	ctx.r[4].s64 = ctx.r[11].s64 + -4188;
	// 8327EC70: 386ADDA0  addi r3, r10, -0x2260
	ctx.r[3].s64 = ctx.r[10].s64 + -8800;
	// 8327EC74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EC78: 4AFAE259  bl 0x8222ced0
	ctx.lr = 0x8327EC7C;
	sub_8222CED0(ctx, base);
	// 8327EC7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EC80: 38690998  addi r3, r9, 0x998
	ctx.r[3].s64 = ctx.r[9].s64 + 2456;
	// 8327EC84: 4BA2B29D  bl 0x82ca9f20
	ctx.lr = 0x8327EC88;
	sub_82CA9F20(ctx, base);
	// 8327EC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EC98 size=64
    let mut pc: u32 = 0x8327EC98;
    'dispatch: loop {
        match pc {
            0x8327EC98 => {
    //   block [0x8327EC98..0x8327ECD8)
	// 8327EC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ECA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ECA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ECA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ECAC: 388BEFAC  addi r4, r11, -0x1054
	ctx.r[4].s64 = ctx.r[11].s64 + -4180;
	// 8327ECB0: 386ADDA4  addi r3, r10, -0x225c
	ctx.r[3].s64 = ctx.r[10].s64 + -8796;
	// 8327ECB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ECB8: 4AFAE219  bl 0x8222ced0
	ctx.lr = 0x8327ECBC;
	sub_8222CED0(ctx, base);
	// 8327ECBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ECC0: 386909A8  addi r3, r9, 0x9a8
	ctx.r[3].s64 = ctx.r[9].s64 + 2472;
	// 8327ECC4: 4BA2B25D  bl 0x82ca9f20
	ctx.lr = 0x8327ECC8;
	sub_82CA9F20(ctx, base);
	// 8327ECC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ECCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ECD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ECD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ECD8 size=64
    let mut pc: u32 = 0x8327ECD8;
    'dispatch: loop {
        match pc {
            0x8327ECD8 => {
    //   block [0x8327ECD8..0x8327ED18)
	// 8327ECD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ECDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ECE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ECE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8327ECE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ECEC: 388B8F6C  addi r4, r11, -0x7094
	ctx.r[4].s64 = ctx.r[11].s64 + -28820;
	// 8327ECF0: 386ADDA8  addi r3, r10, -0x2258
	ctx.r[3].s64 = ctx.r[10].s64 + -8792;
	// 8327ECF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ECF8: 4AFAE1D9  bl 0x8222ced0
	ctx.lr = 0x8327ECFC;
	sub_8222CED0(ctx, base);
	// 8327ECFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED00: 386909B8  addi r3, r9, 0x9b8
	ctx.r[3].s64 = ctx.r[9].s64 + 2488;
	// 8327ED04: 4BA2B21D  bl 0x82ca9f20
	ctx.lr = 0x8327ED08;
	sub_82CA9F20(ctx, base);
	// 8327ED08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED18 size=64
    let mut pc: u32 = 0x8327ED18;
    'dispatch: loop {
        match pc {
            0x8327ED18 => {
    //   block [0x8327ED18..0x8327ED58)
	// 8327ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ED20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ED24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ED28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ED2C: 388BEFBC  addi r4, r11, -0x1044
	ctx.r[4].s64 = ctx.r[11].s64 + -4164;
	// 8327ED30: 386ADDAC  addi r3, r10, -0x2254
	ctx.r[3].s64 = ctx.r[10].s64 + -8788;
	// 8327ED34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ED38: 4AFAE199  bl 0x8222ced0
	ctx.lr = 0x8327ED3C;
	sub_8222CED0(ctx, base);
	// 8327ED3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED40: 386909C8  addi r3, r9, 0x9c8
	ctx.r[3].s64 = ctx.r[9].s64 + 2504;
	// 8327ED44: 4BA2B1DD  bl 0x82ca9f20
	ctx.lr = 0x8327ED48;
	sub_82CA9F20(ctx, base);
	// 8327ED48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED58 size=64
    let mut pc: u32 = 0x8327ED58;
    'dispatch: loop {
        match pc {
            0x8327ED58 => {
    //   block [0x8327ED58..0x8327ED98)
	// 8327ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327ED60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327ED64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327ED68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327ED6C: 388BEFC4  addi r4, r11, -0x103c
	ctx.r[4].s64 = ctx.r[11].s64 + -4156;
	// 8327ED70: 386ADDB0  addi r3, r10, -0x2250
	ctx.r[3].s64 = ctx.r[10].s64 + -8784;
	// 8327ED74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327ED78: 4AFAE159  bl 0x8222ced0
	ctx.lr = 0x8327ED7C;
	sub_8222CED0(ctx, base);
	// 8327ED7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327ED80: 386909D8  addi r3, r9, 0x9d8
	ctx.r[3].s64 = ctx.r[9].s64 + 2520;
	// 8327ED84: 4BA2B19D  bl 0x82ca9f20
	ctx.lr = 0x8327ED88;
	sub_82CA9F20(ctx, base);
	// 8327ED88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327ED8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327ED90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327ED98 size=64
    let mut pc: u32 = 0x8327ED98;
    'dispatch: loop {
        match pc {
            0x8327ED98 => {
    //   block [0x8327ED98..0x8327EDD8)
	// 8327ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EDA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EDAC: 388BEFD8  addi r4, r11, -0x1028
	ctx.r[4].s64 = ctx.r[11].s64 + -4136;
	// 8327EDB0: 386ADDB4  addi r3, r10, -0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + -8780;
	// 8327EDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EDB8: 4AFAE119  bl 0x8222ced0
	ctx.lr = 0x8327EDBC;
	sub_8222CED0(ctx, base);
	// 8327EDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EDC0: 386909E8  addi r3, r9, 0x9e8
	ctx.r[3].s64 = ctx.r[9].s64 + 2536;
	// 8327EDC4: 4BA2B15D  bl 0x82ca9f20
	ctx.lr = 0x8327EDC8;
	sub_82CA9F20(ctx, base);
	// 8327EDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EDD8 size=64
    let mut pc: u32 = 0x8327EDD8;
    'dispatch: loop {
        match pc {
            0x8327EDD8 => {
    //   block [0x8327EDD8..0x8327EE18)
	// 8327EDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EDE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EDEC: 388BEFEC  addi r4, r11, -0x1014
	ctx.r[4].s64 = ctx.r[11].s64 + -4116;
	// 8327EDF0: 386ADDB8  addi r3, r10, -0x2248
	ctx.r[3].s64 = ctx.r[10].s64 + -8776;
	// 8327EDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EDF8: 4AFAE0D9  bl 0x8222ced0
	ctx.lr = 0x8327EDFC;
	sub_8222CED0(ctx, base);
	// 8327EDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE00: 386909F8  addi r3, r9, 0x9f8
	ctx.r[3].s64 = ctx.r[9].s64 + 2552;
	// 8327EE04: 4BA2B11D  bl 0x82ca9f20
	ctx.lr = 0x8327EE08;
	sub_82CA9F20(ctx, base);
	// 8327EE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE18 size=64
    let mut pc: u32 = 0x8327EE18;
    'dispatch: loop {
        match pc {
            0x8327EE18 => {
    //   block [0x8327EE18..0x8327EE58)
	// 8327EE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EE24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EE2C: 388BF004  addi r4, r11, -0xffc
	ctx.r[4].s64 = ctx.r[11].s64 + -4092;
	// 8327EE30: 386ADDBC  addi r3, r10, -0x2244
	ctx.r[3].s64 = ctx.r[10].s64 + -8772;
	// 8327EE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EE38: 4AFAE099  bl 0x8222ced0
	ctx.lr = 0x8327EE3C;
	sub_8222CED0(ctx, base);
	// 8327EE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE40: 38690A08  addi r3, r9, 0xa08
	ctx.r[3].s64 = ctx.r[9].s64 + 2568;
	// 8327EE44: 4BA2B0DD  bl 0x82ca9f20
	ctx.lr = 0x8327EE48;
	sub_82CA9F20(ctx, base);
	// 8327EE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE58 size=64
    let mut pc: u32 = 0x8327EE58;
    'dispatch: loop {
        match pc {
            0x8327EE58 => {
    //   block [0x8327EE58..0x8327EE98)
	// 8327EE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EE64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EE6C: 388BF014  addi r4, r11, -0xfec
	ctx.r[4].s64 = ctx.r[11].s64 + -4076;
	// 8327EE70: 386ADDC0  addi r3, r10, -0x2240
	ctx.r[3].s64 = ctx.r[10].s64 + -8768;
	// 8327EE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EE78: 4AFAE059  bl 0x8222ced0
	ctx.lr = 0x8327EE7C;
	sub_8222CED0(ctx, base);
	// 8327EE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EE80: 38690A18  addi r3, r9, 0xa18
	ctx.r[3].s64 = ctx.r[9].s64 + 2584;
	// 8327EE84: 4BA2B09D  bl 0x82ca9f20
	ctx.lr = 0x8327EE88;
	sub_82CA9F20(ctx, base);
	// 8327EE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EE98 size=64
    let mut pc: u32 = 0x8327EE98;
    'dispatch: loop {
        match pc {
            0x8327EE98 => {
    //   block [0x8327EE98..0x8327EED8)
	// 8327EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EEA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EEAC: 388BF028  addi r4, r11, -0xfd8
	ctx.r[4].s64 = ctx.r[11].s64 + -4056;
	// 8327EEB0: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 8327EEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EEB8: 4AFAE019  bl 0x8222ced0
	ctx.lr = 0x8327EEBC;
	sub_8222CED0(ctx, base);
	// 8327EEBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EEC0: 38690A28  addi r3, r9, 0xa28
	ctx.r[3].s64 = ctx.r[9].s64 + 2600;
	// 8327EEC4: 4BA2B05D  bl 0x82ca9f20
	ctx.lr = 0x8327EEC8;
	sub_82CA9F20(ctx, base);
	// 8327EEC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EED8 size=64
    let mut pc: u32 = 0x8327EED8;
    'dispatch: loop {
        match pc {
            0x8327EED8 => {
    //   block [0x8327EED8..0x8327EF18)
	// 8327EED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EEE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EEE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EEEC: 388BF040  addi r4, r11, -0xfc0
	ctx.r[4].s64 = ctx.r[11].s64 + -4032;
	// 8327EEF0: 386ADDC8  addi r3, r10, -0x2238
	ctx.r[3].s64 = ctx.r[10].s64 + -8760;
	// 8327EEF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EEF8: 4AFADFD9  bl 0x8222ced0
	ctx.lr = 0x8327EEFC;
	sub_8222CED0(ctx, base);
	// 8327EEFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF00: 38690A38  addi r3, r9, 0xa38
	ctx.r[3].s64 = ctx.r[9].s64 + 2616;
	// 8327EF04: 4BA2B01D  bl 0x82ca9f20
	ctx.lr = 0x8327EF08;
	sub_82CA9F20(ctx, base);
	// 8327EF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF18 size=64
    let mut pc: u32 = 0x8327EF18;
    'dispatch: loop {
        match pc {
            0x8327EF18 => {
    //   block [0x8327EF18..0x8327EF58)
	// 8327EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EF24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EF2C: 388BF054  addi r4, r11, -0xfac
	ctx.r[4].s64 = ctx.r[11].s64 + -4012;
	// 8327EF30: 386ADDCC  addi r3, r10, -0x2234
	ctx.r[3].s64 = ctx.r[10].s64 + -8756;
	// 8327EF34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EF38: 4AFADF99  bl 0x8222ced0
	ctx.lr = 0x8327EF3C;
	sub_8222CED0(ctx, base);
	// 8327EF3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF40: 38690A48  addi r3, r9, 0xa48
	ctx.r[3].s64 = ctx.r[9].s64 + 2632;
	// 8327EF44: 4BA2AFDD  bl 0x82ca9f20
	ctx.lr = 0x8327EF48;
	sub_82CA9F20(ctx, base);
	// 8327EF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF58 size=64
    let mut pc: u32 = 0x8327EF58;
    'dispatch: loop {
        match pc {
            0x8327EF58 => {
    //   block [0x8327EF58..0x8327EF98)
	// 8327EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EF64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EF6C: 388BF064  addi r4, r11, -0xf9c
	ctx.r[4].s64 = ctx.r[11].s64 + -3996;
	// 8327EF70: 386ADDD0  addi r3, r10, -0x2230
	ctx.r[3].s64 = ctx.r[10].s64 + -8752;
	// 8327EF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EF78: 4AFADF59  bl 0x8222ced0
	ctx.lr = 0x8327EF7C;
	sub_8222CED0(ctx, base);
	// 8327EF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EF80: 38690A58  addi r3, r9, 0xa58
	ctx.r[3].s64 = ctx.r[9].s64 + 2648;
	// 8327EF84: 4BA2AF9D  bl 0x82ca9f20
	ctx.lr = 0x8327EF88;
	sub_82CA9F20(ctx, base);
	// 8327EF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EF98 size=64
    let mut pc: u32 = 0x8327EF98;
    'dispatch: loop {
        match pc {
            0x8327EF98 => {
    //   block [0x8327EF98..0x8327EFD8)
	// 8327EF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EFA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EFAC: 388BF080  addi r4, r11, -0xf80
	ctx.r[4].s64 = ctx.r[11].s64 + -3968;
	// 8327EFB0: 386ADDD4  addi r3, r10, -0x222c
	ctx.r[3].s64 = ctx.r[10].s64 + -8748;
	// 8327EFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EFB8: 4AFADF19  bl 0x8222ced0
	ctx.lr = 0x8327EFBC;
	sub_8222CED0(ctx, base);
	// 8327EFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327EFC0: 38690A68  addi r3, r9, 0xa68
	ctx.r[3].s64 = ctx.r[9].s64 + 2664;
	// 8327EFC4: 4BA2AF5D  bl 0x82ca9f20
	ctx.lr = 0x8327EFC8;
	sub_82CA9F20(ctx, base);
	// 8327EFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327EFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327EFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327EFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327EFD8 size=64
    let mut pc: u32 = 0x8327EFD8;
    'dispatch: loop {
        match pc {
            0x8327EFD8 => {
    //   block [0x8327EFD8..0x8327F018)
	// 8327EFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327EFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327EFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327EFE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327EFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327EFEC: 388BF090  addi r4, r11, -0xf70
	ctx.r[4].s64 = ctx.r[11].s64 + -3952;
	// 8327EFF0: 386ADDD8  addi r3, r10, -0x2228
	ctx.r[3].s64 = ctx.r[10].s64 + -8744;
	// 8327EFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327EFF8: 4AFADED9  bl 0x8222ced0
	ctx.lr = 0x8327EFFC;
	sub_8222CED0(ctx, base);
	// 8327EFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F000: 38690A78  addi r3, r9, 0xa78
	ctx.r[3].s64 = ctx.r[9].s64 + 2680;
	// 8327F004: 4BA2AF1D  bl 0x82ca9f20
	ctx.lr = 0x8327F008;
	sub_82CA9F20(ctx, base);
	// 8327F008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F018 size=64
    let mut pc: u32 = 0x8327F018;
    'dispatch: loop {
        match pc {
            0x8327F018 => {
    //   block [0x8327F018..0x8327F058)
	// 8327F018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F024: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F02C: 388BF0A8  addi r4, r11, -0xf58
	ctx.r[4].s64 = ctx.r[11].s64 + -3928;
	// 8327F030: 386ADDDC  addi r3, r10, -0x2224
	ctx.r[3].s64 = ctx.r[10].s64 + -8740;
	// 8327F034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F038: 4AFADE99  bl 0x8222ced0
	ctx.lr = 0x8327F03C;
	sub_8222CED0(ctx, base);
	// 8327F03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F040: 38690A88  addi r3, r9, 0xa88
	ctx.r[3].s64 = ctx.r[9].s64 + 2696;
	// 8327F044: 4BA2AEDD  bl 0x82ca9f20
	ctx.lr = 0x8327F048;
	sub_82CA9F20(ctx, base);
	// 8327F048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F058 size=64
    let mut pc: u32 = 0x8327F058;
    'dispatch: loop {
        match pc {
            0x8327F058 => {
    //   block [0x8327F058..0x8327F098)
	// 8327F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F064: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F06C: 388BF0C0  addi r4, r11, -0xf40
	ctx.r[4].s64 = ctx.r[11].s64 + -3904;
	// 8327F070: 386ADDE0  addi r3, r10, -0x2220
	ctx.r[3].s64 = ctx.r[10].s64 + -8736;
	// 8327F074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F078: 4AFADE59  bl 0x8222ced0
	ctx.lr = 0x8327F07C;
	sub_8222CED0(ctx, base);
	// 8327F07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F080: 38690A98  addi r3, r9, 0xa98
	ctx.r[3].s64 = ctx.r[9].s64 + 2712;
	// 8327F084: 4BA2AE9D  bl 0x82ca9f20
	ctx.lr = 0x8327F088;
	sub_82CA9F20(ctx, base);
	// 8327F088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F098 size=64
    let mut pc: u32 = 0x8327F098;
    'dispatch: loop {
        match pc {
            0x8327F098 => {
    //   block [0x8327F098..0x8327F0D8)
	// 8327F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F0A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F0AC: 388BF0D4  addi r4, r11, -0xf2c
	ctx.r[4].s64 = ctx.r[11].s64 + -3884;
	// 8327F0B0: 386ADDE4  addi r3, r10, -0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + -8732;
	// 8327F0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F0B8: 4AFADE19  bl 0x8222ced0
	ctx.lr = 0x8327F0BC;
	sub_8222CED0(ctx, base);
	// 8327F0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F0C0: 38690AA8  addi r3, r9, 0xaa8
	ctx.r[3].s64 = ctx.r[9].s64 + 2728;
	// 8327F0C4: 4BA2AE5D  bl 0x82ca9f20
	ctx.lr = 0x8327F0C8;
	sub_82CA9F20(ctx, base);
	// 8327F0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F0D8 size=64
    let mut pc: u32 = 0x8327F0D8;
    'dispatch: loop {
        match pc {
            0x8327F0D8 => {
    //   block [0x8327F0D8..0x8327F118)
	// 8327F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F0E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F0EC: 388BF0E0  addi r4, r11, -0xf20
	ctx.r[4].s64 = ctx.r[11].s64 + -3872;
	// 8327F0F0: 386ADDE8  addi r3, r10, -0x2218
	ctx.r[3].s64 = ctx.r[10].s64 + -8728;
	// 8327F0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F0F8: 4AFADDD9  bl 0x8222ced0
	ctx.lr = 0x8327F0FC;
	sub_8222CED0(ctx, base);
	// 8327F0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F100: 38690AB8  addi r3, r9, 0xab8
	ctx.r[3].s64 = ctx.r[9].s64 + 2744;
	// 8327F104: 4BA2AE1D  bl 0x82ca9f20
	ctx.lr = 0x8327F108;
	sub_82CA9F20(ctx, base);
	// 8327F108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F118 size=64
    let mut pc: u32 = 0x8327F118;
    'dispatch: loop {
        match pc {
            0x8327F118 => {
    //   block [0x8327F118..0x8327F158)
	// 8327F118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F124: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F12C: 388BF0F8  addi r4, r11, -0xf08
	ctx.r[4].s64 = ctx.r[11].s64 + -3848;
	// 8327F130: 386ADDEC  addi r3, r10, -0x2214
	ctx.r[3].s64 = ctx.r[10].s64 + -8724;
	// 8327F134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F138: 4AFADD99  bl 0x8222ced0
	ctx.lr = 0x8327F13C;
	sub_8222CED0(ctx, base);
	// 8327F13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F140: 38690AC8  addi r3, r9, 0xac8
	ctx.r[3].s64 = ctx.r[9].s64 + 2760;
	// 8327F144: 4BA2ADDD  bl 0x82ca9f20
	ctx.lr = 0x8327F148;
	sub_82CA9F20(ctx, base);
	// 8327F148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F158 size=64
    let mut pc: u32 = 0x8327F158;
    'dispatch: loop {
        match pc {
            0x8327F158 => {
    //   block [0x8327F158..0x8327F198)
	// 8327F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F164: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F16C: 388BF10C  addi r4, r11, -0xef4
	ctx.r[4].s64 = ctx.r[11].s64 + -3828;
	// 8327F170: 386ADDF0  addi r3, r10, -0x2210
	ctx.r[3].s64 = ctx.r[10].s64 + -8720;
	// 8327F174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F178: 4AFADD59  bl 0x8222ced0
	ctx.lr = 0x8327F17C;
	sub_8222CED0(ctx, base);
	// 8327F17C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F180: 38690AD8  addi r3, r9, 0xad8
	ctx.r[3].s64 = ctx.r[9].s64 + 2776;
	// 8327F184: 4BA2AD9D  bl 0x82ca9f20
	ctx.lr = 0x8327F188;
	sub_82CA9F20(ctx, base);
	// 8327F188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F198 size=64
    let mut pc: u32 = 0x8327F198;
    'dispatch: loop {
        match pc {
            0x8327F198 => {
    //   block [0x8327F198..0x8327F1D8)
	// 8327F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F1A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F1A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F1A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F1AC: 388BF120  addi r4, r11, -0xee0
	ctx.r[4].s64 = ctx.r[11].s64 + -3808;
	// 8327F1B0: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 8327F1B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F1B8: 4AFADD19  bl 0x8222ced0
	ctx.lr = 0x8327F1BC;
	sub_8222CED0(ctx, base);
	// 8327F1BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F1C0: 38690AE8  addi r3, r9, 0xae8
	ctx.r[3].s64 = ctx.r[9].s64 + 2792;
	// 8327F1C4: 4BA2AD5D  bl 0x82ca9f20
	ctx.lr = 0x8327F1C8;
	sub_82CA9F20(ctx, base);
	// 8327F1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F1D8 size=64
    let mut pc: u32 = 0x8327F1D8;
    'dispatch: loop {
        match pc {
            0x8327F1D8 => {
    //   block [0x8327F1D8..0x8327F218)
	// 8327F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F1E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F1EC: 388BF134  addi r4, r11, -0xecc
	ctx.r[4].s64 = ctx.r[11].s64 + -3788;
	// 8327F1F0: 386ADDF8  addi r3, r10, -0x2208
	ctx.r[3].s64 = ctx.r[10].s64 + -8712;
	// 8327F1F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F1F8: 4AFADCD9  bl 0x8222ced0
	ctx.lr = 0x8327F1FC;
	sub_8222CED0(ctx, base);
	// 8327F1FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F200: 38690AF8  addi r3, r9, 0xaf8
	ctx.r[3].s64 = ctx.r[9].s64 + 2808;
	// 8327F204: 4BA2AD1D  bl 0x82ca9f20
	ctx.lr = 0x8327F208;
	sub_82CA9F20(ctx, base);
	// 8327F208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F218 size=64
    let mut pc: u32 = 0x8327F218;
    'dispatch: loop {
        match pc {
            0x8327F218 => {
    //   block [0x8327F218..0x8327F258)
	// 8327F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F224: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F22C: 388BF148  addi r4, r11, -0xeb8
	ctx.r[4].s64 = ctx.r[11].s64 + -3768;
	// 8327F230: 386ADDFC  addi r3, r10, -0x2204
	ctx.r[3].s64 = ctx.r[10].s64 + -8708;
	// 8327F234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F238: 4AFADC99  bl 0x8222ced0
	ctx.lr = 0x8327F23C;
	sub_8222CED0(ctx, base);
	// 8327F23C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F240: 38690B08  addi r3, r9, 0xb08
	ctx.r[3].s64 = ctx.r[9].s64 + 2824;
	// 8327F244: 4BA2ACDD  bl 0x82ca9f20
	ctx.lr = 0x8327F248;
	sub_82CA9F20(ctx, base);
	// 8327F248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F258 size=64
    let mut pc: u32 = 0x8327F258;
    'dispatch: loop {
        match pc {
            0x8327F258 => {
    //   block [0x8327F258..0x8327F298)
	// 8327F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F264: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F26C: 388BF160  addi r4, r11, -0xea0
	ctx.r[4].s64 = ctx.r[11].s64 + -3744;
	// 8327F270: 386ADE00  addi r3, r10, -0x2200
	ctx.r[3].s64 = ctx.r[10].s64 + -8704;
	// 8327F274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F278: 4AFADC59  bl 0x8222ced0
	ctx.lr = 0x8327F27C;
	sub_8222CED0(ctx, base);
	// 8327F27C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F280: 38690B18  addi r3, r9, 0xb18
	ctx.r[3].s64 = ctx.r[9].s64 + 2840;
	// 8327F284: 4BA2AC9D  bl 0x82ca9f20
	ctx.lr = 0x8327F288;
	sub_82CA9F20(ctx, base);
	// 8327F288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F298 size=64
    let mut pc: u32 = 0x8327F298;
    'dispatch: loop {
        match pc {
            0x8327F298 => {
    //   block [0x8327F298..0x8327F2D8)
	// 8327F298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F2A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F2A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F2AC: 388BF174  addi r4, r11, -0xe8c
	ctx.r[4].s64 = ctx.r[11].s64 + -3724;
	// 8327F2B0: 386ADE04  addi r3, r10, -0x21fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8700;
	// 8327F2B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F2B8: 4AFADC19  bl 0x8222ced0
	ctx.lr = 0x8327F2BC;
	sub_8222CED0(ctx, base);
	// 8327F2BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F2C0: 38690B28  addi r3, r9, 0xb28
	ctx.r[3].s64 = ctx.r[9].s64 + 2856;
	// 8327F2C4: 4BA2AC5D  bl 0x82ca9f20
	ctx.lr = 0x8327F2C8;
	sub_82CA9F20(ctx, base);
	// 8327F2C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F2D8 size=64
    let mut pc: u32 = 0x8327F2D8;
    'dispatch: loop {
        match pc {
            0x8327F2D8 => {
    //   block [0x8327F2D8..0x8327F318)
	// 8327F2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F2E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F2E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F2E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F2EC: 388BF190  addi r4, r11, -0xe70
	ctx.r[4].s64 = ctx.r[11].s64 + -3696;
	// 8327F2F0: 386ADE08  addi r3, r10, -0x21f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8696;
	// 8327F2F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F2F8: 4AFADBD9  bl 0x8222ced0
	ctx.lr = 0x8327F2FC;
	sub_8222CED0(ctx, base);
	// 8327F2FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F300: 38690B38  addi r3, r9, 0xb38
	ctx.r[3].s64 = ctx.r[9].s64 + 2872;
	// 8327F304: 4BA2AC1D  bl 0x82ca9f20
	ctx.lr = 0x8327F308;
	sub_82CA9F20(ctx, base);
	// 8327F308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F318 size=64
    let mut pc: u32 = 0x8327F318;
    'dispatch: loop {
        match pc {
            0x8327F318 => {
    //   block [0x8327F318..0x8327F358)
	// 8327F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F324: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F32C: 388BF1AC  addi r4, r11, -0xe54
	ctx.r[4].s64 = ctx.r[11].s64 + -3668;
	// 8327F330: 386ADE0C  addi r3, r10, -0x21f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8692;
	// 8327F334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F338: 4AFADB99  bl 0x8222ced0
	ctx.lr = 0x8327F33C;
	sub_8222CED0(ctx, base);
	// 8327F33C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F340: 38690B48  addi r3, r9, 0xb48
	ctx.r[3].s64 = ctx.r[9].s64 + 2888;
	// 8327F344: 4BA2ABDD  bl 0x82ca9f20
	ctx.lr = 0x8327F348;
	sub_82CA9F20(ctx, base);
	// 8327F348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F358 size=64
    let mut pc: u32 = 0x8327F358;
    'dispatch: loop {
        match pc {
            0x8327F358 => {
    //   block [0x8327F358..0x8327F398)
	// 8327F358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F364: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F36C: 388BF1C0  addi r4, r11, -0xe40
	ctx.r[4].s64 = ctx.r[11].s64 + -3648;
	// 8327F370: 386ADE10  addi r3, r10, -0x21f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8688;
	// 8327F374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F378: 4AFADB59  bl 0x8222ced0
	ctx.lr = 0x8327F37C;
	sub_8222CED0(ctx, base);
	// 8327F37C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F380: 38690B58  addi r3, r9, 0xb58
	ctx.r[3].s64 = ctx.r[9].s64 + 2904;
	// 8327F384: 4BA2AB9D  bl 0x82ca9f20
	ctx.lr = 0x8327F388;
	sub_82CA9F20(ctx, base);
	// 8327F388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F398 size=64
    let mut pc: u32 = 0x8327F398;
    'dispatch: loop {
        match pc {
            0x8327F398 => {
    //   block [0x8327F398..0x8327F3D8)
	// 8327F398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F3A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F3AC: 388BF1D4  addi r4, r11, -0xe2c
	ctx.r[4].s64 = ctx.r[11].s64 + -3628;
	// 8327F3B0: 386ADE14  addi r3, r10, -0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8684;
	// 8327F3B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F3B8: 4AFADB19  bl 0x8222ced0
	ctx.lr = 0x8327F3BC;
	sub_8222CED0(ctx, base);
	// 8327F3BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F3C0: 38690B68  addi r3, r9, 0xb68
	ctx.r[3].s64 = ctx.r[9].s64 + 2920;
	// 8327F3C4: 4BA2AB5D  bl 0x82ca9f20
	ctx.lr = 0x8327F3C8;
	sub_82CA9F20(ctx, base);
	// 8327F3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F3D8 size=64
    let mut pc: u32 = 0x8327F3D8;
    'dispatch: loop {
        match pc {
            0x8327F3D8 => {
    //   block [0x8327F3D8..0x8327F418)
	// 8327F3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F3E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F3E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F3E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F3EC: 388BF1EC  addi r4, r11, -0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + -3604;
	// 8327F3F0: 386ADE18  addi r3, r10, -0x21e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8680;
	// 8327F3F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F3F8: 4AFADAD9  bl 0x8222ced0
	ctx.lr = 0x8327F3FC;
	sub_8222CED0(ctx, base);
	// 8327F3FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F400: 38690B78  addi r3, r9, 0xb78
	ctx.r[3].s64 = ctx.r[9].s64 + 2936;
	// 8327F404: 4BA2AB1D  bl 0x82ca9f20
	ctx.lr = 0x8327F408;
	sub_82CA9F20(ctx, base);
	// 8327F408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F418 size=64
    let mut pc: u32 = 0x8327F418;
    'dispatch: loop {
        match pc {
            0x8327F418 => {
    //   block [0x8327F418..0x8327F458)
	// 8327F418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F42C: 388BF204  addi r4, r11, -0xdfc
	ctx.r[4].s64 = ctx.r[11].s64 + -3580;
	// 8327F430: 386ADE1C  addi r3, r10, -0x21e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8676;
	// 8327F434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F438: 4AFADA99  bl 0x8222ced0
	ctx.lr = 0x8327F43C;
	sub_8222CED0(ctx, base);
	// 8327F43C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F440: 38690B88  addi r3, r9, 0xb88
	ctx.r[3].s64 = ctx.r[9].s64 + 2952;
	// 8327F444: 4BA2AADD  bl 0x82ca9f20
	ctx.lr = 0x8327F448;
	sub_82CA9F20(ctx, base);
	// 8327F448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F458 size=64
    let mut pc: u32 = 0x8327F458;
    'dispatch: loop {
        match pc {
            0x8327F458 => {
    //   block [0x8327F458..0x8327F498)
	// 8327F458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F46C: 388BF21C  addi r4, r11, -0xde4
	ctx.r[4].s64 = ctx.r[11].s64 + -3556;
	// 8327F470: 386ADE20  addi r3, r10, -0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8672;
	// 8327F474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F478: 4AFADA59  bl 0x8222ced0
	ctx.lr = 0x8327F47C;
	sub_8222CED0(ctx, base);
	// 8327F47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F480: 38690B98  addi r3, r9, 0xb98
	ctx.r[3].s64 = ctx.r[9].s64 + 2968;
	// 8327F484: 4BA2AA9D  bl 0x82ca9f20
	ctx.lr = 0x8327F488;
	sub_82CA9F20(ctx, base);
	// 8327F488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F498 size=64
    let mut pc: u32 = 0x8327F498;
    'dispatch: loop {
        match pc {
            0x8327F498 => {
    //   block [0x8327F498..0x8327F4D8)
	// 8327F498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F4A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F4AC: 388BF238  addi r4, r11, -0xdc8
	ctx.r[4].s64 = ctx.r[11].s64 + -3528;
	// 8327F4B0: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 8327F4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F4B8: 4AFADA19  bl 0x8222ced0
	ctx.lr = 0x8327F4BC;
	sub_8222CED0(ctx, base);
	// 8327F4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F4C0: 38690BA8  addi r3, r9, 0xba8
	ctx.r[3].s64 = ctx.r[9].s64 + 2984;
	// 8327F4C4: 4BA2AA5D  bl 0x82ca9f20
	ctx.lr = 0x8327F4C8;
	sub_82CA9F20(ctx, base);
	// 8327F4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F4D8 size=64
    let mut pc: u32 = 0x8327F4D8;
    'dispatch: loop {
        match pc {
            0x8327F4D8 => {
    //   block [0x8327F4D8..0x8327F518)
	// 8327F4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F4E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F4EC: 388BF24C  addi r4, r11, -0xdb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3508;
	// 8327F4F0: 386ADE28  addi r3, r10, -0x21d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8664;
	// 8327F4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F4F8: 4AFAD9D9  bl 0x8222ced0
	ctx.lr = 0x8327F4FC;
	sub_8222CED0(ctx, base);
	// 8327F4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F500: 38690BB8  addi r3, r9, 0xbb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3000;
	// 8327F504: 4BA2AA1D  bl 0x82ca9f20
	ctx.lr = 0x8327F508;
	sub_82CA9F20(ctx, base);
	// 8327F508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F518 size=64
    let mut pc: u32 = 0x8327F518;
    'dispatch: loop {
        match pc {
            0x8327F518 => {
    //   block [0x8327F518..0x8327F558)
	// 8327F518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F52C: 388BF260  addi r4, r11, -0xda0
	ctx.r[4].s64 = ctx.r[11].s64 + -3488;
	// 8327F530: 386ADE2C  addi r3, r10, -0x21d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8660;
	// 8327F534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F538: 4AFAD999  bl 0x8222ced0
	ctx.lr = 0x8327F53C;
	sub_8222CED0(ctx, base);
	// 8327F53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F540: 38690BC8  addi r3, r9, 0xbc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3016;
	// 8327F544: 4BA2A9DD  bl 0x82ca9f20
	ctx.lr = 0x8327F548;
	sub_82CA9F20(ctx, base);
	// 8327F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F558 size=64
    let mut pc: u32 = 0x8327F558;
    'dispatch: loop {
        match pc {
            0x8327F558 => {
    //   block [0x8327F558..0x8327F598)
	// 8327F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F56C: 388BF274  addi r4, r11, -0xd8c
	ctx.r[4].s64 = ctx.r[11].s64 + -3468;
	// 8327F570: 386ADE30  addi r3, r10, -0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + -8656;
	// 8327F574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F578: 4AFAD959  bl 0x8222ced0
	ctx.lr = 0x8327F57C;
	sub_8222CED0(ctx, base);
	// 8327F57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F580: 38690BD8  addi r3, r9, 0xbd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3032;
	// 8327F584: 4BA2A99D  bl 0x82ca9f20
	ctx.lr = 0x8327F588;
	sub_82CA9F20(ctx, base);
	// 8327F588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F598 size=64
    let mut pc: u32 = 0x8327F598;
    'dispatch: loop {
        match pc {
            0x8327F598 => {
    //   block [0x8327F598..0x8327F5D8)
	// 8327F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F5A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F5AC: 388BF290  addi r4, r11, -0xd70
	ctx.r[4].s64 = ctx.r[11].s64 + -3440;
	// 8327F5B0: 386ADE34  addi r3, r10, -0x21cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8652;
	// 8327F5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F5B8: 4AFAD919  bl 0x8222ced0
	ctx.lr = 0x8327F5BC;
	sub_8222CED0(ctx, base);
	// 8327F5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F5C0: 38690BE8  addi r3, r9, 0xbe8
	ctx.r[3].s64 = ctx.r[9].s64 + 3048;
	// 8327F5C4: 4BA2A95D  bl 0x82ca9f20
	ctx.lr = 0x8327F5C8;
	sub_82CA9F20(ctx, base);
	// 8327F5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F5D8 size=64
    let mut pc: u32 = 0x8327F5D8;
    'dispatch: loop {
        match pc {
            0x8327F5D8 => {
    //   block [0x8327F5D8..0x8327F618)
	// 8327F5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F5E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F5EC: 388BF2AC  addi r4, r11, -0xd54
	ctx.r[4].s64 = ctx.r[11].s64 + -3412;
	// 8327F5F0: 386ADE38  addi r3, r10, -0x21c8
	ctx.r[3].s64 = ctx.r[10].s64 + -8648;
	// 8327F5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F5F8: 4AFAD8D9  bl 0x8222ced0
	ctx.lr = 0x8327F5FC;
	sub_8222CED0(ctx, base);
	// 8327F5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F600: 38690BF8  addi r3, r9, 0xbf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3064;
	// 8327F604: 4BA2A91D  bl 0x82ca9f20
	ctx.lr = 0x8327F608;
	sub_82CA9F20(ctx, base);
	// 8327F608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F618 size=64
    let mut pc: u32 = 0x8327F618;
    'dispatch: loop {
        match pc {
            0x8327F618 => {
    //   block [0x8327F618..0x8327F658)
	// 8327F618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F62C: 388BF2CC  addi r4, r11, -0xd34
	ctx.r[4].s64 = ctx.r[11].s64 + -3380;
	// 8327F630: 386ADE3C  addi r3, r10, -0x21c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8644;
	// 8327F634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F638: 4AFAD899  bl 0x8222ced0
	ctx.lr = 0x8327F63C;
	sub_8222CED0(ctx, base);
	// 8327F63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F640: 38690C08  addi r3, r9, 0xc08
	ctx.r[3].s64 = ctx.r[9].s64 + 3080;
	// 8327F644: 4BA2A8DD  bl 0x82ca9f20
	ctx.lr = 0x8327F648;
	sub_82CA9F20(ctx, base);
	// 8327F648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F658 size=64
    let mut pc: u32 = 0x8327F658;
    'dispatch: loop {
        match pc {
            0x8327F658 => {
    //   block [0x8327F658..0x8327F698)
	// 8327F658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F66C: 388BF2E4  addi r4, r11, -0xd1c
	ctx.r[4].s64 = ctx.r[11].s64 + -3356;
	// 8327F670: 386ADE40  addi r3, r10, -0x21c0
	ctx.r[3].s64 = ctx.r[10].s64 + -8640;
	// 8327F674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F678: 4AFAD859  bl 0x8222ced0
	ctx.lr = 0x8327F67C;
	sub_8222CED0(ctx, base);
	// 8327F67C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F680: 38690C18  addi r3, r9, 0xc18
	ctx.r[3].s64 = ctx.r[9].s64 + 3096;
	// 8327F684: 4BA2A89D  bl 0x82ca9f20
	ctx.lr = 0x8327F688;
	sub_82CA9F20(ctx, base);
	// 8327F688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F698 size=64
    let mut pc: u32 = 0x8327F698;
    'dispatch: loop {
        match pc {
            0x8327F698 => {
    //   block [0x8327F698..0x8327F6D8)
	// 8327F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F6A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F6A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F6A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F6AC: 388BF2FC  addi r4, r11, -0xd04
	ctx.r[4].s64 = ctx.r[11].s64 + -3332;
	// 8327F6B0: 386ADE44  addi r3, r10, -0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8636;
	// 8327F6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F6B8: 4AFAD819  bl 0x8222ced0
	ctx.lr = 0x8327F6BC;
	sub_8222CED0(ctx, base);
	// 8327F6BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F6C0: 38690C28  addi r3, r9, 0xc28
	ctx.r[3].s64 = ctx.r[9].s64 + 3112;
	// 8327F6C4: 4BA2A85D  bl 0x82ca9f20
	ctx.lr = 0x8327F6C8;
	sub_82CA9F20(ctx, base);
	// 8327F6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F6D8 size=64
    let mut pc: u32 = 0x8327F6D8;
    'dispatch: loop {
        match pc {
            0x8327F6D8 => {
    //   block [0x8327F6D8..0x8327F718)
	// 8327F6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F6E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F6EC: 388BF314  addi r4, r11, -0xcec
	ctx.r[4].s64 = ctx.r[11].s64 + -3308;
	// 8327F6F0: 386ADE48  addi r3, r10, -0x21b8
	ctx.r[3].s64 = ctx.r[10].s64 + -8632;
	// 8327F6F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F6F8: 4AFAD7D9  bl 0x8222ced0
	ctx.lr = 0x8327F6FC;
	sub_8222CED0(ctx, base);
	// 8327F6FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F700: 38690C38  addi r3, r9, 0xc38
	ctx.r[3].s64 = ctx.r[9].s64 + 3128;
	// 8327F704: 4BA2A81D  bl 0x82ca9f20
	ctx.lr = 0x8327F708;
	sub_82CA9F20(ctx, base);
	// 8327F708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F718 size=64
    let mut pc: u32 = 0x8327F718;
    'dispatch: loop {
        match pc {
            0x8327F718 => {
    //   block [0x8327F718..0x8327F758)
	// 8327F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F72C: 388BF330  addi r4, r11, -0xcd0
	ctx.r[4].s64 = ctx.r[11].s64 + -3280;
	// 8327F730: 386ADE4C  addi r3, r10, -0x21b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8628;
	// 8327F734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F738: 4AFAD799  bl 0x8222ced0
	ctx.lr = 0x8327F73C;
	sub_8222CED0(ctx, base);
	// 8327F73C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F740: 38690C48  addi r3, r9, 0xc48
	ctx.r[3].s64 = ctx.r[9].s64 + 3144;
	// 8327F744: 4BA2A7DD  bl 0x82ca9f20
	ctx.lr = 0x8327F748;
	sub_82CA9F20(ctx, base);
	// 8327F748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F758 size=64
    let mut pc: u32 = 0x8327F758;
    'dispatch: loop {
        match pc {
            0x8327F758 => {
    //   block [0x8327F758..0x8327F798)
	// 8327F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F764: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F76C: 388BF34C  addi r4, r11, -0xcb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3252;
	// 8327F770: 386ADE50  addi r3, r10, -0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + -8624;
	// 8327F774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F778: 4AFAD759  bl 0x8222ced0
	ctx.lr = 0x8327F77C;
	sub_8222CED0(ctx, base);
	// 8327F77C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F780: 38690C58  addi r3, r9, 0xc58
	ctx.r[3].s64 = ctx.r[9].s64 + 3160;
	// 8327F784: 4BA2A79D  bl 0x82ca9f20
	ctx.lr = 0x8327F788;
	sub_82CA9F20(ctx, base);
	// 8327F788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F798 size=64
    let mut pc: u32 = 0x8327F798;
    'dispatch: loop {
        match pc {
            0x8327F798 => {
    //   block [0x8327F798..0x8327F7D8)
	// 8327F798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F7A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F7A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F7AC: 388BF36C  addi r4, r11, -0xc94
	ctx.r[4].s64 = ctx.r[11].s64 + -3220;
	// 8327F7B0: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 8327F7B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F7B8: 4AFAD719  bl 0x8222ced0
	ctx.lr = 0x8327F7BC;
	sub_8222CED0(ctx, base);
	// 8327F7BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F7C0: 38690C68  addi r3, r9, 0xc68
	ctx.r[3].s64 = ctx.r[9].s64 + 3176;
	// 8327F7C4: 4BA2A75D  bl 0x82ca9f20
	ctx.lr = 0x8327F7C8;
	sub_82CA9F20(ctx, base);
	// 8327F7C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F7D8 size=64
    let mut pc: u32 = 0x8327F7D8;
    'dispatch: loop {
        match pc {
            0x8327F7D8 => {
    //   block [0x8327F7D8..0x8327F818)
	// 8327F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F7E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F7E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F7EC: 388BF388  addi r4, r11, -0xc78
	ctx.r[4].s64 = ctx.r[11].s64 + -3192;
	// 8327F7F0: 386ADE58  addi r3, r10, -0x21a8
	ctx.r[3].s64 = ctx.r[10].s64 + -8616;
	// 8327F7F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F7F8: 4AFAD6D9  bl 0x8222ced0
	ctx.lr = 0x8327F7FC;
	sub_8222CED0(ctx, base);
	// 8327F7FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F800: 38690C78  addi r3, r9, 0xc78
	ctx.r[3].s64 = ctx.r[9].s64 + 3192;
	// 8327F804: 4BA2A71D  bl 0x82ca9f20
	ctx.lr = 0x8327F808;
	sub_82CA9F20(ctx, base);
	// 8327F808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F818 size=64
    let mut pc: u32 = 0x8327F818;
    'dispatch: loop {
        match pc {
            0x8327F818 => {
    //   block [0x8327F818..0x8327F858)
	// 8327F818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F824: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F82C: 388BF3A0  addi r4, r11, -0xc60
	ctx.r[4].s64 = ctx.r[11].s64 + -3168;
	// 8327F830: 386ADE5C  addi r3, r10, -0x21a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8612;
	// 8327F834: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F838: 4AFAD699  bl 0x8222ced0
	ctx.lr = 0x8327F83C;
	sub_8222CED0(ctx, base);
	// 8327F83C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F840: 38690C88  addi r3, r9, 0xc88
	ctx.r[3].s64 = ctx.r[9].s64 + 3208;
	// 8327F844: 4BA2A6DD  bl 0x82ca9f20
	ctx.lr = 0x8327F848;
	sub_82CA9F20(ctx, base);
	// 8327F848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F858 size=64
    let mut pc: u32 = 0x8327F858;
    'dispatch: loop {
        match pc {
            0x8327F858 => {
    //   block [0x8327F858..0x8327F898)
	// 8327F858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F860: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F864: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F868: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F86C: 388BF3B4  addi r4, r11, -0xc4c
	ctx.r[4].s64 = ctx.r[11].s64 + -3148;
	// 8327F870: 386ADE60  addi r3, r10, -0x21a0
	ctx.r[3].s64 = ctx.r[10].s64 + -8608;
	// 8327F874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F878: 4AFAD659  bl 0x8222ced0
	ctx.lr = 0x8327F87C;
	sub_8222CED0(ctx, base);
	// 8327F87C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F880: 38690C98  addi r3, r9, 0xc98
	ctx.r[3].s64 = ctx.r[9].s64 + 3224;
	// 8327F884: 4BA2A69D  bl 0x82ca9f20
	ctx.lr = 0x8327F888;
	sub_82CA9F20(ctx, base);
	// 8327F888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F898 size=64
    let mut pc: u32 = 0x8327F898;
    'dispatch: loop {
        match pc {
            0x8327F898 => {
    //   block [0x8327F898..0x8327F8D8)
	// 8327F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F8A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F8A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F8A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F8AC: 388BF3CC  addi r4, r11, -0xc34
	ctx.r[4].s64 = ctx.r[11].s64 + -3124;
	// 8327F8B0: 386ADE64  addi r3, r10, -0x219c
	ctx.r[3].s64 = ctx.r[10].s64 + -8604;
	// 8327F8B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F8B8: 4AFAD619  bl 0x8222ced0
	ctx.lr = 0x8327F8BC;
	sub_8222CED0(ctx, base);
	// 8327F8BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F8C0: 38690CA8  addi r3, r9, 0xca8
	ctx.r[3].s64 = ctx.r[9].s64 + 3240;
	// 8327F8C4: 4BA2A65D  bl 0x82ca9f20
	ctx.lr = 0x8327F8C8;
	sub_82CA9F20(ctx, base);
	// 8327F8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F8D8 size=64
    let mut pc: u32 = 0x8327F8D8;
    'dispatch: loop {
        match pc {
            0x8327F8D8 => {
    //   block [0x8327F8D8..0x8327F918)
	// 8327F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F8E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F8E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F8EC: 388BF3E8  addi r4, r11, -0xc18
	ctx.r[4].s64 = ctx.r[11].s64 + -3096;
	// 8327F8F0: 386ADE68  addi r3, r10, -0x2198
	ctx.r[3].s64 = ctx.r[10].s64 + -8600;
	// 8327F8F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F8F8: 4AFAD5D9  bl 0x8222ced0
	ctx.lr = 0x8327F8FC;
	sub_8222CED0(ctx, base);
	// 8327F8FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F900: 38690CB8  addi r3, r9, 0xcb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3256;
	// 8327F904: 4BA2A61D  bl 0x82ca9f20
	ctx.lr = 0x8327F908;
	sub_82CA9F20(ctx, base);
	// 8327F908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F918 size=64
    let mut pc: u32 = 0x8327F918;
    'dispatch: loop {
        match pc {
            0x8327F918 => {
    //   block [0x8327F918..0x8327F958)
	// 8327F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F924: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F92C: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 8327F930: 386ADE6C  addi r3, r10, -0x2194
	ctx.r[3].s64 = ctx.r[10].s64 + -8596;
	// 8327F934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F938: 4AFAD599  bl 0x8222ced0
	ctx.lr = 0x8327F93C;
	sub_8222CED0(ctx, base);
	// 8327F93C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F940: 38690CC8  addi r3, r9, 0xcc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3272;
	// 8327F944: 4BA2A5DD  bl 0x82ca9f20
	ctx.lr = 0x8327F948;
	sub_82CA9F20(ctx, base);
	// 8327F948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F958 size=64
    let mut pc: u32 = 0x8327F958;
    'dispatch: loop {
        match pc {
            0x8327F958 => {
    //   block [0x8327F958..0x8327F998)
	// 8327F958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F964: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F96C: 388BF428  addi r4, r11, -0xbd8
	ctx.r[4].s64 = ctx.r[11].s64 + -3032;
	// 8327F970: 386ADE70  addi r3, r10, -0x2190
	ctx.r[3].s64 = ctx.r[10].s64 + -8592;
	// 8327F974: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F978: 4AFAD559  bl 0x8222ced0
	ctx.lr = 0x8327F97C;
	sub_8222CED0(ctx, base);
	// 8327F97C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F980: 38690CD8  addi r3, r9, 0xcd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3288;
	// 8327F984: 4BA2A59D  bl 0x82ca9f20
	ctx.lr = 0x8327F988;
	sub_82CA9F20(ctx, base);
	// 8327F988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F998 size=64
    let mut pc: u32 = 0x8327F998;
    'dispatch: loop {
        match pc {
            0x8327F998 => {
    //   block [0x8327F998..0x8327F9D8)
	// 8327F998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F9A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F9A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F9A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F9AC: 388BF440  addi r4, r11, -0xbc0
	ctx.r[4].s64 = ctx.r[11].s64 + -3008;
	// 8327F9B0: 386ADE74  addi r3, r10, -0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + -8588;
	// 8327F9B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F9B8: 4AFAD519  bl 0x8222ced0
	ctx.lr = 0x8327F9BC;
	sub_8222CED0(ctx, base);
	// 8327F9BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327F9C0: 38690CE8  addi r3, r9, 0xce8
	ctx.r[3].s64 = ctx.r[9].s64 + 3304;
	// 8327F9C4: 4BA2A55D  bl 0x82ca9f20
	ctx.lr = 0x8327F9C8;
	sub_82CA9F20(ctx, base);
	// 8327F9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327F9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327F9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327F9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327F9D8 size=64
    let mut pc: u32 = 0x8327F9D8;
    'dispatch: loop {
        match pc {
            0x8327F9D8 => {
    //   block [0x8327F9D8..0x8327FA18)
	// 8327F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327F9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327F9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327F9E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327F9E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327F9EC: 388BF454  addi r4, r11, -0xbac
	ctx.r[4].s64 = ctx.r[11].s64 + -2988;
	// 8327F9F0: 386ADE78  addi r3, r10, -0x2188
	ctx.r[3].s64 = ctx.r[10].s64 + -8584;
	// 8327F9F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327F9F8: 4AFAD4D9  bl 0x8222ced0
	ctx.lr = 0x8327F9FC;
	sub_8222CED0(ctx, base);
	// 8327F9FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA00: 38690CF8  addi r3, r9, 0xcf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3320;
	// 8327FA04: 4BA2A51D  bl 0x82ca9f20
	ctx.lr = 0x8327FA08;
	sub_82CA9F20(ctx, base);
	// 8327FA08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA18 size=64
    let mut pc: u32 = 0x8327FA18;
    'dispatch: loop {
        match pc {
            0x8327FA18 => {
    //   block [0x8327FA18..0x8327FA58)
	// 8327FA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FA20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FA24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FA28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FA2C: 388BF470  addi r4, r11, -0xb90
	ctx.r[4].s64 = ctx.r[11].s64 + -2960;
	// 8327FA30: 386ADE7C  addi r3, r10, -0x2184
	ctx.r[3].s64 = ctx.r[10].s64 + -8580;
	// 8327FA34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FA38: 4AFAD499  bl 0x8222ced0
	ctx.lr = 0x8327FA3C;
	sub_8222CED0(ctx, base);
	// 8327FA3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA40: 38690D08  addi r3, r9, 0xd08
	ctx.r[3].s64 = ctx.r[9].s64 + 3336;
	// 8327FA44: 4BA2A4DD  bl 0x82ca9f20
	ctx.lr = 0x8327FA48;
	sub_82CA9F20(ctx, base);
	// 8327FA48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA58 size=64
    let mut pc: u32 = 0x8327FA58;
    'dispatch: loop {
        match pc {
            0x8327FA58 => {
    //   block [0x8327FA58..0x8327FA98)
	// 8327FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FA64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FA6C: 388BF488  addi r4, r11, -0xb78
	ctx.r[4].s64 = ctx.r[11].s64 + -2936;
	// 8327FA70: 386ADE80  addi r3, r10, -0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + -8576;
	// 8327FA74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FA78: 4AFAD459  bl 0x8222ced0
	ctx.lr = 0x8327FA7C;
	sub_8222CED0(ctx, base);
	// 8327FA7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FA80: 38690D18  addi r3, r9, 0xd18
	ctx.r[3].s64 = ctx.r[9].s64 + 3352;
	// 8327FA84: 4BA2A49D  bl 0x82ca9f20
	ctx.lr = 0x8327FA88;
	sub_82CA9F20(ctx, base);
	// 8327FA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FA98 size=64
    let mut pc: u32 = 0x8327FA98;
    'dispatch: loop {
        match pc {
            0x8327FA98 => {
    //   block [0x8327FA98..0x8327FAD8)
	// 8327FA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FAA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FAAC: 388BF4A0  addi r4, r11, -0xb60
	ctx.r[4].s64 = ctx.r[11].s64 + -2912;
	// 8327FAB0: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 8327FAB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FAB8: 4AFAD419  bl 0x8222ced0
	ctx.lr = 0x8327FABC;
	sub_8222CED0(ctx, base);
	// 8327FABC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FAC0: 38690D28  addi r3, r9, 0xd28
	ctx.r[3].s64 = ctx.r[9].s64 + 3368;
	// 8327FAC4: 4BA2A45D  bl 0x82ca9f20
	ctx.lr = 0x8327FAC8;
	sub_82CA9F20(ctx, base);
	// 8327FAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FAD8 size=64
    let mut pc: u32 = 0x8327FAD8;
    'dispatch: loop {
        match pc {
            0x8327FAD8 => {
    //   block [0x8327FAD8..0x8327FB18)
	// 8327FAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FAE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FAE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FAEC: 388BF4BC  addi r4, r11, -0xb44
	ctx.r[4].s64 = ctx.r[11].s64 + -2884;
	// 8327FAF0: 386ADE88  addi r3, r10, -0x2178
	ctx.r[3].s64 = ctx.r[10].s64 + -8568;
	// 8327FAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FAF8: 4AFAD3D9  bl 0x8222ced0
	ctx.lr = 0x8327FAFC;
	sub_8222CED0(ctx, base);
	// 8327FAFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB00: 38690D38  addi r3, r9, 0xd38
	ctx.r[3].s64 = ctx.r[9].s64 + 3384;
	// 8327FB04: 4BA2A41D  bl 0x82ca9f20
	ctx.lr = 0x8327FB08;
	sub_82CA9F20(ctx, base);
	// 8327FB08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB18 size=64
    let mut pc: u32 = 0x8327FB18;
    'dispatch: loop {
        match pc {
            0x8327FB18 => {
    //   block [0x8327FB18..0x8327FB58)
	// 8327FB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FB20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FB24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FB2C: 388BF4D8  addi r4, r11, -0xb28
	ctx.r[4].s64 = ctx.r[11].s64 + -2856;
	// 8327FB30: 386ADE8C  addi r3, r10, -0x2174
	ctx.r[3].s64 = ctx.r[10].s64 + -8564;
	// 8327FB34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FB38: 4AFAD399  bl 0x8222ced0
	ctx.lr = 0x8327FB3C;
	sub_8222CED0(ctx, base);
	// 8327FB3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB40: 38690D48  addi r3, r9, 0xd48
	ctx.r[3].s64 = ctx.r[9].s64 + 3400;
	// 8327FB44: 4BA2A3DD  bl 0x82ca9f20
	ctx.lr = 0x8327FB48;
	sub_82CA9F20(ctx, base);
	// 8327FB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB58 size=64
    let mut pc: u32 = 0x8327FB58;
    'dispatch: loop {
        match pc {
            0x8327FB58 => {
    //   block [0x8327FB58..0x8327FB98)
	// 8327FB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FB64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FB6C: 388BF4F0  addi r4, r11, -0xb10
	ctx.r[4].s64 = ctx.r[11].s64 + -2832;
	// 8327FB70: 386ADE90  addi r3, r10, -0x2170
	ctx.r[3].s64 = ctx.r[10].s64 + -8560;
	// 8327FB74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FB78: 4AFAD359  bl 0x8222ced0
	ctx.lr = 0x8327FB7C;
	sub_8222CED0(ctx, base);
	// 8327FB7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FB80: 38690D58  addi r3, r9, 0xd58
	ctx.r[3].s64 = ctx.r[9].s64 + 3416;
	// 8327FB84: 4BA2A39D  bl 0x82ca9f20
	ctx.lr = 0x8327FB88;
	sub_82CA9F20(ctx, base);
	// 8327FB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FB98 size=64
    let mut pc: u32 = 0x8327FB98;
    'dispatch: loop {
        match pc {
            0x8327FB98 => {
    //   block [0x8327FB98..0x8327FBD8)
	// 8327FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FBA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FBA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FBAC: 388BF510  addi r4, r11, -0xaf0
	ctx.r[4].s64 = ctx.r[11].s64 + -2800;
	// 8327FBB0: 386ADE94  addi r3, r10, -0x216c
	ctx.r[3].s64 = ctx.r[10].s64 + -8556;
	// 8327FBB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FBB8: 4AFAD319  bl 0x8222ced0
	ctx.lr = 0x8327FBBC;
	sub_8222CED0(ctx, base);
	// 8327FBBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FBC0: 38690D68  addi r3, r9, 0xd68
	ctx.r[3].s64 = ctx.r[9].s64 + 3432;
	// 8327FBC4: 4BA2A35D  bl 0x82ca9f20
	ctx.lr = 0x8327FBC8;
	sub_82CA9F20(ctx, base);
	// 8327FBC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FBD8 size=64
    let mut pc: u32 = 0x8327FBD8;
    'dispatch: loop {
        match pc {
            0x8327FBD8 => {
    //   block [0x8327FBD8..0x8327FC18)
	// 8327FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FBE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FBE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FBE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FBEC: 388BF52C  addi r4, r11, -0xad4
	ctx.r[4].s64 = ctx.r[11].s64 + -2772;
	// 8327FBF0: 386ADE98  addi r3, r10, -0x2168
	ctx.r[3].s64 = ctx.r[10].s64 + -8552;
	// 8327FBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FBF8: 4AFAD2D9  bl 0x8222ced0
	ctx.lr = 0x8327FBFC;
	sub_8222CED0(ctx, base);
	// 8327FBFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC00: 38690D78  addi r3, r9, 0xd78
	ctx.r[3].s64 = ctx.r[9].s64 + 3448;
	// 8327FC04: 4BA2A31D  bl 0x82ca9f20
	ctx.lr = 0x8327FC08;
	sub_82CA9F20(ctx, base);
	// 8327FC08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC18 size=64
    let mut pc: u32 = 0x8327FC18;
    'dispatch: loop {
        match pc {
            0x8327FC18 => {
    //   block [0x8327FC18..0x8327FC58)
	// 8327FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FC20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FC24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FC2C: 388BF548  addi r4, r11, -0xab8
	ctx.r[4].s64 = ctx.r[11].s64 + -2744;
	// 8327FC30: 386ADE9C  addi r3, r10, -0x2164
	ctx.r[3].s64 = ctx.r[10].s64 + -8548;
	// 8327FC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FC38: 4AFAD299  bl 0x8222ced0
	ctx.lr = 0x8327FC3C;
	sub_8222CED0(ctx, base);
	// 8327FC3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC40: 38690D88  addi r3, r9, 0xd88
	ctx.r[3].s64 = ctx.r[9].s64 + 3464;
	// 8327FC44: 4BA2A2DD  bl 0x82ca9f20
	ctx.lr = 0x8327FC48;
	sub_82CA9F20(ctx, base);
	// 8327FC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC58 size=64
    let mut pc: u32 = 0x8327FC58;
    'dispatch: loop {
        match pc {
            0x8327FC58 => {
    //   block [0x8327FC58..0x8327FC98)
	// 8327FC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FC68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FC6C: 388BF568  addi r4, r11, -0xa98
	ctx.r[4].s64 = ctx.r[11].s64 + -2712;
	// 8327FC70: 386ADEA0  addi r3, r10, -0x2160
	ctx.r[3].s64 = ctx.r[10].s64 + -8544;
	// 8327FC74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FC78: 4AFAD259  bl 0x8222ced0
	ctx.lr = 0x8327FC7C;
	sub_8222CED0(ctx, base);
	// 8327FC7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FC80: 38690D98  addi r3, r9, 0xd98
	ctx.r[3].s64 = ctx.r[9].s64 + 3480;
	// 8327FC84: 4BA2A29D  bl 0x82ca9f20
	ctx.lr = 0x8327FC88;
	sub_82CA9F20(ctx, base);
	// 8327FC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FC98 size=64
    let mut pc: u32 = 0x8327FC98;
    'dispatch: loop {
        match pc {
            0x8327FC98 => {
    //   block [0x8327FC98..0x8327FCD8)
	// 8327FC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FCA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FCA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FCA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FCAC: 388BF588  addi r4, r11, -0xa78
	ctx.r[4].s64 = ctx.r[11].s64 + -2680;
	// 8327FCB0: 386ADEA4  addi r3, r10, -0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + -8540;
	// 8327FCB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FCB8: 4AFAD219  bl 0x8222ced0
	ctx.lr = 0x8327FCBC;
	sub_8222CED0(ctx, base);
	// 8327FCBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FCC0: 38690DA8  addi r3, r9, 0xda8
	ctx.r[3].s64 = ctx.r[9].s64 + 3496;
	// 8327FCC4: 4BA2A25D  bl 0x82ca9f20
	ctx.lr = 0x8327FCC8;
	sub_82CA9F20(ctx, base);
	// 8327FCC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FCCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FCD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FCD8 size=64
    let mut pc: u32 = 0x8327FCD8;
    'dispatch: loop {
        match pc {
            0x8327FCD8 => {
    //   block [0x8327FCD8..0x8327FD18)
	// 8327FCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FCE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FCE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FCEC: 388BF5A4  addi r4, r11, -0xa5c
	ctx.r[4].s64 = ctx.r[11].s64 + -2652;
	// 8327FCF0: 386ADEA8  addi r3, r10, -0x2158
	ctx.r[3].s64 = ctx.r[10].s64 + -8536;
	// 8327FCF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FCF8: 4AFAD1D9  bl 0x8222ced0
	ctx.lr = 0x8327FCFC;
	sub_8222CED0(ctx, base);
	// 8327FCFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD00: 38690DB8  addi r3, r9, 0xdb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3512;
	// 8327FD04: 4BA2A21D  bl 0x82ca9f20
	ctx.lr = 0x8327FD08;
	sub_82CA9F20(ctx, base);
	// 8327FD08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD18 size=64
    let mut pc: u32 = 0x8327FD18;
    'dispatch: loop {
        match pc {
            0x8327FD18 => {
    //   block [0x8327FD18..0x8327FD58)
	// 8327FD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FD24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FD28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FD2C: 388BF5C0  addi r4, r11, -0xa40
	ctx.r[4].s64 = ctx.r[11].s64 + -2624;
	// 8327FD30: 386ADEAC  addi r3, r10, -0x2154
	ctx.r[3].s64 = ctx.r[10].s64 + -8532;
	// 8327FD34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FD38: 4AFAD199  bl 0x8222ced0
	ctx.lr = 0x8327FD3C;
	sub_8222CED0(ctx, base);
	// 8327FD3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD40: 38690DC8  addi r3, r9, 0xdc8
	ctx.r[3].s64 = ctx.r[9].s64 + 3528;
	// 8327FD44: 4BA2A1DD  bl 0x82ca9f20
	ctx.lr = 0x8327FD48;
	sub_82CA9F20(ctx, base);
	// 8327FD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD58 size=64
    let mut pc: u32 = 0x8327FD58;
    'dispatch: loop {
        match pc {
            0x8327FD58 => {
    //   block [0x8327FD58..0x8327FD98)
	// 8327FD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FD64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FD68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FD6C: 388BF5DC  addi r4, r11, -0xa24
	ctx.r[4].s64 = ctx.r[11].s64 + -2596;
	// 8327FD70: 386ADEB0  addi r3, r10, -0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + -8528;
	// 8327FD74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FD78: 4AFAD159  bl 0x8222ced0
	ctx.lr = 0x8327FD7C;
	sub_8222CED0(ctx, base);
	// 8327FD7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FD80: 38690DD8  addi r3, r9, 0xdd8
	ctx.r[3].s64 = ctx.r[9].s64 + 3544;
	// 8327FD84: 4BA2A19D  bl 0x82ca9f20
	ctx.lr = 0x8327FD88;
	sub_82CA9F20(ctx, base);
	// 8327FD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FD98 size=64
    let mut pc: u32 = 0x8327FD98;
    'dispatch: loop {
        match pc {
            0x8327FD98 => {
    //   block [0x8327FD98..0x8327FDD8)
	// 8327FD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FDA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FDAC: 388BF5FC  addi r4, r11, -0xa04
	ctx.r[4].s64 = ctx.r[11].s64 + -2564;
	// 8327FDB0: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 8327FDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FDB8: 4AFAD119  bl 0x8222ced0
	ctx.lr = 0x8327FDBC;
	sub_8222CED0(ctx, base);
	// 8327FDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FDC0: 38690DE8  addi r3, r9, 0xde8
	ctx.r[3].s64 = ctx.r[9].s64 + 3560;
	// 8327FDC4: 4BA2A15D  bl 0x82ca9f20
	ctx.lr = 0x8327FDC8;
	sub_82CA9F20(ctx, base);
	// 8327FDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FDD8 size=64
    let mut pc: u32 = 0x8327FDD8;
    'dispatch: loop {
        match pc {
            0x8327FDD8 => {
    //   block [0x8327FDD8..0x8327FE18)
	// 8327FDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FDE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FDEC: 388BF620  addi r4, r11, -0x9e0
	ctx.r[4].s64 = ctx.r[11].s64 + -2528;
	// 8327FDF0: 386ADEB8  addi r3, r10, -0x2148
	ctx.r[3].s64 = ctx.r[10].s64 + -8520;
	// 8327FDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FDF8: 4AFAD0D9  bl 0x8222ced0
	ctx.lr = 0x8327FDFC;
	sub_8222CED0(ctx, base);
	// 8327FDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE00: 38690DF8  addi r3, r9, 0xdf8
	ctx.r[3].s64 = ctx.r[9].s64 + 3576;
	// 8327FE04: 4BA2A11D  bl 0x82ca9f20
	ctx.lr = 0x8327FE08;
	sub_82CA9F20(ctx, base);
	// 8327FE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE18 size=64
    let mut pc: u32 = 0x8327FE18;
    'dispatch: loop {
        match pc {
            0x8327FE18 => {
    //   block [0x8327FE18..0x8327FE58)
	// 8327FE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FE24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FE2C: 388BF634  addi r4, r11, -0x9cc
	ctx.r[4].s64 = ctx.r[11].s64 + -2508;
	// 8327FE30: 386ADEBC  addi r3, r10, -0x2144
	ctx.r[3].s64 = ctx.r[10].s64 + -8516;
	// 8327FE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FE38: 4AFAD099  bl 0x8222ced0
	ctx.lr = 0x8327FE3C;
	sub_8222CED0(ctx, base);
	// 8327FE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE40: 38690E08  addi r3, r9, 0xe08
	ctx.r[3].s64 = ctx.r[9].s64 + 3592;
	// 8327FE44: 4BA2A0DD  bl 0x82ca9f20
	ctx.lr = 0x8327FE48;
	sub_82CA9F20(ctx, base);
	// 8327FE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE58 size=64
    let mut pc: u32 = 0x8327FE58;
    'dispatch: loop {
        match pc {
            0x8327FE58 => {
    //   block [0x8327FE58..0x8327FE98)
	// 8327FE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FE64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FE6C: 388BF648  addi r4, r11, -0x9b8
	ctx.r[4].s64 = ctx.r[11].s64 + -2488;
	// 8327FE70: 386ADEC0  addi r3, r10, -0x2140
	ctx.r[3].s64 = ctx.r[10].s64 + -8512;
	// 8327FE74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FE78: 4AFAD059  bl 0x8222ced0
	ctx.lr = 0x8327FE7C;
	sub_8222CED0(ctx, base);
	// 8327FE7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FE80: 38690E18  addi r3, r9, 0xe18
	ctx.r[3].s64 = ctx.r[9].s64 + 3608;
	// 8327FE84: 4BA2A09D  bl 0x82ca9f20
	ctx.lr = 0x8327FE88;
	sub_82CA9F20(ctx, base);
	// 8327FE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FE98 size=64
    let mut pc: u32 = 0x8327FE98;
    'dispatch: loop {
        match pc {
            0x8327FE98 => {
    //   block [0x8327FE98..0x8327FED8)
	// 8327FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FEA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FEA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FEAC: 388BF65C  addi r4, r11, -0x9a4
	ctx.r[4].s64 = ctx.r[11].s64 + -2468;
	// 8327FEB0: 386ADEC4  addi r3, r10, -0x213c
	ctx.r[3].s64 = ctx.r[10].s64 + -8508;
	// 8327FEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FEB8: 4AFAD019  bl 0x8222ced0
	ctx.lr = 0x8327FEBC;
	sub_8222CED0(ctx, base);
	// 8327FEBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FEC0: 38690E28  addi r3, r9, 0xe28
	ctx.r[3].s64 = ctx.r[9].s64 + 3624;
	// 8327FEC4: 4BA2A05D  bl 0x82ca9f20
	ctx.lr = 0x8327FEC8;
	sub_82CA9F20(ctx, base);
	// 8327FEC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FED8 size=64
    let mut pc: u32 = 0x8327FED8;
    'dispatch: loop {
        match pc {
            0x8327FED8 => {
    //   block [0x8327FED8..0x8327FF18)
	// 8327FED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FEE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FEE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FEE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FEEC: 388BF670  addi r4, r11, -0x990
	ctx.r[4].s64 = ctx.r[11].s64 + -2448;
	// 8327FEF0: 386ADEC8  addi r3, r10, -0x2138
	ctx.r[3].s64 = ctx.r[10].s64 + -8504;
	// 8327FEF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FEF8: 4AFACFD9  bl 0x8222ced0
	ctx.lr = 0x8327FEFC;
	sub_8222CED0(ctx, base);
	// 8327FEFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF00: 38690E38  addi r3, r9, 0xe38
	ctx.r[3].s64 = ctx.r[9].s64 + 3640;
	// 8327FF04: 4BA2A01D  bl 0x82ca9f20
	ctx.lr = 0x8327FF08;
	sub_82CA9F20(ctx, base);
	// 8327FF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF18 size=64
    let mut pc: u32 = 0x8327FF18;
    'dispatch: loop {
        match pc {
            0x8327FF18 => {
    //   block [0x8327FF18..0x8327FF58)
	// 8327FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FF24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FF2C: 388BF688  addi r4, r11, -0x978
	ctx.r[4].s64 = ctx.r[11].s64 + -2424;
	// 8327FF30: 386ADECC  addi r3, r10, -0x2134
	ctx.r[3].s64 = ctx.r[10].s64 + -8500;
	// 8327FF34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FF38: 4AFACF99  bl 0x8222ced0
	ctx.lr = 0x8327FF3C;
	sub_8222CED0(ctx, base);
	// 8327FF3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF40: 38690E48  addi r3, r9, 0xe48
	ctx.r[3].s64 = ctx.r[9].s64 + 3656;
	// 8327FF44: 4BA29FDD  bl 0x82ca9f20
	ctx.lr = 0x8327FF48;
	sub_82CA9F20(ctx, base);
	// 8327FF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF58 size=64
    let mut pc: u32 = 0x8327FF58;
    'dispatch: loop {
        match pc {
            0x8327FF58 => {
    //   block [0x8327FF58..0x8327FF98)
	// 8327FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FF64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FF6C: 388BF6A0  addi r4, r11, -0x960
	ctx.r[4].s64 = ctx.r[11].s64 + -2400;
	// 8327FF70: 386ADED0  addi r3, r10, -0x2130
	ctx.r[3].s64 = ctx.r[10].s64 + -8496;
	// 8327FF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FF78: 4AFACF59  bl 0x8222ced0
	ctx.lr = 0x8327FF7C;
	sub_8222CED0(ctx, base);
	// 8327FF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FF80: 38690E58  addi r3, r9, 0xe58
	ctx.r[3].s64 = ctx.r[9].s64 + 3672;
	// 8327FF84: 4BA29F9D  bl 0x82ca9f20
	ctx.lr = 0x8327FF88;
	sub_82CA9F20(ctx, base);
	// 8327FF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FF98 size=64
    let mut pc: u32 = 0x8327FF98;
    'dispatch: loop {
        match pc {
            0x8327FF98 => {
    //   block [0x8327FF98..0x8327FFD8)
	// 8327FF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FFA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FFAC: 388BF6B8  addi r4, r11, -0x948
	ctx.r[4].s64 = ctx.r[11].s64 + -2376;
	// 8327FFB0: 386ADED4  addi r3, r10, -0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + -8492;
	// 8327FFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FFB8: 4AFACF19  bl 0x8222ced0
	ctx.lr = 0x8327FFBC;
	sub_8222CED0(ctx, base);
	// 8327FFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8327FFC0: 38690E68  addi r3, r9, 0xe68
	ctx.r[3].s64 = ctx.r[9].s64 + 3688;
	// 8327FFC4: 4BA29F5D  bl 0x82ca9f20
	ctx.lr = 0x8327FFC8;
	sub_82CA9F20(ctx, base);
	// 8327FFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327FFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8327FFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327FFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8327FFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8327FFD8 size=64
    let mut pc: u32 = 0x8327FFD8;
    'dispatch: loop {
        match pc {
            0x8327FFD8 => {
    //   block [0x8327FFD8..0x83280018)
	// 8327FFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327FFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8327FFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327FFE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8327FFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327FFEC: 388BF6D0  addi r4, r11, -0x930
	ctx.r[4].s64 = ctx.r[11].s64 + -2352;
	// 8327FFF0: 386ADED8  addi r3, r10, -0x2128
	ctx.r[3].s64 = ctx.r[10].s64 + -8488;
	// 8327FFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8327FFF8: 4AFACED9  bl 0x8222ced0
	ctx.lr = 0x8327FFFC;
	sub_8222CED0(ctx, base);
	// 8327FFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280000: 38690E78  addi r3, r9, 0xe78
	ctx.r[3].s64 = ctx.r[9].s64 + 3704;
	// 83280004: 4BA29F1D  bl 0x82ca9f20
	ctx.lr = 0x83280008;
	sub_82CA9F20(ctx, base);
	// 83280008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328000C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280018 size=64
    let mut pc: u32 = 0x83280018;
    'dispatch: loop {
        match pc {
            0x83280018 => {
    //   block [0x83280018..0x83280058)
	// 83280018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328001C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280024: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328002C: 388BF6E8  addi r4, r11, -0x918
	ctx.r[4].s64 = ctx.r[11].s64 + -2328;
	// 83280030: 386ADEDC  addi r3, r10, -0x2124
	ctx.r[3].s64 = ctx.r[10].s64 + -8484;
	// 83280034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280038: 4AFACE99  bl 0x8222ced0
	ctx.lr = 0x8328003C;
	sub_8222CED0(ctx, base);
	// 8328003C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280040: 38690E88  addi r3, r9, 0xe88
	ctx.r[3].s64 = ctx.r[9].s64 + 3720;
	// 83280044: 4BA29EDD  bl 0x82ca9f20
	ctx.lr = 0x83280048;
	sub_82CA9F20(ctx, base);
	// 83280048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328004C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280058 size=64
    let mut pc: u32 = 0x83280058;
    'dispatch: loop {
        match pc {
            0x83280058 => {
    //   block [0x83280058..0x83280098)
	// 83280058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328005C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280064: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328006C: 388BF704  addi r4, r11, -0x8fc
	ctx.r[4].s64 = ctx.r[11].s64 + -2300;
	// 83280070: 386ADEE0  addi r3, r10, -0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + -8480;
	// 83280074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280078: 4AFACE59  bl 0x8222ced0
	ctx.lr = 0x8328007C;
	sub_8222CED0(ctx, base);
	// 8328007C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280080: 38690E98  addi r3, r9, 0xe98
	ctx.r[3].s64 = ctx.r[9].s64 + 3736;
	// 83280084: 4BA29E9D  bl 0x82ca9f20
	ctx.lr = 0x83280088;
	sub_82CA9F20(ctx, base);
	// 83280088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328008C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280098 size=64
    let mut pc: u32 = 0x83280098;
    'dispatch: loop {
        match pc {
            0x83280098 => {
    //   block [0x83280098..0x832800D8)
	// 83280098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328009C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832800A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832800A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832800A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832800AC: 388BF71C  addi r4, r11, -0x8e4
	ctx.r[4].s64 = ctx.r[11].s64 + -2276;
	// 832800B0: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 832800B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832800B8: 4AFACE19  bl 0x8222ced0
	ctx.lr = 0x832800BC;
	sub_8222CED0(ctx, base);
	// 832800BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832800C0: 38690EA8  addi r3, r9, 0xea8
	ctx.r[3].s64 = ctx.r[9].s64 + 3752;
	// 832800C4: 4BA29E5D  bl 0x82ca9f20
	ctx.lr = 0x832800C8;
	sub_82CA9F20(ctx, base);
	// 832800C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832800CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832800D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832800D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832800D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832800D8 size=64
    let mut pc: u32 = 0x832800D8;
    'dispatch: loop {
        match pc {
            0x832800D8 => {
    //   block [0x832800D8..0x83280118)
	// 832800D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832800DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832800E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832800E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832800E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832800EC: 388BF73C  addi r4, r11, -0x8c4
	ctx.r[4].s64 = ctx.r[11].s64 + -2244;
	// 832800F0: 386ADEE8  addi r3, r10, -0x2118
	ctx.r[3].s64 = ctx.r[10].s64 + -8472;
	// 832800F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832800F8: 4AFACDD9  bl 0x8222ced0
	ctx.lr = 0x832800FC;
	sub_8222CED0(ctx, base);
	// 832800FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280100: 38690EB8  addi r3, r9, 0xeb8
	ctx.r[3].s64 = ctx.r[9].s64 + 3768;
	// 83280104: 4BA29E1D  bl 0x82ca9f20
	ctx.lr = 0x83280108;
	sub_82CA9F20(ctx, base);
	// 83280108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280118 size=64
    let mut pc: u32 = 0x83280118;
    'dispatch: loop {
        match pc {
            0x83280118 => {
    //   block [0x83280118..0x83280158)
	// 83280118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280124: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328012C: 388BF75C  addi r4, r11, -0x8a4
	ctx.r[4].s64 = ctx.r[11].s64 + -2212;
	// 83280130: 386ADEEC  addi r3, r10, -0x2114
	ctx.r[3].s64 = ctx.r[10].s64 + -8468;
	// 83280134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280138: 4AFACD99  bl 0x8222ced0
	ctx.lr = 0x8328013C;
	sub_8222CED0(ctx, base);
	// 8328013C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280140: 38690EC8  addi r3, r9, 0xec8
	ctx.r[3].s64 = ctx.r[9].s64 + 3784;
	// 83280144: 4BA29DDD  bl 0x82ca9f20
	ctx.lr = 0x83280148;
	sub_82CA9F20(ctx, base);
	// 83280148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280158 size=64
    let mut pc: u32 = 0x83280158;
    'dispatch: loop {
        match pc {
            0x83280158 => {
    //   block [0x83280158..0x83280198)
	// 83280158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280164: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328016C: 388BF774  addi r4, r11, -0x88c
	ctx.r[4].s64 = ctx.r[11].s64 + -2188;
	// 83280170: 386ADEF0  addi r3, r10, -0x2110
	ctx.r[3].s64 = ctx.r[10].s64 + -8464;
	// 83280174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280178: 4AFACD59  bl 0x8222ced0
	ctx.lr = 0x8328017C;
	sub_8222CED0(ctx, base);
	// 8328017C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280180: 38690ED8  addi r3, r9, 0xed8
	ctx.r[3].s64 = ctx.r[9].s64 + 3800;
	// 83280184: 4BA29D9D  bl 0x82ca9f20
	ctx.lr = 0x83280188;
	sub_82CA9F20(ctx, base);
	// 83280188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328018C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280198 size=64
    let mut pc: u32 = 0x83280198;
    'dispatch: loop {
        match pc {
            0x83280198 => {
    //   block [0x83280198..0x832801D8)
	// 83280198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832801A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832801A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832801A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832801AC: 388BF78C  addi r4, r11, -0x874
	ctx.r[4].s64 = ctx.r[11].s64 + -2164;
	// 832801B0: 386ADEF4  addi r3, r10, -0x210c
	ctx.r[3].s64 = ctx.r[10].s64 + -8460;
	// 832801B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832801B8: 4AFACD19  bl 0x8222ced0
	ctx.lr = 0x832801BC;
	sub_8222CED0(ctx, base);
	// 832801BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832801C0: 38690EE8  addi r3, r9, 0xee8
	ctx.r[3].s64 = ctx.r[9].s64 + 3816;
	// 832801C4: 4BA29D5D  bl 0x82ca9f20
	ctx.lr = 0x832801C8;
	sub_82CA9F20(ctx, base);
	// 832801C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832801CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832801D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832801D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832801D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832801D8 size=64
    let mut pc: u32 = 0x832801D8;
    'dispatch: loop {
        match pc {
            0x832801D8 => {
    //   block [0x832801D8..0x83280218)
	// 832801D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832801DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832801E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832801E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832801E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832801EC: 388BF7A4  addi r4, r11, -0x85c
	ctx.r[4].s64 = ctx.r[11].s64 + -2140;
	// 832801F0: 386ADEF8  addi r3, r10, -0x2108
	ctx.r[3].s64 = ctx.r[10].s64 + -8456;
	// 832801F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832801F8: 4AFACCD9  bl 0x8222ced0
	ctx.lr = 0x832801FC;
	sub_8222CED0(ctx, base);
	// 832801FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280200: 38690EF8  addi r3, r9, 0xef8
	ctx.r[3].s64 = ctx.r[9].s64 + 3832;
	// 83280204: 4BA29D1D  bl 0x82ca9f20
	ctx.lr = 0x83280208;
	sub_82CA9F20(ctx, base);
	// 83280208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280218 size=64
    let mut pc: u32 = 0x83280218;
    'dispatch: loop {
        match pc {
            0x83280218 => {
    //   block [0x83280218..0x83280258)
	// 83280218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280224: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328022C: 388BF7CC  addi r4, r11, -0x834
	ctx.r[4].s64 = ctx.r[11].s64 + -2100;
	// 83280230: 386ADEFC  addi r3, r10, -0x2104
	ctx.r[3].s64 = ctx.r[10].s64 + -8452;
	// 83280234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280238: 4AFACC99  bl 0x8222ced0
	ctx.lr = 0x8328023C;
	sub_8222CED0(ctx, base);
	// 8328023C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280240: 38690F08  addi r3, r9, 0xf08
	ctx.r[3].s64 = ctx.r[9].s64 + 3848;
	// 83280244: 4BA29CDD  bl 0x82ca9f20
	ctx.lr = 0x83280248;
	sub_82CA9F20(ctx, base);
	// 83280248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280258 size=64
    let mut pc: u32 = 0x83280258;
    'dispatch: loop {
        match pc {
            0x83280258 => {
    //   block [0x83280258..0x83280298)
	// 83280258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280264: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328026C: 388BF7EC  addi r4, r11, -0x814
	ctx.r[4].s64 = ctx.r[11].s64 + -2068;
	// 83280270: 386ADF00  addi r3, r10, -0x2100
	ctx.r[3].s64 = ctx.r[10].s64 + -8448;
	// 83280274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280278: 4AFACC59  bl 0x8222ced0
	ctx.lr = 0x8328027C;
	sub_8222CED0(ctx, base);
	// 8328027C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280280: 38690F18  addi r3, r9, 0xf18
	ctx.r[3].s64 = ctx.r[9].s64 + 3864;
	// 83280284: 4BA29C9D  bl 0x82ca9f20
	ctx.lr = 0x83280288;
	sub_82CA9F20(ctx, base);
	// 83280288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328028C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280298 size=64
    let mut pc: u32 = 0x83280298;
    'dispatch: loop {
        match pc {
            0x83280298 => {
    //   block [0x83280298..0x832802D8)
	// 83280298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328029C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832802A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832802A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832802A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832802AC: 388BF808  addi r4, r11, -0x7f8
	ctx.r[4].s64 = ctx.r[11].s64 + -2040;
	// 832802B0: 386ADF04  addi r3, r10, -0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8444;
	// 832802B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832802B8: 4AFACC19  bl 0x8222ced0
	ctx.lr = 0x832802BC;
	sub_8222CED0(ctx, base);
	// 832802BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832802C0: 38690F28  addi r3, r9, 0xf28
	ctx.r[3].s64 = ctx.r[9].s64 + 3880;
	// 832802C4: 4BA29C5D  bl 0x82ca9f20
	ctx.lr = 0x832802C8;
	sub_82CA9F20(ctx, base);
	// 832802C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832802CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832802D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832802D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832802D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832802D8 size=64
    let mut pc: u32 = 0x832802D8;
    'dispatch: loop {
        match pc {
            0x832802D8 => {
    //   block [0x832802D8..0x83280318)
	// 832802D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832802DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832802E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832802E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832802E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832802EC: 388BF824  addi r4, r11, -0x7dc
	ctx.r[4].s64 = ctx.r[11].s64 + -2012;
	// 832802F0: 386ADF08  addi r3, r10, -0x20f8
	ctx.r[3].s64 = ctx.r[10].s64 + -8440;
	// 832802F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832802F8: 4AFACBD9  bl 0x8222ced0
	ctx.lr = 0x832802FC;
	sub_8222CED0(ctx, base);
	// 832802FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280300: 38690F38  addi r3, r9, 0xf38
	ctx.r[3].s64 = ctx.r[9].s64 + 3896;
	// 83280304: 4BA29C1D  bl 0x82ca9f20
	ctx.lr = 0x83280308;
	sub_82CA9F20(ctx, base);
	// 83280308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328030C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280318 size=64
    let mut pc: u32 = 0x83280318;
    'dispatch: loop {
        match pc {
            0x83280318 => {
    //   block [0x83280318..0x83280358)
	// 83280318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328031C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280324: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328032C: 388BF83C  addi r4, r11, -0x7c4
	ctx.r[4].s64 = ctx.r[11].s64 + -1988;
	// 83280330: 386ADF0C  addi r3, r10, -0x20f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8436;
	// 83280334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280338: 4AFACB99  bl 0x8222ced0
	ctx.lr = 0x8328033C;
	sub_8222CED0(ctx, base);
	// 8328033C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280340: 38690F48  addi r3, r9, 0xf48
	ctx.r[3].s64 = ctx.r[9].s64 + 3912;
	// 83280344: 4BA29BDD  bl 0x82ca9f20
	ctx.lr = 0x83280348;
	sub_82CA9F20(ctx, base);
	// 83280348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280358 size=64
    let mut pc: u32 = 0x83280358;
    'dispatch: loop {
        match pc {
            0x83280358 => {
    //   block [0x83280358..0x83280398)
	// 83280358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280364: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328036C: 388BF850  addi r4, r11, -0x7b0
	ctx.r[4].s64 = ctx.r[11].s64 + -1968;
	// 83280370: 386ADF10  addi r3, r10, -0x20f0
	ctx.r[3].s64 = ctx.r[10].s64 + -8432;
	// 83280374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280378: 4AFACB59  bl 0x8222ced0
	ctx.lr = 0x8328037C;
	sub_8222CED0(ctx, base);
	// 8328037C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280380: 38690F58  addi r3, r9, 0xf58
	ctx.r[3].s64 = ctx.r[9].s64 + 3928;
	// 83280384: 4BA29B9D  bl 0x82ca9f20
	ctx.lr = 0x83280388;
	sub_82CA9F20(ctx, base);
	// 83280388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328038C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280398 size=64
    let mut pc: u32 = 0x83280398;
    'dispatch: loop {
        match pc {
            0x83280398 => {
    //   block [0x83280398..0x832803D8)
	// 83280398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832803A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832803A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832803A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832803AC: 388BF86C  addi r4, r11, -0x794
	ctx.r[4].s64 = ctx.r[11].s64 + -1940;
	// 832803B0: 386ADF14  addi r3, r10, -0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8428;
	// 832803B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832803B8: 4AFACB19  bl 0x8222ced0
	ctx.lr = 0x832803BC;
	sub_8222CED0(ctx, base);
	// 832803BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832803C0: 38690F68  addi r3, r9, 0xf68
	ctx.r[3].s64 = ctx.r[9].s64 + 3944;
	// 832803C4: 4BA29B5D  bl 0x82ca9f20
	ctx.lr = 0x832803C8;
	sub_82CA9F20(ctx, base);
	// 832803C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832803CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832803D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832803D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832803D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832803D8 size=64
    let mut pc: u32 = 0x832803D8;
    'dispatch: loop {
        match pc {
            0x832803D8 => {
    //   block [0x832803D8..0x83280418)
	// 832803D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832803DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832803E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832803E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832803E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832803EC: 388BF884  addi r4, r11, -0x77c
	ctx.r[4].s64 = ctx.r[11].s64 + -1916;
	// 832803F0: 386ADF18  addi r3, r10, -0x20e8
	ctx.r[3].s64 = ctx.r[10].s64 + -8424;
	// 832803F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832803F8: 4AFACAD9  bl 0x8222ced0
	ctx.lr = 0x832803FC;
	sub_8222CED0(ctx, base);
	// 832803FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280400: 38690F78  addi r3, r9, 0xf78
	ctx.r[3].s64 = ctx.r[9].s64 + 3960;
	// 83280404: 4BA29B1D  bl 0x82ca9f20
	ctx.lr = 0x83280408;
	sub_82CA9F20(ctx, base);
	// 83280408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280418 size=64
    let mut pc: u32 = 0x83280418;
    'dispatch: loop {
        match pc {
            0x83280418 => {
    //   block [0x83280418..0x83280458)
	// 83280418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328042C: 388BF8A0  addi r4, r11, -0x760
	ctx.r[4].s64 = ctx.r[11].s64 + -1888;
	// 83280430: 386ADF1C  addi r3, r10, -0x20e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8420;
	// 83280434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280438: 4AFACA99  bl 0x8222ced0
	ctx.lr = 0x8328043C;
	sub_8222CED0(ctx, base);
	// 8328043C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280440: 38690F88  addi r3, r9, 0xf88
	ctx.r[3].s64 = ctx.r[9].s64 + 3976;
	// 83280444: 4BA29ADD  bl 0x82ca9f20
	ctx.lr = 0x83280448;
	sub_82CA9F20(ctx, base);
	// 83280448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328044C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280458 size=64
    let mut pc: u32 = 0x83280458;
    'dispatch: loop {
        match pc {
            0x83280458 => {
    //   block [0x83280458..0x83280498)
	// 83280458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328045C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328046C: 388BF8B4  addi r4, r11, -0x74c
	ctx.r[4].s64 = ctx.r[11].s64 + -1868;
	// 83280470: 386ADF20  addi r3, r10, -0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + -8416;
	// 83280474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280478: 4AFACA59  bl 0x8222ced0
	ctx.lr = 0x8328047C;
	sub_8222CED0(ctx, base);
	// 8328047C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280480: 38690F98  addi r3, r9, 0xf98
	ctx.r[3].s64 = ctx.r[9].s64 + 3992;
	// 83280484: 4BA29A9D  bl 0x82ca9f20
	ctx.lr = 0x83280488;
	sub_82CA9F20(ctx, base);
	// 83280488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328048C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280498 size=64
    let mut pc: u32 = 0x83280498;
    'dispatch: loop {
        match pc {
            0x83280498 => {
    //   block [0x83280498..0x832804D8)
	// 83280498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328049C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832804A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832804A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832804A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832804AC: 388BF8C0  addi r4, r11, -0x740
	ctx.r[4].s64 = ctx.r[11].s64 + -1856;
	// 832804B0: 386ADF24  addi r3, r10, -0x20dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8412;
	// 832804B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832804B8: 4AFACA19  bl 0x8222ced0
	ctx.lr = 0x832804BC;
	sub_8222CED0(ctx, base);
	// 832804BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832804C0: 38690FA8  addi r3, r9, 0xfa8
	ctx.r[3].s64 = ctx.r[9].s64 + 4008;
	// 832804C4: 4BA29A5D  bl 0x82ca9f20
	ctx.lr = 0x832804C8;
	sub_82CA9F20(ctx, base);
	// 832804C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832804CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832804D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832804D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832804D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832804D8 size=64
    let mut pc: u32 = 0x832804D8;
    'dispatch: loop {
        match pc {
            0x832804D8 => {
    //   block [0x832804D8..0x83280518)
	// 832804D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832804DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832804E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832804E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832804E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832804EC: 388BF8D4  addi r4, r11, -0x72c
	ctx.r[4].s64 = ctx.r[11].s64 + -1836;
	// 832804F0: 386ADF28  addi r3, r10, -0x20d8
	ctx.r[3].s64 = ctx.r[10].s64 + -8408;
	// 832804F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832804F8: 4AFAC9D9  bl 0x8222ced0
	ctx.lr = 0x832804FC;
	sub_8222CED0(ctx, base);
	// 832804FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280500: 38690FB8  addi r3, r9, 0xfb8
	ctx.r[3].s64 = ctx.r[9].s64 + 4024;
	// 83280504: 4BA29A1D  bl 0x82ca9f20
	ctx.lr = 0x83280508;
	sub_82CA9F20(ctx, base);
	// 83280508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328050C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280518 size=56
    let mut pc: u32 = 0x83280518;
    'dispatch: loop {
        match pc {
            0x83280518 => {
    //   block [0x83280518..0x83280550)
	// 83280518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328052C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83280530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280534: 4AF73825  bl 0x821f3d58
	ctx.lr = 0x83280538;
	sub_821F3D58(ctx, base);
	// 83280538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328053C: 906ADF2C  stw r3, -0x20d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8404 as u32), ctx.r[3].u32 ) };
	// 83280540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328054C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280550 size=56
    let mut pc: u32 = 0x83280550;
    'dispatch: loop {
        match pc {
            0x83280550 => {
    //   block [0x83280550..0x83280588)
	// 83280550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328055C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280564: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83280568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328056C: 4AF737ED  bl 0x821f3d58
	ctx.lr = 0x83280570;
	sub_821F3D58(ctx, base);
	// 83280570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280574: 906ADF30  stw r3, -0x20d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8400 as u32), ctx.r[3].u32 ) };
	// 83280578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328057C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280588 size=56
    let mut pc: u32 = 0x83280588;
    'dispatch: loop {
        match pc {
            0x83280588 => {
    //   block [0x83280588..0x832805C0)
	// 83280588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328059C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832805A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832805A4: 4AF737B5  bl 0x821f3d58
	ctx.lr = 0x832805A8;
	sub_821F3D58(ctx, base);
	// 832805A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832805AC: 906ADF34  stw r3, -0x20cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8396 as u32), ctx.r[3].u32 ) };
	// 832805B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832805B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832805B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832805BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832805C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832805C0 size=56
    let mut pc: u32 = 0x832805C0;
    'dispatch: loop {
        match pc {
            0x832805C0 => {
    //   block [0x832805C0..0x832805F8)
	// 832805C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832805C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832805C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832805CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832805D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832805D4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832805D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832805DC: 4AF7377D  bl 0x821f3d58
	ctx.lr = 0x832805E0;
	sub_821F3D58(ctx, base);
	// 832805E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832805E4: 906ADF38  stw r3, -0x20c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8392 as u32), ctx.r[3].u32 ) };
	// 832805E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832805EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832805F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832805F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832805F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832805F8 size=56
    let mut pc: u32 = 0x832805F8;
    'dispatch: loop {
        match pc {
            0x832805F8 => {
    //   block [0x832805F8..0x83280630)
	// 832805F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832805FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328060C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83280610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280614: 4AF73745  bl 0x821f3d58
	ctx.lr = 0x83280618;
	sub_821F3D58(ctx, base);
	// 83280618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328061C: 906ADF3C  stw r3, -0x20c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8388 as u32), ctx.r[3].u32 ) };
	// 83280620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328062C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280630 size=56
    let mut pc: u32 = 0x83280630;
    'dispatch: loop {
        match pc {
            0x83280630 => {
    //   block [0x83280630..0x83280668)
	// 83280630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328063C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280644: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83280648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328064C: 4AF7370D  bl 0x821f3d58
	ctx.lr = 0x83280650;
	sub_821F3D58(ctx, base);
	// 83280650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280654: 906ADF40  stw r3, -0x20c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8384 as u32), ctx.r[3].u32 ) };
	// 83280658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328065C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280668 size=56
    let mut pc: u32 = 0x83280668;
    'dispatch: loop {
        match pc {
            0x83280668 => {
    //   block [0x83280668..0x832806A0)
	// 83280668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328066C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328067C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83280680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280684: 4AF736D5  bl 0x821f3d58
	ctx.lr = 0x83280688;
	sub_821F3D58(ctx, base);
	// 83280688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328068C: 906ADF44  stw r3, -0x20bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8380 as u32), ctx.r[3].u32 ) };
	// 83280690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328069C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832806A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832806A0 size=56
    let mut pc: u32 = 0x832806A0;
    'dispatch: loop {
        match pc {
            0x832806A0 => {
    //   block [0x832806A0..0x832806D8)
	// 832806A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832806A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832806A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832806AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832806B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832806B4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832806B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832806BC: 4AF7369D  bl 0x821f3d58
	ctx.lr = 0x832806C0;
	sub_821F3D58(ctx, base);
	// 832806C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832806C4: 906ADF48  stw r3, -0x20b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8376 as u32), ctx.r[3].u32 ) };
	// 832806C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832806CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832806D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832806D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832806D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832806D8 size=56
    let mut pc: u32 = 0x832806D8;
    'dispatch: loop {
        match pc {
            0x832806D8 => {
    //   block [0x832806D8..0x83280710)
	// 832806D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832806DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832806E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832806E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832806E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832806EC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832806F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832806F4: 4AF73665  bl 0x821f3d58
	ctx.lr = 0x832806F8;
	sub_821F3D58(ctx, base);
	// 832806F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832806FC: 906ADF4C  stw r3, -0x20b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8372 as u32), ctx.r[3].u32 ) };
	// 83280700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328070C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280710 size=56
    let mut pc: u32 = 0x83280710;
    'dispatch: loop {
        match pc {
            0x83280710 => {
    //   block [0x83280710..0x83280748)
	// 83280710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328071C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280724: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83280728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328072C: 4AF7362D  bl 0x821f3d58
	ctx.lr = 0x83280730;
	sub_821F3D58(ctx, base);
	// 83280730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280734: 906ADF50  stw r3, -0x20b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8368 as u32), ctx.r[3].u32 ) };
	// 83280738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328073C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280748 size=56
    let mut pc: u32 = 0x83280748;
    'dispatch: loop {
        match pc {
            0x83280748 => {
    //   block [0x83280748..0x83280780)
	// 83280748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328075C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83280760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280764: 4AF735F5  bl 0x821f3d58
	ctx.lr = 0x83280768;
	sub_821F3D58(ctx, base);
	// 83280768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328076C: 906ADF54  stw r3, -0x20ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8364 as u32), ctx.r[3].u32 ) };
	// 83280770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328077C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280780 size=56
    let mut pc: u32 = 0x83280780;
    'dispatch: loop {
        match pc {
            0x83280780 => {
    //   block [0x83280780..0x832807B8)
	// 83280780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328078C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280794: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83280798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328079C: 4AF735BD  bl 0x821f3d58
	ctx.lr = 0x832807A0;
	sub_821F3D58(ctx, base);
	// 832807A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832807A4: 906ADF58  stw r3, -0x20a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8360 as u32), ctx.r[3].u32 ) };
	// 832807A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832807AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832807B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832807B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832807B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832807B8 size=56
    let mut pc: u32 = 0x832807B8;
    'dispatch: loop {
        match pc {
            0x832807B8 => {
    //   block [0x832807B8..0x832807F0)
	// 832807B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832807BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832807C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832807C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832807C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832807CC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832807D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832807D4: 4AF73585  bl 0x821f3d58
	ctx.lr = 0x832807D8;
	sub_821F3D58(ctx, base);
	// 832807D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832807DC: 906ADF5C  stw r3, -0x20a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8356 as u32), ctx.r[3].u32 ) };
	// 832807E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832807E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832807E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832807EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832807F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832807F0 size=56
    let mut pc: u32 = 0x832807F0;
    'dispatch: loop {
        match pc {
            0x832807F0 => {
    //   block [0x832807F0..0x83280828)
	// 832807F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832807F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832807F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832807FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280804: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83280808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328080C: 4AF7354D  bl 0x821f3d58
	ctx.lr = 0x83280810;
	sub_821F3D58(ctx, base);
	// 83280810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280814: 906ADF60  stw r3, -0x20a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8352 as u32), ctx.r[3].u32 ) };
	// 83280818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328081C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280828 size=56
    let mut pc: u32 = 0x83280828;
    'dispatch: loop {
        match pc {
            0x83280828 => {
    //   block [0x83280828..0x83280860)
	// 83280828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328082C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328083C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83280840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280844: 4AF73515  bl 0x821f3d58
	ctx.lr = 0x83280848;
	sub_821F3D58(ctx, base);
	// 83280848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328084C: 906ADF64  stw r3, -0x209c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8348 as u32), ctx.r[3].u32 ) };
	// 83280850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328085C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280860 size=56
    let mut pc: u32 = 0x83280860;
    'dispatch: loop {
        match pc {
            0x83280860 => {
    //   block [0x83280860..0x83280898)
	// 83280860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328086C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280874: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83280878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328087C: 4AF734DD  bl 0x821f3d58
	ctx.lr = 0x83280880;
	sub_821F3D58(ctx, base);
	// 83280880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280884: 906ADF68  stw r3, -0x2098(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8344 as u32), ctx.r[3].u32 ) };
	// 83280888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328088C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280898 size=56
    let mut pc: u32 = 0x83280898;
    'dispatch: loop {
        match pc {
            0x83280898 => {
    //   block [0x83280898..0x832808D0)
	// 83280898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328089C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832808A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832808A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832808A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832808AC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832808B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832808B4: 4AF734A5  bl 0x821f3d58
	ctx.lr = 0x832808B8;
	sub_821F3D58(ctx, base);
	// 832808B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832808BC: 906ADF6C  stw r3, -0x2094(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8340 as u32), ctx.r[3].u32 ) };
	// 832808C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832808C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832808C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832808CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832808D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832808D0 size=56
    let mut pc: u32 = 0x832808D0;
    'dispatch: loop {
        match pc {
            0x832808D0 => {
    //   block [0x832808D0..0x83280908)
	// 832808D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832808D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832808D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832808DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832808E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832808E4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832808E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832808EC: 4AF7346D  bl 0x821f3d58
	ctx.lr = 0x832808F0;
	sub_821F3D58(ctx, base);
	// 832808F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832808F4: 906ADF70  stw r3, -0x2090(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8336 as u32), ctx.r[3].u32 ) };
	// 832808F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832808FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280908 size=56
    let mut pc: u32 = 0x83280908;
    'dispatch: loop {
        match pc {
            0x83280908 => {
    //   block [0x83280908..0x83280940)
	// 83280908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328091C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83280920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280924: 4AF73435  bl 0x821f3d58
	ctx.lr = 0x83280928;
	sub_821F3D58(ctx, base);
	// 83280928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328092C: 906ADF74  stw r3, -0x208c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8332 as u32), ctx.r[3].u32 ) };
	// 83280930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328093C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280940 size=56
    let mut pc: u32 = 0x83280940;
    'dispatch: loop {
        match pc {
            0x83280940 => {
    //   block [0x83280940..0x83280978)
	// 83280940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328094C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280954: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83280958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328095C: 4AF733FD  bl 0x821f3d58
	ctx.lr = 0x83280960;
	sub_821F3D58(ctx, base);
	// 83280960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280964: 906ADF78  stw r3, -0x2088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8328 as u32), ctx.r[3].u32 ) };
	// 83280968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328096C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280978 size=56
    let mut pc: u32 = 0x83280978;
    'dispatch: loop {
        match pc {
            0x83280978 => {
    //   block [0x83280978..0x832809B0)
	// 83280978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328098C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83280990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280994: 4AF733C5  bl 0x821f3d58
	ctx.lr = 0x83280998;
	sub_821F3D58(ctx, base);
	// 83280998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328099C: 906ADF7C  stw r3, -0x2084(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8324 as u32), ctx.r[3].u32 ) };
	// 832809A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832809A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832809A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832809AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832809B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832809B0 size=64
    let mut pc: u32 = 0x832809B0;
    'dispatch: loop {
        match pc {
            0x832809B0 => {
    //   block [0x832809B0..0x832809F0)
	// 832809B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832809B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832809B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832809BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832809C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832809C4: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 832809C8: 386ADF80  addi r3, r10, -0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + -8320;
	// 832809CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832809D0: 4AFAC501  bl 0x8222ced0
	ctx.lr = 0x832809D4;
	sub_8222CED0(ctx, base);
	// 832809D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832809D8: 38690FC8  addi r3, r9, 0xfc8
	ctx.r[3].s64 = ctx.r[9].s64 + 4040;
	// 832809DC: 4BA29545  bl 0x82ca9f20
	ctx.lr = 0x832809E0;
	sub_82CA9F20(ctx, base);
	// 832809E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832809E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832809E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832809EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832809F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832809F0 size=64
    let mut pc: u32 = 0x832809F0;
    'dispatch: loop {
        match pc {
            0x832809F0 => {
    //   block [0x832809F0..0x83280A30)
	// 832809F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832809F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832809F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832809FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A04: 388BF934  addi r4, r11, -0x6cc
	ctx.r[4].s64 = ctx.r[11].s64 + -1740;
	// 83280A08: 386ADF84  addi r3, r10, -0x207c
	ctx.r[3].s64 = ctx.r[10].s64 + -8316;
	// 83280A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A10: 4AFAC4C1  bl 0x8222ced0
	ctx.lr = 0x83280A14;
	sub_8222CED0(ctx, base);
	// 83280A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A18: 38690FD8  addi r3, r9, 0xfd8
	ctx.r[3].s64 = ctx.r[9].s64 + 4056;
	// 83280A1C: 4BA29505  bl 0x82ca9f20
	ctx.lr = 0x83280A20;
	sub_82CA9F20(ctx, base);
	// 83280A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280A30 size=64
    let mut pc: u32 = 0x83280A30;
    'dispatch: loop {
        match pc {
            0x83280A30 => {
    //   block [0x83280A30..0x83280A70)
	// 83280A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280A3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A44: 388BF968  addi r4, r11, -0x698
	ctx.r[4].s64 = ctx.r[11].s64 + -1688;
	// 83280A48: 386ADF88  addi r3, r10, -0x2078
	ctx.r[3].s64 = ctx.r[10].s64 + -8312;
	// 83280A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A50: 4AFAC481  bl 0x8222ced0
	ctx.lr = 0x83280A54;
	sub_8222CED0(ctx, base);
	// 83280A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A58: 38690FE8  addi r3, r9, 0xfe8
	ctx.r[3].s64 = ctx.r[9].s64 + 4072;
	// 83280A5C: 4BA294C5  bl 0x82ca9f20
	ctx.lr = 0x83280A60;
	sub_82CA9F20(ctx, base);
	// 83280A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280A70 size=64
    let mut pc: u32 = 0x83280A70;
    'dispatch: loop {
        match pc {
            0x83280A70 => {
    //   block [0x83280A70..0x83280AB0)
	// 83280A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280A7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280A84: 388BF9A0  addi r4, r11, -0x660
	ctx.r[4].s64 = ctx.r[11].s64 + -1632;
	// 83280A88: 386ADF8C  addi r3, r10, -0x2074
	ctx.r[3].s64 = ctx.r[10].s64 + -8308;
	// 83280A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280A90: 4AFAC441  bl 0x8222ced0
	ctx.lr = 0x83280A94;
	sub_8222CED0(ctx, base);
	// 83280A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280A98: 38690FF8  addi r3, r9, 0xff8
	ctx.r[3].s64 = ctx.r[9].s64 + 4088;
	// 83280A9C: 4BA29485  bl 0x82ca9f20
	ctx.lr = 0x83280AA0;
	sub_82CA9F20(ctx, base);
	// 83280AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280AB0 size=64
    let mut pc: u32 = 0x83280AB0;
    'dispatch: loop {
        match pc {
            0x83280AB0 => {
    //   block [0x83280AB0..0x83280AF0)
	// 83280AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280ABC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280AC4: 388BF9B4  addi r4, r11, -0x64c
	ctx.r[4].s64 = ctx.r[11].s64 + -1612;
	// 83280AC8: 386ADF90  addi r3, r10, -0x2070
	ctx.r[3].s64 = ctx.r[10].s64 + -8304;
	// 83280ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280AD0: 4AFAC401  bl 0x8222ced0
	ctx.lr = 0x83280AD4;
	sub_8222CED0(ctx, base);
	// 83280AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280AD8: 38691008  addi r3, r9, 0x1008
	ctx.r[3].s64 = ctx.r[9].s64 + 4104;
	// 83280ADC: 4BA29445  bl 0x82ca9f20
	ctx.lr = 0x83280AE0;
	sub_82CA9F20(ctx, base);
	// 83280AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280AF0 size=64
    let mut pc: u32 = 0x83280AF0;
    'dispatch: loop {
        match pc {
            0x83280AF0 => {
    //   block [0x83280AF0..0x83280B30)
	// 83280AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280AFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B04: 388BF9C4  addi r4, r11, -0x63c
	ctx.r[4].s64 = ctx.r[11].s64 + -1596;
	// 83280B08: 386ADF94  addi r3, r10, -0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + -8300;
	// 83280B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280B10: 4AFAC3C1  bl 0x8222ced0
	ctx.lr = 0x83280B14;
	sub_8222CED0(ctx, base);
	// 83280B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280B18: 38691018  addi r3, r9, 0x1018
	ctx.r[3].s64 = ctx.r[9].s64 + 4120;
	// 83280B1C: 4BA29405  bl 0x82ca9f20
	ctx.lr = 0x83280B20;
	sub_82CA9F20(ctx, base);
	// 83280B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280B30 size=64
    let mut pc: u32 = 0x83280B30;
    'dispatch: loop {
        match pc {
            0x83280B30 => {
    //   block [0x83280B30..0x83280B70)
	// 83280B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280B3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83280B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B44: 388BF9D4  addi r4, r11, -0x62c
	ctx.r[4].s64 = ctx.r[11].s64 + -1580;
	// 83280B48: 386ADF98  addi r3, r10, -0x2068
	ctx.r[3].s64 = ctx.r[10].s64 + -8296;
	// 83280B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83280B50: 4AFAC381  bl 0x8222ced0
	ctx.lr = 0x83280B54;
	sub_8222CED0(ctx, base);
	// 83280B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83280B58: 38691028  addi r3, r9, 0x1028
	ctx.r[3].s64 = ctx.r[9].s64 + 4136;
	// 83280B5C: 4BA293C5  bl 0x82ca9f20
	ctx.lr = 0x83280B60;
	sub_82CA9F20(ctx, base);
	// 83280B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280B70 size=56
    let mut pc: u32 = 0x83280B70;
    'dispatch: loop {
        match pc {
            0x83280B70 => {
    //   block [0x83280B70..0x83280BA8)
	// 83280B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280B80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280B84: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83280B88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280B8C: 4AF731CD  bl 0x821f3d58
	ctx.lr = 0x83280B90;
	sub_821F3D58(ctx, base);
	// 83280B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280B94: 906ADF9C  stw r3, -0x2064(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8292 as u32), ctx.r[3].u32 ) };
	// 83280B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280BA8 size=56
    let mut pc: u32 = 0x83280BA8;
    'dispatch: loop {
        match pc {
            0x83280BA8 => {
    //   block [0x83280BA8..0x83280BE0)
	// 83280BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280BB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280BB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280BBC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83280BC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280BC4: 4AF73195  bl 0x821f3d58
	ctx.lr = 0x83280BC8;
	sub_821F3D58(ctx, base);
	// 83280BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280BCC: 906ADFA0  stw r3, -0x2060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8288 as u32), ctx.r[3].u32 ) };
	// 83280BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280BE0 size=56
    let mut pc: u32 = 0x83280BE0;
    'dispatch: loop {
        match pc {
            0x83280BE0 => {
    //   block [0x83280BE0..0x83280C18)
	// 83280BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280BEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280BF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280BF4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83280BF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280BFC: 4AF7315D  bl 0x821f3d58
	ctx.lr = 0x83280C00;
	sub_821F3D58(ctx, base);
	// 83280C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C04: 906ADFA4  stw r3, -0x205c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8284 as u32), ctx.r[3].u32 ) };
	// 83280C08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C18 size=56
    let mut pc: u32 = 0x83280C18;
    'dispatch: loop {
        match pc {
            0x83280C18 => {
    //   block [0x83280C18..0x83280C50)
	// 83280C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C2C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83280C30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280C34: 4AF73125  bl 0x821f3d58
	ctx.lr = 0x83280C38;
	sub_821F3D58(ctx, base);
	// 83280C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C3C: 906ADFA8  stw r3, -0x2058(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8280 as u32), ctx.r[3].u32 ) };
	// 83280C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C50 size=56
    let mut pc: u32 = 0x83280C50;
    'dispatch: loop {
        match pc {
            0x83280C50 => {
    //   block [0x83280C50..0x83280C88)
	// 83280C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C64: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83280C68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280C6C: 4AF730ED  bl 0x821f3d58
	ctx.lr = 0x83280C70;
	sub_821F3D58(ctx, base);
	// 83280C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280C74: 906ADFAC  stw r3, -0x2054(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8276 as u32), ctx.r[3].u32 ) };
	// 83280C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280C88 size=56
    let mut pc: u32 = 0x83280C88;
    'dispatch: loop {
        match pc {
            0x83280C88 => {
    //   block [0x83280C88..0x83280CC0)
	// 83280C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280C90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280C94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280C98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280C9C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83280CA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280CA4: 4AF730B5  bl 0x821f3d58
	ctx.lr = 0x83280CA8;
	sub_821F3D58(ctx, base);
	// 83280CA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280CAC: 906ADFB0  stw r3, -0x2050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8272 as u32), ctx.r[3].u32 ) };
	// 83280CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280CC0 size=56
    let mut pc: u32 = 0x83280CC0;
    'dispatch: loop {
        match pc {
            0x83280CC0 => {
    //   block [0x83280CC0..0x83280CF8)
	// 83280CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280CCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280CD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280CD4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83280CD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280CDC: 4AF7307D  bl 0x821f3d58
	ctx.lr = 0x83280CE0;
	sub_821F3D58(ctx, base);
	// 83280CE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280CE4: 906ADFB4  stw r3, -0x204c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8268 as u32), ctx.r[3].u32 ) };
	// 83280CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280CF8 size=56
    let mut pc: u32 = 0x83280CF8;
    'dispatch: loop {
        match pc {
            0x83280CF8 => {
    //   block [0x83280CF8..0x83280D30)
	// 83280CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D0C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83280D10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D14: 4AF73045  bl 0x821f3d58
	ctx.lr = 0x83280D18;
	sub_821F3D58(ctx, base);
	// 83280D18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D1C: 906ADFB8  stw r3, -0x2048(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8264 as u32), ctx.r[3].u32 ) };
	// 83280D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280D30 size=56
    let mut pc: u32 = 0x83280D30;
    'dispatch: loop {
        match pc {
            0x83280D30 => {
    //   block [0x83280D30..0x83280D68)
	// 83280D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D44: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83280D48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D4C: 4AF7300D  bl 0x821f3d58
	ctx.lr = 0x83280D50;
	sub_821F3D58(ctx, base);
	// 83280D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D54: 906ADFBC  stw r3, -0x2044(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8260 as u32), ctx.r[3].u32 ) };
	// 83280D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280D68 size=56
    let mut pc: u32 = 0x83280D68;
    'dispatch: loop {
        match pc {
            0x83280D68 => {
    //   block [0x83280D68..0x83280DA0)
	// 83280D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280D74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280D78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280D7C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83280D80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280D84: 4AF72FD5  bl 0x821f3d58
	ctx.lr = 0x83280D88;
	sub_821F3D58(ctx, base);
	// 83280D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280D8C: 906ADFC0  stw r3, -0x2040(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8256 as u32), ctx.r[3].u32 ) };
	// 83280D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280DA0 size=56
    let mut pc: u32 = 0x83280DA0;
    'dispatch: loop {
        match pc {
            0x83280DA0 => {
    //   block [0x83280DA0..0x83280DD8)
	// 83280DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280DAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280DB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280DB4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83280DB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280DBC: 4AF72F9D  bl 0x821f3d58
	ctx.lr = 0x83280DC0;
	sub_821F3D58(ctx, base);
	// 83280DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280DC4: 906ADFC4  stw r3, -0x203c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8252 as u32), ctx.r[3].u32 ) };
	// 83280DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280DD8 size=56
    let mut pc: u32 = 0x83280DD8;
    'dispatch: loop {
        match pc {
            0x83280DD8 => {
    //   block [0x83280DD8..0x83280E10)
	// 83280DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280DE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280DEC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83280DF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280DF4: 4AF72F65  bl 0x821f3d58
	ctx.lr = 0x83280DF8;
	sub_821F3D58(ctx, base);
	// 83280DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280DFC: 906ADFC8  stw r3, -0x2038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8248 as u32), ctx.r[3].u32 ) };
	// 83280E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E10 size=56
    let mut pc: u32 = 0x83280E10;
    'dispatch: loop {
        match pc {
            0x83280E10 => {
    //   block [0x83280E10..0x83280E48)
	// 83280E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E24: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83280E28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E2C: 4AF72F2D  bl 0x821f3d58
	ctx.lr = 0x83280E30;
	sub_821F3D58(ctx, base);
	// 83280E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280E34: 906ADFCC  stw r3, -0x2034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8244 as u32), ctx.r[3].u32 ) };
	// 83280E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E48 size=56
    let mut pc: u32 = 0x83280E48;
    'dispatch: loop {
        match pc {
            0x83280E48 => {
    //   block [0x83280E48..0x83280E80)
	// 83280E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E5C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83280E60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E64: 4AF72EF5  bl 0x821f3d58
	ctx.lr = 0x83280E68;
	sub_821F3D58(ctx, base);
	// 83280E68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280E6C: 906ADFD0  stw r3, -0x2030(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8240 as u32), ctx.r[3].u32 ) };
	// 83280E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280E80 size=56
    let mut pc: u32 = 0x83280E80;
    'dispatch: loop {
        match pc {
            0x83280E80 => {
    //   block [0x83280E80..0x83280EB8)
	// 83280E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280E8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280E90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280E94: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83280E98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280E9C: 4AF72EBD  bl 0x821f3d58
	ctx.lr = 0x83280EA0;
	sub_821F3D58(ctx, base);
	// 83280EA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280EA4: 906ADFD4  stw r3, -0x202c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8236 as u32), ctx.r[3].u32 ) };
	// 83280EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280EB8 size=56
    let mut pc: u32 = 0x83280EB8;
    'dispatch: loop {
        match pc {
            0x83280EB8 => {
    //   block [0x83280EB8..0x83280EF0)
	// 83280EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280EC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280EC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280ECC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83280ED0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280ED4: 4AF72E85  bl 0x821f3d58
	ctx.lr = 0x83280ED8;
	sub_821F3D58(ctx, base);
	// 83280ED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280EDC: 906ADFD8  stw r3, -0x2028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8232 as u32), ctx.r[3].u32 ) };
	// 83280EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280EF0 size=56
    let mut pc: u32 = 0x83280EF0;
    'dispatch: loop {
        match pc {
            0x83280EF0 => {
    //   block [0x83280EF0..0x83280F28)
	// 83280EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F04: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83280F08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F0C: 4AF72E4D  bl 0x821f3d58
	ctx.lr = 0x83280F10;
	sub_821F3D58(ctx, base);
	// 83280F10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F14: 906ADFDC  stw r3, -0x2024(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8228 as u32), ctx.r[3].u32 ) };
	// 83280F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F28 size=56
    let mut pc: u32 = 0x83280F28;
    'dispatch: loop {
        match pc {
            0x83280F28 => {
    //   block [0x83280F28..0x83280F60)
	// 83280F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280F34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F3C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83280F40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F44: 4AF72E15  bl 0x821f3d58
	ctx.lr = 0x83280F48;
	sub_821F3D58(ctx, base);
	// 83280F48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F4C: 906ADFE0  stw r3, -0x2020(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8224 as u32), ctx.r[3].u32 ) };
	// 83280F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F60 size=56
    let mut pc: u32 = 0x83280F60;
    'dispatch: loop {
        match pc {
            0x83280F60 => {
    //   block [0x83280F60..0x83280F98)
	// 83280F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280F6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280F70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280F74: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83280F78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280F7C: 4AF72DDD  bl 0x821f3d58
	ctx.lr = 0x83280F80;
	sub_821F3D58(ctx, base);
	// 83280F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280F84: 906ADFE4  stw r3, -0x201c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8220 as u32), ctx.r[3].u32 ) };
	// 83280F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280F98 size=56
    let mut pc: u32 = 0x83280F98;
    'dispatch: loop {
        match pc {
            0x83280F98 => {
    //   block [0x83280F98..0x83280FD0)
	// 83280F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280FA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280FA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280FAC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83280FB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280FB4: 4AF72DA5  bl 0x821f3d58
	ctx.lr = 0x83280FB8;
	sub_821F3D58(ctx, base);
	// 83280FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280FBC: 906ADFE8  stw r3, -0x2018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8216 as u32), ctx.r[3].u32 ) };
	// 83280FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83280FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83280FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83280FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83280FD0 size=56
    let mut pc: u32 = 0x83280FD0;
    'dispatch: loop {
        match pc {
            0x83280FD0 => {
    //   block [0x83280FD0..0x83281008)
	// 83280FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83280FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83280FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83280FDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83280FE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83280FE4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83280FE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83280FEC: 4AF72D6D  bl 0x821f3d58
	ctx.lr = 0x83280FF0;
	sub_821F3D58(ctx, base);
	// 83280FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83280FF4: 906ADFEC  stw r3, -0x2014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8212 as u32), ctx.r[3].u32 ) };
	// 83280FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83280FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281008 size=64
    let mut pc: u32 = 0x83281008;
    'dispatch: loop {
        match pc {
            0x83281008 => {
    //   block [0x83281008..0x83281048)
	// 83281008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328100C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281014: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328101C: 388BE638  addi r4, r11, -0x19c8
	ctx.r[4].s64 = ctx.r[11].s64 + -6600;
	// 83281020: 386ADFF0  addi r3, r10, -0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + -8208;
	// 83281024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281028: 4AFABEA9  bl 0x8222ced0
	ctx.lr = 0x8328102C;
	sub_8222CED0(ctx, base);
	// 8328102C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281030: 38691038  addi r3, r9, 0x1038
	ctx.r[3].s64 = ctx.r[9].s64 + 4152;
	// 83281034: 4BA28EED  bl 0x82ca9f20
	ctx.lr = 0x83281038;
	sub_82CA9F20(ctx, base);
	// 83281038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281048 size=64
    let mut pc: u32 = 0x83281048;
    'dispatch: loop {
        match pc {
            0x83281048 => {
    //   block [0x83281048..0x83281088)
	// 83281048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281054: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328105C: 388BFC54  addi r4, r11, -0x3ac
	ctx.r[4].s64 = ctx.r[11].s64 + -940;
	// 83281060: 386ADFF4  addi r3, r10, -0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + -8204;
	// 83281064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281068: 4AFABE69  bl 0x8222ced0
	ctx.lr = 0x8328106C;
	sub_8222CED0(ctx, base);
	// 8328106C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281070: 38691048  addi r3, r9, 0x1048
	ctx.r[3].s64 = ctx.r[9].s64 + 4168;
	// 83281074: 4BA28EAD  bl 0x82ca9f20
	ctx.lr = 0x83281078;
	sub_82CA9F20(ctx, base);
	// 83281078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328107C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281088 size=64
    let mut pc: u32 = 0x83281088;
    'dispatch: loop {
        match pc {
            0x83281088 => {
    //   block [0x83281088..0x832810C8)
	// 83281088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281094: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281098: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328109C: 388BFC84  addi r4, r11, -0x37c
	ctx.r[4].s64 = ctx.r[11].s64 + -892;
	// 832810A0: 386ADFF8  addi r3, r10, -0x2008
	ctx.r[3].s64 = ctx.r[10].s64 + -8200;
	// 832810A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832810A8: 4AFABE29  bl 0x8222ced0
	ctx.lr = 0x832810AC;
	sub_8222CED0(ctx, base);
	// 832810AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832810B0: 38691058  addi r3, r9, 0x1058
	ctx.r[3].s64 = ctx.r[9].s64 + 4184;
	// 832810B4: 4BA28E6D  bl 0x82ca9f20
	ctx.lr = 0x832810B8;
	sub_82CA9F20(ctx, base);
	// 832810B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832810BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832810C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832810C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832810C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832810C8 size=64
    let mut pc: u32 = 0x832810C8;
    'dispatch: loop {
        match pc {
            0x832810C8 => {
    //   block [0x832810C8..0x83281108)
	// 832810C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832810CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832810D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832810D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832810D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832810DC: 388BFCBC  addi r4, r11, -0x344
	ctx.r[4].s64 = ctx.r[11].s64 + -836;
	// 832810E0: 386ADFFC  addi r3, r10, -0x2004
	ctx.r[3].s64 = ctx.r[10].s64 + -8196;
	// 832810E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832810E8: 4AFABDE9  bl 0x8222ced0
	ctx.lr = 0x832810EC;
	sub_8222CED0(ctx, base);
	// 832810EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832810F0: 38691068  addi r3, r9, 0x1068
	ctx.r[3].s64 = ctx.r[9].s64 + 4200;
	// 832810F4: 4BA28E2D  bl 0x82ca9f20
	ctx.lr = 0x832810F8;
	sub_82CA9F20(ctx, base);
	// 832810F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832810FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281108 size=64
    let mut pc: u32 = 0x83281108;
    'dispatch: loop {
        match pc {
            0x83281108 => {
    //   block [0x83281108..0x83281148)
	// 83281108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281114: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281118: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328111C: 388BFCE8  addi r4, r11, -0x318
	ctx.r[4].s64 = ctx.r[11].s64 + -792;
	// 83281120: 386AE000  addi r3, r10, -0x2000
	ctx.r[3].s64 = ctx.r[10].s64 + -8192;
	// 83281124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281128: 4AFABDA9  bl 0x8222ced0
	ctx.lr = 0x8328112C;
	sub_8222CED0(ctx, base);
	// 8328112C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281130: 38691078  addi r3, r9, 0x1078
	ctx.r[3].s64 = ctx.r[9].s64 + 4216;
	// 83281134: 4BA28DED  bl 0x82ca9f20
	ctx.lr = 0x83281138;
	sub_82CA9F20(ctx, base);
	// 83281138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281148 size=64
    let mut pc: u32 = 0x83281148;
    'dispatch: loop {
        match pc {
            0x83281148 => {
    //   block [0x83281148..0x83281188)
	// 83281148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281154: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328115C: 388BFD1C  addi r4, r11, -0x2e4
	ctx.r[4].s64 = ctx.r[11].s64 + -740;
	// 83281160: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 83281164: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281168: 4AFABD69  bl 0x8222ced0
	ctx.lr = 0x8328116C;
	sub_8222CED0(ctx, base);
	// 8328116C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281170: 38691088  addi r3, r9, 0x1088
	ctx.r[3].s64 = ctx.r[9].s64 + 4232;
	// 83281174: 4BA28DAD  bl 0x82ca9f20
	ctx.lr = 0x83281178;
	sub_82CA9F20(ctx, base);
	// 83281178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281188 size=64
    let mut pc: u32 = 0x83281188;
    'dispatch: loop {
        match pc {
            0x83281188 => {
    //   block [0x83281188..0x832811C8)
	// 83281188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281194: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328119C: 388BFD58  addi r4, r11, -0x2a8
	ctx.r[4].s64 = ctx.r[11].s64 + -680;
	// 832811A0: 386AE008  addi r3, r10, -0x1ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -8184;
	// 832811A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832811A8: 4AFABD29  bl 0x8222ced0
	ctx.lr = 0x832811AC;
	sub_8222CED0(ctx, base);
	// 832811AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832811B0: 38691098  addi r3, r9, 0x1098
	ctx.r[3].s64 = ctx.r[9].s64 + 4248;
	// 832811B4: 4BA28D6D  bl 0x82ca9f20
	ctx.lr = 0x832811B8;
	sub_82CA9F20(ctx, base);
	// 832811B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832811BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832811C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832811C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832811C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832811C8 size=64
    let mut pc: u32 = 0x832811C8;
    'dispatch: loop {
        match pc {
            0x832811C8 => {
    //   block [0x832811C8..0x83281208)
	// 832811C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832811CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832811D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832811D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832811D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832811DC: 388BFD98  addi r4, r11, -0x268
	ctx.r[4].s64 = ctx.r[11].s64 + -616;
	// 832811E0: 386AE00C  addi r3, r10, -0x1ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -8180;
	// 832811E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832811E8: 4AFABCE9  bl 0x8222ced0
	ctx.lr = 0x832811EC;
	sub_8222CED0(ctx, base);
	// 832811EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832811F0: 386910A8  addi r3, r9, 0x10a8
	ctx.r[3].s64 = ctx.r[9].s64 + 4264;
	// 832811F4: 4BA28D2D  bl 0x82ca9f20
	ctx.lr = 0x832811F8;
	sub_82CA9F20(ctx, base);
	// 832811F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832811FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281208 size=64
    let mut pc: u32 = 0x83281208;
    'dispatch: loop {
        match pc {
            0x83281208 => {
    //   block [0x83281208..0x83281248)
	// 83281208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281214: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83281218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328121C: 388BFDD4  addi r4, r11, -0x22c
	ctx.r[4].s64 = ctx.r[11].s64 + -556;
	// 83281220: 386AE010  addi r3, r10, -0x1ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -8176;
	// 83281224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83281228: 4AFABCA9  bl 0x8222ced0
	ctx.lr = 0x8328122C;
	sub_8222CED0(ctx, base);
	// 8328122C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83281230: 386910B8  addi r3, r9, 0x10b8
	ctx.r[3].s64 = ctx.r[9].s64 + 4280;
	// 83281234: 4BA28CED  bl 0x82ca9f20
	ctx.lr = 0x83281238;
	sub_82CA9F20(ctx, base);
	// 83281238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328123C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281248 size=56
    let mut pc: u32 = 0x83281248;
    'dispatch: loop {
        match pc {
            0x83281248 => {
    //   block [0x83281248..0x83281280)
	// 83281248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328124C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281254: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281258: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328125C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83281260: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281264: 4AF72AF5  bl 0x821f3d58
	ctx.lr = 0x83281268;
	sub_821F3D58(ctx, base);
	// 83281268: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328126C: 906AE014  stw r3, -0x1fec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8172 as u32), ctx.r[3].u32 ) };
	// 83281270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281280 size=56
    let mut pc: u32 = 0x83281280;
    'dispatch: loop {
        match pc {
            0x83281280 => {
    //   block [0x83281280..0x832812B8)
	// 83281280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328128C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281290: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281294: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83281298: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328129C: 4AF72ABD  bl 0x821f3d58
	ctx.lr = 0x832812A0;
	sub_821F3D58(ctx, base);
	// 832812A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832812A4: 906AE018  stw r3, -0x1fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8168 as u32), ctx.r[3].u32 ) };
	// 832812A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832812AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832812B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832812B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832812B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832812B8 size=56
    let mut pc: u32 = 0x832812B8;
    'dispatch: loop {
        match pc {
            0x832812B8 => {
    //   block [0x832812B8..0x832812F0)
	// 832812B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832812BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832812C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832812C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832812C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832812CC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832812D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832812D4: 4AF72A85  bl 0x821f3d58
	ctx.lr = 0x832812D8;
	sub_821F3D58(ctx, base);
	// 832812D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832812DC: 906AE01C  stw r3, -0x1fe4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8164 as u32), ctx.r[3].u32 ) };
	// 832812E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832812E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832812E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832812EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832812F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832812F0 size=56
    let mut pc: u32 = 0x832812F0;
    'dispatch: loop {
        match pc {
            0x832812F0 => {
    //   block [0x832812F0..0x83281328)
	// 832812F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832812F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832812F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832812FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281304: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83281308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328130C: 4AF72A4D  bl 0x821f3d58
	ctx.lr = 0x83281310;
	sub_821F3D58(ctx, base);
	// 83281310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281314: 906AE020  stw r3, -0x1fe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8160 as u32), ctx.r[3].u32 ) };
	// 83281318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328131C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281328 size=56
    let mut pc: u32 = 0x83281328;
    'dispatch: loop {
        match pc {
            0x83281328 => {
    //   block [0x83281328..0x83281360)
	// 83281328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328132C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83281334: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8328133C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83281340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83281344: 4AF72A15  bl 0x821f3d58
	ctx.lr = 0x83281348;
	sub_821F3D58(ctx, base);
	// 83281348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328134C: 906AE024  stw r3, -0x1fdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8156 as u32), ctx.r[3].u32 ) };
	// 83281350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83281354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281360 size=56
    let mut pc: u32 = 0x83281360;
    'dispatch: loop {
        match pc {
            0x83281360 => {
    //   block [0x83281360..0x83281398)
	// 83281360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83281364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83281368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328136C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83281370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83281374: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83281378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8328137C: 4AF729DD  bl 0x821f3d58
	ctx.lr = 0x83281380;
	sub_821F3D58(ctx, base);
	// 83281380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83281384: 906AE028  stw r3, -0x1fd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8152 as u32), ctx.r[3].u32 ) };
	// 83281388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328138C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83281390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83281394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83281398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83281398 size=56
    let mut pc: u32 = 0x83281398;
    'dispatch: loop {
        match pc {
            0x83281398 => {
    //   block [0x83281398..0x832813D0)
	// 83281398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328139C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832813A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832813A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832813A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832813AC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832813B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832813B4: 4AF729A5  bl 0x821f3d58
	ctx.lr = 0x832813B8;
	sub_821F3D58(ctx, base);
	// 832813B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832813BC: 906AE02C  stw r3, -0x1fd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8148 as u32), ctx.r[3].u32 ) };
	// 832813C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832813C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832813C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832813CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


