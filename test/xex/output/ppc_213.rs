pub fn sub_8325A7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7B0 size=64
    let mut pc: u32 = 0x8325A7B0;
    'dispatch: loop {
        match pc {
            0x8325A7B0 => {
    //   block [0x8325A7B0..0x8325A7F0)
	// 8325A7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A7BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A7C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325A7C8: 386AAA90  addi r3, r10, -0x5570
	ctx.r[3].s64 = ctx.r[10].s64 + -21872;
	// 8325A7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A7D0: 4AFD2701  bl 0x8222ced0
	ctx.lr = 0x8325A7D4;
	sub_8222CED0(ctx, base);
	// 8325A7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A7D8: 3869BE58  addi r3, r9, -0x41a8
	ctx.r[3].s64 = ctx.r[9].s64 + -16808;
	// 8325A7DC: 4BA4F745  bl 0x82ca9f20
	ctx.lr = 0x8325A7E0;
	sub_82CA9F20(ctx, base);
	// 8325A7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A7F0 size=64
    let mut pc: u32 = 0x8325A7F0;
    'dispatch: loop {
        match pc {
            0x8325A7F0 => {
    //   block [0x8325A7F0..0x8325A830)
	// 8325A7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A7FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A804: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325A808: 386AAA94  addi r3, r10, -0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + -21868;
	// 8325A80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A810: 4AFD26C1  bl 0x8222ced0
	ctx.lr = 0x8325A814;
	sub_8222CED0(ctx, base);
	// 8325A814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A818: 3869BE68  addi r3, r9, -0x4198
	ctx.r[3].s64 = ctx.r[9].s64 + -16792;
	// 8325A81C: 4BA4F705  bl 0x82ca9f20
	ctx.lr = 0x8325A820;
	sub_82CA9F20(ctx, base);
	// 8325A820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A830 size=64
    let mut pc: u32 = 0x8325A830;
    'dispatch: loop {
        match pc {
            0x8325A830 => {
    //   block [0x8325A830..0x8325A870)
	// 8325A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325A840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A844: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325A848: 386AAA98  addi r3, r10, -0x5568
	ctx.r[3].s64 = ctx.r[10].s64 + -21864;
	// 8325A84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A850: 4AFD2681  bl 0x8222ced0
	ctx.lr = 0x8325A854;
	sub_8222CED0(ctx, base);
	// 8325A854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A858: 3869BE78  addi r3, r9, -0x4188
	ctx.r[3].s64 = ctx.r[9].s64 + -16776;
	// 8325A85C: 4BA4F6C5  bl 0x82ca9f20
	ctx.lr = 0x8325A860;
	sub_82CA9F20(ctx, base);
	// 8325A860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8325A870 size=12
    let mut pc: u32 = 0x8325A870;
    'dispatch: loop {
        match pc {
            0x8325A870 => {
    //   block [0x8325A870..0x8325A87C)
	// 8325A870: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325A874: 386BBE88  addi r3, r11, -0x4178
	ctx.r[3].s64 = ctx.r[11].s64 + -16760;
	// 8325A878: 4BA4F6A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A880 size=64
    let mut pc: u32 = 0x8325A880;
    'dispatch: loop {
        match pc {
            0x8325A880 => {
    //   block [0x8325A880..0x8325A8C0)
	// 8325A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A88C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A894: 388BBA64  addi r4, r11, -0x459c
	ctx.r[4].s64 = ctx.r[11].s64 + -17820;
	// 8325A898: 386AAAB0  addi r3, r10, -0x5550
	ctx.r[3].s64 = ctx.r[10].s64 + -21840;
	// 8325A89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A8A0: 4AFD2631  bl 0x8222ced0
	ctx.lr = 0x8325A8A4;
	sub_8222CED0(ctx, base);
	// 8325A8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A8A8: 3869BEF0  addi r3, r9, -0x4110
	ctx.r[3].s64 = ctx.r[9].s64 + -16656;
	// 8325A8AC: 4BA4F675  bl 0x82ca9f20
	ctx.lr = 0x8325A8B0;
	sub_82CA9F20(ctx, base);
	// 8325A8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A8C0 size=64
    let mut pc: u32 = 0x8325A8C0;
    'dispatch: loop {
        match pc {
            0x8325A8C0 => {
    //   block [0x8325A8C0..0x8325A900)
	// 8325A8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A8CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8325A8D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A8D4: 388B0E20  addi r4, r11, 0xe20
	ctx.r[4].s64 = ctx.r[11].s64 + 3616;
	// 8325A8D8: 386AAAB4  addi r3, r10, -0x554c
	ctx.r[3].s64 = ctx.r[10].s64 + -21836;
	// 8325A8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A8E0: 4AFD25F1  bl 0x8222ced0
	ctx.lr = 0x8325A8E4;
	sub_8222CED0(ctx, base);
	// 8325A8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A8E8: 3869BF00  addi r3, r9, -0x4100
	ctx.r[3].s64 = ctx.r[9].s64 + -16640;
	// 8325A8EC: 4BA4F635  bl 0x82ca9f20
	ctx.lr = 0x8325A8F0;
	sub_82CA9F20(ctx, base);
	// 8325A8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A900 size=64
    let mut pc: u32 = 0x8325A900;
    'dispatch: loop {
        match pc {
            0x8325A900 => {
    //   block [0x8325A900..0x8325A940)
	// 8325A900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A90C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A910: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A914: 388BBA78  addi r4, r11, -0x4588
	ctx.r[4].s64 = ctx.r[11].s64 + -17800;
	// 8325A918: 386AAAB8  addi r3, r10, -0x5548
	ctx.r[3].s64 = ctx.r[10].s64 + -21832;
	// 8325A91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A920: 4AFD25B1  bl 0x8222ced0
	ctx.lr = 0x8325A924;
	sub_8222CED0(ctx, base);
	// 8325A924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A928: 3869BF10  addi r3, r9, -0x40f0
	ctx.r[3].s64 = ctx.r[9].s64 + -16624;
	// 8325A92C: 4BA4F5F5  bl 0x82ca9f20
	ctx.lr = 0x8325A930;
	sub_82CA9F20(ctx, base);
	// 8325A930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A940 size=64
    let mut pc: u32 = 0x8325A940;
    'dispatch: loop {
        match pc {
            0x8325A940 => {
    //   block [0x8325A940..0x8325A980)
	// 8325A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A94C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A950: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A954: 388BBA9C  addi r4, r11, -0x4564
	ctx.r[4].s64 = ctx.r[11].s64 + -17764;
	// 8325A958: 386AAABC  addi r3, r10, -0x5544
	ctx.r[3].s64 = ctx.r[10].s64 + -21828;
	// 8325A95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A960: 4AFD2571  bl 0x8222ced0
	ctx.lr = 0x8325A964;
	sub_8222CED0(ctx, base);
	// 8325A964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A968: 3869BF20  addi r3, r9, -0x40e0
	ctx.r[3].s64 = ctx.r[9].s64 + -16608;
	// 8325A96C: 4BA4F5B5  bl 0x82ca9f20
	ctx.lr = 0x8325A970;
	sub_82CA9F20(ctx, base);
	// 8325A970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A980 size=64
    let mut pc: u32 = 0x8325A980;
    'dispatch: loop {
        match pc {
            0x8325A980 => {
    //   block [0x8325A980..0x8325A9C0)
	// 8325A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A98C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A994: 388BBABC  addi r4, r11, -0x4544
	ctx.r[4].s64 = ctx.r[11].s64 + -17732;
	// 8325A998: 386AAAC0  addi r3, r10, -0x5540
	ctx.r[3].s64 = ctx.r[10].s64 + -21824;
	// 8325A99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A9A0: 4AFD2531  bl 0x8222ced0
	ctx.lr = 0x8325A9A4;
	sub_8222CED0(ctx, base);
	// 8325A9A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A9A8: 3869BF30  addi r3, r9, -0x40d0
	ctx.r[3].s64 = ctx.r[9].s64 + -16592;
	// 8325A9AC: 4BA4F575  bl 0x82ca9f20
	ctx.lr = 0x8325A9B0;
	sub_82CA9F20(ctx, base);
	// 8325A9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325A9C0 size=64
    let mut pc: u32 = 0x8325A9C0;
    'dispatch: loop {
        match pc {
            0x8325A9C0 => {
    //   block [0x8325A9C0..0x8325AA00)
	// 8325A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325A9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325A9CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325A9D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325A9D4: 388BBAF4  addi r4, r11, -0x450c
	ctx.r[4].s64 = ctx.r[11].s64 + -17676;
	// 8325A9D8: 386AAAC4  addi r3, r10, -0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + -21820;
	// 8325A9DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325A9E0: 4AFD24F1  bl 0x8222ced0
	ctx.lr = 0x8325A9E4;
	sub_8222CED0(ctx, base);
	// 8325A9E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325A9E8: 3869BF40  addi r3, r9, -0x40c0
	ctx.r[3].s64 = ctx.r[9].s64 + -16576;
	// 8325A9EC: 4BA4F535  bl 0x82ca9f20
	ctx.lr = 0x8325A9F0;
	sub_82CA9F20(ctx, base);
	// 8325A9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325A9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325A9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA00 size=64
    let mut pc: u32 = 0x8325AA00;
    'dispatch: loop {
        match pc {
            0x8325AA00 => {
    //   block [0x8325AA00..0x8325AA40)
	// 8325AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA14: 388BBB18  addi r4, r11, -0x44e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17640;
	// 8325AA18: 386AAAC8  addi r3, r10, -0x5538
	ctx.r[3].s64 = ctx.r[10].s64 + -21816;
	// 8325AA1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AA20: 4AFD24B1  bl 0x8222ced0
	ctx.lr = 0x8325AA24;
	sub_8222CED0(ctx, base);
	// 8325AA24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AA28: 3869BF50  addi r3, r9, -0x40b0
	ctx.r[3].s64 = ctx.r[9].s64 + -16560;
	// 8325AA2C: 4BA4F4F5  bl 0x82ca9f20
	ctx.lr = 0x8325AA30;
	sub_82CA9F20(ctx, base);
	// 8325AA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA40 size=64
    let mut pc: u32 = 0x8325AA40;
    'dispatch: loop {
        match pc {
            0x8325AA40 => {
    //   block [0x8325AA40..0x8325AA80)
	// 8325AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA54: 388BBB48  addi r4, r11, -0x44b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17592;
	// 8325AA58: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 8325AA5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AA60: 4AFD2471  bl 0x8222ced0
	ctx.lr = 0x8325AA64;
	sub_8222CED0(ctx, base);
	// 8325AA64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AA68: 3869BF60  addi r3, r9, -0x40a0
	ctx.r[3].s64 = ctx.r[9].s64 + -16544;
	// 8325AA6C: 4BA4F4B5  bl 0x82ca9f20
	ctx.lr = 0x8325AA70;
	sub_82CA9F20(ctx, base);
	// 8325AA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AA80 size=64
    let mut pc: u32 = 0x8325AA80;
    'dispatch: loop {
        match pc {
            0x8325AA80 => {
    //   block [0x8325AA80..0x8325AAC0)
	// 8325AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AA8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AA94: 388BBB78  addi r4, r11, -0x4488
	ctx.r[4].s64 = ctx.r[11].s64 + -17544;
	// 8325AA98: 386AAAD0  addi r3, r10, -0x5530
	ctx.r[3].s64 = ctx.r[10].s64 + -21808;
	// 8325AA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AAA0: 4AFD2431  bl 0x8222ced0
	ctx.lr = 0x8325AAA4;
	sub_8222CED0(ctx, base);
	// 8325AAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AAA8: 3869BF70  addi r3, r9, -0x4090
	ctx.r[3].s64 = ctx.r[9].s64 + -16528;
	// 8325AAAC: 4BA4F475  bl 0x82ca9f20
	ctx.lr = 0x8325AAB0;
	sub_82CA9F20(ctx, base);
	// 8325AAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AAC0 size=64
    let mut pc: u32 = 0x8325AAC0;
    'dispatch: loop {
        match pc {
            0x8325AAC0 => {
    //   block [0x8325AAC0..0x8325AB00)
	// 8325AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AAD4: 388BBB9C  addi r4, r11, -0x4464
	ctx.r[4].s64 = ctx.r[11].s64 + -17508;
	// 8325AAD8: 386AAAD4  addi r3, r10, -0x552c
	ctx.r[3].s64 = ctx.r[10].s64 + -21804;
	// 8325AADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AAE0: 4AFD23F1  bl 0x8222ced0
	ctx.lr = 0x8325AAE4;
	sub_8222CED0(ctx, base);
	// 8325AAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AAE8: 3869BF80  addi r3, r9, -0x4080
	ctx.r[3].s64 = ctx.r[9].s64 + -16512;
	// 8325AAEC: 4BA4F435  bl 0x82ca9f20
	ctx.lr = 0x8325AAF0;
	sub_82CA9F20(ctx, base);
	// 8325AAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB00 size=64
    let mut pc: u32 = 0x8325AB00;
    'dispatch: loop {
        match pc {
            0x8325AB00 => {
    //   block [0x8325AB00..0x8325AB40)
	// 8325AB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB14: 388BBBC8  addi r4, r11, -0x4438
	ctx.r[4].s64 = ctx.r[11].s64 + -17464;
	// 8325AB18: 386AAAD8  addi r3, r10, -0x5528
	ctx.r[3].s64 = ctx.r[10].s64 + -21800;
	// 8325AB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AB20: 4AFD23B1  bl 0x8222ced0
	ctx.lr = 0x8325AB24;
	sub_8222CED0(ctx, base);
	// 8325AB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AB28: 3869BF90  addi r3, r9, -0x4070
	ctx.r[3].s64 = ctx.r[9].s64 + -16496;
	// 8325AB2C: 4BA4F3F5  bl 0x82ca9f20
	ctx.lr = 0x8325AB30;
	sub_82CA9F20(ctx, base);
	// 8325AB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB40 size=64
    let mut pc: u32 = 0x8325AB40;
    'dispatch: loop {
        match pc {
            0x8325AB40 => {
    //   block [0x8325AB40..0x8325AB80)
	// 8325AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AB50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB54: 388BBC08  addi r4, r11, -0x43f8
	ctx.r[4].s64 = ctx.r[11].s64 + -17400;
	// 8325AB58: 386AAADC  addi r3, r10, -0x5524
	ctx.r[3].s64 = ctx.r[10].s64 + -21796;
	// 8325AB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AB60: 4AFD2371  bl 0x8222ced0
	ctx.lr = 0x8325AB64;
	sub_8222CED0(ctx, base);
	// 8325AB64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AB68: 3869BFA0  addi r3, r9, -0x4060
	ctx.r[3].s64 = ctx.r[9].s64 + -16480;
	// 8325AB6C: 4BA4F3B5  bl 0x82ca9f20
	ctx.lr = 0x8325AB70;
	sub_82CA9F20(ctx, base);
	// 8325AB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AB80 size=64
    let mut pc: u32 = 0x8325AB80;
    'dispatch: loop {
        match pc {
            0x8325AB80 => {
    //   block [0x8325AB80..0x8325ABC0)
	// 8325AB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AB8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AB90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AB94: 388B2D5C  addi r4, r11, 0x2d5c
	ctx.r[4].s64 = ctx.r[11].s64 + 11612;
	// 8325AB98: 386AAAE0  addi r3, r10, -0x5520
	ctx.r[3].s64 = ctx.r[10].s64 + -21792;
	// 8325AB9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ABA0: 4AFD2331  bl 0x8222ced0
	ctx.lr = 0x8325ABA4;
	sub_8222CED0(ctx, base);
	// 8325ABA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ABA8: 3869BFB0  addi r3, r9, -0x4050
	ctx.r[3].s64 = ctx.r[9].s64 + -16464;
	// 8325ABAC: 4BA4F375  bl 0x82ca9f20
	ctx.lr = 0x8325ABB0;
	sub_82CA9F20(ctx, base);
	// 8325ABB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ABC0 size=64
    let mut pc: u32 = 0x8325ABC0;
    'dispatch: loop {
        match pc {
            0x8325ABC0 => {
    //   block [0x8325ABC0..0x8325AC00)
	// 8325ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ABC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ABCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325ABD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ABD4: 388B2D68  addi r4, r11, 0x2d68
	ctx.r[4].s64 = ctx.r[11].s64 + 11624;
	// 8325ABD8: 386AAAE4  addi r3, r10, -0x551c
	ctx.r[3].s64 = ctx.r[10].s64 + -21788;
	// 8325ABDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ABE0: 4AFD22F1  bl 0x8222ced0
	ctx.lr = 0x8325ABE4;
	sub_8222CED0(ctx, base);
	// 8325ABE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ABE8: 3869BFC0  addi r3, r9, -0x4040
	ctx.r[3].s64 = ctx.r[9].s64 + -16448;
	// 8325ABEC: 4BA4F335  bl 0x82ca9f20
	ctx.lr = 0x8325ABF0;
	sub_82CA9F20(ctx, base);
	// 8325ABF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ABF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ABF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ABFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC00 size=64
    let mut pc: u32 = 0x8325AC00;
    'dispatch: loop {
        match pc {
            0x8325AC00 => {
    //   block [0x8325AC00..0x8325AC40)
	// 8325AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC14: 388B12FC  addi r4, r11, 0x12fc
	ctx.r[4].s64 = ctx.r[11].s64 + 4860;
	// 8325AC18: 386AAAE8  addi r3, r10, -0x5518
	ctx.r[3].s64 = ctx.r[10].s64 + -21784;
	// 8325AC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AC20: 4AFD22B1  bl 0x8222ced0
	ctx.lr = 0x8325AC24;
	sub_8222CED0(ctx, base);
	// 8325AC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AC28: 3869BFD0  addi r3, r9, -0x4030
	ctx.r[3].s64 = ctx.r[9].s64 + -16432;
	// 8325AC2C: 4BA4F2F5  bl 0x82ca9f20
	ctx.lr = 0x8325AC30;
	sub_82CA9F20(ctx, base);
	// 8325AC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC40 size=64
    let mut pc: u32 = 0x8325AC40;
    'dispatch: loop {
        match pc {
            0x8325AC40 => {
    //   block [0x8325AC40..0x8325AC80)
	// 8325AC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC54: 388B12D4  addi r4, r11, 0x12d4
	ctx.r[4].s64 = ctx.r[11].s64 + 4820;
	// 8325AC58: 386AAAEC  addi r3, r10, -0x5514
	ctx.r[3].s64 = ctx.r[10].s64 + -21780;
	// 8325AC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AC60: 4AFD2271  bl 0x8222ced0
	ctx.lr = 0x8325AC64;
	sub_8222CED0(ctx, base);
	// 8325AC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AC68: 3869BFE0  addi r3, r9, -0x4020
	ctx.r[3].s64 = ctx.r[9].s64 + -16416;
	// 8325AC6C: 4BA4F2B5  bl 0x82ca9f20
	ctx.lr = 0x8325AC70;
	sub_82CA9F20(ctx, base);
	// 8325AC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AC80 size=64
    let mut pc: u32 = 0x8325AC80;
    'dispatch: loop {
        match pc {
            0x8325AC80 => {
    //   block [0x8325AC80..0x8325ACC0)
	// 8325AC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AC8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325AC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AC94: 388B12EC  addi r4, r11, 0x12ec
	ctx.r[4].s64 = ctx.r[11].s64 + 4844;
	// 8325AC98: 386AAAF0  addi r3, r10, -0x5510
	ctx.r[3].s64 = ctx.r[10].s64 + -21776;
	// 8325AC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ACA0: 4AFD2231  bl 0x8222ced0
	ctx.lr = 0x8325ACA4;
	sub_8222CED0(ctx, base);
	// 8325ACA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ACA8: 3869BFF0  addi r3, r9, -0x4010
	ctx.r[3].s64 = ctx.r[9].s64 + -16400;
	// 8325ACAC: 4BA4F275  bl 0x82ca9f20
	ctx.lr = 0x8325ACB0;
	sub_82CA9F20(ctx, base);
	// 8325ACB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ACC0 size=64
    let mut pc: u32 = 0x8325ACC0;
    'dispatch: loop {
        match pc {
            0x8325ACC0 => {
    //   block [0x8325ACC0..0x8325AD00)
	// 8325ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ACC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ACCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ACD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ACD4: 388BBC44  addi r4, r11, -0x43bc
	ctx.r[4].s64 = ctx.r[11].s64 + -17340;
	// 8325ACD8: 386AAAF4  addi r3, r10, -0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + -21772;
	// 8325ACDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ACE0: 4AFD21F1  bl 0x8222ced0
	ctx.lr = 0x8325ACE4;
	sub_8222CED0(ctx, base);
	// 8325ACE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ACE8: 3869C000  addi r3, r9, -0x4000
	ctx.r[3].s64 = ctx.r[9].s64 + -16384;
	// 8325ACEC: 4BA4F235  bl 0x82ca9f20
	ctx.lr = 0x8325ACF0;
	sub_82CA9F20(ctx, base);
	// 8325ACF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ACFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD00 size=64
    let mut pc: u32 = 0x8325AD00;
    'dispatch: loop {
        match pc {
            0x8325AD00 => {
    //   block [0x8325AD00..0x8325AD40)
	// 8325AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD14: 388BBC50  addi r4, r11, -0x43b0
	ctx.r[4].s64 = ctx.r[11].s64 + -17328;
	// 8325AD18: 386AAAF8  addi r3, r10, -0x5508
	ctx.r[3].s64 = ctx.r[10].s64 + -21768;
	// 8325AD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AD20: 4AFD21B1  bl 0x8222ced0
	ctx.lr = 0x8325AD24;
	sub_8222CED0(ctx, base);
	// 8325AD24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AD28: 3869C010  addi r3, r9, -0x3ff0
	ctx.r[3].s64 = ctx.r[9].s64 + -16368;
	// 8325AD2C: 4BA4F1F5  bl 0x82ca9f20
	ctx.lr = 0x8325AD30;
	sub_82CA9F20(ctx, base);
	// 8325AD30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD40 size=64
    let mut pc: u32 = 0x8325AD40;
    'dispatch: loop {
        match pc {
            0x8325AD40 => {
    //   block [0x8325AD40..0x8325AD80)
	// 8325AD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD54: 388BBC5C  addi r4, r11, -0x43a4
	ctx.r[4].s64 = ctx.r[11].s64 + -17316;
	// 8325AD58: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 8325AD5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AD60: 4AFD2171  bl 0x8222ced0
	ctx.lr = 0x8325AD64;
	sub_8222CED0(ctx, base);
	// 8325AD64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AD68: 3869C020  addi r3, r9, -0x3fe0
	ctx.r[3].s64 = ctx.r[9].s64 + -16352;
	// 8325AD6C: 4BA4F1B5  bl 0x82ca9f20
	ctx.lr = 0x8325AD70;
	sub_82CA9F20(ctx, base);
	// 8325AD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AD80 size=64
    let mut pc: u32 = 0x8325AD80;
    'dispatch: loop {
        match pc {
            0x8325AD80 => {
    //   block [0x8325AD80..0x8325ADC0)
	// 8325AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AD8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AD94: 388BBC68  addi r4, r11, -0x4398
	ctx.r[4].s64 = ctx.r[11].s64 + -17304;
	// 8325AD98: 386AAB00  addi r3, r10, -0x5500
	ctx.r[3].s64 = ctx.r[10].s64 + -21760;
	// 8325AD9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ADA0: 4AFD2131  bl 0x8222ced0
	ctx.lr = 0x8325ADA4;
	sub_8222CED0(ctx, base);
	// 8325ADA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ADA8: 3869C030  addi r3, r9, -0x3fd0
	ctx.r[3].s64 = ctx.r[9].s64 + -16336;
	// 8325ADAC: 4BA4F175  bl 0x82ca9f20
	ctx.lr = 0x8325ADB0;
	sub_82CA9F20(ctx, base);
	// 8325ADB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ADB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ADB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ADBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ADC0 size=64
    let mut pc: u32 = 0x8325ADC0;
    'dispatch: loop {
        match pc {
            0x8325ADC0 => {
    //   block [0x8325ADC0..0x8325AE00)
	// 8325ADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ADC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ADCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ADD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ADD4: 388BBC74  addi r4, r11, -0x438c
	ctx.r[4].s64 = ctx.r[11].s64 + -17292;
	// 8325ADD8: 386AAB04  addi r3, r10, -0x54fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21756;
	// 8325ADDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ADE0: 4AFD20F1  bl 0x8222ced0
	ctx.lr = 0x8325ADE4;
	sub_8222CED0(ctx, base);
	// 8325ADE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ADE8: 3869C040  addi r3, r9, -0x3fc0
	ctx.r[3].s64 = ctx.r[9].s64 + -16320;
	// 8325ADEC: 4BA4F135  bl 0x82ca9f20
	ctx.lr = 0x8325ADF0;
	sub_82CA9F20(ctx, base);
	// 8325ADF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ADF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ADF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ADFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE00 size=64
    let mut pc: u32 = 0x8325AE00;
    'dispatch: loop {
        match pc {
            0x8325AE00 => {
    //   block [0x8325AE00..0x8325AE40)
	// 8325AE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AE0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325AE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AE14: 388BBC8C  addi r4, r11, -0x4374
	ctx.r[4].s64 = ctx.r[11].s64 + -17268;
	// 8325AE18: 386AAB08  addi r3, r10, -0x54f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21752;
	// 8325AE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AE20: 4AFD20B1  bl 0x8222ced0
	ctx.lr = 0x8325AE24;
	sub_8222CED0(ctx, base);
	// 8325AE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AE28: 3869C050  addi r3, r9, -0x3fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -16304;
	// 8325AE2C: 4BA4F0F5  bl 0x82ca9f20
	ctx.lr = 0x8325AE30;
	sub_82CA9F20(ctx, base);
	// 8325AE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AE40 size=108
    let mut pc: u32 = 0x8325AE40;
    'dispatch: loop {
        match pc {
            0x8325AE40 => {
    //   block [0x8325AE40..0x8325AEAC)
	// 8325AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AE48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AE4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AE50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AE54: 3BEBAB0C  addi r31, r11, -0x54f4
	ctx.r[31].s64 = ctx.r[11].s64 + -21748;
	// 8325AE58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AE5C: 4B0C7AA5  bl 0x82322900
	ctx.lr = 0x8325AE60;
	sub_82322900(ctx, base);
	// 8325AE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8325AE64: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8325AE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8325AE6C: 99230015  stb r9, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[9].u8 ) };
	// 8325AE70: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8325AE74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE78: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8325AE7C: 3868C060  addi r3, r8, -0x3fa0
	ctx.r[3].s64 = ctx.r[8].s64 + -16288;
	// 8325AE80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE84: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8325AE88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8325AE8C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8325AE90: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325AE94: 4BA4F08D  bl 0x82ca9f20
	ctx.lr = 0x8325AE98;
	sub_82CA9F20(ctx, base);
	// 8325AE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AEA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325AEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEB0 size=64
    let mut pc: u32 = 0x8325AEB0;
    'dispatch: loop {
        match pc {
            0x8325AEB0 => {
    //   block [0x8325AEB0..0x8325AEF0)
	// 8325AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AEC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AEC4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325AEC8: 386AAB18  addi r3, r10, -0x54e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21736;
	// 8325AECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AED0: 4AFD2001  bl 0x8222ced0
	ctx.lr = 0x8325AED4;
	sub_8222CED0(ctx, base);
	// 8325AED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AED8: 3869C070  addi r3, r9, -0x3f90
	ctx.r[3].s64 = ctx.r[9].s64 + -16272;
	// 8325AEDC: 4BA4F045  bl 0x82ca9f20
	ctx.lr = 0x8325AEE0;
	sub_82CA9F20(ctx, base);
	// 8325AEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AEF0 size=64
    let mut pc: u32 = 0x8325AEF0;
    'dispatch: loop {
        match pc {
            0x8325AEF0 => {
    //   block [0x8325AEF0..0x8325AF30)
	// 8325AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AEF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AEFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AF04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325AF08: 386AAB1C  addi r3, r10, -0x54e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21732;
	// 8325AF0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AF10: 4AFD1FC1  bl 0x8222ced0
	ctx.lr = 0x8325AF14;
	sub_8222CED0(ctx, base);
	// 8325AF14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AF18: 3869C080  addi r3, r9, -0x3f80
	ctx.r[3].s64 = ctx.r[9].s64 + -16256;
	// 8325AF1C: 4BA4F005  bl 0x82ca9f20
	ctx.lr = 0x8325AF20;
	sub_82CA9F20(ctx, base);
	// 8325AF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF30 size=64
    let mut pc: u32 = 0x8325AF30;
    'dispatch: loop {
        match pc {
            0x8325AF30 => {
    //   block [0x8325AF30..0x8325AF70)
	// 8325AF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AF3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325AF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325AF44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325AF48: 386AAB20  addi r3, r10, -0x54e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21728;
	// 8325AF4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AF50: 4AFD1F81  bl 0x8222ced0
	ctx.lr = 0x8325AF54;
	sub_8222CED0(ctx, base);
	// 8325AF54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325AF58: 3869C090  addi r3, r9, -0x3f70
	ctx.r[3].s64 = ctx.r[9].s64 + -16240;
	// 8325AF5C: 4BA4EFC5  bl 0x82ca9f20
	ctx.lr = 0x8325AF60;
	sub_82CA9F20(ctx, base);
	// 8325AF60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AF70 size=52
    let mut pc: u32 = 0x8325AF70;
    'dispatch: loop {
        match pc {
            0x8325AF70 => {
    //   block [0x8325AF70..0x8325AFA4)
	// 8325AF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AF78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AF7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AF80: 386BAB24  addi r3, r11, -0x54dc
	ctx.r[3].s64 = ctx.r[11].s64 + -21724;
	// 8325AF84: 4B2252B5  bl 0x82480238
	ctx.lr = 0x8325AF88;
	sub_82480238(ctx, base);
	// 8325AF88: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325AF8C: 386AC0A0  addi r3, r10, -0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + -16224;
	// 8325AF90: 4BA4EF91  bl 0x82ca9f20
	ctx.lr = 0x8325AF94;
	sub_82CA9F20(ctx, base);
	// 8325AF94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325AFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325AFA8 size=236
    let mut pc: u32 = 0x8325AFA8;
    'dispatch: loop {
        match pc {
            0x8325AFA8 => {
    //   block [0x8325AFA8..0x8325B094)
	// 8325AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325AFB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325AFB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325AFB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325AFBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325AFC0: 3BEBAB30  addi r31, r11, -0x54d0
	ctx.r[31].s64 = ctx.r[11].s64 + -21712;
	// 8325AFC4: 388AC798  addi r4, r10, -0x3868
	ctx.r[4].s64 = ctx.r[10].s64 + -14440;
	// 8325AFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8325AFCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFD0: 4AFD1F01  bl 0x8222ced0
	ctx.lr = 0x8325AFD4;
	sub_8222CED0(ctx, base);
	// 8325AFD4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8325AFD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFDC: 3889C77C  addi r4, r9, -0x3884
	ctx.r[4].s64 = ctx.r[9].s64 + -14468;
	// 8325AFE0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8325AFE4: 4AFD1EED  bl 0x8222ced0
	ctx.lr = 0x8325AFE8;
	sub_8222CED0(ctx, base);
	// 8325AFE8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8325AFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325AFF0: 3888C760  addi r4, r8, -0x38a0
	ctx.r[4].s64 = ctx.r[8].s64 + -14496;
	// 8325AFF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325AFF8: 4AFD1ED9  bl 0x8222ced0
	ctx.lr = 0x8325AFFC;
	sub_8222CED0(ctx, base);
	// 8325AFFC: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8325B000: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B004: 3887C744  addi r4, r7, -0x38bc
	ctx.r[4].s64 = ctx.r[7].s64 + -14524;
	// 8325B008: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325B00C: 4AFD1EC5  bl 0x8222ced0
	ctx.lr = 0x8325B010;
	sub_8222CED0(ctx, base);
	// 8325B010: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8325B014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B018: 3886C728  addi r4, r6, -0x38d8
	ctx.r[4].s64 = ctx.r[6].s64 + -14552;
	// 8325B01C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8325B020: 4AFD1EB1  bl 0x8222ced0
	ctx.lr = 0x8325B024;
	sub_8222CED0(ctx, base);
	// 8325B024: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8325B028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B02C: 3884C70C  addi r4, r4, -0x38f4
	ctx.r[4].s64 = ctx.r[4].s64 + -14580;
	// 8325B030: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8325B034: 4AFD1E9D  bl 0x8222ced0
	ctx.lr = 0x8325B038;
	sub_8222CED0(ctx, base);
	// 8325B038: 3C60820D  lis r3, -0x7df3
	ctx.r[3].s64 = -2113077248;
	// 8325B03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B040: 3883C6F0  addi r4, r3, -0x3910
	ctx.r[4].s64 = ctx.r[3].s64 + -14608;
	// 8325B044: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325B048: 4AFD1E89  bl 0x8222ced0
	ctx.lr = 0x8325B04C;
	sub_8222CED0(ctx, base);
	// 8325B04C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B054: 388BC6D4  addi r4, r11, -0x392c
	ctx.r[4].s64 = ctx.r[11].s64 + -14636;
	// 8325B058: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8325B05C: 4AFD1E75  bl 0x8222ced0
	ctx.lr = 0x8325B060;
	sub_8222CED0(ctx, base);
	// 8325B060: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8325B064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B068: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 8325B06C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8325B070: 4AFD1E61  bl 0x8222ced0
	ctx.lr = 0x8325B074;
	sub_8222CED0(ctx, base);
	// 8325B074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B078: 3869C0B0  addi r3, r9, -0x3f50
	ctx.r[3].s64 = ctx.r[9].s64 + -16208;
	// 8325B07C: 4BA4EEA5  bl 0x82ca9f20
	ctx.lr = 0x8325B080;
	sub_82CA9F20(ctx, base);
	// 8325B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B08C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325B090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B098 size=64
    let mut pc: u32 = 0x8325B098;
    'dispatch: loop {
        match pc {
            0x8325B098 => {
    //   block [0x8325B098..0x8325B0D8)
	// 8325B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B0A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B0AC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325B0B0: 386AAB54  addi r3, r10, -0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21676;
	// 8325B0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B0B8: 4AFD1E19  bl 0x8222ced0
	ctx.lr = 0x8325B0BC;
	sub_8222CED0(ctx, base);
	// 8325B0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B0C0: 3869C138  addi r3, r9, -0x3ec8
	ctx.r[3].s64 = ctx.r[9].s64 + -16072;
	// 8325B0C4: 4BA4EE5D  bl 0x82ca9f20
	ctx.lr = 0x8325B0C8;
	sub_82CA9F20(ctx, base);
	// 8325B0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B0D8 size=64
    let mut pc: u32 = 0x8325B0D8;
    'dispatch: loop {
        match pc {
            0x8325B0D8 => {
    //   block [0x8325B0D8..0x8325B118)
	// 8325B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B0E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B0E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B0EC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325B0F0: 386AAB58  addi r3, r10, -0x54a8
	ctx.r[3].s64 = ctx.r[10].s64 + -21672;
	// 8325B0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B0F8: 4AFD1DD9  bl 0x8222ced0
	ctx.lr = 0x8325B0FC;
	sub_8222CED0(ctx, base);
	// 8325B0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B100: 3869C148  addi r3, r9, -0x3eb8
	ctx.r[3].s64 = ctx.r[9].s64 + -16056;
	// 8325B104: 4BA4EE1D  bl 0x82ca9f20
	ctx.lr = 0x8325B108;
	sub_82CA9F20(ctx, base);
	// 8325B108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B118 size=64
    let mut pc: u32 = 0x8325B118;
    'dispatch: loop {
        match pc {
            0x8325B118 => {
    //   block [0x8325B118..0x8325B158)
	// 8325B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B12C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325B130: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 8325B134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B138: 4AFD1D99  bl 0x8222ced0
	ctx.lr = 0x8325B13C;
	sub_8222CED0(ctx, base);
	// 8325B13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B140: 3869C158  addi r3, r9, -0x3ea8
	ctx.r[3].s64 = ctx.r[9].s64 + -16040;
	// 8325B144: 4BA4EDDD  bl 0x82ca9f20
	ctx.lr = 0x8325B148;
	sub_82CA9F20(ctx, base);
	// 8325B148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B158 size=56
    let mut pc: u32 = 0x8325B158;
    'dispatch: loop {
        match pc {
            0x8325B158 => {
    //   block [0x8325B158..0x8325B190)
	// 8325B158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B16C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325B170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B174: 4AF98BE5  bl 0x821f3d58
	ctx.lr = 0x8325B178;
	sub_821F3D58(ctx, base);
	// 8325B178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B17C: 906AAB60  stw r3, -0x54a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21664 as u32), ctx.r[3].u32 ) };
	// 8325B180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B190 size=56
    let mut pc: u32 = 0x8325B190;
    'dispatch: loop {
        match pc {
            0x8325B190 => {
    //   block [0x8325B190..0x8325B1C8)
	// 8325B190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325B1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1AC: 4AF98BAD  bl 0x821f3d58
	ctx.lr = 0x8325B1B0;
	sub_821F3D58(ctx, base);
	// 8325B1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1B4: 906AAB64  stw r3, -0x549c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21660 as u32), ctx.r[3].u32 ) };
	// 8325B1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B1C8 size=56
    let mut pc: u32 = 0x8325B1C8;
    'dispatch: loop {
        match pc {
            0x8325B1C8 => {
    //   block [0x8325B1C8..0x8325B200)
	// 8325B1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B1DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325B1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B1E4: 4AF98B75  bl 0x821f3d58
	ctx.lr = 0x8325B1E8;
	sub_821F3D58(ctx, base);
	// 8325B1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B1EC: 906AAB68  stw r3, -0x5498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21656 as u32), ctx.r[3].u32 ) };
	// 8325B1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B200 size=56
    let mut pc: u32 = 0x8325B200;
    'dispatch: loop {
        match pc {
            0x8325B200 => {
    //   block [0x8325B200..0x8325B238)
	// 8325B200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B214: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325B218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B21C: 4AF98B3D  bl 0x821f3d58
	ctx.lr = 0x8325B220;
	sub_821F3D58(ctx, base);
	// 8325B220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B224: 906AAB6C  stw r3, -0x5494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21652 as u32), ctx.r[3].u32 ) };
	// 8325B228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B238 size=56
    let mut pc: u32 = 0x8325B238;
    'dispatch: loop {
        match pc {
            0x8325B238 => {
    //   block [0x8325B238..0x8325B270)
	// 8325B238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B24C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325B250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B254: 4AF98B05  bl 0x821f3d58
	ctx.lr = 0x8325B258;
	sub_821F3D58(ctx, base);
	// 8325B258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B25C: 906AAB70  stw r3, -0x5490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21648 as u32), ctx.r[3].u32 ) };
	// 8325B260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B270 size=56
    let mut pc: u32 = 0x8325B270;
    'dispatch: loop {
        match pc {
            0x8325B270 => {
    //   block [0x8325B270..0x8325B2A8)
	// 8325B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B284: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325B288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B28C: 4AF98ACD  bl 0x821f3d58
	ctx.lr = 0x8325B290;
	sub_821F3D58(ctx, base);
	// 8325B290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B294: 906AAB74  stw r3, -0x548c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21644 as u32), ctx.r[3].u32 ) };
	// 8325B298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2A8 size=56
    let mut pc: u32 = 0x8325B2A8;
    'dispatch: loop {
        match pc {
            0x8325B2A8 => {
    //   block [0x8325B2A8..0x8325B2E0)
	// 8325B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325B2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2C4: 4AF98A95  bl 0x821f3d58
	ctx.lr = 0x8325B2C8;
	sub_821F3D58(ctx, base);
	// 8325B2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B2CC: 906AAB78  stw r3, -0x5488(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21640 as u32), ctx.r[3].u32 ) };
	// 8325B2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B2E0 size=56
    let mut pc: u32 = 0x8325B2E0;
    'dispatch: loop {
        match pc {
            0x8325B2E0 => {
    //   block [0x8325B2E0..0x8325B318)
	// 8325B2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B2F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325B2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B2FC: 4AF98A5D  bl 0x821f3d58
	ctx.lr = 0x8325B300;
	sub_821F3D58(ctx, base);
	// 8325B300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B304: 906AAB7C  stw r3, -0x5484(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21636 as u32), ctx.r[3].u32 ) };
	// 8325B308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B318 size=56
    let mut pc: u32 = 0x8325B318;
    'dispatch: loop {
        match pc {
            0x8325B318 => {
    //   block [0x8325B318..0x8325B350)
	// 8325B318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B32C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325B330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B334: 4AF98A25  bl 0x821f3d58
	ctx.lr = 0x8325B338;
	sub_821F3D58(ctx, base);
	// 8325B338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B33C: 906AAB80  stw r3, -0x5480(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21632 as u32), ctx.r[3].u32 ) };
	// 8325B340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B350 size=56
    let mut pc: u32 = 0x8325B350;
    'dispatch: loop {
        match pc {
            0x8325B350 => {
    //   block [0x8325B350..0x8325B388)
	// 8325B350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B364: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325B368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B36C: 4AF989ED  bl 0x821f3d58
	ctx.lr = 0x8325B370;
	sub_821F3D58(ctx, base);
	// 8325B370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B374: 906AAB84  stw r3, -0x547c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21628 as u32), ctx.r[3].u32 ) };
	// 8325B378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B388 size=56
    let mut pc: u32 = 0x8325B388;
    'dispatch: loop {
        match pc {
            0x8325B388 => {
    //   block [0x8325B388..0x8325B3C0)
	// 8325B388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B39C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325B3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3A4: 4AF989B5  bl 0x821f3d58
	ctx.lr = 0x8325B3A8;
	sub_821F3D58(ctx, base);
	// 8325B3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3AC: 906AAB88  stw r3, -0x5478(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21624 as u32), ctx.r[3].u32 ) };
	// 8325B3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3C0 size=56
    let mut pc: u32 = 0x8325B3C0;
    'dispatch: loop {
        match pc {
            0x8325B3C0 => {
    //   block [0x8325B3C0..0x8325B3F8)
	// 8325B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B3D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325B3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B3DC: 4AF9897D  bl 0x821f3d58
	ctx.lr = 0x8325B3E0;
	sub_821F3D58(ctx, base);
	// 8325B3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B3E4: 906AAB8C  stw r3, -0x5474(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21620 as u32), ctx.r[3].u32 ) };
	// 8325B3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B3F8 size=56
    let mut pc: u32 = 0x8325B3F8;
    'dispatch: loop {
        match pc {
            0x8325B3F8 => {
    //   block [0x8325B3F8..0x8325B430)
	// 8325B3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B40C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325B410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B414: 4AF98945  bl 0x821f3d58
	ctx.lr = 0x8325B418;
	sub_821F3D58(ctx, base);
	// 8325B418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B41C: 906AAB90  stw r3, -0x5470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21616 as u32), ctx.r[3].u32 ) };
	// 8325B420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B430 size=56
    let mut pc: u32 = 0x8325B430;
    'dispatch: loop {
        match pc {
            0x8325B430 => {
    //   block [0x8325B430..0x8325B468)
	// 8325B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B444: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325B448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B44C: 4AF9890D  bl 0x821f3d58
	ctx.lr = 0x8325B450;
	sub_821F3D58(ctx, base);
	// 8325B450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B454: 906AAB94  stw r3, -0x546c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21612 as u32), ctx.r[3].u32 ) };
	// 8325B458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B468 size=56
    let mut pc: u32 = 0x8325B468;
    'dispatch: loop {
        match pc {
            0x8325B468 => {
    //   block [0x8325B468..0x8325B4A0)
	// 8325B468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B47C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325B480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B484: 4AF988D5  bl 0x821f3d58
	ctx.lr = 0x8325B488;
	sub_821F3D58(ctx, base);
	// 8325B488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B48C: 906AAB98  stw r3, -0x5468(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21608 as u32), ctx.r[3].u32 ) };
	// 8325B490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4A0 size=56
    let mut pc: u32 = 0x8325B4A0;
    'dispatch: loop {
        match pc {
            0x8325B4A0 => {
    //   block [0x8325B4A0..0x8325B4D8)
	// 8325B4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325B4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4BC: 4AF9889D  bl 0x821f3d58
	ctx.lr = 0x8325B4C0;
	sub_821F3D58(ctx, base);
	// 8325B4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4C4: 906AAB9C  stw r3, -0x5464(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21604 as u32), ctx.r[3].u32 ) };
	// 8325B4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B4D8 size=56
    let mut pc: u32 = 0x8325B4D8;
    'dispatch: loop {
        match pc {
            0x8325B4D8 => {
    //   block [0x8325B4D8..0x8325B510)
	// 8325B4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B4EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325B4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B4F4: 4AF98865  bl 0x821f3d58
	ctx.lr = 0x8325B4F8;
	sub_821F3D58(ctx, base);
	// 8325B4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B4FC: 906AABA0  stw r3, -0x5460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21600 as u32), ctx.r[3].u32 ) };
	// 8325B500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B510 size=56
    let mut pc: u32 = 0x8325B510;
    'dispatch: loop {
        match pc {
            0x8325B510 => {
    //   block [0x8325B510..0x8325B548)
	// 8325B510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B524: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325B528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B52C: 4AF9882D  bl 0x821f3d58
	ctx.lr = 0x8325B530;
	sub_821F3D58(ctx, base);
	// 8325B530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B534: 906AABA4  stw r3, -0x545c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21596 as u32), ctx.r[3].u32 ) };
	// 8325B538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B548 size=56
    let mut pc: u32 = 0x8325B548;
    'dispatch: loop {
        match pc {
            0x8325B548 => {
    //   block [0x8325B548..0x8325B580)
	// 8325B548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B55C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325B560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B564: 4AF987F5  bl 0x821f3d58
	ctx.lr = 0x8325B568;
	sub_821F3D58(ctx, base);
	// 8325B568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B56C: 906AABA8  stw r3, -0x5458(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21592 as u32), ctx.r[3].u32 ) };
	// 8325B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B580 size=56
    let mut pc: u32 = 0x8325B580;
    'dispatch: loop {
        match pc {
            0x8325B580 => {
    //   block [0x8325B580..0x8325B5B8)
	// 8325B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B594: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325B598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B59C: 4AF987BD  bl 0x821f3d58
	ctx.lr = 0x8325B5A0;
	sub_821F3D58(ctx, base);
	// 8325B5A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5A4: 906AABAC  stw r3, -0x5454(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21588 as u32), ctx.r[3].u32 ) };
	// 8325B5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5B8 size=56
    let mut pc: u32 = 0x8325B5B8;
    'dispatch: loop {
        match pc {
            0x8325B5B8 => {
    //   block [0x8325B5B8..0x8325B5F0)
	// 8325B5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325B5CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325B5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325B5D4: 4AF98785  bl 0x821f3d58
	ctx.lr = 0x8325B5D8;
	sub_821F3D58(ctx, base);
	// 8325B5D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B5DC: 906AABB0  stw r3, -0x5450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21584 as u32), ctx.r[3].u32 ) };
	// 8325B5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B5F0 size=64
    let mut pc: u32 = 0x8325B5F0;
    'dispatch: loop {
        match pc {
            0x8325B5F0 => {
    //   block [0x8325B5F0..0x8325B630)
	// 8325B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B5FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B604: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325B608: 386AABB4  addi r3, r10, -0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + -21580;
	// 8325B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B610: 4AFD18C1  bl 0x8222ced0
	ctx.lr = 0x8325B614;
	sub_8222CED0(ctx, base);
	// 8325B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B618: 3869C168  addi r3, r9, -0x3e98
	ctx.r[3].s64 = ctx.r[9].s64 + -16024;
	// 8325B61C: 4BA4E905  bl 0x82ca9f20
	ctx.lr = 0x8325B620;
	sub_82CA9F20(ctx, base);
	// 8325B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B630 size=64
    let mut pc: u32 = 0x8325B630;
    'dispatch: loop {
        match pc {
            0x8325B630 => {
    //   block [0x8325B630..0x8325B670)
	// 8325B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B63C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B644: 388BCE84  addi r4, r11, -0x317c
	ctx.r[4].s64 = ctx.r[11].s64 + -12668;
	// 8325B648: 386AABB8  addi r3, r10, -0x5448
	ctx.r[3].s64 = ctx.r[10].s64 + -21576;
	// 8325B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B650: 4AFD1881  bl 0x8222ced0
	ctx.lr = 0x8325B654;
	sub_8222CED0(ctx, base);
	// 8325B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B658: 3869C178  addi r3, r9, -0x3e88
	ctx.r[3].s64 = ctx.r[9].s64 + -16008;
	// 8325B65C: 4BA4E8C5  bl 0x82ca9f20
	ctx.lr = 0x8325B660;
	sub_82CA9F20(ctx, base);
	// 8325B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B670 size=64
    let mut pc: u32 = 0x8325B670;
    'dispatch: loop {
        match pc {
            0x8325B670 => {
    //   block [0x8325B670..0x8325B6B0)
	// 8325B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325B680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B684: 388B4A34  addi r4, r11, 0x4a34
	ctx.r[4].s64 = ctx.r[11].s64 + 18996;
	// 8325B688: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 8325B68C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B690: 4AFD1841  bl 0x8222ced0
	ctx.lr = 0x8325B694;
	sub_8222CED0(ctx, base);
	// 8325B694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B698: 3869C188  addi r3, r9, -0x3e78
	ctx.r[3].s64 = ctx.r[9].s64 + -15992;
	// 8325B69C: 4BA4E885  bl 0x82ca9f20
	ctx.lr = 0x8325B6A0;
	sub_82CA9F20(ctx, base);
	// 8325B6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6B0 size=64
    let mut pc: u32 = 0x8325B6B0;
    'dispatch: loop {
        match pc {
            0x8325B6B0 => {
    //   block [0x8325B6B0..0x8325B6F0)
	// 8325B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B6BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B6C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B6C4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325B6C8: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 8325B6CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B6D0: 4AFD1801  bl 0x8222ced0
	ctx.lr = 0x8325B6D4;
	sub_8222CED0(ctx, base);
	// 8325B6D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B6D8: 3869C198  addi r3, r9, -0x3e68
	ctx.r[3].s64 = ctx.r[9].s64 + -15976;
	// 8325B6DC: 4BA4E845  bl 0x82ca9f20
	ctx.lr = 0x8325B6E0;
	sub_82CA9F20(ctx, base);
	// 8325B6E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B6F0 size=64
    let mut pc: u32 = 0x8325B6F0;
    'dispatch: loop {
        match pc {
            0x8325B6F0 => {
    //   block [0x8325B6F0..0x8325B730)
	// 8325B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B6F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B6FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B704: 388BCEC4  addi r4, r11, -0x313c
	ctx.r[4].s64 = ctx.r[11].s64 + -12604;
	// 8325B708: 386AABC4  addi r3, r10, -0x543c
	ctx.r[3].s64 = ctx.r[10].s64 + -21564;
	// 8325B70C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B710: 4AFD17C1  bl 0x8222ced0
	ctx.lr = 0x8325B714;
	sub_8222CED0(ctx, base);
	// 8325B714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B718: 3869C1A8  addi r3, r9, -0x3e58
	ctx.r[3].s64 = ctx.r[9].s64 + -15960;
	// 8325B71C: 4BA4E805  bl 0x82ca9f20
	ctx.lr = 0x8325B720;
	sub_82CA9F20(ctx, base);
	// 8325B720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B730 size=64
    let mut pc: u32 = 0x8325B730;
    'dispatch: loop {
        match pc {
            0x8325B730 => {
    //   block [0x8325B730..0x8325B770)
	// 8325B730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B73C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B744: 388BCED8  addi r4, r11, -0x3128
	ctx.r[4].s64 = ctx.r[11].s64 + -12584;
	// 8325B748: 386AABC8  addi r3, r10, -0x5438
	ctx.r[3].s64 = ctx.r[10].s64 + -21560;
	// 8325B74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B750: 4AFD1781  bl 0x8222ced0
	ctx.lr = 0x8325B754;
	sub_8222CED0(ctx, base);
	// 8325B754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B758: 3869C1B8  addi r3, r9, -0x3e48
	ctx.r[3].s64 = ctx.r[9].s64 + -15944;
	// 8325B75C: 4BA4E7C5  bl 0x82ca9f20
	ctx.lr = 0x8325B760;
	sub_82CA9F20(ctx, base);
	// 8325B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B770 size=64
    let mut pc: u32 = 0x8325B770;
    'dispatch: loop {
        match pc {
            0x8325B770 => {
    //   block [0x8325B770..0x8325B7B0)
	// 8325B770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B77C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B784: 388BCEF8  addi r4, r11, -0x3108
	ctx.r[4].s64 = ctx.r[11].s64 + -12552;
	// 8325B788: 386AABCC  addi r3, r10, -0x5434
	ctx.r[3].s64 = ctx.r[10].s64 + -21556;
	// 8325B78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B790: 4AFD1741  bl 0x8222ced0
	ctx.lr = 0x8325B794;
	sub_8222CED0(ctx, base);
	// 8325B794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B798: 3869C1C8  addi r3, r9, -0x3e38
	ctx.r[3].s64 = ctx.r[9].s64 + -15928;
	// 8325B79C: 4BA4E785  bl 0x82ca9f20
	ctx.lr = 0x8325B7A0;
	sub_82CA9F20(ctx, base);
	// 8325B7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B7B0 size=64
    let mut pc: u32 = 0x8325B7B0;
    'dispatch: loop {
        match pc {
            0x8325B7B0 => {
    //   block [0x8325B7B0..0x8325B7F0)
	// 8325B7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B7B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B7BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B7C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B7C4: 388BCF1C  addi r4, r11, -0x30e4
	ctx.r[4].s64 = ctx.r[11].s64 + -12516;
	// 8325B7C8: 386AABD0  addi r3, r10, -0x5430
	ctx.r[3].s64 = ctx.r[10].s64 + -21552;
	// 8325B7CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B7D0: 4AFD1701  bl 0x8222ced0
	ctx.lr = 0x8325B7D4;
	sub_8222CED0(ctx, base);
	// 8325B7D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B7D8: 3869C1D8  addi r3, r9, -0x3e28
	ctx.r[3].s64 = ctx.r[9].s64 + -15912;
	// 8325B7DC: 4BA4E745  bl 0x82ca9f20
	ctx.lr = 0x8325B7E0;
	sub_82CA9F20(ctx, base);
	// 8325B7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B7F0 size=64
    let mut pc: u32 = 0x8325B7F0;
    'dispatch: loop {
        match pc {
            0x8325B7F0 => {
    //   block [0x8325B7F0..0x8325B830)
	// 8325B7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B7F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B7FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B804: 388BCF58  addi r4, r11, -0x30a8
	ctx.r[4].s64 = ctx.r[11].s64 + -12456;
	// 8325B808: 386AABD4  addi r3, r10, -0x542c
	ctx.r[3].s64 = ctx.r[10].s64 + -21548;
	// 8325B80C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B810: 4AFD16C1  bl 0x8222ced0
	ctx.lr = 0x8325B814;
	sub_8222CED0(ctx, base);
	// 8325B814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B818: 3869C1E8  addi r3, r9, -0x3e18
	ctx.r[3].s64 = ctx.r[9].s64 + -15896;
	// 8325B81C: 4BA4E705  bl 0x82ca9f20
	ctx.lr = 0x8325B820;
	sub_82CA9F20(ctx, base);
	// 8325B820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B830 size=64
    let mut pc: u32 = 0x8325B830;
    'dispatch: loop {
        match pc {
            0x8325B830 => {
    //   block [0x8325B830..0x8325B870)
	// 8325B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B83C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B844: 388BCF94  addi r4, r11, -0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + -12396;
	// 8325B848: 386AABD8  addi r3, r10, -0x5428
	ctx.r[3].s64 = ctx.r[10].s64 + -21544;
	// 8325B84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B850: 4AFD1681  bl 0x8222ced0
	ctx.lr = 0x8325B854;
	sub_8222CED0(ctx, base);
	// 8325B854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B858: 3869C1F8  addi r3, r9, -0x3e08
	ctx.r[3].s64 = ctx.r[9].s64 + -15880;
	// 8325B85C: 4BA4E6C5  bl 0x82ca9f20
	ctx.lr = 0x8325B860;
	sub_82CA9F20(ctx, base);
	// 8325B860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B870 size=64
    let mut pc: u32 = 0x8325B870;
    'dispatch: loop {
        match pc {
            0x8325B870 => {
    //   block [0x8325B870..0x8325B8B0)
	// 8325B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B87C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B884: 388BCFD0  addi r4, r11, -0x3030
	ctx.r[4].s64 = ctx.r[11].s64 + -12336;
	// 8325B888: 386AABDC  addi r3, r10, -0x5424
	ctx.r[3].s64 = ctx.r[10].s64 + -21540;
	// 8325B88C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B890: 4AFD1641  bl 0x8222ced0
	ctx.lr = 0x8325B894;
	sub_8222CED0(ctx, base);
	// 8325B894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B898: 3869C208  addi r3, r9, -0x3df8
	ctx.r[3].s64 = ctx.r[9].s64 + -15864;
	// 8325B89C: 4BA4E685  bl 0x82ca9f20
	ctx.lr = 0x8325B8A0;
	sub_82CA9F20(ctx, base);
	// 8325B8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B8B0 size=64
    let mut pc: u32 = 0x8325B8B0;
    'dispatch: loop {
        match pc {
            0x8325B8B0 => {
    //   block [0x8325B8B0..0x8325B8F0)
	// 8325B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B8B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B8BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B8C4: 388BD008  addi r4, r11, -0x2ff8
	ctx.r[4].s64 = ctx.r[11].s64 + -12280;
	// 8325B8C8: 386AABE0  addi r3, r10, -0x5420
	ctx.r[3].s64 = ctx.r[10].s64 + -21536;
	// 8325B8CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B8D0: 4AFD1601  bl 0x8222ced0
	ctx.lr = 0x8325B8D4;
	sub_8222CED0(ctx, base);
	// 8325B8D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B8D8: 3869C218  addi r3, r9, -0x3de8
	ctx.r[3].s64 = ctx.r[9].s64 + -15848;
	// 8325B8DC: 4BA4E645  bl 0x82ca9f20
	ctx.lr = 0x8325B8E0;
	sub_82CA9F20(ctx, base);
	// 8325B8E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B8F0 size=64
    let mut pc: u32 = 0x8325B8F0;
    'dispatch: loop {
        match pc {
            0x8325B8F0 => {
    //   block [0x8325B8F0..0x8325B930)
	// 8325B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B8FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B904: 388BD030  addi r4, r11, -0x2fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -12240;
	// 8325B908: 386AABE4  addi r3, r10, -0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + -21532;
	// 8325B90C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B910: 4AFD15C1  bl 0x8222ced0
	ctx.lr = 0x8325B914;
	sub_8222CED0(ctx, base);
	// 8325B914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B918: 3869C228  addi r3, r9, -0x3dd8
	ctx.r[3].s64 = ctx.r[9].s64 + -15832;
	// 8325B91C: 4BA4E605  bl 0x82ca9f20
	ctx.lr = 0x8325B920;
	sub_82CA9F20(ctx, base);
	// 8325B920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B930 size=64
    let mut pc: u32 = 0x8325B930;
    'dispatch: loop {
        match pc {
            0x8325B930 => {
    //   block [0x8325B930..0x8325B970)
	// 8325B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B93C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B944: 388BD06C  addi r4, r11, -0x2f94
	ctx.r[4].s64 = ctx.r[11].s64 + -12180;
	// 8325B948: 386AABE8  addi r3, r10, -0x5418
	ctx.r[3].s64 = ctx.r[10].s64 + -21528;
	// 8325B94C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B950: 4AFD1581  bl 0x8222ced0
	ctx.lr = 0x8325B954;
	sub_8222CED0(ctx, base);
	// 8325B954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B958: 3869C238  addi r3, r9, -0x3dc8
	ctx.r[3].s64 = ctx.r[9].s64 + -15816;
	// 8325B95C: 4BA4E5C5  bl 0x82ca9f20
	ctx.lr = 0x8325B960;
	sub_82CA9F20(ctx, base);
	// 8325B960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B970 size=64
    let mut pc: u32 = 0x8325B970;
    'dispatch: loop {
        match pc {
            0x8325B970 => {
    //   block [0x8325B970..0x8325B9B0)
	// 8325B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B97C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B984: 388BD09C  addi r4, r11, -0x2f64
	ctx.r[4].s64 = ctx.r[11].s64 + -12132;
	// 8325B988: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 8325B98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B990: 4AFD1541  bl 0x8222ced0
	ctx.lr = 0x8325B994;
	sub_8222CED0(ctx, base);
	// 8325B994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B998: 3869C248  addi r3, r9, -0x3db8
	ctx.r[3].s64 = ctx.r[9].s64 + -15800;
	// 8325B99C: 4BA4E585  bl 0x82ca9f20
	ctx.lr = 0x8325B9A0;
	sub_82CA9F20(ctx, base);
	// 8325B9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B9B0 size=64
    let mut pc: u32 = 0x8325B9B0;
    'dispatch: loop {
        match pc {
            0x8325B9B0 => {
    //   block [0x8325B9B0..0x8325B9F0)
	// 8325B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B9B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B9BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325B9C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325B9C4: 388BD0C8  addi r4, r11, -0x2f38
	ctx.r[4].s64 = ctx.r[11].s64 + -12088;
	// 8325B9C8: 386AABF0  addi r3, r10, -0x5410
	ctx.r[3].s64 = ctx.r[10].s64 + -21520;
	// 8325B9CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325B9D0: 4AFD1501  bl 0x8222ced0
	ctx.lr = 0x8325B9D4;
	sub_8222CED0(ctx, base);
	// 8325B9D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325B9D8: 3869C258  addi r3, r9, -0x3da8
	ctx.r[3].s64 = ctx.r[9].s64 + -15784;
	// 8325B9DC: 4BA4E545  bl 0x82ca9f20
	ctx.lr = 0x8325B9E0;
	sub_82CA9F20(ctx, base);
	// 8325B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325B9F0 size=64
    let mut pc: u32 = 0x8325B9F0;
    'dispatch: loop {
        match pc {
            0x8325B9F0 => {
    //   block [0x8325B9F0..0x8325BA30)
	// 8325B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325B9FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA04: 388BD0F0  addi r4, r11, -0x2f10
	ctx.r[4].s64 = ctx.r[11].s64 + -12048;
	// 8325BA08: 386AABF4  addi r3, r10, -0x540c
	ctx.r[3].s64 = ctx.r[10].s64 + -21516;
	// 8325BA0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA10: 4AFD14C1  bl 0x8222ced0
	ctx.lr = 0x8325BA14;
	sub_8222CED0(ctx, base);
	// 8325BA14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA18: 3869C268  addi r3, r9, -0x3d98
	ctx.r[3].s64 = ctx.r[9].s64 + -15768;
	// 8325BA1C: 4BA4E505  bl 0x82ca9f20
	ctx.lr = 0x8325BA20;
	sub_82CA9F20(ctx, base);
	// 8325BA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BA30 size=64
    let mut pc: u32 = 0x8325BA30;
    'dispatch: loop {
        match pc {
            0x8325BA30 => {
    //   block [0x8325BA30..0x8325BA70)
	// 8325BA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BA3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA44: 388BD114  addi r4, r11, -0x2eec
	ctx.r[4].s64 = ctx.r[11].s64 + -12012;
	// 8325BA48: 386AABF8  addi r3, r10, -0x5408
	ctx.r[3].s64 = ctx.r[10].s64 + -21512;
	// 8325BA4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA50: 4AFD1481  bl 0x8222ced0
	ctx.lr = 0x8325BA54;
	sub_8222CED0(ctx, base);
	// 8325BA54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA58: 3869C278  addi r3, r9, -0x3d88
	ctx.r[3].s64 = ctx.r[9].s64 + -15752;
	// 8325BA5C: 4BA4E4C5  bl 0x82ca9f20
	ctx.lr = 0x8325BA60;
	sub_82CA9F20(ctx, base);
	// 8325BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BA70 size=64
    let mut pc: u32 = 0x8325BA70;
    'dispatch: loop {
        match pc {
            0x8325BA70 => {
    //   block [0x8325BA70..0x8325BAB0)
	// 8325BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BA7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BA84: 388BD134  addi r4, r11, -0x2ecc
	ctx.r[4].s64 = ctx.r[11].s64 + -11980;
	// 8325BA88: 386AABFC  addi r3, r10, -0x5404
	ctx.r[3].s64 = ctx.r[10].s64 + -21508;
	// 8325BA8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BA90: 4AFD1441  bl 0x8222ced0
	ctx.lr = 0x8325BA94;
	sub_8222CED0(ctx, base);
	// 8325BA94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BA98: 3869C288  addi r3, r9, -0x3d78
	ctx.r[3].s64 = ctx.r[9].s64 + -15736;
	// 8325BA9C: 4BA4E485  bl 0x82ca9f20
	ctx.lr = 0x8325BAA0;
	sub_82CA9F20(ctx, base);
	// 8325BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BAB0 size=64
    let mut pc: u32 = 0x8325BAB0;
    'dispatch: loop {
        match pc {
            0x8325BAB0 => {
    //   block [0x8325BAB0..0x8325BAF0)
	// 8325BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BABC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BAC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BAC4: 388BD158  addi r4, r11, -0x2ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -11944;
	// 8325BAC8: 386AAC00  addi r3, r10, -0x5400
	ctx.r[3].s64 = ctx.r[10].s64 + -21504;
	// 8325BACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BAD0: 4AFD1401  bl 0x8222ced0
	ctx.lr = 0x8325BAD4;
	sub_8222CED0(ctx, base);
	// 8325BAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BAD8: 3869C298  addi r3, r9, -0x3d68
	ctx.r[3].s64 = ctx.r[9].s64 + -15720;
	// 8325BADC: 4BA4E445  bl 0x82ca9f20
	ctx.lr = 0x8325BAE0;
	sub_82CA9F20(ctx, base);
	// 8325BAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BAF0 size=64
    let mut pc: u32 = 0x8325BAF0;
    'dispatch: loop {
        match pc {
            0x8325BAF0 => {
    //   block [0x8325BAF0..0x8325BB30)
	// 8325BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BAFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB04: 388BD180  addi r4, r11, -0x2e80
	ctx.r[4].s64 = ctx.r[11].s64 + -11904;
	// 8325BB08: 386AAC04  addi r3, r10, -0x53fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21500;
	// 8325BB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB10: 4AFD13C1  bl 0x8222ced0
	ctx.lr = 0x8325BB14;
	sub_8222CED0(ctx, base);
	// 8325BB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB18: 3869C2A8  addi r3, r9, -0x3d58
	ctx.r[3].s64 = ctx.r[9].s64 + -15704;
	// 8325BB1C: 4BA4E405  bl 0x82ca9f20
	ctx.lr = 0x8325BB20;
	sub_82CA9F20(ctx, base);
	// 8325BB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BB30 size=64
    let mut pc: u32 = 0x8325BB30;
    'dispatch: loop {
        match pc {
            0x8325BB30 => {
    //   block [0x8325BB30..0x8325BB70)
	// 8325BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BB3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB44: 388BD1B0  addi r4, r11, -0x2e50
	ctx.r[4].s64 = ctx.r[11].s64 + -11856;
	// 8325BB48: 386AAC08  addi r3, r10, -0x53f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21496;
	// 8325BB4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB50: 4AFD1381  bl 0x8222ced0
	ctx.lr = 0x8325BB54;
	sub_8222CED0(ctx, base);
	// 8325BB54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB58: 3869C2B8  addi r3, r9, -0x3d48
	ctx.r[3].s64 = ctx.r[9].s64 + -15688;
	// 8325BB5C: 4BA4E3C5  bl 0x82ca9f20
	ctx.lr = 0x8325BB60;
	sub_82CA9F20(ctx, base);
	// 8325BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BB70 size=64
    let mut pc: u32 = 0x8325BB70;
    'dispatch: loop {
        match pc {
            0x8325BB70 => {
    //   block [0x8325BB70..0x8325BBB0)
	// 8325BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BB78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BB7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BB84: 388BD1D4  addi r4, r11, -0x2e2c
	ctx.r[4].s64 = ctx.r[11].s64 + -11820;
	// 8325BB88: 386AAC0C  addi r3, r10, -0x53f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21492;
	// 8325BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BB90: 4AFD1341  bl 0x8222ced0
	ctx.lr = 0x8325BB94;
	sub_8222CED0(ctx, base);
	// 8325BB94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BB98: 3869C2C8  addi r3, r9, -0x3d38
	ctx.r[3].s64 = ctx.r[9].s64 + -15672;
	// 8325BB9C: 4BA4E385  bl 0x82ca9f20
	ctx.lr = 0x8325BBA0;
	sub_82CA9F20(ctx, base);
	// 8325BBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BBB0 size=64
    let mut pc: u32 = 0x8325BBB0;
    'dispatch: loop {
        match pc {
            0x8325BBB0 => {
    //   block [0x8325BBB0..0x8325BBF0)
	// 8325BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BBBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BBC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BBC4: 388BD20C  addi r4, r11, -0x2df4
	ctx.r[4].s64 = ctx.r[11].s64 + -11764;
	// 8325BBC8: 386AAC10  addi r3, r10, -0x53f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21488;
	// 8325BBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BBD0: 4AFD1301  bl 0x8222ced0
	ctx.lr = 0x8325BBD4;
	sub_8222CED0(ctx, base);
	// 8325BBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325BBD8: 3869C2D8  addi r3, r9, -0x3d28
	ctx.r[3].s64 = ctx.r[9].s64 + -15656;
	// 8325BBDC: 4BA4E345  bl 0x82ca9f20
	ctx.lr = 0x8325BBE0;
	sub_82CA9F20(ctx, base);
	// 8325BBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325BBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BBF0 size=192
    let mut pc: u32 = 0x8325BBF0;
    'dispatch: loop {
        match pc {
            0x8325BBF0 => {
    //   block [0x8325BBF0..0x8325BC48)
	// 8325BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BBF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BBFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BC00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BC08: 388BD244  addi r4, r11, -0x2dbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11708;
	// 8325BC0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BC10: 4AFD12C1  bl 0x8222ced0
	ctx.lr = 0x8325BC14;
	sub_8222CED0(ctx, base);
	// 8325BC14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC1C: 4AF92F1D  bl 0x821eeb38
	ctx.lr = 0x8325BC20;
	sub_821EEB38(ctx, base);
	// 8325BC20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC24: 4B9A7BCD  bl 0x82c037f0
	ctx.lr = 0x8325BC28;
	sub_82C037F0(ctx, base);
	// 8325BC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BC2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BC30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BC34: 916AAC14  stw r11, -0x53ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21484 as u32), ctx.r[11].u32 ) };
	// 8325BC38: 4AF6AB31  bl 0x821c6768
	ctx.lr = 0x8325BC3C;
	sub_821C6768(ctx, base);
	// 8325BC3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BC40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BC44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BC48; continue 'dispatch;
            }
            0x8325BC48 => {
    //   block [0x8325BC48..0x8325BC74)
	// 8325BC48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BC4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BC54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BC58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BC5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC60: 4082FFE8  bne 0x8325bc48
	if !ctx.cr[0].eq {
	pc = 0x8325BC48; continue 'dispatch;
	}
	// 8325BC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BC68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BC6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BC70: 4AF6AAF9  bl 0x821c6768
	ctx.lr = 0x8325BC74;
	sub_821C6768(ctx, base);
	pc = 0x8325BC74; continue 'dispatch;
            }
            0x8325BC74 => {
    //   block [0x8325BC74..0x8325BCB0)
	// 8325BC74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BC78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BC80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BC84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BC88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BC8C: 4082FFE8  bne 0x8325bc74
	if !ctx.cr[0].eq {
	pc = 0x8325BC74; continue 'dispatch;
	}
	// 8325BC90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BC94: 386BC2E8  addi r3, r11, -0x3d18
	ctx.r[3].s64 = ctx.r[11].s64 + -15640;
	// 8325BC98: 4BA4E289  bl 0x82ca9f20
	ctx.lr = 0x8325BC9C;
	sub_82CA9F20(ctx, base);
	// 8325BC9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BCA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BCB0 size=192
    let mut pc: u32 = 0x8325BCB0;
    'dispatch: loop {
        match pc {
            0x8325BCB0 => {
    //   block [0x8325BCB0..0x8325BD08)
	// 8325BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BCB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BCBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BCC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BCC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BCC8: 388BD270  addi r4, r11, -0x2d90
	ctx.r[4].s64 = ctx.r[11].s64 + -11664;
	// 8325BCCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BCD0: 4AFD1201  bl 0x8222ced0
	ctx.lr = 0x8325BCD4;
	sub_8222CED0(ctx, base);
	// 8325BCD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BCD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCDC: 4AF92E5D  bl 0x821eeb38
	ctx.lr = 0x8325BCE0;
	sub_821EEB38(ctx, base);
	// 8325BCE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCE4: 4B9A7B0D  bl 0x82c037f0
	ctx.lr = 0x8325BCE8;
	sub_82C037F0(ctx, base);
	// 8325BCE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BCEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BCF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BCF4: 916AAC18  stw r11, -0x53e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 8325BCF8: 4AF6AA71  bl 0x821c6768
	ctx.lr = 0x8325BCFC;
	sub_821C6768(ctx, base);
	// 8325BCFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BD00: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BD04: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BD08; continue 'dispatch;
            }
            0x8325BD08 => {
    //   block [0x8325BD08..0x8325BD34)
	// 8325BD08: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BD0C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD10: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BD14: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BD18: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BD1C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD20: 4082FFE8  bne 0x8325bd08
	if !ctx.cr[0].eq {
	pc = 0x8325BD08; continue 'dispatch;
	}
	// 8325BD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BD28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BD2C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BD30: 4AF6AA39  bl 0x821c6768
	ctx.lr = 0x8325BD34;
	sub_821C6768(ctx, base);
	pc = 0x8325BD34; continue 'dispatch;
            }
            0x8325BD34 => {
    //   block [0x8325BD34..0x8325BD70)
	// 8325BD34: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BD38: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD3C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BD40: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BD44: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BD48: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BD4C: 4082FFE8  bne 0x8325bd34
	if !ctx.cr[0].eq {
	pc = 0x8325BD34; continue 'dispatch;
	}
	// 8325BD50: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BD54: 386BC2F0  addi r3, r11, -0x3d10
	ctx.r[3].s64 = ctx.r[11].s64 + -15632;
	// 8325BD58: 4BA4E1C9  bl 0x82ca9f20
	ctx.lr = 0x8325BD5C;
	sub_82CA9F20(ctx, base);
	// 8325BD5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BD70 size=192
    let mut pc: u32 = 0x8325BD70;
    'dispatch: loop {
        match pc {
            0x8325BD70 => {
    //   block [0x8325BD70..0x8325BDC8)
	// 8325BD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BD78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BD7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BD80: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BD84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BD88: 388BD29C  addi r4, r11, -0x2d64
	ctx.r[4].s64 = ctx.r[11].s64 + -11620;
	// 8325BD8C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BD90: 4AFD1141  bl 0x8222ced0
	ctx.lr = 0x8325BD94;
	sub_8222CED0(ctx, base);
	// 8325BD94: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BD98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BD9C: 4AF92D9D  bl 0x821eeb38
	ctx.lr = 0x8325BDA0;
	sub_821EEB38(ctx, base);
	// 8325BDA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BDA4: 4B9A7A4D  bl 0x82c037f0
	ctx.lr = 0x8325BDA8;
	sub_82C037F0(ctx, base);
	// 8325BDA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BDAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BDB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BDB4: 916AAC1C  stw r11, -0x53e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21476 as u32), ctx.r[11].u32 ) };
	// 8325BDB8: 4AF6A9B1  bl 0x821c6768
	ctx.lr = 0x8325BDBC;
	sub_821C6768(ctx, base);
	// 8325BDBC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BDC0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BDC4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BDC8; continue 'dispatch;
            }
            0x8325BDC8 => {
    //   block [0x8325BDC8..0x8325BDF4)
	// 8325BDC8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BDCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDD0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BDD4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BDD8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BDDC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDE0: 4082FFE8  bne 0x8325bdc8
	if !ctx.cr[0].eq {
	pc = 0x8325BDC8; continue 'dispatch;
	}
	// 8325BDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BDE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BDEC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BDF0: 4AF6A979  bl 0x821c6768
	ctx.lr = 0x8325BDF4;
	sub_821C6768(ctx, base);
	pc = 0x8325BDF4; continue 'dispatch;
            }
            0x8325BDF4 => {
    //   block [0x8325BDF4..0x8325BE30)
	// 8325BDF4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BDF8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BDFC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BE00: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BE04: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BE08: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BE0C: 4082FFE8  bne 0x8325bdf4
	if !ctx.cr[0].eq {
	pc = 0x8325BDF4; continue 'dispatch;
	}
	// 8325BE10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BE14: 386BC2F8  addi r3, r11, -0x3d08
	ctx.r[3].s64 = ctx.r[11].s64 + -15624;
	// 8325BE18: 4BA4E109  bl 0x82ca9f20
	ctx.lr = 0x8325BE1C;
	sub_82CA9F20(ctx, base);
	// 8325BE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BE28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BE30 size=192
    let mut pc: u32 = 0x8325BE30;
    'dispatch: loop {
        match pc {
            0x8325BE30 => {
    //   block [0x8325BE30..0x8325BE88)
	// 8325BE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BE38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BE3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BE40: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BE44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BE48: 388BD2C4  addi r4, r11, -0x2d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -11580;
	// 8325BE4C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BE50: 4AFD1081  bl 0x8222ced0
	ctx.lr = 0x8325BE54;
	sub_8222CED0(ctx, base);
	// 8325BE54: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BE58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE5C: 4AF92CDD  bl 0x821eeb38
	ctx.lr = 0x8325BE60;
	sub_821EEB38(ctx, base);
	// 8325BE60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE64: 4B9A798D  bl 0x82c037f0
	ctx.lr = 0x8325BE68;
	sub_82C037F0(ctx, base);
	// 8325BE68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BE70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BE74: 916AAC20  stw r11, -0x53e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21472 as u32), ctx.r[11].u32 ) };
	// 8325BE78: 4AF6A8F1  bl 0x821c6768
	ctx.lr = 0x8325BE7C;
	sub_821C6768(ctx, base);
	// 8325BE7C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BE80: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BE84: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BE88; continue 'dispatch;
            }
            0x8325BE88 => {
    //   block [0x8325BE88..0x8325BEB4)
	// 8325BE88: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BE8C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BE90: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BE94: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BE98: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BE9C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BEA0: 4082FFE8  bne 0x8325be88
	if !ctx.cr[0].eq {
	pc = 0x8325BE88; continue 'dispatch;
	}
	// 8325BEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BEA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BEAC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BEB0: 4AF6A8B9  bl 0x821c6768
	ctx.lr = 0x8325BEB4;
	sub_821C6768(ctx, base);
	pc = 0x8325BEB4; continue 'dispatch;
            }
            0x8325BEB4 => {
    //   block [0x8325BEB4..0x8325BEF0)
	// 8325BEB4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BEB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BEBC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BEC0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BEC4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BEC8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BECC: 4082FFE8  bne 0x8325beb4
	if !ctx.cr[0].eq {
	pc = 0x8325BEB4; continue 'dispatch;
	}
	// 8325BED0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BED4: 386BC300  addi r3, r11, -0x3d00
	ctx.r[3].s64 = ctx.r[11].s64 + -15616;
	// 8325BED8: 4BA4E049  bl 0x82ca9f20
	ctx.lr = 0x8325BEDC;
	sub_82CA9F20(ctx, base);
	// 8325BEDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BEF0 size=192
    let mut pc: u32 = 0x8325BEF0;
    'dispatch: loop {
        match pc {
            0x8325BEF0 => {
    //   block [0x8325BEF0..0x8325BF48)
	// 8325BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BEFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BF00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BF04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BF08: 388BD2F0  addi r4, r11, -0x2d10
	ctx.r[4].s64 = ctx.r[11].s64 + -11536;
	// 8325BF0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BF10: 4AFD0FC1  bl 0x8222ced0
	ctx.lr = 0x8325BF14;
	sub_8222CED0(ctx, base);
	// 8325BF14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BF18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF1C: 4AF92C1D  bl 0x821eeb38
	ctx.lr = 0x8325BF20;
	sub_821EEB38(ctx, base);
	// 8325BF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF24: 4B9A78CD  bl 0x82c037f0
	ctx.lr = 0x8325BF28;
	sub_82C037F0(ctx, base);
	// 8325BF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BF2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BF30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BF34: 916AAC24  stw r11, -0x53dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21468 as u32), ctx.r[11].u32 ) };
	// 8325BF38: 4AF6A831  bl 0x821c6768
	ctx.lr = 0x8325BF3C;
	sub_821C6768(ctx, base);
	// 8325BF3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325BF40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325BF44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325BF48; continue 'dispatch;
            }
            0x8325BF48 => {
    //   block [0x8325BF48..0x8325BF74)
	// 8325BF48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325BF4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325BF54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325BF58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BF5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF60: 4082FFE8  bne 0x8325bf48
	if !ctx.cr[0].eq {
	pc = 0x8325BF48; continue 'dispatch;
	}
	// 8325BF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325BF68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BF6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325BF70: 4AF6A7F9  bl 0x821c6768
	ctx.lr = 0x8325BF74;
	sub_821C6768(ctx, base);
	pc = 0x8325BF74; continue 'dispatch;
            }
            0x8325BF74 => {
    //   block [0x8325BF74..0x8325BFB0)
	// 8325BF74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325BF78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325BF80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325BF84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325BF88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325BF8C: 4082FFE8  bne 0x8325bf74
	if !ctx.cr[0].eq {
	pc = 0x8325BF74; continue 'dispatch;
	}
	// 8325BF90: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325BF94: 386BC308  addi r3, r11, -0x3cf8
	ctx.r[3].s64 = ctx.r[11].s64 + -15608;
	// 8325BF98: 4BA4DF89  bl 0x82ca9f20
	ctx.lr = 0x8325BF9C;
	sub_82CA9F20(ctx, base);
	// 8325BF9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325BFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325BFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325BFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325BFB0 size=192
    let mut pc: u32 = 0x8325BFB0;
    'dispatch: loop {
        match pc {
            0x8325BFB0 => {
    //   block [0x8325BFB0..0x8325C008)
	// 8325BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325BFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325BFBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325BFC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325BFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325BFC8: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 8325BFCC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325BFD0: 4AFD0F01  bl 0x8222ced0
	ctx.lr = 0x8325BFD4;
	sub_8222CED0(ctx, base);
	// 8325BFD4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325BFD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFDC: 4AF92B5D  bl 0x821eeb38
	ctx.lr = 0x8325BFE0;
	sub_821EEB38(ctx, base);
	// 8325BFE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFE4: 4B9A780D  bl 0x82c037f0
	ctx.lr = 0x8325BFE8;
	sub_82C037F0(ctx, base);
	// 8325BFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325BFEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325BFF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325BFF4: 916AAC28  stw r11, -0x53d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21464 as u32), ctx.r[11].u32 ) };
	// 8325BFF8: 4AF6A771  bl 0x821c6768
	ctx.lr = 0x8325BFFC;
	sub_821C6768(ctx, base);
	// 8325BFFC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C000: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C004: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C008; continue 'dispatch;
            }
            0x8325C008 => {
    //   block [0x8325C008..0x8325C034)
	// 8325C008: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C00C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C010: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C014: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C018: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C01C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C020: 4082FFE8  bne 0x8325c008
	if !ctx.cr[0].eq {
	pc = 0x8325C008; continue 'dispatch;
	}
	// 8325C024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C028: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C02C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C030: 4AF6A739  bl 0x821c6768
	ctx.lr = 0x8325C034;
	sub_821C6768(ctx, base);
	pc = 0x8325C034; continue 'dispatch;
            }
            0x8325C034 => {
    //   block [0x8325C034..0x8325C070)
	// 8325C034: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C03C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C040: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C044: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C048: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C04C: 4082FFE8  bne 0x8325c034
	if !ctx.cr[0].eq {
	pc = 0x8325C034; continue 'dispatch;
	}
	// 8325C050: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C054: 386BC310  addi r3, r11, -0x3cf0
	ctx.r[3].s64 = ctx.r[11].s64 + -15600;
	// 8325C058: 4BA4DEC9  bl 0x82ca9f20
	ctx.lr = 0x8325C05C;
	sub_82CA9F20(ctx, base);
	// 8325C05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C070 size=192
    let mut pc: u32 = 0x8325C070;
    'dispatch: loop {
        match pc {
            0x8325C070 => {
    //   block [0x8325C070..0x8325C0C8)
	// 8325C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C080: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C088: 388BD344  addi r4, r11, -0x2cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -11452;
	// 8325C08C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C090: 4AFD0E41  bl 0x8222ced0
	ctx.lr = 0x8325C094;
	sub_8222CED0(ctx, base);
	// 8325C094: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C09C: 4AF92A9D  bl 0x821eeb38
	ctx.lr = 0x8325C0A0;
	sub_821EEB38(ctx, base);
	// 8325C0A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C0A4: 4B9A774D  bl 0x82c037f0
	ctx.lr = 0x8325C0A8;
	sub_82C037F0(ctx, base);
	// 8325C0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C0AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C0B4: 916AAC2C  stw r11, -0x53d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21460 as u32), ctx.r[11].u32 ) };
	// 8325C0B8: 4AF6A6B1  bl 0x821c6768
	ctx.lr = 0x8325C0BC;
	sub_821C6768(ctx, base);
	// 8325C0BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C0C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C0C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C0C8; continue 'dispatch;
            }
            0x8325C0C8 => {
    //   block [0x8325C0C8..0x8325C0F4)
	// 8325C0C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C0CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C0D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C0D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C0DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0E0: 4082FFE8  bne 0x8325c0c8
	if !ctx.cr[0].eq {
	pc = 0x8325C0C8; continue 'dispatch;
	}
	// 8325C0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C0E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C0EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C0F0: 4AF6A679  bl 0x821c6768
	ctx.lr = 0x8325C0F4;
	sub_821C6768(ctx, base);
	pc = 0x8325C0F4; continue 'dispatch;
            }
            0x8325C0F4 => {
    //   block [0x8325C0F4..0x8325C130)
	// 8325C0F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C0F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C0FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C100: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C104: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C108: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C10C: 4082FFE8  bne 0x8325c0f4
	if !ctx.cr[0].eq {
	pc = 0x8325C0F4; continue 'dispatch;
	}
	// 8325C110: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C114: 386BC318  addi r3, r11, -0x3ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -15592;
	// 8325C118: 4BA4DE09  bl 0x82ca9f20
	ctx.lr = 0x8325C11C;
	sub_82CA9F20(ctx, base);
	// 8325C11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C130 size=192
    let mut pc: u32 = 0x8325C130;
    'dispatch: loop {
        match pc {
            0x8325C130 => {
    //   block [0x8325C130..0x8325C188)
	// 8325C130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C13C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C140: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C148: 388BD318  addi r4, r11, -0x2ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -11496;
	// 8325C14C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C150: 4AFD0D81  bl 0x8222ced0
	ctx.lr = 0x8325C154;
	sub_8222CED0(ctx, base);
	// 8325C154: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C15C: 4AF929DD  bl 0x821eeb38
	ctx.lr = 0x8325C160;
	sub_821EEB38(ctx, base);
	// 8325C160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C164: 4B9A768D  bl 0x82c037f0
	ctx.lr = 0x8325C168;
	sub_82C037F0(ctx, base);
	// 8325C168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C16C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C174: 916AAC30  stw r11, -0x53d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21456 as u32), ctx.r[11].u32 ) };
	// 8325C178: 4AF6A5F1  bl 0x821c6768
	ctx.lr = 0x8325C17C;
	sub_821C6768(ctx, base);
	// 8325C17C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C180: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C184: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C188; continue 'dispatch;
            }
            0x8325C188 => {
    //   block [0x8325C188..0x8325C1B4)
	// 8325C188: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C18C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C190: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C194: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C198: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C19C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1A0: 4082FFE8  bne 0x8325c188
	if !ctx.cr[0].eq {
	pc = 0x8325C188; continue 'dispatch;
	}
	// 8325C1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C1A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C1AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C1B0: 4AF6A5B9  bl 0x821c6768
	ctx.lr = 0x8325C1B4;
	sub_821C6768(ctx, base);
	pc = 0x8325C1B4; continue 'dispatch;
            }
            0x8325C1B4 => {
    //   block [0x8325C1B4..0x8325C1F0)
	// 8325C1B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C1B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C1C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C1C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C1C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C1CC: 4082FFE8  bne 0x8325c1b4
	if !ctx.cr[0].eq {
	pc = 0x8325C1B4; continue 'dispatch;
	}
	// 8325C1D0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C1D4: 386BC320  addi r3, r11, -0x3ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -15584;
	// 8325C1D8: 4BA4DD49  bl 0x82ca9f20
	ctx.lr = 0x8325C1DC;
	sub_82CA9F20(ctx, base);
	// 8325C1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C1E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C1F0 size=192
    let mut pc: u32 = 0x8325C1F0;
    'dispatch: loop {
        match pc {
            0x8325C1F0 => {
    //   block [0x8325C1F0..0x8325C248)
	// 8325C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C1F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C1FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C200: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C208: 388BD370  addi r4, r11, -0x2c90
	ctx.r[4].s64 = ctx.r[11].s64 + -11408;
	// 8325C20C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C210: 4AFD0CC1  bl 0x8222ced0
	ctx.lr = 0x8325C214;
	sub_8222CED0(ctx, base);
	// 8325C214: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C21C: 4AF9291D  bl 0x821eeb38
	ctx.lr = 0x8325C220;
	sub_821EEB38(ctx, base);
	// 8325C220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C224: 4B9A75CD  bl 0x82c037f0
	ctx.lr = 0x8325C228;
	sub_82C037F0(ctx, base);
	// 8325C228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C22C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C234: 916AAC34  stw r11, -0x53cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21452 as u32), ctx.r[11].u32 ) };
	// 8325C238: 4AF6A531  bl 0x821c6768
	ctx.lr = 0x8325C23C;
	sub_821C6768(ctx, base);
	// 8325C23C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C240: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C244: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C248; continue 'dispatch;
            }
            0x8325C248 => {
    //   block [0x8325C248..0x8325C274)
	// 8325C248: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C24C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C250: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C254: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C258: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C25C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C260: 4082FFE8  bne 0x8325c248
	if !ctx.cr[0].eq {
	pc = 0x8325C248; continue 'dispatch;
	}
	// 8325C264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C268: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C26C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C270: 4AF6A4F9  bl 0x821c6768
	ctx.lr = 0x8325C274;
	sub_821C6768(ctx, base);
	pc = 0x8325C274; continue 'dispatch;
            }
            0x8325C274 => {
    //   block [0x8325C274..0x8325C2B0)
	// 8325C274: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C278: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C27C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C280: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C284: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C288: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C28C: 4082FFE8  bne 0x8325c274
	if !ctx.cr[0].eq {
	pc = 0x8325C274; continue 'dispatch;
	}
	// 8325C290: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C294: 386BC328  addi r3, r11, -0x3cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -15576;
	// 8325C298: 4BA4DC89  bl 0x82ca9f20
	ctx.lr = 0x8325C29C;
	sub_82CA9F20(ctx, base);
	// 8325C29C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C2B0 size=192
    let mut pc: u32 = 0x8325C2B0;
    'dispatch: loop {
        match pc {
            0x8325C2B0 => {
    //   block [0x8325C2B0..0x8325C308)
	// 8325C2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C2BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C2C0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C2C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C2C8: 388BD39C  addi r4, r11, -0x2c64
	ctx.r[4].s64 = ctx.r[11].s64 + -11364;
	// 8325C2CC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C2D0: 4AFD0C01  bl 0x8222ced0
	ctx.lr = 0x8325C2D4;
	sub_8222CED0(ctx, base);
	// 8325C2D4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C2D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2DC: 4AF9285D  bl 0x821eeb38
	ctx.lr = 0x8325C2E0;
	sub_821EEB38(ctx, base);
	// 8325C2E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2E4: 4B9A750D  bl 0x82c037f0
	ctx.lr = 0x8325C2E8;
	sub_82C037F0(ctx, base);
	// 8325C2E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C2EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C2F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C2F4: 916AAC38  stw r11, -0x53c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21448 as u32), ctx.r[11].u32 ) };
	// 8325C2F8: 4AF6A471  bl 0x821c6768
	ctx.lr = 0x8325C2FC;
	sub_821C6768(ctx, base);
	// 8325C2FC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C300: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C304: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C308; continue 'dispatch;
            }
            0x8325C308 => {
    //   block [0x8325C308..0x8325C334)
	// 8325C308: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C30C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C310: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C314: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C318: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C31C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C320: 4082FFE8  bne 0x8325c308
	if !ctx.cr[0].eq {
	pc = 0x8325C308; continue 'dispatch;
	}
	// 8325C324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C328: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C32C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C330: 4AF6A439  bl 0x821c6768
	ctx.lr = 0x8325C334;
	sub_821C6768(ctx, base);
	pc = 0x8325C334; continue 'dispatch;
            }
            0x8325C334 => {
    //   block [0x8325C334..0x8325C370)
	// 8325C334: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C338: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C33C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C340: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C344: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C348: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C34C: 4082FFE8  bne 0x8325c334
	if !ctx.cr[0].eq {
	pc = 0x8325C334; continue 'dispatch;
	}
	// 8325C350: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C354: 386BC330  addi r3, r11, -0x3cd0
	ctx.r[3].s64 = ctx.r[11].s64 + -15568;
	// 8325C358: 4BA4DBC9  bl 0x82ca9f20
	ctx.lr = 0x8325C35C;
	sub_82CA9F20(ctx, base);
	// 8325C35C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C370 size=192
    let mut pc: u32 = 0x8325C370;
    'dispatch: loop {
        match pc {
            0x8325C370 => {
    //   block [0x8325C370..0x8325C3C8)
	// 8325C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325C37C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C380: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C388: 388BD3C8  addi r4, r11, -0x2c38
	ctx.r[4].s64 = ctx.r[11].s64 + -11320;
	// 8325C38C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C390: 4AFD0B41  bl 0x8222ced0
	ctx.lr = 0x8325C394;
	sub_8222CED0(ctx, base);
	// 8325C394: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8325C398: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C39C: 4AF9279D  bl 0x821eeb38
	ctx.lr = 0x8325C3A0;
	sub_821EEB38(ctx, base);
	// 8325C3A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C3A4: 4B9A744D  bl 0x82c037f0
	ctx.lr = 0x8325C3A8;
	sub_82C037F0(ctx, base);
	// 8325C3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C3AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8325C3B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325C3B4: 916AAC3C  stw r11, -0x53c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21444 as u32), ctx.r[11].u32 ) };
	// 8325C3B8: 4AF6A3B1  bl 0x821c6768
	ctx.lr = 0x8325C3BC;
	sub_821C6768(ctx, base);
	// 8325C3BC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8325C3C0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8325C3C4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8325C3C8; continue 'dispatch;
            }
            0x8325C3C8 => {
    //   block [0x8325C3C8..0x8325C3F4)
	// 8325C3C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325C3CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325C3D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8325C3D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C3DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3E0: 4082FFE8  bne 0x8325c3c8
	if !ctx.cr[0].eq {
	pc = 0x8325C3C8; continue 'dispatch;
	}
	// 8325C3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8325C3E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325C3EC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8325C3F0: 4AF6A379  bl 0x821c6768
	ctx.lr = 0x8325C3F4;
	sub_821C6768(ctx, base);
	pc = 0x8325C3F4; continue 'dispatch;
            }
            0x8325C3F4 => {
    //   block [0x8325C3F4..0x8325C430)
	// 8325C3F4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8325C3F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C3FC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8325C400: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325C404: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325C408: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325C40C: 4082FFE8  bne 0x8325c3f4
	if !ctx.cr[0].eq {
	pc = 0x8325C3F4; continue 'dispatch;
	}
	// 8325C410: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325C414: 386BC338  addi r3, r11, -0x3cc8
	ctx.r[3].s64 = ctx.r[11].s64 + -15560;
	// 8325C418: 4BA4DB09  bl 0x82ca9f20
	ctx.lr = 0x8325C41C;
	sub_82CA9F20(ctx, base);
	// 8325C41C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8325C420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C430 size=64
    let mut pc: u32 = 0x8325C430;
    'dispatch: loop {
        match pc {
            0x8325C430 => {
    //   block [0x8325C430..0x8325C470)
	// 8325C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C444: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325C448: 386AAC40  addi r3, r10, -0x53c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21440;
	// 8325C44C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C450: 4AFD0A81  bl 0x8222ced0
	ctx.lr = 0x8325C454;
	sub_8222CED0(ctx, base);
	// 8325C454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C458: 3869C340  addi r3, r9, -0x3cc0
	ctx.r[3].s64 = ctx.r[9].s64 + -15552;
	// 8325C45C: 4BA4DAC5  bl 0x82ca9f20
	ctx.lr = 0x8325C460;
	sub_82CA9F20(ctx, base);
	// 8325C460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C470 size=64
    let mut pc: u32 = 0x8325C470;
    'dispatch: loop {
        match pc {
            0x8325C470 => {
    //   block [0x8325C470..0x8325C4B0)
	// 8325C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C47C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C484: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325C488: 386AAC44  addi r3, r10, -0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21436;
	// 8325C48C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C490: 4AFD0A41  bl 0x8222ced0
	ctx.lr = 0x8325C494;
	sub_8222CED0(ctx, base);
	// 8325C494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C498: 3869C350  addi r3, r9, -0x3cb0
	ctx.r[3].s64 = ctx.r[9].s64 + -15536;
	// 8325C49C: 4BA4DA85  bl 0x82ca9f20
	ctx.lr = 0x8325C4A0;
	sub_82CA9F20(ctx, base);
	// 8325C4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C4B0 size=64
    let mut pc: u32 = 0x8325C4B0;
    'dispatch: loop {
        match pc {
            0x8325C4B0 => {
    //   block [0x8325C4B0..0x8325C4F0)
	// 8325C4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C4BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C4C4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325C4C8: 386AAC48  addi r3, r10, -0x53b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21432;
	// 8325C4CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C4D0: 4AFD0A01  bl 0x8222ced0
	ctx.lr = 0x8325C4D4;
	sub_8222CED0(ctx, base);
	// 8325C4D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C4D8: 3869C360  addi r3, r9, -0x3ca0
	ctx.r[3].s64 = ctx.r[9].s64 + -15520;
	// 8325C4DC: 4BA4DA45  bl 0x82ca9f20
	ctx.lr = 0x8325C4E0;
	sub_82CA9F20(ctx, base);
	// 8325C4E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C4F0 size=56
    let mut pc: u32 = 0x8325C4F0;
    'dispatch: loop {
        match pc {
            0x8325C4F0 => {
    //   block [0x8325C4F0..0x8325C528)
	// 8325C4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C4FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C500: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C504: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325C508: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C50C: 4AF9784D  bl 0x821f3d58
	ctx.lr = 0x8325C510;
	sub_821F3D58(ctx, base);
	// 8325C510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C514: 906AAC4C  stw r3, -0x53b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21428 as u32), ctx.r[3].u32 ) };
	// 8325C518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C528 size=56
    let mut pc: u32 = 0x8325C528;
    'dispatch: loop {
        match pc {
            0x8325C528 => {
    //   block [0x8325C528..0x8325C560)
	// 8325C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C534: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C538: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C53C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325C540: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C544: 4AF97815  bl 0x821f3d58
	ctx.lr = 0x8325C548;
	sub_821F3D58(ctx, base);
	// 8325C548: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C54C: 906AAC50  stw r3, -0x53b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21424 as u32), ctx.r[3].u32 ) };
	// 8325C550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C560 size=56
    let mut pc: u32 = 0x8325C560;
    'dispatch: loop {
        match pc {
            0x8325C560 => {
    //   block [0x8325C560..0x8325C598)
	// 8325C560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C56C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C570: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C574: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325C578: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C57C: 4AF977DD  bl 0x821f3d58
	ctx.lr = 0x8325C580;
	sub_821F3D58(ctx, base);
	// 8325C580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C584: 906AAC54  stw r3, -0x53ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21420 as u32), ctx.r[3].u32 ) };
	// 8325C588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C598 size=56
    let mut pc: u32 = 0x8325C598;
    'dispatch: loop {
        match pc {
            0x8325C598 => {
    //   block [0x8325C598..0x8325C5D0)
	// 8325C598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C5A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C5A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C5AC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325C5B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C5B4: 4AF977A5  bl 0x821f3d58
	ctx.lr = 0x8325C5B8;
	sub_821F3D58(ctx, base);
	// 8325C5B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C5BC: 906AAC58  stw r3, -0x53a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21416 as u32), ctx.r[3].u32 ) };
	// 8325C5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C5D0 size=56
    let mut pc: u32 = 0x8325C5D0;
    'dispatch: loop {
        match pc {
            0x8325C5D0 => {
    //   block [0x8325C5D0..0x8325C608)
	// 8325C5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C5DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C5E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C5E4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325C5E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C5EC: 4AF9776D  bl 0x821f3d58
	ctx.lr = 0x8325C5F0;
	sub_821F3D58(ctx, base);
	// 8325C5F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C5F4: 906AAC5C  stw r3, -0x53a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21412 as u32), ctx.r[3].u32 ) };
	// 8325C5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C608 size=56
    let mut pc: u32 = 0x8325C608;
    'dispatch: loop {
        match pc {
            0x8325C608 => {
    //   block [0x8325C608..0x8325C640)
	// 8325C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C614: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C618: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C61C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325C620: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C624: 4AF97735  bl 0x821f3d58
	ctx.lr = 0x8325C628;
	sub_821F3D58(ctx, base);
	// 8325C628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C62C: 906AAC60  stw r3, -0x53a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21408 as u32), ctx.r[3].u32 ) };
	// 8325C630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C640 size=56
    let mut pc: u32 = 0x8325C640;
    'dispatch: loop {
        match pc {
            0x8325C640 => {
    //   block [0x8325C640..0x8325C678)
	// 8325C640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C64C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C650: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C654: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325C658: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C65C: 4AF976FD  bl 0x821f3d58
	ctx.lr = 0x8325C660;
	sub_821F3D58(ctx, base);
	// 8325C660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C664: 906AAC64  stw r3, -0x539c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21404 as u32), ctx.r[3].u32 ) };
	// 8325C668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C678 size=56
    let mut pc: u32 = 0x8325C678;
    'dispatch: loop {
        match pc {
            0x8325C678 => {
    //   block [0x8325C678..0x8325C6B0)
	// 8325C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C68C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325C690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C694: 4AF976C5  bl 0x821f3d58
	ctx.lr = 0x8325C698;
	sub_821F3D58(ctx, base);
	// 8325C698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C69C: 906AAC68  stw r3, -0x5398(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21400 as u32), ctx.r[3].u32 ) };
	// 8325C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C6B0 size=56
    let mut pc: u32 = 0x8325C6B0;
    'dispatch: loop {
        match pc {
            0x8325C6B0 => {
    //   block [0x8325C6B0..0x8325C6E8)
	// 8325C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C6BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C6C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C6C4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325C6C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C6CC: 4AF9768D  bl 0x821f3d58
	ctx.lr = 0x8325C6D0;
	sub_821F3D58(ctx, base);
	// 8325C6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C6D4: 906AAC6C  stw r3, -0x5394(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21396 as u32), ctx.r[3].u32 ) };
	// 8325C6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C6E8 size=56
    let mut pc: u32 = 0x8325C6E8;
    'dispatch: loop {
        match pc {
            0x8325C6E8 => {
    //   block [0x8325C6E8..0x8325C720)
	// 8325C6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C6F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C6F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C6FC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325C700: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C704: 4AF97655  bl 0x821f3d58
	ctx.lr = 0x8325C708;
	sub_821F3D58(ctx, base);
	// 8325C708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C70C: 906AAC70  stw r3, -0x5390(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21392 as u32), ctx.r[3].u32 ) };
	// 8325C710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C720 size=56
    let mut pc: u32 = 0x8325C720;
    'dispatch: loop {
        match pc {
            0x8325C720 => {
    //   block [0x8325C720..0x8325C758)
	// 8325C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C72C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C734: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325C738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C73C: 4AF9761D  bl 0x821f3d58
	ctx.lr = 0x8325C740;
	sub_821F3D58(ctx, base);
	// 8325C740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C744: 906AAC74  stw r3, -0x538c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21388 as u32), ctx.r[3].u32 ) };
	// 8325C748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C758 size=56
    let mut pc: u32 = 0x8325C758;
    'dispatch: loop {
        match pc {
            0x8325C758 => {
    //   block [0x8325C758..0x8325C790)
	// 8325C758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C76C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325C770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C774: 4AF975E5  bl 0x821f3d58
	ctx.lr = 0x8325C778;
	sub_821F3D58(ctx, base);
	// 8325C778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C77C: 906AAC78  stw r3, -0x5388(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21384 as u32), ctx.r[3].u32 ) };
	// 8325C780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C790 size=56
    let mut pc: u32 = 0x8325C790;
    'dispatch: loop {
        match pc {
            0x8325C790 => {
    //   block [0x8325C790..0x8325C7C8)
	// 8325C790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C79C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C7A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C7A4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325C7A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C7AC: 4AF975AD  bl 0x821f3d58
	ctx.lr = 0x8325C7B0;
	sub_821F3D58(ctx, base);
	// 8325C7B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C7B4: 906AAC7C  stw r3, -0x5384(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21380 as u32), ctx.r[3].u32 ) };
	// 8325C7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C7C8 size=56
    let mut pc: u32 = 0x8325C7C8;
    'dispatch: loop {
        match pc {
            0x8325C7C8 => {
    //   block [0x8325C7C8..0x8325C800)
	// 8325C7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C7D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C7DC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325C7E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C7E4: 4AF97575  bl 0x821f3d58
	ctx.lr = 0x8325C7E8;
	sub_821F3D58(ctx, base);
	// 8325C7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C7EC: 906AAC80  stw r3, -0x5380(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21376 as u32), ctx.r[3].u32 ) };
	// 8325C7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C800 size=56
    let mut pc: u32 = 0x8325C800;
    'dispatch: loop {
        match pc {
            0x8325C800 => {
    //   block [0x8325C800..0x8325C838)
	// 8325C800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C814: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325C818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C81C: 4AF9753D  bl 0x821f3d58
	ctx.lr = 0x8325C820;
	sub_821F3D58(ctx, base);
	// 8325C820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C824: 906AAC84  stw r3, -0x537c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21372 as u32), ctx.r[3].u32 ) };
	// 8325C828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C838 size=56
    let mut pc: u32 = 0x8325C838;
    'dispatch: loop {
        match pc {
            0x8325C838 => {
    //   block [0x8325C838..0x8325C870)
	// 8325C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C84C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325C850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C854: 4AF97505  bl 0x821f3d58
	ctx.lr = 0x8325C858;
	sub_821F3D58(ctx, base);
	// 8325C858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C85C: 906AAC88  stw r3, -0x5378(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21368 as u32), ctx.r[3].u32 ) };
	// 8325C860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C870 size=56
    let mut pc: u32 = 0x8325C870;
    'dispatch: loop {
        match pc {
            0x8325C870 => {
    //   block [0x8325C870..0x8325C8A8)
	// 8325C870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C87C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C884: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325C888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C88C: 4AF974CD  bl 0x821f3d58
	ctx.lr = 0x8325C890;
	sub_821F3D58(ctx, base);
	// 8325C890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C894: 906AAC8C  stw r3, -0x5374(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21364 as u32), ctx.r[3].u32 ) };
	// 8325C898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C8A8 size=56
    let mut pc: u32 = 0x8325C8A8;
    'dispatch: loop {
        match pc {
            0x8325C8A8 => {
    //   block [0x8325C8A8..0x8325C8E0)
	// 8325C8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C8B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C8BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325C8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C8C4: 4AF97495  bl 0x821f3d58
	ctx.lr = 0x8325C8C8;
	sub_821F3D58(ctx, base);
	// 8325C8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C8CC: 906AAC90  stw r3, -0x5370(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21360 as u32), ctx.r[3].u32 ) };
	// 8325C8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C8E0 size=56
    let mut pc: u32 = 0x8325C8E0;
    'dispatch: loop {
        match pc {
            0x8325C8E0 => {
    //   block [0x8325C8E0..0x8325C918)
	// 8325C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C8F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325C8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C8FC: 4AF9745D  bl 0x821f3d58
	ctx.lr = 0x8325C900;
	sub_821F3D58(ctx, base);
	// 8325C900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C904: 906AAC94  stw r3, -0x536c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21356 as u32), ctx.r[3].u32 ) };
	// 8325C908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C918 size=56
    let mut pc: u32 = 0x8325C918;
    'dispatch: loop {
        match pc {
            0x8325C918 => {
    //   block [0x8325C918..0x8325C950)
	// 8325C918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C92C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325C930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C934: 4AF97425  bl 0x821f3d58
	ctx.lr = 0x8325C938;
	sub_821F3D58(ctx, base);
	// 8325C938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C93C: 906AAC98  stw r3, -0x5368(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21352 as u32), ctx.r[3].u32 ) };
	// 8325C940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C950 size=56
    let mut pc: u32 = 0x8325C950;
    'dispatch: loop {
        match pc {
            0x8325C950 => {
    //   block [0x8325C950..0x8325C988)
	// 8325C950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C95C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325C964: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325C968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325C96C: 4AF973ED  bl 0x821f3d58
	ctx.lr = 0x8325C970;
	sub_821F3D58(ctx, base);
	// 8325C970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C974: 906AAC9C  stw r3, -0x5364(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21348 as u32), ctx.r[3].u32 ) };
	// 8325C978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C988 size=64
    let mut pc: u32 = 0x8325C988;
    'dispatch: loop {
        match pc {
            0x8325C988 => {
    //   block [0x8325C988..0x8325C9C8)
	// 8325C988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325C998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C99C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325C9A0: 386AACA0  addi r3, r10, -0x5360
	ctx.r[3].s64 = ctx.r[10].s64 + -21344;
	// 8325C9A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C9A8: 4AFD0529  bl 0x8222ced0
	ctx.lr = 0x8325C9AC;
	sub_8222CED0(ctx, base);
	// 8325C9AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C9B0: 3869C370  addi r3, r9, -0x3c90
	ctx.r[3].s64 = ctx.r[9].s64 + -15504;
	// 8325C9B4: 4BA4D56D  bl 0x82ca9f20
	ctx.lr = 0x8325C9B8;
	sub_82CA9F20(ctx, base);
	// 8325C9B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325C9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325C9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325C9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325C9C8 size=64
    let mut pc: u32 = 0x8325C9C8;
    'dispatch: loop {
        match pc {
            0x8325C9C8 => {
    //   block [0x8325C9C8..0x8325CA08)
	// 8325C9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325C9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325C9D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325C9D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325C9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325C9DC: 388BDE44  addi r4, r11, -0x21bc
	ctx.r[4].s64 = ctx.r[11].s64 + -8636;
	// 8325C9E0: 386AACA4  addi r3, r10, -0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + -21340;
	// 8325C9E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325C9E8: 4AFD04E9  bl 0x8222ced0
	ctx.lr = 0x8325C9EC;
	sub_8222CED0(ctx, base);
	// 8325C9EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325C9F0: 3869C380  addi r3, r9, -0x3c80
	ctx.r[3].s64 = ctx.r[9].s64 + -15488;
	// 8325C9F4: 4BA4D52D  bl 0x82ca9f20
	ctx.lr = 0x8325C9F8;
	sub_82CA9F20(ctx, base);
	// 8325C9F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325C9FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA08 size=64
    let mut pc: u32 = 0x8325CA08;
    'dispatch: loop {
        match pc {
            0x8325CA08 => {
    //   block [0x8325CA08..0x8325CA48)
	// 8325CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA14: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CA18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CA1C: 388BDE70  addi r4, r11, -0x2190
	ctx.r[4].s64 = ctx.r[11].s64 + -8592;
	// 8325CA20: 386AACA8  addi r3, r10, -0x5358
	ctx.r[3].s64 = ctx.r[10].s64 + -21336;
	// 8325CA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CA28: 4AFD04A9  bl 0x8222ced0
	ctx.lr = 0x8325CA2C;
	sub_8222CED0(ctx, base);
	// 8325CA2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CA30: 3869C390  addi r3, r9, -0x3c70
	ctx.r[3].s64 = ctx.r[9].s64 + -15472;
	// 8325CA34: 4BA4D4ED  bl 0x82ca9f20
	ctx.lr = 0x8325CA38;
	sub_82CA9F20(ctx, base);
	// 8325CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA48 size=56
    let mut pc: u32 = 0x8325CA48;
    'dispatch: loop {
        match pc {
            0x8325CA48 => {
    //   block [0x8325CA48..0x8325CA80)
	// 8325CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CA5C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CA64: 4AF972F5  bl 0x821f3d58
	ctx.lr = 0x8325CA68;
	sub_821F3D58(ctx, base);
	// 8325CA68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CA6C: 906AACAC  stw r3, -0x5354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21332 as u32), ctx.r[3].u32 ) };
	// 8325CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CA80 size=56
    let mut pc: u32 = 0x8325CA80;
    'dispatch: loop {
        match pc {
            0x8325CA80 => {
    //   block [0x8325CA80..0x8325CAB8)
	// 8325CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CA94: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CA9C: 4AF972BD  bl 0x821f3d58
	ctx.lr = 0x8325CAA0;
	sub_821F3D58(ctx, base);
	// 8325CAA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CAA4: 906AACB0  stw r3, -0x5350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21328 as u32), ctx.r[3].u32 ) };
	// 8325CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CAB8 size=56
    let mut pc: u32 = 0x8325CAB8;
    'dispatch: loop {
        match pc {
            0x8325CAB8 => {
    //   block [0x8325CAB8..0x8325CAF0)
	// 8325CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CACC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CAD4: 4AF97285  bl 0x821f3d58
	ctx.lr = 0x8325CAD8;
	sub_821F3D58(ctx, base);
	// 8325CAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CADC: 906AACB4  stw r3, -0x534c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21324 as u32), ctx.r[3].u32 ) };
	// 8325CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CAF0 size=56
    let mut pc: u32 = 0x8325CAF0;
    'dispatch: loop {
        match pc {
            0x8325CAF0 => {
    //   block [0x8325CAF0..0x8325CB28)
	// 8325CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB04: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB0C: 4AF9724D  bl 0x821f3d58
	ctx.lr = 0x8325CB10;
	sub_821F3D58(ctx, base);
	// 8325CB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB14: 906AACB8  stw r3, -0x5348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21320 as u32), ctx.r[3].u32 ) };
	// 8325CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB28 size=56
    let mut pc: u32 = 0x8325CB28;
    'dispatch: loop {
        match pc {
            0x8325CB28 => {
    //   block [0x8325CB28..0x8325CB60)
	// 8325CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB3C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB44: 4AF97215  bl 0x821f3d58
	ctx.lr = 0x8325CB48;
	sub_821F3D58(ctx, base);
	// 8325CB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB4C: 906AACBC  stw r3, -0x5344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21316 as u32), ctx.r[3].u32 ) };
	// 8325CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB60 size=56
    let mut pc: u32 = 0x8325CB60;
    'dispatch: loop {
        match pc {
            0x8325CB60 => {
    //   block [0x8325CB60..0x8325CB98)
	// 8325CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CB74: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CB7C: 4AF971DD  bl 0x821f3d58
	ctx.lr = 0x8325CB80;
	sub_821F3D58(ctx, base);
	// 8325CB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CB84: 906AACC0  stw r3, -0x5340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21312 as u32), ctx.r[3].u32 ) };
	// 8325CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CB98 size=56
    let mut pc: u32 = 0x8325CB98;
    'dispatch: loop {
        match pc {
            0x8325CB98 => {
    //   block [0x8325CB98..0x8325CBD0)
	// 8325CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CBAC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325CBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CBB4: 4AF971A5  bl 0x821f3d58
	ctx.lr = 0x8325CBB8;
	sub_821F3D58(ctx, base);
	// 8325CBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CBBC: 906AACC4  stw r3, -0x533c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21308 as u32), ctx.r[3].u32 ) };
	// 8325CBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CBD0 size=56
    let mut pc: u32 = 0x8325CBD0;
    'dispatch: loop {
        match pc {
            0x8325CBD0 => {
    //   block [0x8325CBD0..0x8325CC08)
	// 8325CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CBE4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325CBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CBEC: 4AF9716D  bl 0x821f3d58
	ctx.lr = 0x8325CBF0;
	sub_821F3D58(ctx, base);
	// 8325CBF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CBF4: 906AACC8  stw r3, -0x5338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21304 as u32), ctx.r[3].u32 ) };
	// 8325CBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC08 size=56
    let mut pc: u32 = 0x8325CC08;
    'dispatch: loop {
        match pc {
            0x8325CC08 => {
    //   block [0x8325CC08..0x8325CC40)
	// 8325CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC1C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325CC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC24: 4AF97135  bl 0x821f3d58
	ctx.lr = 0x8325CC28;
	sub_821F3D58(ctx, base);
	// 8325CC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC2C: 906AACCC  stw r3, -0x5334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21300 as u32), ctx.r[3].u32 ) };
	// 8325CC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC40 size=56
    let mut pc: u32 = 0x8325CC40;
    'dispatch: loop {
        match pc {
            0x8325CC40 => {
    //   block [0x8325CC40..0x8325CC78)
	// 8325CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC54: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325CC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC5C: 4AF970FD  bl 0x821f3d58
	ctx.lr = 0x8325CC60;
	sub_821F3D58(ctx, base);
	// 8325CC60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC64: 906AACD0  stw r3, -0x5330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21296 as u32), ctx.r[3].u32 ) };
	// 8325CC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CC78 size=56
    let mut pc: u32 = 0x8325CC78;
    'dispatch: loop {
        match pc {
            0x8325CC78 => {
    //   block [0x8325CC78..0x8325CCB0)
	// 8325CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CC8C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325CC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CC94: 4AF970C5  bl 0x821f3d58
	ctx.lr = 0x8325CC98;
	sub_821F3D58(ctx, base);
	// 8325CC98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CC9C: 906AACD4  stw r3, -0x532c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21292 as u32), ctx.r[3].u32 ) };
	// 8325CCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CCB0 size=56
    let mut pc: u32 = 0x8325CCB0;
    'dispatch: loop {
        match pc {
            0x8325CCB0 => {
    //   block [0x8325CCB0..0x8325CCE8)
	// 8325CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CCC4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325CCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CCCC: 4AF9708D  bl 0x821f3d58
	ctx.lr = 0x8325CCD0;
	sub_821F3D58(ctx, base);
	// 8325CCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CCD4: 906AACD8  stw r3, -0x5328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21288 as u32), ctx.r[3].u32 ) };
	// 8325CCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CCE8 size=56
    let mut pc: u32 = 0x8325CCE8;
    'dispatch: loop {
        match pc {
            0x8325CCE8 => {
    //   block [0x8325CCE8..0x8325CD20)
	// 8325CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CCFC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325CD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD04: 4AF97055  bl 0x821f3d58
	ctx.lr = 0x8325CD08;
	sub_821F3D58(ctx, base);
	// 8325CD08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD0C: 906AACDC  stw r3, -0x5324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21284 as u32), ctx.r[3].u32 ) };
	// 8325CD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD20 size=56
    let mut pc: u32 = 0x8325CD20;
    'dispatch: loop {
        match pc {
            0x8325CD20 => {
    //   block [0x8325CD20..0x8325CD58)
	// 8325CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CD34: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325CD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD3C: 4AF9701D  bl 0x821f3d58
	ctx.lr = 0x8325CD40;
	sub_821F3D58(ctx, base);
	// 8325CD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD44: 906AACE0  stw r3, -0x5320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21280 as u32), ctx.r[3].u32 ) };
	// 8325CD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD58 size=56
    let mut pc: u32 = 0x8325CD58;
    'dispatch: loop {
        match pc {
            0x8325CD58 => {
    //   block [0x8325CD58..0x8325CD90)
	// 8325CD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CD6C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325CD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CD74: 4AF96FE5  bl 0x821f3d58
	ctx.lr = 0x8325CD78;
	sub_821F3D58(ctx, base);
	// 8325CD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CD7C: 906AACE4  stw r3, -0x531c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21276 as u32), ctx.r[3].u32 ) };
	// 8325CD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CD90 size=56
    let mut pc: u32 = 0x8325CD90;
    'dispatch: loop {
        match pc {
            0x8325CD90 => {
    //   block [0x8325CD90..0x8325CDC8)
	// 8325CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CDA4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325CDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CDAC: 4AF96FAD  bl 0x821f3d58
	ctx.lr = 0x8325CDB0;
	sub_821F3D58(ctx, base);
	// 8325CDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CDB4: 906AACE8  stw r3, -0x5318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21272 as u32), ctx.r[3].u32 ) };
	// 8325CDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CDC8 size=56
    let mut pc: u32 = 0x8325CDC8;
    'dispatch: loop {
        match pc {
            0x8325CDC8 => {
    //   block [0x8325CDC8..0x8325CE00)
	// 8325CDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CDDC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325CDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CDE4: 4AF96F75  bl 0x821f3d58
	ctx.lr = 0x8325CDE8;
	sub_821F3D58(ctx, base);
	// 8325CDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CDEC: 906AACEC  stw r3, -0x5314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21268 as u32), ctx.r[3].u32 ) };
	// 8325CDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE00 size=56
    let mut pc: u32 = 0x8325CE00;
    'dispatch: loop {
        match pc {
            0x8325CE00 => {
    //   block [0x8325CE00..0x8325CE38)
	// 8325CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE14: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325CE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE1C: 4AF96F3D  bl 0x821f3d58
	ctx.lr = 0x8325CE20;
	sub_821F3D58(ctx, base);
	// 8325CE20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE24: 906AACF0  stw r3, -0x5310(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21264 as u32), ctx.r[3].u32 ) };
	// 8325CE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE38 size=56
    let mut pc: u32 = 0x8325CE38;
    'dispatch: loop {
        match pc {
            0x8325CE38 => {
    //   block [0x8325CE38..0x8325CE70)
	// 8325CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE4C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325CE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE54: 4AF96F05  bl 0x821f3d58
	ctx.lr = 0x8325CE58;
	sub_821F3D58(ctx, base);
	// 8325CE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE5C: 906AACF4  stw r3, -0x530c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21260 as u32), ctx.r[3].u32 ) };
	// 8325CE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CE70 size=56
    let mut pc: u32 = 0x8325CE70;
    'dispatch: loop {
        match pc {
            0x8325CE70 => {
    //   block [0x8325CE70..0x8325CEA8)
	// 8325CE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CE84: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325CE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CE8C: 4AF96ECD  bl 0x821f3d58
	ctx.lr = 0x8325CE90;
	sub_821F3D58(ctx, base);
	// 8325CE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CE94: 906AACF8  stw r3, -0x5308(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21256 as u32), ctx.r[3].u32 ) };
	// 8325CE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CEA8 size=56
    let mut pc: u32 = 0x8325CEA8;
    'dispatch: loop {
        match pc {
            0x8325CEA8 => {
    //   block [0x8325CEA8..0x8325CEE0)
	// 8325CEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CEB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325CEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325CEBC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325CEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325CEC4: 4AF96E95  bl 0x821f3d58
	ctx.lr = 0x8325CEC8;
	sub_821F3D58(ctx, base);
	// 8325CEC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CECC: 906AACFC  stw r3, -0x5304(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21252 as u32), ctx.r[3].u32 ) };
	// 8325CED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CEE0 size=64
    let mut pc: u32 = 0x8325CEE0;
    'dispatch: loop {
        match pc {
            0x8325CEE0 => {
    //   block [0x8325CEE0..0x8325CF20)
	// 8325CEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CEEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CEF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CEF4: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 8325CEF8: 386AAD00  addi r3, r10, -0x5300
	ctx.r[3].s64 = ctx.r[10].s64 + -21248;
	// 8325CEFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF00: 4AFCFFD1  bl 0x8222ced0
	ctx.lr = 0x8325CF04;
	sub_8222CED0(ctx, base);
	// 8325CF04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF08: 3869C3A0  addi r3, r9, -0x3c60
	ctx.r[3].s64 = ctx.r[9].s64 + -15456;
	// 8325CF0C: 4BA4D015  bl 0x82ca9f20
	ctx.lr = 0x8325CF10;
	sub_82CA9F20(ctx, base);
	// 8325CF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CF20 size=64
    let mut pc: u32 = 0x8325CF20;
    'dispatch: loop {
        match pc {
            0x8325CF20 => {
    //   block [0x8325CF20..0x8325CF60)
	// 8325CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CF2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CF30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CF34: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325CF38: 386AAD04  addi r3, r10, -0x52fc
	ctx.r[3].s64 = ctx.r[10].s64 + -21244;
	// 8325CF3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF40: 4AFCFF91  bl 0x8222ced0
	ctx.lr = 0x8325CF44;
	sub_8222CED0(ctx, base);
	// 8325CF44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF48: 3869C3B0  addi r3, r9, -0x3c50
	ctx.r[3].s64 = ctx.r[9].s64 + -15440;
	// 8325CF4C: 4BA4CFD5  bl 0x82ca9f20
	ctx.lr = 0x8325CF50;
	sub_82CA9F20(ctx, base);
	// 8325CF50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CF60 size=64
    let mut pc: u32 = 0x8325CF60;
    'dispatch: loop {
        match pc {
            0x8325CF60 => {
    //   block [0x8325CF60..0x8325CFA0)
	// 8325CF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CF68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CF6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CF70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CF74: 388BDEE0  addi r4, r11, -0x2120
	ctx.r[4].s64 = ctx.r[11].s64 + -8480;
	// 8325CF78: 386AAD08  addi r3, r10, -0x52f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21240;
	// 8325CF7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CF80: 4AFCFF51  bl 0x8222ced0
	ctx.lr = 0x8325CF84;
	sub_8222CED0(ctx, base);
	// 8325CF84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CF88: 3869C3C0  addi r3, r9, -0x3c40
	ctx.r[3].s64 = ctx.r[9].s64 + -15424;
	// 8325CF8C: 4BA4CF95  bl 0x82ca9f20
	ctx.lr = 0x8325CF90;
	sub_82CA9F20(ctx, base);
	// 8325CF90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CFA0 size=64
    let mut pc: u32 = 0x8325CFA0;
    'dispatch: loop {
        match pc {
            0x8325CFA0 => {
    //   block [0x8325CFA0..0x8325CFE0)
	// 8325CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CFA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CFAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CFB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CFB4: 388BDF1C  addi r4, r11, -0x20e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8420;
	// 8325CFB8: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 8325CFBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325CFC0: 4AFCFF11  bl 0x8222ced0
	ctx.lr = 0x8325CFC4;
	sub_8222CED0(ctx, base);
	// 8325CFC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325CFC8: 3869C3D0  addi r3, r9, -0x3c30
	ctx.r[3].s64 = ctx.r[9].s64 + -15408;
	// 8325CFCC: 4BA4CF55  bl 0x82ca9f20
	ctx.lr = 0x8325CFD0;
	sub_82CA9F20(ctx, base);
	// 8325CFD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325CFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325CFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325CFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325CFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325CFE0 size=64
    let mut pc: u32 = 0x8325CFE0;
    'dispatch: loop {
        match pc {
            0x8325CFE0 => {
    //   block [0x8325CFE0..0x8325D020)
	// 8325CFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325CFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325CFE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325CFEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325CFF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325CFF4: 388BDF50  addi r4, r11, -0x20b0
	ctx.r[4].s64 = ctx.r[11].s64 + -8368;
	// 8325CFF8: 386AAD10  addi r3, r10, -0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21232;
	// 8325CFFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D000: 4AFCFED1  bl 0x8222ced0
	ctx.lr = 0x8325D004;
	sub_8222CED0(ctx, base);
	// 8325D004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D008: 3869C3E0  addi r3, r9, -0x3c20
	ctx.r[3].s64 = ctx.r[9].s64 + -15392;
	// 8325D00C: 4BA4CF15  bl 0x82ca9f20
	ctx.lr = 0x8325D010;
	sub_82CA9F20(ctx, base);
	// 8325D010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D020 size=64
    let mut pc: u32 = 0x8325D020;
    'dispatch: loop {
        match pc {
            0x8325D020 => {
    //   block [0x8325D020..0x8325D060)
	// 8325D020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D02C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D034: 388BDF8C  addi r4, r11, -0x2074
	ctx.r[4].s64 = ctx.r[11].s64 + -8308;
	// 8325D038: 386AAD14  addi r3, r10, -0x52ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21228;
	// 8325D03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D040: 4AFCFE91  bl 0x8222ced0
	ctx.lr = 0x8325D044;
	sub_8222CED0(ctx, base);
	// 8325D044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D048: 3869C3F0  addi r3, r9, -0x3c10
	ctx.r[3].s64 = ctx.r[9].s64 + -15376;
	// 8325D04C: 4BA4CED5  bl 0x82ca9f20
	ctx.lr = 0x8325D050;
	sub_82CA9F20(ctx, base);
	// 8325D050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D060 size=64
    let mut pc: u32 = 0x8325D060;
    'dispatch: loop {
        match pc {
            0x8325D060 => {
    //   block [0x8325D060..0x8325D0A0)
	// 8325D060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D06C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D074: 388BDFC0  addi r4, r11, -0x2040
	ctx.r[4].s64 = ctx.r[11].s64 + -8256;
	// 8325D078: 386AAD18  addi r3, r10, -0x52e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21224;
	// 8325D07C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D080: 4AFCFE51  bl 0x8222ced0
	ctx.lr = 0x8325D084;
	sub_8222CED0(ctx, base);
	// 8325D084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D088: 3869C400  addi r3, r9, -0x3c00
	ctx.r[3].s64 = ctx.r[9].s64 + -15360;
	// 8325D08C: 4BA4CE95  bl 0x82ca9f20
	ctx.lr = 0x8325D090;
	sub_82CA9F20(ctx, base);
	// 8325D090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D0A0 size=64
    let mut pc: u32 = 0x8325D0A0;
    'dispatch: loop {
        match pc {
            0x8325D0A0 => {
    //   block [0x8325D0A0..0x8325D0E0)
	// 8325D0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D0AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D0B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D0B4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325D0B8: 386AAD1C  addi r3, r10, -0x52e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21220;
	// 8325D0BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D0C0: 4AFCFE11  bl 0x8222ced0
	ctx.lr = 0x8325D0C4;
	sub_8222CED0(ctx, base);
	// 8325D0C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D0C8: 3869C410  addi r3, r9, -0x3bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -15344;
	// 8325D0CC: 4BA4CE55  bl 0x82ca9f20
	ctx.lr = 0x8325D0D0;
	sub_82CA9F20(ctx, base);
	// 8325D0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D0E0 size=64
    let mut pc: u32 = 0x8325D0E0;
    'dispatch: loop {
        match pc {
            0x8325D0E0 => {
    //   block [0x8325D0E0..0x8325D120)
	// 8325D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D0EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325D0F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D0F4: 388BE9D4  addi r4, r11, -0x162c
	ctx.r[4].s64 = ctx.r[11].s64 + -5676;
	// 8325D0F8: 386AAD20  addi r3, r10, -0x52e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21216;
	// 8325D0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D100: 4AFCFDD1  bl 0x8222ced0
	ctx.lr = 0x8325D104;
	sub_8222CED0(ctx, base);
	// 8325D104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D108: 3869C420  addi r3, r9, -0x3be0
	ctx.r[3].s64 = ctx.r[9].s64 + -15328;
	// 8325D10C: 4BA4CE15  bl 0x82ca9f20
	ctx.lr = 0x8325D110;
	sub_82CA9F20(ctx, base);
	// 8325D110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D120 size=64
    let mut pc: u32 = 0x8325D120;
    'dispatch: loop {
        match pc {
            0x8325D120 => {
    //   block [0x8325D120..0x8325D160)
	// 8325D120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D12C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D134: 388BE094  addi r4, r11, -0x1f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -8044;
	// 8325D138: 386AAD24  addi r3, r10, -0x52dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21212;
	// 8325D13C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D140: 4AFCFD91  bl 0x8222ced0
	ctx.lr = 0x8325D144;
	sub_8222CED0(ctx, base);
	// 8325D144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D148: 3869C430  addi r3, r9, -0x3bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -15312;
	// 8325D14C: 4BA4CDD5  bl 0x82ca9f20
	ctx.lr = 0x8325D150;
	sub_82CA9F20(ctx, base);
	// 8325D150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D160 size=64
    let mut pc: u32 = 0x8325D160;
    'dispatch: loop {
        match pc {
            0x8325D160 => {
    //   block [0x8325D160..0x8325D1A0)
	// 8325D160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D16C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D174: 388BE0AC  addi r4, r11, -0x1f54
	ctx.r[4].s64 = ctx.r[11].s64 + -8020;
	// 8325D178: 386AAD28  addi r3, r10, -0x52d8
	ctx.r[3].s64 = ctx.r[10].s64 + -21208;
	// 8325D17C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D180: 4AFCFD51  bl 0x8222ced0
	ctx.lr = 0x8325D184;
	sub_8222CED0(ctx, base);
	// 8325D184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D188: 3869C440  addi r3, r9, -0x3bc0
	ctx.r[3].s64 = ctx.r[9].s64 + -15296;
	// 8325D18C: 4BA4CD95  bl 0x82ca9f20
	ctx.lr = 0x8325D190;
	sub_82CA9F20(ctx, base);
	// 8325D190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D1A0 size=64
    let mut pc: u32 = 0x8325D1A0;
    'dispatch: loop {
        match pc {
            0x8325D1A0 => {
    //   block [0x8325D1A0..0x8325D1E0)
	// 8325D1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D1AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D1B4: 388BE0CC  addi r4, r11, -0x1f34
	ctx.r[4].s64 = ctx.r[11].s64 + -7988;
	// 8325D1B8: 386AAD2C  addi r3, r10, -0x52d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21204;
	// 8325D1BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D1C0: 4AFCFD11  bl 0x8222ced0
	ctx.lr = 0x8325D1C4;
	sub_8222CED0(ctx, base);
	// 8325D1C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D1C8: 3869C450  addi r3, r9, -0x3bb0
	ctx.r[3].s64 = ctx.r[9].s64 + -15280;
	// 8325D1CC: 4BA4CD55  bl 0x82ca9f20
	ctx.lr = 0x8325D1D0;
	sub_82CA9F20(ctx, base);
	// 8325D1D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D1E0 size=64
    let mut pc: u32 = 0x8325D1E0;
    'dispatch: loop {
        match pc {
            0x8325D1E0 => {
    //   block [0x8325D1E0..0x8325D220)
	// 8325D1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D1E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D1EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D1F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D1F4: 388BE0F0  addi r4, r11, -0x1f10
	ctx.r[4].s64 = ctx.r[11].s64 + -7952;
	// 8325D1F8: 386AAD30  addi r3, r10, -0x52d0
	ctx.r[3].s64 = ctx.r[10].s64 + -21200;
	// 8325D1FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D200: 4AFCFCD1  bl 0x8222ced0
	ctx.lr = 0x8325D204;
	sub_8222CED0(ctx, base);
	// 8325D204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D208: 3869C460  addi r3, r9, -0x3ba0
	ctx.r[3].s64 = ctx.r[9].s64 + -15264;
	// 8325D20C: 4BA4CD15  bl 0x82ca9f20
	ctx.lr = 0x8325D210;
	sub_82CA9F20(ctx, base);
	// 8325D210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D220 size=64
    let mut pc: u32 = 0x8325D220;
    'dispatch: loop {
        match pc {
            0x8325D220 => {
    //   block [0x8325D220..0x8325D260)
	// 8325D220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D22C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D234: 388BE0FC  addi r4, r11, -0x1f04
	ctx.r[4].s64 = ctx.r[11].s64 + -7940;
	// 8325D238: 386AAD34  addi r3, r10, -0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + -21196;
	// 8325D23C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D240: 4AFCFC91  bl 0x8222ced0
	ctx.lr = 0x8325D244;
	sub_8222CED0(ctx, base);
	// 8325D244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D248: 3869C470  addi r3, r9, -0x3b90
	ctx.r[3].s64 = ctx.r[9].s64 + -15248;
	// 8325D24C: 4BA4CCD5  bl 0x82ca9f20
	ctx.lr = 0x8325D250;
	sub_82CA9F20(ctx, base);
	// 8325D250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D260 size=64
    let mut pc: u32 = 0x8325D260;
    'dispatch: loop {
        match pc {
            0x8325D260 => {
    //   block [0x8325D260..0x8325D2A0)
	// 8325D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D26C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D274: 388BE108  addi r4, r11, -0x1ef8
	ctx.r[4].s64 = ctx.r[11].s64 + -7928;
	// 8325D278: 386AAD38  addi r3, r10, -0x52c8
	ctx.r[3].s64 = ctx.r[10].s64 + -21192;
	// 8325D27C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D280: 4AFCFC51  bl 0x8222ced0
	ctx.lr = 0x8325D284;
	sub_8222CED0(ctx, base);
	// 8325D284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D288: 3869C480  addi r3, r9, -0x3b80
	ctx.r[3].s64 = ctx.r[9].s64 + -15232;
	// 8325D28C: 4BA4CC95  bl 0x82ca9f20
	ctx.lr = 0x8325D290;
	sub_82CA9F20(ctx, base);
	// 8325D290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D2A0 size=64
    let mut pc: u32 = 0x8325D2A0;
    'dispatch: loop {
        match pc {
            0x8325D2A0 => {
    //   block [0x8325D2A0..0x8325D2E0)
	// 8325D2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D2A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D2AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D2B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D2B4: 388BE110  addi r4, r11, -0x1ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -7920;
	// 8325D2B8: 386AAD3C  addi r3, r10, -0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21188;
	// 8325D2BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D2C0: 4AFCFC11  bl 0x8222ced0
	ctx.lr = 0x8325D2C4;
	sub_8222CED0(ctx, base);
	// 8325D2C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D2C8: 3869C490  addi r3, r9, -0x3b70
	ctx.r[3].s64 = ctx.r[9].s64 + -15216;
	// 8325D2CC: 4BA4CC55  bl 0x82ca9f20
	ctx.lr = 0x8325D2D0;
	sub_82CA9F20(ctx, base);
	// 8325D2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D2E0 size=64
    let mut pc: u32 = 0x8325D2E0;
    'dispatch: loop {
        match pc {
            0x8325D2E0 => {
    //   block [0x8325D2E0..0x8325D320)
	// 8325D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D2EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D2F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D2F4: 388BE110  addi r4, r11, -0x1ef0
	ctx.r[4].s64 = ctx.r[11].s64 + -7920;
	// 8325D2F8: 386AAD40  addi r3, r10, -0x52c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21184;
	// 8325D2FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D300: 4AFCFBD1  bl 0x8222ced0
	ctx.lr = 0x8325D304;
	sub_8222CED0(ctx, base);
	// 8325D304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D308: 3869C4A0  addi r3, r9, -0x3b60
	ctx.r[3].s64 = ctx.r[9].s64 + -15200;
	// 8325D30C: 4BA4CC15  bl 0x82ca9f20
	ctx.lr = 0x8325D310;
	sub_82CA9F20(ctx, base);
	// 8325D310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D320 size=64
    let mut pc: u32 = 0x8325D320;
    'dispatch: loop {
        match pc {
            0x8325D320 => {
    //   block [0x8325D320..0x8325D360)
	// 8325D320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D32C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D334: 388BE130  addi r4, r11, -0x1ed0
	ctx.r[4].s64 = ctx.r[11].s64 + -7888;
	// 8325D338: 386AAD44  addi r3, r10, -0x52bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21180;
	// 8325D33C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D340: 4AFCFB91  bl 0x8222ced0
	ctx.lr = 0x8325D344;
	sub_8222CED0(ctx, base);
	// 8325D344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D348: 3869C4B0  addi r3, r9, -0x3b50
	ctx.r[3].s64 = ctx.r[9].s64 + -15184;
	// 8325D34C: 4BA4CBD5  bl 0x82ca9f20
	ctx.lr = 0x8325D350;
	sub_82CA9F20(ctx, base);
	// 8325D350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D360 size=64
    let mut pc: u32 = 0x8325D360;
    'dispatch: loop {
        match pc {
            0x8325D360 => {
    //   block [0x8325D360..0x8325D3A0)
	// 8325D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D36C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D374: 388BE150  addi r4, r11, -0x1eb0
	ctx.r[4].s64 = ctx.r[11].s64 + -7856;
	// 8325D378: 386AAD48  addi r3, r10, -0x52b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21176;
	// 8325D37C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D380: 4AFCFB51  bl 0x8222ced0
	ctx.lr = 0x8325D384;
	sub_8222CED0(ctx, base);
	// 8325D384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D388: 3869C4C0  addi r3, r9, -0x3b40
	ctx.r[3].s64 = ctx.r[9].s64 + -15168;
	// 8325D38C: 4BA4CB95  bl 0x82ca9f20
	ctx.lr = 0x8325D390;
	sub_82CA9F20(ctx, base);
	// 8325D390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D3A0 size=64
    let mut pc: u32 = 0x8325D3A0;
    'dispatch: loop {
        match pc {
            0x8325D3A0 => {
    //   block [0x8325D3A0..0x8325D3E0)
	// 8325D3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D3A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D3AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D3B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D3B4: 388BE174  addi r4, r11, -0x1e8c
	ctx.r[4].s64 = ctx.r[11].s64 + -7820;
	// 8325D3B8: 386AAD4C  addi r3, r10, -0x52b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21172;
	// 8325D3BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D3C0: 4AFCFB11  bl 0x8222ced0
	ctx.lr = 0x8325D3C4;
	sub_8222CED0(ctx, base);
	// 8325D3C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D3C8: 3869C4D0  addi r3, r9, -0x3b30
	ctx.r[3].s64 = ctx.r[9].s64 + -15152;
	// 8325D3CC: 4BA4CB55  bl 0x82ca9f20
	ctx.lr = 0x8325D3D0;
	sub_82CA9F20(ctx, base);
	// 8325D3D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D3E0 size=64
    let mut pc: u32 = 0x8325D3E0;
    'dispatch: loop {
        match pc {
            0x8325D3E0 => {
    //   block [0x8325D3E0..0x8325D420)
	// 8325D3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D3E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D3EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D3F4: 388BE198  addi r4, r11, -0x1e68
	ctx.r[4].s64 = ctx.r[11].s64 + -7784;
	// 8325D3F8: 386AAD50  addi r3, r10, -0x52b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21168;
	// 8325D3FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D400: 4AFCFAD1  bl 0x8222ced0
	ctx.lr = 0x8325D404;
	sub_8222CED0(ctx, base);
	// 8325D404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D408: 3869C4E0  addi r3, r9, -0x3b20
	ctx.r[3].s64 = ctx.r[9].s64 + -15136;
	// 8325D40C: 4BA4CB15  bl 0x82ca9f20
	ctx.lr = 0x8325D410;
	sub_82CA9F20(ctx, base);
	// 8325D410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D420 size=64
    let mut pc: u32 = 0x8325D420;
    'dispatch: loop {
        match pc {
            0x8325D420 => {
    //   block [0x8325D420..0x8325D460)
	// 8325D420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D42C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D434: 388BE1BC  addi r4, r11, -0x1e44
	ctx.r[4].s64 = ctx.r[11].s64 + -7748;
	// 8325D438: 386AAD54  addi r3, r10, -0x52ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21164;
	// 8325D43C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D440: 4AFCFA91  bl 0x8222ced0
	ctx.lr = 0x8325D444;
	sub_8222CED0(ctx, base);
	// 8325D444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D448: 3869C4F0  addi r3, r9, -0x3b10
	ctx.r[3].s64 = ctx.r[9].s64 + -15120;
	// 8325D44C: 4BA4CAD5  bl 0x82ca9f20
	ctx.lr = 0x8325D450;
	sub_82CA9F20(ctx, base);
	// 8325D450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D460 size=64
    let mut pc: u32 = 0x8325D460;
    'dispatch: loop {
        match pc {
            0x8325D460 => {
    //   block [0x8325D460..0x8325D4A0)
	// 8325D460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D46C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D474: 388BE1C8  addi r4, r11, -0x1e38
	ctx.r[4].s64 = ctx.r[11].s64 + -7736;
	// 8325D478: 386AAD58  addi r3, r10, -0x52a8
	ctx.r[3].s64 = ctx.r[10].s64 + -21160;
	// 8325D47C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D480: 4AFCFA51  bl 0x8222ced0
	ctx.lr = 0x8325D484;
	sub_8222CED0(ctx, base);
	// 8325D484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D488: 3869C500  addi r3, r9, -0x3b00
	ctx.r[3].s64 = ctx.r[9].s64 + -15104;
	// 8325D48C: 4BA4CA95  bl 0x82ca9f20
	ctx.lr = 0x8325D490;
	sub_82CA9F20(ctx, base);
	// 8325D490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D4A0 size=64
    let mut pc: u32 = 0x8325D4A0;
    'dispatch: loop {
        match pc {
            0x8325D4A0 => {
    //   block [0x8325D4A0..0x8325D4E0)
	// 8325D4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D4AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D4B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D4B4: 388BE1E4  addi r4, r11, -0x1e1c
	ctx.r[4].s64 = ctx.r[11].s64 + -7708;
	// 8325D4B8: 386AAD5C  addi r3, r10, -0x52a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21156;
	// 8325D4BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D4C0: 4AFCFA11  bl 0x8222ced0
	ctx.lr = 0x8325D4C4;
	sub_8222CED0(ctx, base);
	// 8325D4C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D4C8: 3869C510  addi r3, r9, -0x3af0
	ctx.r[3].s64 = ctx.r[9].s64 + -15088;
	// 8325D4CC: 4BA4CA55  bl 0x82ca9f20
	ctx.lr = 0x8325D4D0;
	sub_82CA9F20(ctx, base);
	// 8325D4D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D4E0 size=64
    let mut pc: u32 = 0x8325D4E0;
    'dispatch: loop {
        match pc {
            0x8325D4E0 => {
    //   block [0x8325D4E0..0x8325D520)
	// 8325D4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D4E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D4EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D4F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D4F4: 388BE1F8  addi r4, r11, -0x1e08
	ctx.r[4].s64 = ctx.r[11].s64 + -7688;
	// 8325D4F8: 386AAD60  addi r3, r10, -0x52a0
	ctx.r[3].s64 = ctx.r[10].s64 + -21152;
	// 8325D4FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D500: 4AFCF9D1  bl 0x8222ced0
	ctx.lr = 0x8325D504;
	sub_8222CED0(ctx, base);
	// 8325D504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D508: 3869C520  addi r3, r9, -0x3ae0
	ctx.r[3].s64 = ctx.r[9].s64 + -15072;
	// 8325D50C: 4BA4CA15  bl 0x82ca9f20
	ctx.lr = 0x8325D510;
	sub_82CA9F20(ctx, base);
	// 8325D510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D520 size=64
    let mut pc: u32 = 0x8325D520;
    'dispatch: loop {
        match pc {
            0x8325D520 => {
    //   block [0x8325D520..0x8325D560)
	// 8325D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D52C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D534: 388BE214  addi r4, r11, -0x1dec
	ctx.r[4].s64 = ctx.r[11].s64 + -7660;
	// 8325D538: 386AAD64  addi r3, r10, -0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + -21148;
	// 8325D53C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D540: 4AFCF991  bl 0x8222ced0
	ctx.lr = 0x8325D544;
	sub_8222CED0(ctx, base);
	// 8325D544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D548: 3869C530  addi r3, r9, -0x3ad0
	ctx.r[3].s64 = ctx.r[9].s64 + -15056;
	// 8325D54C: 4BA4C9D5  bl 0x82ca9f20
	ctx.lr = 0x8325D550;
	sub_82CA9F20(ctx, base);
	// 8325D550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D560 size=64
    let mut pc: u32 = 0x8325D560;
    'dispatch: loop {
        match pc {
            0x8325D560 => {
    //   block [0x8325D560..0x8325D5A0)
	// 8325D560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D56C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D574: 388BE1BC  addi r4, r11, -0x1e44
	ctx.r[4].s64 = ctx.r[11].s64 + -7748;
	// 8325D578: 386AAD68  addi r3, r10, -0x5298
	ctx.r[3].s64 = ctx.r[10].s64 + -21144;
	// 8325D57C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D580: 4AFCF951  bl 0x8222ced0
	ctx.lr = 0x8325D584;
	sub_8222CED0(ctx, base);
	// 8325D584: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D588: 3869C540  addi r3, r9, -0x3ac0
	ctx.r[3].s64 = ctx.r[9].s64 + -15040;
	// 8325D58C: 4BA4C995  bl 0x82ca9f20
	ctx.lr = 0x8325D590;
	sub_82CA9F20(ctx, base);
	// 8325D590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D5A0 size=64
    let mut pc: u32 = 0x8325D5A0;
    'dispatch: loop {
        match pc {
            0x8325D5A0 => {
    //   block [0x8325D5A0..0x8325D5E0)
	// 8325D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D5A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D5AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D5B4: 388BE230  addi r4, r11, -0x1dd0
	ctx.r[4].s64 = ctx.r[11].s64 + -7632;
	// 8325D5B8: 386AAD6C  addi r3, r10, -0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + -21140;
	// 8325D5BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D5C0: 4AFCF911  bl 0x8222ced0
	ctx.lr = 0x8325D5C4;
	sub_8222CED0(ctx, base);
	// 8325D5C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D5C8: 3869C550  addi r3, r9, -0x3ab0
	ctx.r[3].s64 = ctx.r[9].s64 + -15024;
	// 8325D5CC: 4BA4C955  bl 0x82ca9f20
	ctx.lr = 0x8325D5D0;
	sub_82CA9F20(ctx, base);
	// 8325D5D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D5E0 size=64
    let mut pc: u32 = 0x8325D5E0;
    'dispatch: loop {
        match pc {
            0x8325D5E0 => {
    //   block [0x8325D5E0..0x8325D620)
	// 8325D5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D5E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D5EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8325D5F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D5F4: 388B250C  addi r4, r11, 0x250c
	ctx.r[4].s64 = ctx.r[11].s64 + 9484;
	// 8325D5F8: 386AAD70  addi r3, r10, -0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + -21136;
	// 8325D5FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D600: 4AFCF8D1  bl 0x8222ced0
	ctx.lr = 0x8325D604;
	sub_8222CED0(ctx, base);
	// 8325D604: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D608: 3869C560  addi r3, r9, -0x3aa0
	ctx.r[3].s64 = ctx.r[9].s64 + -15008;
	// 8325D60C: 4BA4C915  bl 0x82ca9f20
	ctx.lr = 0x8325D610;
	sub_82CA9F20(ctx, base);
	// 8325D610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D620 size=64
    let mut pc: u32 = 0x8325D620;
    'dispatch: loop {
        match pc {
            0x8325D620 => {
    //   block [0x8325D620..0x8325D660)
	// 8325D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D62C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325D630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D634: 388BE9AC  addi r4, r11, -0x1654
	ctx.r[4].s64 = ctx.r[11].s64 + -5716;
	// 8325D638: 386AAD74  addi r3, r10, -0x528c
	ctx.r[3].s64 = ctx.r[10].s64 + -21132;
	// 8325D63C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D640: 4AFCF891  bl 0x8222ced0
	ctx.lr = 0x8325D644;
	sub_8222CED0(ctx, base);
	// 8325D644: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D648: 3869C570  addi r3, r9, -0x3a90
	ctx.r[3].s64 = ctx.r[9].s64 + -14992;
	// 8325D64C: 4BA4C8D5  bl 0x82ca9f20
	ctx.lr = 0x8325D650;
	sub_82CA9F20(ctx, base);
	// 8325D650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D660 size=64
    let mut pc: u32 = 0x8325D660;
    'dispatch: loop {
        match pc {
            0x8325D660 => {
    //   block [0x8325D660..0x8325D6A0)
	// 8325D660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D66C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325D670: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D674: 388B0350  addi r4, r11, 0x350
	ctx.r[4].s64 = ctx.r[11].s64 + 848;
	// 8325D678: 386AAD78  addi r3, r10, -0x5288
	ctx.r[3].s64 = ctx.r[10].s64 + -21128;
	// 8325D67C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D680: 4AFCF851  bl 0x8222ced0
	ctx.lr = 0x8325D684;
	sub_8222CED0(ctx, base);
	// 8325D684: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D688: 3869C580  addi r3, r9, -0x3a80
	ctx.r[3].s64 = ctx.r[9].s64 + -14976;
	// 8325D68C: 4BA4C895  bl 0x82ca9f20
	ctx.lr = 0x8325D690;
	sub_82CA9F20(ctx, base);
	// 8325D690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D6A0 size=64
    let mut pc: u32 = 0x8325D6A0;
    'dispatch: loop {
        match pc {
            0x8325D6A0 => {
    //   block [0x8325D6A0..0x8325D6E0)
	// 8325D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D6A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D6AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325D6B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D6B4: 388B036C  addi r4, r11, 0x36c
	ctx.r[4].s64 = ctx.r[11].s64 + 876;
	// 8325D6B8: 386AAD7C  addi r3, r10, -0x5284
	ctx.r[3].s64 = ctx.r[10].s64 + -21124;
	// 8325D6BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D6C0: 4AFCF811  bl 0x8222ced0
	ctx.lr = 0x8325D6C4;
	sub_8222CED0(ctx, base);
	// 8325D6C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D6C8: 3869C590  addi r3, r9, -0x3a70
	ctx.r[3].s64 = ctx.r[9].s64 + -14960;
	// 8325D6CC: 4BA4C855  bl 0x82ca9f20
	ctx.lr = 0x8325D6D0;
	sub_82CA9F20(ctx, base);
	// 8325D6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D6E0 size=64
    let mut pc: u32 = 0x8325D6E0;
    'dispatch: loop {
        match pc {
            0x8325D6E0 => {
    //   block [0x8325D6E0..0x8325D720)
	// 8325D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D6EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325D6F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D6F4: 388BE250  addi r4, r11, -0x1db0
	ctx.r[4].s64 = ctx.r[11].s64 + -7600;
	// 8325D6F8: 386AAD80  addi r3, r10, -0x5280
	ctx.r[3].s64 = ctx.r[10].s64 + -21120;
	// 8325D6FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325D700: 4AFCF7D1  bl 0x8222ced0
	ctx.lr = 0x8325D704;
	sub_8222CED0(ctx, base);
	// 8325D704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325D708: 3869C5A0  addi r3, r9, -0x3a60
	ctx.r[3].s64 = ctx.r[9].s64 + -14944;
	// 8325D70C: 4BA4C815  bl 0x82ca9f20
	ctx.lr = 0x8325D710;
	sub_82CA9F20(ctx, base);
	// 8325D710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D720 size=56
    let mut pc: u32 = 0x8325D720;
    'dispatch: loop {
        match pc {
            0x8325D720 => {
    //   block [0x8325D720..0x8325D758)
	// 8325D720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D72C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D734: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325D738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D73C: 4AF9661D  bl 0x821f3d58
	ctx.lr = 0x8325D740;
	sub_821F3D58(ctx, base);
	// 8325D740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D744: 906AAD84  stw r3, -0x527c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21116 as u32), ctx.r[3].u32 ) };
	// 8325D748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D758 size=56
    let mut pc: u32 = 0x8325D758;
    'dispatch: loop {
        match pc {
            0x8325D758 => {
    //   block [0x8325D758..0x8325D790)
	// 8325D758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D76C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325D770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D774: 4AF965E5  bl 0x821f3d58
	ctx.lr = 0x8325D778;
	sub_821F3D58(ctx, base);
	// 8325D778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D77C: 906AAD88  stw r3, -0x5278(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21112 as u32), ctx.r[3].u32 ) };
	// 8325D780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D790 size=56
    let mut pc: u32 = 0x8325D790;
    'dispatch: loop {
        match pc {
            0x8325D790 => {
    //   block [0x8325D790..0x8325D7C8)
	// 8325D790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D79C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D7A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D7A4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325D7A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D7AC: 4AF965AD  bl 0x821f3d58
	ctx.lr = 0x8325D7B0;
	sub_821F3D58(ctx, base);
	// 8325D7B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D7B4: 906AAD8C  stw r3, -0x5274(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21108 as u32), ctx.r[3].u32 ) };
	// 8325D7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D7C8 size=56
    let mut pc: u32 = 0x8325D7C8;
    'dispatch: loop {
        match pc {
            0x8325D7C8 => {
    //   block [0x8325D7C8..0x8325D800)
	// 8325D7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D7D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D7DC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325D7E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D7E4: 4AF96575  bl 0x821f3d58
	ctx.lr = 0x8325D7E8;
	sub_821F3D58(ctx, base);
	// 8325D7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D7EC: 906AAD90  stw r3, -0x5270(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21104 as u32), ctx.r[3].u32 ) };
	// 8325D7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D800 size=56
    let mut pc: u32 = 0x8325D800;
    'dispatch: loop {
        match pc {
            0x8325D800 => {
    //   block [0x8325D800..0x8325D838)
	// 8325D800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D814: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325D818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D81C: 4AF9653D  bl 0x821f3d58
	ctx.lr = 0x8325D820;
	sub_821F3D58(ctx, base);
	// 8325D820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D824: 906AAD94  stw r3, -0x526c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21100 as u32), ctx.r[3].u32 ) };
	// 8325D828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D838 size=56
    let mut pc: u32 = 0x8325D838;
    'dispatch: loop {
        match pc {
            0x8325D838 => {
    //   block [0x8325D838..0x8325D870)
	// 8325D838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D84C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325D850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D854: 4AF96505  bl 0x821f3d58
	ctx.lr = 0x8325D858;
	sub_821F3D58(ctx, base);
	// 8325D858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D85C: 906AAD98  stw r3, -0x5268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21096 as u32), ctx.r[3].u32 ) };
	// 8325D860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D870 size=56
    let mut pc: u32 = 0x8325D870;
    'dispatch: loop {
        match pc {
            0x8325D870 => {
    //   block [0x8325D870..0x8325D8A8)
	// 8325D870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D87C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D884: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325D888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D88C: 4AF964CD  bl 0x821f3d58
	ctx.lr = 0x8325D890;
	sub_821F3D58(ctx, base);
	// 8325D890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D894: 906AAD9C  stw r3, -0x5264(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21092 as u32), ctx.r[3].u32 ) };
	// 8325D898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D8A8 size=56
    let mut pc: u32 = 0x8325D8A8;
    'dispatch: loop {
        match pc {
            0x8325D8A8 => {
    //   block [0x8325D8A8..0x8325D8E0)
	// 8325D8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D8B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D8BC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325D8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D8C4: 4AF96495  bl 0x821f3d58
	ctx.lr = 0x8325D8C8;
	sub_821F3D58(ctx, base);
	// 8325D8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D8CC: 906AADA0  stw r3, -0x5260(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21088 as u32), ctx.r[3].u32 ) };
	// 8325D8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D8E0 size=56
    let mut pc: u32 = 0x8325D8E0;
    'dispatch: loop {
        match pc {
            0x8325D8E0 => {
    //   block [0x8325D8E0..0x8325D918)
	// 8325D8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D8F4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325D8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D8FC: 4AF9645D  bl 0x821f3d58
	ctx.lr = 0x8325D900;
	sub_821F3D58(ctx, base);
	// 8325D900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D904: 906AADA4  stw r3, -0x525c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21084 as u32), ctx.r[3].u32 ) };
	// 8325D908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D918 size=56
    let mut pc: u32 = 0x8325D918;
    'dispatch: loop {
        match pc {
            0x8325D918 => {
    //   block [0x8325D918..0x8325D950)
	// 8325D918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D92C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325D930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D934: 4AF96425  bl 0x821f3d58
	ctx.lr = 0x8325D938;
	sub_821F3D58(ctx, base);
	// 8325D938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D93C: 906AADA8  stw r3, -0x5258(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21080 as u32), ctx.r[3].u32 ) };
	// 8325D940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D950 size=56
    let mut pc: u32 = 0x8325D950;
    'dispatch: loop {
        match pc {
            0x8325D950 => {
    //   block [0x8325D950..0x8325D988)
	// 8325D950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D95C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D964: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325D968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D96C: 4AF963ED  bl 0x821f3d58
	ctx.lr = 0x8325D970;
	sub_821F3D58(ctx, base);
	// 8325D970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D974: 906AADAC  stw r3, -0x5254(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21076 as u32), ctx.r[3].u32 ) };
	// 8325D978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D988 size=56
    let mut pc: u32 = 0x8325D988;
    'dispatch: loop {
        match pc {
            0x8325D988 => {
    //   block [0x8325D988..0x8325D9C0)
	// 8325D988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D99C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325D9A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D9A4: 4AF963B5  bl 0x821f3d58
	ctx.lr = 0x8325D9A8;
	sub_821F3D58(ctx, base);
	// 8325D9A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D9AC: 906AADB0  stw r3, -0x5250(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21072 as u32), ctx.r[3].u32 ) };
	// 8325D9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D9C0 size=56
    let mut pc: u32 = 0x8325D9C0;
    'dispatch: loop {
        match pc {
            0x8325D9C0 => {
    //   block [0x8325D9C0..0x8325D9F8)
	// 8325D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325D9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325D9CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325D9D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325D9D4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325D9D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325D9DC: 4AF9637D  bl 0x821f3d58
	ctx.lr = 0x8325D9E0;
	sub_821F3D58(ctx, base);
	// 8325D9E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325D9E4: 906AADB4  stw r3, -0x524c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21068 as u32), ctx.r[3].u32 ) };
	// 8325D9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325D9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325D9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325D9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325D9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325D9F8 size=56
    let mut pc: u32 = 0x8325D9F8;
    'dispatch: loop {
        match pc {
            0x8325D9F8 => {
    //   block [0x8325D9F8..0x8325DA30)
	// 8325D9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325D9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DA04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DA08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DA0C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325DA10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DA14: 4AF96345  bl 0x821f3d58
	ctx.lr = 0x8325DA18;
	sub_821F3D58(ctx, base);
	// 8325DA18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DA1C: 906AADB8  stw r3, -0x5248(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21064 as u32), ctx.r[3].u32 ) };
	// 8325DA20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DA30 size=56
    let mut pc: u32 = 0x8325DA30;
    'dispatch: loop {
        match pc {
            0x8325DA30 => {
    //   block [0x8325DA30..0x8325DA68)
	// 8325DA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DA3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DA40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DA44: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325DA48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DA4C: 4AF9630D  bl 0x821f3d58
	ctx.lr = 0x8325DA50;
	sub_821F3D58(ctx, base);
	// 8325DA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DA54: 906AADBC  stw r3, -0x5244(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21060 as u32), ctx.r[3].u32 ) };
	// 8325DA58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DA68 size=56
    let mut pc: u32 = 0x8325DA68;
    'dispatch: loop {
        match pc {
            0x8325DA68 => {
    //   block [0x8325DA68..0x8325DAA0)
	// 8325DA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DA70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DA74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DA78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DA7C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325DA80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DA84: 4AF962D5  bl 0x821f3d58
	ctx.lr = 0x8325DA88;
	sub_821F3D58(ctx, base);
	// 8325DA88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DA8C: 906AADC0  stw r3, -0x5240(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21056 as u32), ctx.r[3].u32 ) };
	// 8325DA90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DAA0 size=56
    let mut pc: u32 = 0x8325DAA0;
    'dispatch: loop {
        match pc {
            0x8325DAA0 => {
    //   block [0x8325DAA0..0x8325DAD8)
	// 8325DAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DAA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DAAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DAB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DAB4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325DAB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DABC: 4AF9629D  bl 0x821f3d58
	ctx.lr = 0x8325DAC0;
	sub_821F3D58(ctx, base);
	// 8325DAC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DAC4: 906AADC4  stw r3, -0x523c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21052 as u32), ctx.r[3].u32 ) };
	// 8325DAC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DAD8 size=56
    let mut pc: u32 = 0x8325DAD8;
    'dispatch: loop {
        match pc {
            0x8325DAD8 => {
    //   block [0x8325DAD8..0x8325DB10)
	// 8325DAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DAE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DAE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DAE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DAEC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325DAF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DAF4: 4AF96265  bl 0x821f3d58
	ctx.lr = 0x8325DAF8;
	sub_821F3D58(ctx, base);
	// 8325DAF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DAFC: 906AADC8  stw r3, -0x5238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21048 as u32), ctx.r[3].u32 ) };
	// 8325DB00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DB10 size=56
    let mut pc: u32 = 0x8325DB10;
    'dispatch: loop {
        match pc {
            0x8325DB10 => {
    //   block [0x8325DB10..0x8325DB48)
	// 8325DB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DB1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DB20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DB24: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325DB28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DB2C: 4AF9622D  bl 0x821f3d58
	ctx.lr = 0x8325DB30;
	sub_821F3D58(ctx, base);
	// 8325DB30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DB34: 906AADCC  stw r3, -0x5234(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21044 as u32), ctx.r[3].u32 ) };
	// 8325DB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DB48 size=56
    let mut pc: u32 = 0x8325DB48;
    'dispatch: loop {
        match pc {
            0x8325DB48 => {
    //   block [0x8325DB48..0x8325DB80)
	// 8325DB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DB50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DB54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DB58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DB5C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325DB60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DB64: 4AF961F5  bl 0x821f3d58
	ctx.lr = 0x8325DB68;
	sub_821F3D58(ctx, base);
	// 8325DB68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DB6C: 906AADD0  stw r3, -0x5230(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21040 as u32), ctx.r[3].u32 ) };
	// 8325DB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DB80 size=56
    let mut pc: u32 = 0x8325DB80;
    'dispatch: loop {
        match pc {
            0x8325DB80 => {
    //   block [0x8325DB80..0x8325DBB8)
	// 8325DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DB8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DB90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DB94: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325DB98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DB9C: 4AF961BD  bl 0x821f3d58
	ctx.lr = 0x8325DBA0;
	sub_821F3D58(ctx, base);
	// 8325DBA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DBA4: 906AADD4  stw r3, -0x522c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21036 as u32), ctx.r[3].u32 ) };
	// 8325DBA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DBB8 size=64
    let mut pc: u32 = 0x8325DBB8;
    'dispatch: loop {
        match pc {
            0x8325DBB8 => {
    //   block [0x8325DBB8..0x8325DBF8)
	// 8325DBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DBC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DBC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325DBC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DBCC: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325DBD0: 386AADD8  addi r3, r10, -0x5228
	ctx.r[3].s64 = ctx.r[10].s64 + -21032;
	// 8325DBD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325DBD8: 4AFCF2F9  bl 0x8222ced0
	ctx.lr = 0x8325DBDC;
	sub_8222CED0(ctx, base);
	// 8325DBDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325DBE0: 3869C5B0  addi r3, r9, -0x3a50
	ctx.r[3].s64 = ctx.r[9].s64 + -14928;
	// 8325DBE4: 4BA4C33D  bl 0x82ca9f20
	ctx.lr = 0x8325DBE8;
	sub_82CA9F20(ctx, base);
	// 8325DBE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DBF8 size=64
    let mut pc: u32 = 0x8325DBF8;
    'dispatch: loop {
        match pc {
            0x8325DBF8 => {
    //   block [0x8325DBF8..0x8325DC38)
	// 8325DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DC00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DC04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325DC08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DC0C: 388BE434  addi r4, r11, -0x1bcc
	ctx.r[4].s64 = ctx.r[11].s64 + -7116;
	// 8325DC10: 386AADDC  addi r3, r10, -0x5224
	ctx.r[3].s64 = ctx.r[10].s64 + -21028;
	// 8325DC14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325DC18: 4AFCF2B9  bl 0x8222ced0
	ctx.lr = 0x8325DC1C;
	sub_8222CED0(ctx, base);
	// 8325DC1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325DC20: 3869C5C0  addi r3, r9, -0x3a40
	ctx.r[3].s64 = ctx.r[9].s64 + -14912;
	// 8325DC24: 4BA4C2FD  bl 0x82ca9f20
	ctx.lr = 0x8325DC28;
	sub_82CA9F20(ctx, base);
	// 8325DC28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DC38 size=64
    let mut pc: u32 = 0x8325DC38;
    'dispatch: loop {
        match pc {
            0x8325DC38 => {
    //   block [0x8325DC38..0x8325DC78)
	// 8325DC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DC44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325DC48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DC4C: 388BE458  addi r4, r11, -0x1ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -7080;
	// 8325DC50: 386AADE0  addi r3, r10, -0x5220
	ctx.r[3].s64 = ctx.r[10].s64 + -21024;
	// 8325DC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325DC58: 4AFCF279  bl 0x8222ced0
	ctx.lr = 0x8325DC5C;
	sub_8222CED0(ctx, base);
	// 8325DC5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325DC60: 3869C5D0  addi r3, r9, -0x3a30
	ctx.r[3].s64 = ctx.r[9].s64 + -14896;
	// 8325DC64: 4BA4C2BD  bl 0x82ca9f20
	ctx.lr = 0x8325DC68;
	sub_82CA9F20(ctx, base);
	// 8325DC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DC78 size=56
    let mut pc: u32 = 0x8325DC78;
    'dispatch: loop {
        match pc {
            0x8325DC78 => {
    //   block [0x8325DC78..0x8325DCB0)
	// 8325DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DC8C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325DC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DC94: 4AF960C5  bl 0x821f3d58
	ctx.lr = 0x8325DC98;
	sub_821F3D58(ctx, base);
	// 8325DC98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DC9C: 906AADE4  stw r3, -0x521c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21020 as u32), ctx.r[3].u32 ) };
	// 8325DCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DCB0 size=56
    let mut pc: u32 = 0x8325DCB0;
    'dispatch: loop {
        match pc {
            0x8325DCB0 => {
    //   block [0x8325DCB0..0x8325DCE8)
	// 8325DCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DCC4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325DCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DCCC: 4AF9608D  bl 0x821f3d58
	ctx.lr = 0x8325DCD0;
	sub_821F3D58(ctx, base);
	// 8325DCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DCD4: 906AADE8  stw r3, -0x5218(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21016 as u32), ctx.r[3].u32 ) };
	// 8325DCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DCE8 size=56
    let mut pc: u32 = 0x8325DCE8;
    'dispatch: loop {
        match pc {
            0x8325DCE8 => {
    //   block [0x8325DCE8..0x8325DD20)
	// 8325DCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DCFC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325DD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DD04: 4AF96055  bl 0x821f3d58
	ctx.lr = 0x8325DD08;
	sub_821F3D58(ctx, base);
	// 8325DD08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DD0C: 906AADEC  stw r3, -0x5214(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21012 as u32), ctx.r[3].u32 ) };
	// 8325DD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DD20 size=56
    let mut pc: u32 = 0x8325DD20;
    'dispatch: loop {
        match pc {
            0x8325DD20 => {
    //   block [0x8325DD20..0x8325DD58)
	// 8325DD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DD34: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325DD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DD3C: 4AF9601D  bl 0x821f3d58
	ctx.lr = 0x8325DD40;
	sub_821F3D58(ctx, base);
	// 8325DD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DD44: 906AADF0  stw r3, -0x5210(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21008 as u32), ctx.r[3].u32 ) };
	// 8325DD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DD58 size=56
    let mut pc: u32 = 0x8325DD58;
    'dispatch: loop {
        match pc {
            0x8325DD58 => {
    //   block [0x8325DD58..0x8325DD90)
	// 8325DD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DD6C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325DD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DD74: 4AF95FE5  bl 0x821f3d58
	ctx.lr = 0x8325DD78;
	sub_821F3D58(ctx, base);
	// 8325DD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DD7C: 906AADF4  stw r3, -0x520c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21004 as u32), ctx.r[3].u32 ) };
	// 8325DD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DD90 size=56
    let mut pc: u32 = 0x8325DD90;
    'dispatch: loop {
        match pc {
            0x8325DD90 => {
    //   block [0x8325DD90..0x8325DDC8)
	// 8325DD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DDA4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325DDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DDAC: 4AF95FAD  bl 0x821f3d58
	ctx.lr = 0x8325DDB0;
	sub_821F3D58(ctx, base);
	// 8325DDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DDB4: 906AADF8  stw r3, -0x5208(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21000 as u32), ctx.r[3].u32 ) };
	// 8325DDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DDC8 size=56
    let mut pc: u32 = 0x8325DDC8;
    'dispatch: loop {
        match pc {
            0x8325DDC8 => {
    //   block [0x8325DDC8..0x8325DE00)
	// 8325DDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DDDC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325DDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DDE4: 4AF95F75  bl 0x821f3d58
	ctx.lr = 0x8325DDE8;
	sub_821F3D58(ctx, base);
	// 8325DDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DDEC: 906AADFC  stw r3, -0x5204(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20996 as u32), ctx.r[3].u32 ) };
	// 8325DDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DE00 size=56
    let mut pc: u32 = 0x8325DE00;
    'dispatch: loop {
        match pc {
            0x8325DE00 => {
    //   block [0x8325DE00..0x8325DE38)
	// 8325DE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DE14: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325DE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DE1C: 4AF95F3D  bl 0x821f3d58
	ctx.lr = 0x8325DE20;
	sub_821F3D58(ctx, base);
	// 8325DE20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DE24: 906AAE00  stw r3, -0x5200(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20992 as u32), ctx.r[3].u32 ) };
	// 8325DE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DE38 size=56
    let mut pc: u32 = 0x8325DE38;
    'dispatch: loop {
        match pc {
            0x8325DE38 => {
    //   block [0x8325DE38..0x8325DE70)
	// 8325DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DE4C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325DE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DE54: 4AF95F05  bl 0x821f3d58
	ctx.lr = 0x8325DE58;
	sub_821F3D58(ctx, base);
	// 8325DE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DE5C: 906AAE04  stw r3, -0x51fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20988 as u32), ctx.r[3].u32 ) };
	// 8325DE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DE70 size=56
    let mut pc: u32 = 0x8325DE70;
    'dispatch: loop {
        match pc {
            0x8325DE70 => {
    //   block [0x8325DE70..0x8325DEA8)
	// 8325DE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DE84: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325DE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DE8C: 4AF95ECD  bl 0x821f3d58
	ctx.lr = 0x8325DE90;
	sub_821F3D58(ctx, base);
	// 8325DE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DE94: 906AAE08  stw r3, -0x51f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20984 as u32), ctx.r[3].u32 ) };
	// 8325DE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DEA8 size=56
    let mut pc: u32 = 0x8325DEA8;
    'dispatch: loop {
        match pc {
            0x8325DEA8 => {
    //   block [0x8325DEA8..0x8325DEE0)
	// 8325DEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DEB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DEBC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325DEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DEC4: 4AF95E95  bl 0x821f3d58
	ctx.lr = 0x8325DEC8;
	sub_821F3D58(ctx, base);
	// 8325DEC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DECC: 906AAE0C  stw r3, -0x51f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20980 as u32), ctx.r[3].u32 ) };
	// 8325DED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DEE0 size=56
    let mut pc: u32 = 0x8325DEE0;
    'dispatch: loop {
        match pc {
            0x8325DEE0 => {
    //   block [0x8325DEE0..0x8325DF18)
	// 8325DEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DEEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DEF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DEF4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325DEF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DEFC: 4AF95E5D  bl 0x821f3d58
	ctx.lr = 0x8325DF00;
	sub_821F3D58(ctx, base);
	// 8325DF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DF04: 906AAE10  stw r3, -0x51f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20976 as u32), ctx.r[3].u32 ) };
	// 8325DF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DF18 size=56
    let mut pc: u32 = 0x8325DF18;
    'dispatch: loop {
        match pc {
            0x8325DF18 => {
    //   block [0x8325DF18..0x8325DF50)
	// 8325DF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DF24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DF2C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325DF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DF34: 4AF95E25  bl 0x821f3d58
	ctx.lr = 0x8325DF38;
	sub_821F3D58(ctx, base);
	// 8325DF38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DF3C: 906AAE14  stw r3, -0x51ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20972 as u32), ctx.r[3].u32 ) };
	// 8325DF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DF50 size=56
    let mut pc: u32 = 0x8325DF50;
    'dispatch: loop {
        match pc {
            0x8325DF50 => {
    //   block [0x8325DF50..0x8325DF88)
	// 8325DF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DF5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DF60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DF64: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325DF68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DF6C: 4AF95DED  bl 0x821f3d58
	ctx.lr = 0x8325DF70;
	sub_821F3D58(ctx, base);
	// 8325DF70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DF74: 906AAE18  stw r3, -0x51e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20968 as u32), ctx.r[3].u32 ) };
	// 8325DF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DF88 size=56
    let mut pc: u32 = 0x8325DF88;
    'dispatch: loop {
        match pc {
            0x8325DF88 => {
    //   block [0x8325DF88..0x8325DFC0)
	// 8325DF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DF94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DF98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DF9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325DFA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DFA4: 4AF95DB5  bl 0x821f3d58
	ctx.lr = 0x8325DFA8;
	sub_821F3D58(ctx, base);
	// 8325DFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DFAC: 906AAE1C  stw r3, -0x51e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20964 as u32), ctx.r[3].u32 ) };
	// 8325DFB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DFC0 size=56
    let mut pc: u32 = 0x8325DFC0;
    'dispatch: loop {
        match pc {
            0x8325DFC0 => {
    //   block [0x8325DFC0..0x8325DFF8)
	// 8325DFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325DFC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325DFCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325DFD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325DFD4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325DFD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325DFDC: 4AF95D7D  bl 0x821f3d58
	ctx.lr = 0x8325DFE0;
	sub_821F3D58(ctx, base);
	// 8325DFE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325DFE4: 906AAE20  stw r3, -0x51e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20960 as u32), ctx.r[3].u32 ) };
	// 8325DFE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325DFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325DFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325DFF8 size=56
    let mut pc: u32 = 0x8325DFF8;
    'dispatch: loop {
        match pc {
            0x8325DFF8 => {
    //   block [0x8325DFF8..0x8325E030)
	// 8325DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325DFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E004: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E008: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E00C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325E010: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E014: 4AF95D45  bl 0x821f3d58
	ctx.lr = 0x8325E018;
	sub_821F3D58(ctx, base);
	// 8325E018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E01C: 906AAE24  stw r3, -0x51dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20956 as u32), ctx.r[3].u32 ) };
	// 8325E020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E030 size=56
    let mut pc: u32 = 0x8325E030;
    'dispatch: loop {
        match pc {
            0x8325E030 => {
    //   block [0x8325E030..0x8325E068)
	// 8325E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E03C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E040: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E044: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325E048: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E04C: 4AF95D0D  bl 0x821f3d58
	ctx.lr = 0x8325E050;
	sub_821F3D58(ctx, base);
	// 8325E050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E054: 906AAE28  stw r3, -0x51d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20952 as u32), ctx.r[3].u32 ) };
	// 8325E058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E068 size=56
    let mut pc: u32 = 0x8325E068;
    'dispatch: loop {
        match pc {
            0x8325E068 => {
    //   block [0x8325E068..0x8325E0A0)
	// 8325E068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E074: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E078: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E07C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325E080: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E084: 4AF95CD5  bl 0x821f3d58
	ctx.lr = 0x8325E088;
	sub_821F3D58(ctx, base);
	// 8325E088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E08C: 906AAE2C  stw r3, -0x51d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20948 as u32), ctx.r[3].u32 ) };
	// 8325E090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E0A0 size=56
    let mut pc: u32 = 0x8325E0A0;
    'dispatch: loop {
        match pc {
            0x8325E0A0 => {
    //   block [0x8325E0A0..0x8325E0D8)
	// 8325E0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E0AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E0B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E0B4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325E0B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E0BC: 4AF95C9D  bl 0x821f3d58
	ctx.lr = 0x8325E0C0;
	sub_821F3D58(ctx, base);
	// 8325E0C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E0C4: 906AAE30  stw r3, -0x51d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20944 as u32), ctx.r[3].u32 ) };
	// 8325E0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E0D8 size=56
    let mut pc: u32 = 0x8325E0D8;
    'dispatch: loop {
        match pc {
            0x8325E0D8 => {
    //   block [0x8325E0D8..0x8325E110)
	// 8325E0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E0E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E0E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E0EC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325E0F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E0F4: 4AF95C65  bl 0x821f3d58
	ctx.lr = 0x8325E0F8;
	sub_821F3D58(ctx, base);
	// 8325E0F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E0FC: 906AAE34  stw r3, -0x51cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20940 as u32), ctx.r[3].u32 ) };
	// 8325E100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E110 size=64
    let mut pc: u32 = 0x8325E110;
    'dispatch: loop {
        match pc {
            0x8325E110 => {
    //   block [0x8325E110..0x8325E150)
	// 8325E110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E11C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E124: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 8325E128: 386AAE38  addi r3, r10, -0x51c8
	ctx.r[3].s64 = ctx.r[10].s64 + -20936;
	// 8325E12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325E130: 4AFCEDA1  bl 0x8222ced0
	ctx.lr = 0x8325E134;
	sub_8222CED0(ctx, base);
	// 8325E134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325E138: 3869C5E0  addi r3, r9, -0x3a20
	ctx.r[3].s64 = ctx.r[9].s64 + -14880;
	// 8325E13C: 4BA4BDE5  bl 0x82ca9f20
	ctx.lr = 0x8325E140;
	sub_82CA9F20(ctx, base);
	// 8325E140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E150 size=64
    let mut pc: u32 = 0x8325E150;
    'dispatch: loop {
        match pc {
            0x8325E150 => {
    //   block [0x8325E150..0x8325E190)
	// 8325E150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E15C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E164: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325E168: 386AAE3C  addi r3, r10, -0x51c4
	ctx.r[3].s64 = ctx.r[10].s64 + -20932;
	// 8325E16C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325E170: 4AFCED61  bl 0x8222ced0
	ctx.lr = 0x8325E174;
	sub_8222CED0(ctx, base);
	// 8325E174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325E178: 3869C5F0  addi r3, r9, -0x3a10
	ctx.r[3].s64 = ctx.r[9].s64 + -14864;
	// 8325E17C: 4BA4BDA5  bl 0x82ca9f20
	ctx.lr = 0x8325E180;
	sub_82CA9F20(ctx, base);
	// 8325E180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E190 size=56
    let mut pc: u32 = 0x8325E190;
    'dispatch: loop {
        match pc {
            0x8325E190 => {
    //   block [0x8325E190..0x8325E1C8)
	// 8325E190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E19C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E1A4: 386BE734  addi r3, r11, -0x18cc
	ctx.r[3].s64 = ctx.r[11].s64 + -6348;
	// 8325E1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E1AC: 4AF95BAD  bl 0x821f3d58
	ctx.lr = 0x8325E1B0;
	sub_821F3D58(ctx, base);
	// 8325E1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E1B4: 906AAE40  stw r3, -0x51c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20928 as u32), ctx.r[3].u32 ) };
	// 8325E1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E1C8 size=56
    let mut pc: u32 = 0x8325E1C8;
    'dispatch: loop {
        match pc {
            0x8325E1C8 => {
    //   block [0x8325E1C8..0x8325E200)
	// 8325E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E1D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E1DC: 386BEA84  addi r3, r11, -0x157c
	ctx.r[3].s64 = ctx.r[11].s64 + -5500;
	// 8325E1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E1E4: 4AF95B75  bl 0x821f3d58
	ctx.lr = 0x8325E1E8;
	sub_821F3D58(ctx, base);
	// 8325E1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E1EC: 906AAE44  stw r3, -0x51bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20924 as u32), ctx.r[3].u32 ) };
	// 8325E1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E200 size=56
    let mut pc: u32 = 0x8325E200;
    'dispatch: loop {
        match pc {
            0x8325E200 => {
    //   block [0x8325E200..0x8325E238)
	// 8325E200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E214: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325E218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E21C: 4AF95B3D  bl 0x821f3d58
	ctx.lr = 0x8325E220;
	sub_821F3D58(ctx, base);
	// 8325E220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E224: 906AAE48  stw r3, -0x51b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20920 as u32), ctx.r[3].u32 ) };
	// 8325E228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E238 size=56
    let mut pc: u32 = 0x8325E238;
    'dispatch: loop {
        match pc {
            0x8325E238 => {
    //   block [0x8325E238..0x8325E270)
	// 8325E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E24C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325E250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E254: 4AF95B05  bl 0x821f3d58
	ctx.lr = 0x8325E258;
	sub_821F3D58(ctx, base);
	// 8325E258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E25C: 906AAE4C  stw r3, -0x51b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20916 as u32), ctx.r[3].u32 ) };
	// 8325E260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E270 size=56
    let mut pc: u32 = 0x8325E270;
    'dispatch: loop {
        match pc {
            0x8325E270 => {
    //   block [0x8325E270..0x8325E2A8)
	// 8325E270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E284: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325E288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E28C: 4AF95ACD  bl 0x821f3d58
	ctx.lr = 0x8325E290;
	sub_821F3D58(ctx, base);
	// 8325E290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E294: 906AAE50  stw r3, -0x51b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20912 as u32), ctx.r[3].u32 ) };
	// 8325E298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E2A8 size=56
    let mut pc: u32 = 0x8325E2A8;
    'dispatch: loop {
        match pc {
            0x8325E2A8 => {
    //   block [0x8325E2A8..0x8325E2E0)
	// 8325E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E2BC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325E2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E2C4: 4AF95A95  bl 0x821f3d58
	ctx.lr = 0x8325E2C8;
	sub_821F3D58(ctx, base);
	// 8325E2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E2CC: 906AAE54  stw r3, -0x51ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20908 as u32), ctx.r[3].u32 ) };
	// 8325E2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E2E0 size=56
    let mut pc: u32 = 0x8325E2E0;
    'dispatch: loop {
        match pc {
            0x8325E2E0 => {
    //   block [0x8325E2E0..0x8325E318)
	// 8325E2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E2F4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325E2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E2FC: 4AF95A5D  bl 0x821f3d58
	ctx.lr = 0x8325E300;
	sub_821F3D58(ctx, base);
	// 8325E300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E304: 906AAE58  stw r3, -0x51a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20904 as u32), ctx.r[3].u32 ) };
	// 8325E308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E318 size=56
    let mut pc: u32 = 0x8325E318;
    'dispatch: loop {
        match pc {
            0x8325E318 => {
    //   block [0x8325E318..0x8325E350)
	// 8325E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E32C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325E330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E334: 4AF95A25  bl 0x821f3d58
	ctx.lr = 0x8325E338;
	sub_821F3D58(ctx, base);
	// 8325E338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E33C: 906AAE5C  stw r3, -0x51a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20900 as u32), ctx.r[3].u32 ) };
	// 8325E340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E350 size=56
    let mut pc: u32 = 0x8325E350;
    'dispatch: loop {
        match pc {
            0x8325E350 => {
    //   block [0x8325E350..0x8325E388)
	// 8325E350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E364: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325E368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E36C: 4AF959ED  bl 0x821f3d58
	ctx.lr = 0x8325E370;
	sub_821F3D58(ctx, base);
	// 8325E370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E374: 906AAE60  stw r3, -0x51a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20896 as u32), ctx.r[3].u32 ) };
	// 8325E378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E388 size=56
    let mut pc: u32 = 0x8325E388;
    'dispatch: loop {
        match pc {
            0x8325E388 => {
    //   block [0x8325E388..0x8325E3C0)
	// 8325E388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E39C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325E3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E3A4: 4AF959B5  bl 0x821f3d58
	ctx.lr = 0x8325E3A8;
	sub_821F3D58(ctx, base);
	// 8325E3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E3AC: 906AAE64  stw r3, -0x519c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20892 as u32), ctx.r[3].u32 ) };
	// 8325E3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E3C0 size=56
    let mut pc: u32 = 0x8325E3C0;
    'dispatch: loop {
        match pc {
            0x8325E3C0 => {
    //   block [0x8325E3C0..0x8325E3F8)
	// 8325E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E3D4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325E3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E3DC: 4AF9597D  bl 0x821f3d58
	ctx.lr = 0x8325E3E0;
	sub_821F3D58(ctx, base);
	// 8325E3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E3E4: 906AAE68  stw r3, -0x5198(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20888 as u32), ctx.r[3].u32 ) };
	// 8325E3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E3F8 size=56
    let mut pc: u32 = 0x8325E3F8;
    'dispatch: loop {
        match pc {
            0x8325E3F8 => {
    //   block [0x8325E3F8..0x8325E430)
	// 8325E3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E40C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325E410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E414: 4AF95945  bl 0x821f3d58
	ctx.lr = 0x8325E418;
	sub_821F3D58(ctx, base);
	// 8325E418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E41C: 906AAE6C  stw r3, -0x5194(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20884 as u32), ctx.r[3].u32 ) };
	// 8325E420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E430 size=56
    let mut pc: u32 = 0x8325E430;
    'dispatch: loop {
        match pc {
            0x8325E430 => {
    //   block [0x8325E430..0x8325E468)
	// 8325E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E444: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325E448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E44C: 4AF9590D  bl 0x821f3d58
	ctx.lr = 0x8325E450;
	sub_821F3D58(ctx, base);
	// 8325E450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E454: 906AAE70  stw r3, -0x5190(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20880 as u32), ctx.r[3].u32 ) };
	// 8325E458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E468 size=56
    let mut pc: u32 = 0x8325E468;
    'dispatch: loop {
        match pc {
            0x8325E468 => {
    //   block [0x8325E468..0x8325E4A0)
	// 8325E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E47C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325E480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E484: 4AF958D5  bl 0x821f3d58
	ctx.lr = 0x8325E488;
	sub_821F3D58(ctx, base);
	// 8325E488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E48C: 906AAE74  stw r3, -0x518c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20876 as u32), ctx.r[3].u32 ) };
	// 8325E490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E4A0 size=56
    let mut pc: u32 = 0x8325E4A0;
    'dispatch: loop {
        match pc {
            0x8325E4A0 => {
    //   block [0x8325E4A0..0x8325E4D8)
	// 8325E4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E4B4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325E4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E4BC: 4AF9589D  bl 0x821f3d58
	ctx.lr = 0x8325E4C0;
	sub_821F3D58(ctx, base);
	// 8325E4C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E4C4: 906AAE78  stw r3, -0x5188(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20872 as u32), ctx.r[3].u32 ) };
	// 8325E4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E4D8 size=56
    let mut pc: u32 = 0x8325E4D8;
    'dispatch: loop {
        match pc {
            0x8325E4D8 => {
    //   block [0x8325E4D8..0x8325E510)
	// 8325E4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E4EC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325E4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E4F4: 4AF95865  bl 0x821f3d58
	ctx.lr = 0x8325E4F8;
	sub_821F3D58(ctx, base);
	// 8325E4F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E4FC: 906AAE7C  stw r3, -0x5184(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20868 as u32), ctx.r[3].u32 ) };
	// 8325E500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E510 size=56
    let mut pc: u32 = 0x8325E510;
    'dispatch: loop {
        match pc {
            0x8325E510 => {
    //   block [0x8325E510..0x8325E548)
	// 8325E510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E524: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325E528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E52C: 4AF9582D  bl 0x821f3d58
	ctx.lr = 0x8325E530;
	sub_821F3D58(ctx, base);
	// 8325E530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E534: 906AAE80  stw r3, -0x5180(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20864 as u32), ctx.r[3].u32 ) };
	// 8325E538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E548 size=56
    let mut pc: u32 = 0x8325E548;
    'dispatch: loop {
        match pc {
            0x8325E548 => {
    //   block [0x8325E548..0x8325E580)
	// 8325E548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E55C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325E560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E564: 4AF957F5  bl 0x821f3d58
	ctx.lr = 0x8325E568;
	sub_821F3D58(ctx, base);
	// 8325E568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E56C: 906AAE84  stw r3, -0x517c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20860 as u32), ctx.r[3].u32 ) };
	// 8325E570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E580 size=56
    let mut pc: u32 = 0x8325E580;
    'dispatch: loop {
        match pc {
            0x8325E580 => {
    //   block [0x8325E580..0x8325E5B8)
	// 8325E580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E594: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325E598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E59C: 4AF957BD  bl 0x821f3d58
	ctx.lr = 0x8325E5A0;
	sub_821F3D58(ctx, base);
	// 8325E5A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E5A4: 906AAE88  stw r3, -0x5178(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20856 as u32), ctx.r[3].u32 ) };
	// 8325E5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E5B8 size=56
    let mut pc: u32 = 0x8325E5B8;
    'dispatch: loop {
        match pc {
            0x8325E5B8 => {
    //   block [0x8325E5B8..0x8325E5F0)
	// 8325E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E5CC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325E5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E5D4: 4AF95785  bl 0x821f3d58
	ctx.lr = 0x8325E5D8;
	sub_821F3D58(ctx, base);
	// 8325E5D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E5DC: 906AAE8C  stw r3, -0x5174(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20852 as u32), ctx.r[3].u32 ) };
	// 8325E5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E5F0 size=56
    let mut pc: u32 = 0x8325E5F0;
    'dispatch: loop {
        match pc {
            0x8325E5F0 => {
    //   block [0x8325E5F0..0x8325E628)
	// 8325E5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E5FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E600: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E604: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325E608: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E60C: 4AF9574D  bl 0x821f3d58
	ctx.lr = 0x8325E610;
	sub_821F3D58(ctx, base);
	// 8325E610: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E614: 906AAE90  stw r3, -0x5170(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20848 as u32), ctx.r[3].u32 ) };
	// 8325E618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E628 size=56
    let mut pc: u32 = 0x8325E628;
    'dispatch: loop {
        match pc {
            0x8325E628 => {
    //   block [0x8325E628..0x8325E660)
	// 8325E628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E634: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E638: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E63C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325E640: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E644: 4AF95715  bl 0x821f3d58
	ctx.lr = 0x8325E648;
	sub_821F3D58(ctx, base);
	// 8325E648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E64C: 906AAE94  stw r3, -0x516c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20844 as u32), ctx.r[3].u32 ) };
	// 8325E650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E660 size=56
    let mut pc: u32 = 0x8325E660;
    'dispatch: loop {
        match pc {
            0x8325E660 => {
    //   block [0x8325E660..0x8325E698)
	// 8325E660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E66C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E670: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E674: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325E678: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E67C: 4AF956DD  bl 0x821f3d58
	ctx.lr = 0x8325E680;
	sub_821F3D58(ctx, base);
	// 8325E680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E684: 906AAE98  stw r3, -0x5168(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20840 as u32), ctx.r[3].u32 ) };
	// 8325E688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E698 size=64
    let mut pc: u32 = 0x8325E698;
    'dispatch: loop {
        match pc {
            0x8325E698 => {
    //   block [0x8325E698..0x8325E6D8)
	// 8325E698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E6A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E6A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E6A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E6AC: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325E6B0: 386AAE9C  addi r3, r10, -0x5164
	ctx.r[3].s64 = ctx.r[10].s64 + -20836;
	// 8325E6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325E6B8: 4AFCE819  bl 0x8222ced0
	ctx.lr = 0x8325E6BC;
	sub_8222CED0(ctx, base);
	// 8325E6BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325E6C0: 3869C600  addi r3, r9, -0x3a00
	ctx.r[3].s64 = ctx.r[9].s64 + -14848;
	// 8325E6C4: 4BA4B85D  bl 0x82ca9f20
	ctx.lr = 0x8325E6C8;
	sub_82CA9F20(ctx, base);
	// 8325E6C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E6CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E6D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E6D8 size=64
    let mut pc: u32 = 0x8325E6D8;
    'dispatch: loop {
        match pc {
            0x8325E6D8 => {
    //   block [0x8325E6D8..0x8325E718)
	// 8325E6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E6E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E6E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325E6E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E6EC: 388BE458  addi r4, r11, -0x1ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -7080;
	// 8325E6F0: 386AAEA0  addi r3, r10, -0x5160
	ctx.r[3].s64 = ctx.r[10].s64 + -20832;
	// 8325E6F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325E6F8: 4AFCE7D9  bl 0x8222ced0
	ctx.lr = 0x8325E6FC;
	sub_8222CED0(ctx, base);
	// 8325E6FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325E700: 3869C610  addi r3, r9, -0x39f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14832;
	// 8325E704: 4BA4B81D  bl 0x82ca9f20
	ctx.lr = 0x8325E708;
	sub_82CA9F20(ctx, base);
	// 8325E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E718 size=56
    let mut pc: u32 = 0x8325E718;
    'dispatch: loop {
        match pc {
            0x8325E718 => {
    //   block [0x8325E718..0x8325E750)
	// 8325E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E72C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E734: 4AF95625  bl 0x821f3d58
	ctx.lr = 0x8325E738;
	sub_821F3D58(ctx, base);
	// 8325E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E73C: 906AAEA4  stw r3, -0x515c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20828 as u32), ctx.r[3].u32 ) };
	// 8325E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E750 size=56
    let mut pc: u32 = 0x8325E750;
    'dispatch: loop {
        match pc {
            0x8325E750 => {
    //   block [0x8325E750..0x8325E788)
	// 8325E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E764: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E76C: 4AF955ED  bl 0x821f3d58
	ctx.lr = 0x8325E770;
	sub_821F3D58(ctx, base);
	// 8325E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E774: 906AAEA8  stw r3, -0x5158(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20824 as u32), ctx.r[3].u32 ) };
	// 8325E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E788 size=56
    let mut pc: u32 = 0x8325E788;
    'dispatch: loop {
        match pc {
            0x8325E788 => {
    //   block [0x8325E788..0x8325E7C0)
	// 8325E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E79C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E7A4: 4AF955B5  bl 0x821f3d58
	ctx.lr = 0x8325E7A8;
	sub_821F3D58(ctx, base);
	// 8325E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E7AC: 906AAEAC  stw r3, -0x5154(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20820 as u32), ctx.r[3].u32 ) };
	// 8325E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E7C0 size=56
    let mut pc: u32 = 0x8325E7C0;
    'dispatch: loop {
        match pc {
            0x8325E7C0 => {
    //   block [0x8325E7C0..0x8325E7F8)
	// 8325E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E7D4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E7DC: 4AF9557D  bl 0x821f3d58
	ctx.lr = 0x8325E7E0;
	sub_821F3D58(ctx, base);
	// 8325E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E7E4: 906AAEB0  stw r3, -0x5150(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20816 as u32), ctx.r[3].u32 ) };
	// 8325E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E7F8 size=56
    let mut pc: u32 = 0x8325E7F8;
    'dispatch: loop {
        match pc {
            0x8325E7F8 => {
    //   block [0x8325E7F8..0x8325E830)
	// 8325E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E80C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E814: 4AF95545  bl 0x821f3d58
	ctx.lr = 0x8325E818;
	sub_821F3D58(ctx, base);
	// 8325E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E81C: 906AAEB4  stw r3, -0x514c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20812 as u32), ctx.r[3].u32 ) };
	// 8325E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


