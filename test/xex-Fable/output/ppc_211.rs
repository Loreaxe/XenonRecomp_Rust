pub fn sub_83254870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254870 size=64
    let mut pc: u32 = 0x83254870;
    'dispatch: loop {
        match pc {
            0x83254870 => {
    //   block [0x83254870..0x832548B0)
	// 83254870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325487C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83254880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254884: 388B921C  addi r4, r11, -0x6de4
	ctx.r[4].s64 = ctx.r[11].s64 + -28132;
	// 83254888: 386A84D8  addi r3, r10, -0x7b28
	ctx.r[3].s64 = ctx.r[10].s64 + -31528;
	// 8325488C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254890: 4AFD8641  bl 0x8222ced0
	ctx.lr = 0x83254894;
	sub_8222CED0(ctx, base);
	// 83254894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254898: 3869A330  addi r3, r9, -0x5cd0
	ctx.r[3].s64 = ctx.r[9].s64 + -23760;
	// 8325489C: 4BA55685  bl 0x82ca9f20
	ctx.lr = 0x832548A0;
	sub_82CA9F20(ctx, base);
	// 832548A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832548A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832548A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832548AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832548B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832548B0 size=64
    let mut pc: u32 = 0x832548B0;
    'dispatch: loop {
        match pc {
            0x832548B0 => {
    //   block [0x832548B0..0x832548F0)
	// 832548B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832548B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832548B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832548BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832548C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832548C4: 388BEC40  addi r4, r11, -0x13c0
	ctx.r[4].s64 = ctx.r[11].s64 + -5056;
	// 832548C8: 386A84DC  addi r3, r10, -0x7b24
	ctx.r[3].s64 = ctx.r[10].s64 + -31524;
	// 832548CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832548D0: 4AFD8601  bl 0x8222ced0
	ctx.lr = 0x832548D4;
	sub_8222CED0(ctx, base);
	// 832548D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832548D8: 3869A340  addi r3, r9, -0x5cc0
	ctx.r[3].s64 = ctx.r[9].s64 + -23744;
	// 832548DC: 4BA55645  bl 0x82ca9f20
	ctx.lr = 0x832548E0;
	sub_82CA9F20(ctx, base);
	// 832548E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832548E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832548E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832548EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832548F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832548F0 size=64
    let mut pc: u32 = 0x832548F0;
    'dispatch: loop {
        match pc {
            0x832548F0 => {
    //   block [0x832548F0..0x83254930)
	// 832548F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832548F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832548F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832548FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254904: 388BF540  addi r4, r11, -0xac0
	ctx.r[4].s64 = ctx.r[11].s64 + -2752;
	// 83254908: 386A84E0  addi r3, r10, -0x7b20
	ctx.r[3].s64 = ctx.r[10].s64 + -31520;
	// 8325490C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254910: 4AFD85C1  bl 0x8222ced0
	ctx.lr = 0x83254914;
	sub_8222CED0(ctx, base);
	// 83254914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254918: 3869A350  addi r3, r9, -0x5cb0
	ctx.r[3].s64 = ctx.r[9].s64 + -23728;
	// 8325491C: 4BA55605  bl 0x82ca9f20
	ctx.lr = 0x83254920;
	sub_82CA9F20(ctx, base);
	// 83254920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325492C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254930 size=64
    let mut pc: u32 = 0x83254930;
    'dispatch: loop {
        match pc {
            0x83254930 => {
    //   block [0x83254930..0x83254970)
	// 83254930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325493C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254944: 388BF548  addi r4, r11, -0xab8
	ctx.r[4].s64 = ctx.r[11].s64 + -2744;
	// 83254948: 386A84E4  addi r3, r10, -0x7b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31516;
	// 8325494C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254950: 4AFD8581  bl 0x8222ced0
	ctx.lr = 0x83254954;
	sub_8222CED0(ctx, base);
	// 83254954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254958: 3869A360  addi r3, r9, -0x5ca0
	ctx.r[3].s64 = ctx.r[9].s64 + -23712;
	// 8325495C: 4BA555C5  bl 0x82ca9f20
	ctx.lr = 0x83254960;
	sub_82CA9F20(ctx, base);
	// 83254960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325496C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254970 size=64
    let mut pc: u32 = 0x83254970;
    'dispatch: loop {
        match pc {
            0x83254970 => {
    //   block [0x83254970..0x832549B0)
	// 83254970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325497C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254984: 388BF550  addi r4, r11, -0xab0
	ctx.r[4].s64 = ctx.r[11].s64 + -2736;
	// 83254988: 386A84E8  addi r3, r10, -0x7b18
	ctx.r[3].s64 = ctx.r[10].s64 + -31512;
	// 8325498C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254990: 4AFD8541  bl 0x8222ced0
	ctx.lr = 0x83254994;
	sub_8222CED0(ctx, base);
	// 83254994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254998: 3869A370  addi r3, r9, -0x5c90
	ctx.r[3].s64 = ctx.r[9].s64 + -23696;
	// 8325499C: 4BA55585  bl 0x82ca9f20
	ctx.lr = 0x832549A0;
	sub_82CA9F20(ctx, base);
	// 832549A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832549A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832549A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832549AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832549B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832549B0 size=64
    let mut pc: u32 = 0x832549B0;
    'dispatch: loop {
        match pc {
            0x832549B0 => {
    //   block [0x832549B0..0x832549F0)
	// 832549B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832549B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832549B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832549BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832549C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832549C4: 388BF558  addi r4, r11, -0xaa8
	ctx.r[4].s64 = ctx.r[11].s64 + -2728;
	// 832549C8: 386A84EC  addi r3, r10, -0x7b14
	ctx.r[3].s64 = ctx.r[10].s64 + -31508;
	// 832549CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832549D0: 4AFD8501  bl 0x8222ced0
	ctx.lr = 0x832549D4;
	sub_8222CED0(ctx, base);
	// 832549D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832549D8: 3869A380  addi r3, r9, -0x5c80
	ctx.r[3].s64 = ctx.r[9].s64 + -23680;
	// 832549DC: 4BA55545  bl 0x82ca9f20
	ctx.lr = 0x832549E0;
	sub_82CA9F20(ctx, base);
	// 832549E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832549E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832549E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832549EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832549F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832549F0 size=64
    let mut pc: u32 = 0x832549F0;
    'dispatch: loop {
        match pc {
            0x832549F0 => {
    //   block [0x832549F0..0x83254A30)
	// 832549F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832549F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832549F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832549FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254A04: 388BF560  addi r4, r11, -0xaa0
	ctx.r[4].s64 = ctx.r[11].s64 + -2720;
	// 83254A08: 386A84F0  addi r3, r10, -0x7b10
	ctx.r[3].s64 = ctx.r[10].s64 + -31504;
	// 83254A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254A10: 4AFD84C1  bl 0x8222ced0
	ctx.lr = 0x83254A14;
	sub_8222CED0(ctx, base);
	// 83254A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254A18: 3869A390  addi r3, r9, -0x5c70
	ctx.r[3].s64 = ctx.r[9].s64 + -23664;
	// 83254A1C: 4BA55505  bl 0x82ca9f20
	ctx.lr = 0x83254A20;
	sub_82CA9F20(ctx, base);
	// 83254A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254A30 size=64
    let mut pc: u32 = 0x83254A30;
    'dispatch: loop {
        match pc {
            0x83254A30 => {
    //   block [0x83254A30..0x83254A70)
	// 83254A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254A3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254A44: 388BF568  addi r4, r11, -0xa98
	ctx.r[4].s64 = ctx.r[11].s64 + -2712;
	// 83254A48: 386A84F4  addi r3, r10, -0x7b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31500;
	// 83254A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254A50: 4AFD8481  bl 0x8222ced0
	ctx.lr = 0x83254A54;
	sub_8222CED0(ctx, base);
	// 83254A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254A58: 3869A3A0  addi r3, r9, -0x5c60
	ctx.r[3].s64 = ctx.r[9].s64 + -23648;
	// 83254A5C: 4BA554C5  bl 0x82ca9f20
	ctx.lr = 0x83254A60;
	sub_82CA9F20(ctx, base);
	// 83254A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254A70 size=64
    let mut pc: u32 = 0x83254A70;
    'dispatch: loop {
        match pc {
            0x83254A70 => {
    //   block [0x83254A70..0x83254AB0)
	// 83254A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254A7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254A84: 388BF570  addi r4, r11, -0xa90
	ctx.r[4].s64 = ctx.r[11].s64 + -2704;
	// 83254A88: 386A84F8  addi r3, r10, -0x7b08
	ctx.r[3].s64 = ctx.r[10].s64 + -31496;
	// 83254A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254A90: 4AFD8441  bl 0x8222ced0
	ctx.lr = 0x83254A94;
	sub_8222CED0(ctx, base);
	// 83254A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254A98: 3869A3B0  addi r3, r9, -0x5c50
	ctx.r[3].s64 = ctx.r[9].s64 + -23632;
	// 83254A9C: 4BA55485  bl 0x82ca9f20
	ctx.lr = 0x83254AA0;
	sub_82CA9F20(ctx, base);
	// 83254AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254AB0 size=64
    let mut pc: u32 = 0x83254AB0;
    'dispatch: loop {
        match pc {
            0x83254AB0 => {
    //   block [0x83254AB0..0x83254AF0)
	// 83254AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254ABC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254AC4: 388BF580  addi r4, r11, -0xa80
	ctx.r[4].s64 = ctx.r[11].s64 + -2688;
	// 83254AC8: 386A84FC  addi r3, r10, -0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + -31492;
	// 83254ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254AD0: 4AFD8401  bl 0x8222ced0
	ctx.lr = 0x83254AD4;
	sub_8222CED0(ctx, base);
	// 83254AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254AD8: 3869A3C0  addi r3, r9, -0x5c40
	ctx.r[3].s64 = ctx.r[9].s64 + -23616;
	// 83254ADC: 4BA55445  bl 0x82ca9f20
	ctx.lr = 0x83254AE0;
	sub_82CA9F20(ctx, base);
	// 83254AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254AF0 size=64
    let mut pc: u32 = 0x83254AF0;
    'dispatch: loop {
        match pc {
            0x83254AF0 => {
    //   block [0x83254AF0..0x83254B30)
	// 83254AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254AFC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254B04: 388BF590  addi r4, r11, -0xa70
	ctx.r[4].s64 = ctx.r[11].s64 + -2672;
	// 83254B08: 386A8500  addi r3, r10, -0x7b00
	ctx.r[3].s64 = ctx.r[10].s64 + -31488;
	// 83254B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254B10: 4AFD83C1  bl 0x8222ced0
	ctx.lr = 0x83254B14;
	sub_8222CED0(ctx, base);
	// 83254B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254B18: 3869A3D0  addi r3, r9, -0x5c30
	ctx.r[3].s64 = ctx.r[9].s64 + -23600;
	// 83254B1C: 4BA55405  bl 0x82ca9f20
	ctx.lr = 0x83254B20;
	sub_82CA9F20(ctx, base);
	// 83254B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254B30 size=64
    let mut pc: u32 = 0x83254B30;
    'dispatch: loop {
        match pc {
            0x83254B30 => {
    //   block [0x83254B30..0x83254B70)
	// 83254B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254B3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254B44: 388BF598  addi r4, r11, -0xa68
	ctx.r[4].s64 = ctx.r[11].s64 + -2664;
	// 83254B48: 386A8504  addi r3, r10, -0x7afc
	ctx.r[3].s64 = ctx.r[10].s64 + -31484;
	// 83254B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254B50: 4AFD8381  bl 0x8222ced0
	ctx.lr = 0x83254B54;
	sub_8222CED0(ctx, base);
	// 83254B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254B58: 3869A3E0  addi r3, r9, -0x5c20
	ctx.r[3].s64 = ctx.r[9].s64 + -23584;
	// 83254B5C: 4BA553C5  bl 0x82ca9f20
	ctx.lr = 0x83254B60;
	sub_82CA9F20(ctx, base);
	// 83254B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254B70 size=64
    let mut pc: u32 = 0x83254B70;
    'dispatch: loop {
        match pc {
            0x83254B70 => {
    //   block [0x83254B70..0x83254BB0)
	// 83254B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254B7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254B84: 388BF5A0  addi r4, r11, -0xa60
	ctx.r[4].s64 = ctx.r[11].s64 + -2656;
	// 83254B88: 386A8508  addi r3, r10, -0x7af8
	ctx.r[3].s64 = ctx.r[10].s64 + -31480;
	// 83254B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254B90: 4AFD8341  bl 0x8222ced0
	ctx.lr = 0x83254B94;
	sub_8222CED0(ctx, base);
	// 83254B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254B98: 3869A3F0  addi r3, r9, -0x5c10
	ctx.r[3].s64 = ctx.r[9].s64 + -23568;
	// 83254B9C: 4BA55385  bl 0x82ca9f20
	ctx.lr = 0x83254BA0;
	sub_82CA9F20(ctx, base);
	// 83254BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254BB0 size=64
    let mut pc: u32 = 0x83254BB0;
    'dispatch: loop {
        match pc {
            0x83254BB0 => {
    //   block [0x83254BB0..0x83254BF0)
	// 83254BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254BBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254BC4: 388BF5AC  addi r4, r11, -0xa54
	ctx.r[4].s64 = ctx.r[11].s64 + -2644;
	// 83254BC8: 386A850C  addi r3, r10, -0x7af4
	ctx.r[3].s64 = ctx.r[10].s64 + -31476;
	// 83254BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254BD0: 4AFD8301  bl 0x8222ced0
	ctx.lr = 0x83254BD4;
	sub_8222CED0(ctx, base);
	// 83254BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254BD8: 3869A400  addi r3, r9, -0x5c00
	ctx.r[3].s64 = ctx.r[9].s64 + -23552;
	// 83254BDC: 4BA55345  bl 0x82ca9f20
	ctx.lr = 0x83254BE0;
	sub_82CA9F20(ctx, base);
	// 83254BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254BF0 size=64
    let mut pc: u32 = 0x83254BF0;
    'dispatch: loop {
        match pc {
            0x83254BF0 => {
    //   block [0x83254BF0..0x83254C30)
	// 83254BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254C04: 388B4A24  addi r4, r11, 0x4a24
	ctx.r[4].s64 = ctx.r[11].s64 + 18980;
	// 83254C08: 386A8510  addi r3, r10, -0x7af0
	ctx.r[3].s64 = ctx.r[10].s64 + -31472;
	// 83254C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254C10: 4AFD82C1  bl 0x8222ced0
	ctx.lr = 0x83254C14;
	sub_8222CED0(ctx, base);
	// 83254C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254C18: 3869A410  addi r3, r9, -0x5bf0
	ctx.r[3].s64 = ctx.r[9].s64 + -23536;
	// 83254C1C: 4BA55305  bl 0x82ca9f20
	ctx.lr = 0x83254C20;
	sub_82CA9F20(ctx, base);
	// 83254C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254C30 size=64
    let mut pc: u32 = 0x83254C30;
    'dispatch: loop {
        match pc {
            0x83254C30 => {
    //   block [0x83254C30..0x83254C70)
	// 83254C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254C3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254C44: 388BF5B4  addi r4, r11, -0xa4c
	ctx.r[4].s64 = ctx.r[11].s64 + -2636;
	// 83254C48: 386A8514  addi r3, r10, -0x7aec
	ctx.r[3].s64 = ctx.r[10].s64 + -31468;
	// 83254C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254C50: 4AFD8281  bl 0x8222ced0
	ctx.lr = 0x83254C54;
	sub_8222CED0(ctx, base);
	// 83254C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254C58: 3869A420  addi r3, r9, -0x5be0
	ctx.r[3].s64 = ctx.r[9].s64 + -23520;
	// 83254C5C: 4BA552C5  bl 0x82ca9f20
	ctx.lr = 0x83254C60;
	sub_82CA9F20(ctx, base);
	// 83254C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254C70 size=64
    let mut pc: u32 = 0x83254C70;
    'dispatch: loop {
        match pc {
            0x83254C70 => {
    //   block [0x83254C70..0x83254CB0)
	// 83254C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254C7C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83254C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254C84: 388B921C  addi r4, r11, -0x6de4
	ctx.r[4].s64 = ctx.r[11].s64 + -28132;
	// 83254C88: 386A8518  addi r3, r10, -0x7ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -31464;
	// 83254C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254C90: 4AFD8241  bl 0x8222ced0
	ctx.lr = 0x83254C94;
	sub_8222CED0(ctx, base);
	// 83254C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254C98: 3869A430  addi r3, r9, -0x5bd0
	ctx.r[3].s64 = ctx.r[9].s64 + -23504;
	// 83254C9C: 4BA55285  bl 0x82ca9f20
	ctx.lr = 0x83254CA0;
	sub_82CA9F20(ctx, base);
	// 83254CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254CB0 size=64
    let mut pc: u32 = 0x83254CB0;
    'dispatch: loop {
        match pc {
            0x83254CB0 => {
    //   block [0x83254CB0..0x83254CF0)
	// 83254CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254CBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254CC4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83254CC8: 386A851C  addi r3, r10, -0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -31460;
	// 83254CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254CD0: 4AFD8201  bl 0x8222ced0
	ctx.lr = 0x83254CD4;
	sub_8222CED0(ctx, base);
	// 83254CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254CD8: 3869A4A8  addi r3, r9, -0x5b58
	ctx.r[3].s64 = ctx.r[9].s64 + -23384;
	// 83254CDC: 4BA55245  bl 0x82ca9f20
	ctx.lr = 0x83254CE0;
	sub_82CA9F20(ctx, base);
	// 83254CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254CF0 size=64
    let mut pc: u32 = 0x83254CF0;
    'dispatch: loop {
        match pc {
            0x83254CF0 => {
    //   block [0x83254CF0..0x83254D30)
	// 83254CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254D04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83254D08: 386A8520  addi r3, r10, -0x7ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -31456;
	// 83254D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254D10: 4AFD81C1  bl 0x8222ced0
	ctx.lr = 0x83254D14;
	sub_8222CED0(ctx, base);
	// 83254D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254D18: 3869A4B8  addi r3, r9, -0x5b48
	ctx.r[3].s64 = ctx.r[9].s64 + -23368;
	// 83254D1C: 4BA55205  bl 0x82ca9f20
	ctx.lr = 0x83254D20;
	sub_82CA9F20(ctx, base);
	// 83254D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254D30 size=64
    let mut pc: u32 = 0x83254D30;
    'dispatch: loop {
        match pc {
            0x83254D30 => {
    //   block [0x83254D30..0x83254D70)
	// 83254D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254D3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254D44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254D48: 386A8524  addi r3, r10, -0x7adc
	ctx.r[3].s64 = ctx.r[10].s64 + -31452;
	// 83254D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254D50: 4AFD8181  bl 0x8222ced0
	ctx.lr = 0x83254D54;
	sub_8222CED0(ctx, base);
	// 83254D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254D58: 3869A4C8  addi r3, r9, -0x5b38
	ctx.r[3].s64 = ctx.r[9].s64 + -23352;
	// 83254D5C: 4BA551C5  bl 0x82ca9f20
	ctx.lr = 0x83254D60;
	sub_82CA9F20(ctx, base);
	// 83254D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254D70 size=64
    let mut pc: u32 = 0x83254D70;
    'dispatch: loop {
        match pc {
            0x83254D70 => {
    //   block [0x83254D70..0x83254DB0)
	// 83254D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254D7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254D84: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83254D88: 386A8528  addi r3, r10, -0x7ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -31448;
	// 83254D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254D90: 4AFD8141  bl 0x8222ced0
	ctx.lr = 0x83254D94;
	sub_8222CED0(ctx, base);
	// 83254D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254D98: 3869A4D8  addi r3, r9, -0x5b28
	ctx.r[3].s64 = ctx.r[9].s64 + -23336;
	// 83254D9C: 4BA55185  bl 0x82ca9f20
	ctx.lr = 0x83254DA0;
	sub_82CA9F20(ctx, base);
	// 83254DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254DB0 size=64
    let mut pc: u32 = 0x83254DB0;
    'dispatch: loop {
        match pc {
            0x83254DB0 => {
    //   block [0x83254DB0..0x83254DF0)
	// 83254DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254DC4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83254DC8: 386A852C  addi r3, r10, -0x7ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -31444;
	// 83254DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254DD0: 4AFD8101  bl 0x8222ced0
	ctx.lr = 0x83254DD4;
	sub_8222CED0(ctx, base);
	// 83254DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254DD8: 3869A4E8  addi r3, r9, -0x5b18
	ctx.r[3].s64 = ctx.r[9].s64 + -23320;
	// 83254DDC: 4BA55145  bl 0x82ca9f20
	ctx.lr = 0x83254DE0;
	sub_82CA9F20(ctx, base);
	// 83254DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254DF0 size=64
    let mut pc: u32 = 0x83254DF0;
    'dispatch: loop {
        match pc {
            0x83254DF0 => {
    //   block [0x83254DF0..0x83254E30)
	// 83254DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254DFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254E04: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254E08: 386A8530  addi r3, r10, -0x7ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -31440;
	// 83254E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254E10: 4AFD80C1  bl 0x8222ced0
	ctx.lr = 0x83254E14;
	sub_8222CED0(ctx, base);
	// 83254E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254E18: 3869A4F8  addi r3, r9, -0x5b08
	ctx.r[3].s64 = ctx.r[9].s64 + -23304;
	// 83254E1C: 4BA55105  bl 0x82ca9f20
	ctx.lr = 0x83254E20;
	sub_82CA9F20(ctx, base);
	// 83254E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254E30 size=56
    let mut pc: u32 = 0x83254E30;
    'dispatch: loop {
        match pc {
            0x83254E30 => {
    //   block [0x83254E30..0x83254E68)
	// 83254E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254E3C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254E40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83254E44: 386BFD58  addi r3, r11, -0x2a8
	ctx.r[3].s64 = ctx.r[11].s64 + -680;
	// 83254E48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254E4C: 4AF9EF0D  bl 0x821f3d58
	ctx.lr = 0x83254E50;
	sub_821F3D58(ctx, base);
	// 83254E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254E54: 906A8534  stw r3, -0x7acc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31436 as u32), ctx.r[3].u32 ) };
	// 83254E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254E68 size=56
    let mut pc: u32 = 0x83254E68;
    'dispatch: loop {
        match pc {
            0x83254E68 => {
    //   block [0x83254E68..0x83254EA0)
	// 83254E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254E74: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254E78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83254E7C: 386BFD68  addi r3, r11, -0x298
	ctx.r[3].s64 = ctx.r[11].s64 + -664;
	// 83254E80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254E84: 4AF9EED5  bl 0x821f3d58
	ctx.lr = 0x83254E88;
	sub_821F3D58(ctx, base);
	// 83254E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254E8C: 906A8538  stw r3, -0x7ac8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31432 as u32), ctx.r[3].u32 ) };
	// 83254E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83254EA0 size=72
    let mut pc: u32 = 0x83254EA0;
    'dispatch: loop {
        match pc {
            0x83254EA0 => {
    //   block [0x83254EA0..0x83254EA8)
	// 83254EA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83254EA4: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x83254EA8; continue 'dispatch;
            }
            0x83254EA8 => {
    //   block [0x83254EA8..0x83254EE8)
	// 83254EA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83254EAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254EB0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83254EB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83254EB8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83254EBC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254EC0: 4082FFE8  bne 0x83254ea8
	if !ctx.cr[0].eq {
	pc = 0x83254EA8; continue 'dispatch;
	}
	// 83254EC4: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 83254EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83254ECC: 38C7853C  addi r6, r7, -0x7ac4
	ctx.r[6].s64 = ctx.r[7].s64 + -31428;
	// 83254ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83254ED4: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83254ED8: 3865A508  addi r3, r5, -0x5af8
	ctx.r[3].s64 = ctx.r[5].s64 + -23288;
	// 83254EDC: 9166000C  stw r11, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83254EE0: 99460010  stb r10, 0x10(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 83254EE4: 4BA5503C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83254EE8 size=12
    let mut pc: u32 = 0x83254EE8;
    'dispatch: loop {
        match pc {
            0x83254EE8 => {
    //   block [0x83254EE8..0x83254EF4)
	// 83254EE8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83254EEC: 386BA580  addi r3, r11, -0x5a80
	ctx.r[3].s64 = ctx.r[11].s64 + -23168;
	// 83254EF0: 4BA55030  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254EF8 size=64
    let mut pc: u32 = 0x83254EF8;
    'dispatch: loop {
        match pc {
            0x83254EF8 => {
    //   block [0x83254EF8..0x83254F38)
	// 83254EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254F0C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83254F10: 386A8558  addi r3, r10, -0x7aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -31400;
	// 83254F14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254F18: 4AFD7FB9  bl 0x8222ced0
	ctx.lr = 0x83254F1C;
	sub_8222CED0(ctx, base);
	// 83254F1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254F20: 3869A590  addi r3, r9, -0x5a70
	ctx.r[3].s64 = ctx.r[9].s64 + -23152;
	// 83254F24: 4BA54FFD  bl 0x82ca9f20
	ctx.lr = 0x83254F28;
	sub_82CA9F20(ctx, base);
	// 83254F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254F38 size=64
    let mut pc: u32 = 0x83254F38;
    'dispatch: loop {
        match pc {
            0x83254F38 => {
    //   block [0x83254F38..0x83254F78)
	// 83254F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254F48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254F4C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83254F50: 386A855C  addi r3, r10, -0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -31396;
	// 83254F54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254F58: 4AFD7F79  bl 0x8222ced0
	ctx.lr = 0x83254F5C;
	sub_8222CED0(ctx, base);
	// 83254F5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254F60: 3869A5A0  addi r3, r9, -0x5a60
	ctx.r[3].s64 = ctx.r[9].s64 + -23136;
	// 83254F64: 4BA54FBD  bl 0x82ca9f20
	ctx.lr = 0x83254F68;
	sub_82CA9F20(ctx, base);
	// 83254F68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254F78 size=64
    let mut pc: u32 = 0x83254F78;
    'dispatch: loop {
        match pc {
            0x83254F78 => {
    //   block [0x83254F78..0x83254FB8)
	// 83254F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254F84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254F88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254F8C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254F90: 386A8560  addi r3, r10, -0x7aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -31392;
	// 83254F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254F98: 4AFD7F39  bl 0x8222ced0
	ctx.lr = 0x83254F9C;
	sub_8222CED0(ctx, base);
	// 83254F9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254FA0: 3869A5B0  addi r3, r9, -0x5a50
	ctx.r[3].s64 = ctx.r[9].s64 + -23120;
	// 83254FA4: 4BA54F7D  bl 0x82ca9f20
	ctx.lr = 0x83254FA8;
	sub_82CA9F20(ctx, base);
	// 83254FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254FB8 size=848
    let mut pc: u32 = 0x83254FB8;
    'dispatch: loop {
        match pc {
            0x83254FB8 => {
    //   block [0x83254FB8..0x83255308)
	// 83254FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83254FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83254FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254FCC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83254FD0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83254FD4: 3BEB8568  addi r31, r11, -0x7a98
	ctx.r[31].s64 = ctx.r[11].s64 + -31384;
	// 83254FD8: 388A0840  addi r4, r10, 0x840
	ctx.r[4].s64 = ctx.r[10].s64 + 2112;
	// 83254FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83254FE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254FE4: 4AFD7EED  bl 0x8222ced0
	ctx.lr = 0x83254FE8;
	sub_8222CED0(ctx, base);
	// 83254FE8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83254FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254FF0: 3BC90CA0  addi r30, r9, 0xca0
	ctx.r[30].s64 = ctx.r[9].s64 + 3232;
	// 83254FF4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83254FF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83254FFC: 4AFD7ED5  bl 0x8222ced0
	ctx.lr = 0x83255000;
	sub_8222CED0(ctx, base);
	// 83255000: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255008: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8325500C: 4AFD7EC5  bl 0x8222ced0
	ctx.lr = 0x83255010;
	sub_8222CED0(ctx, base);
	// 83255010: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83255014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255018: 38880830  addi r4, r8, 0x830
	ctx.r[4].s64 = ctx.r[8].s64 + 2096;
	// 8325501C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83255020: 4AFD7EB1  bl 0x8222ced0
	ctx.lr = 0x83255024;
	sub_8222CED0(ctx, base);
	// 83255024: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83255028: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325502C: 3887081C  addi r4, r7, 0x81c
	ctx.r[4].s64 = ctx.r[7].s64 + 2076;
	// 83255030: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83255034: 4AFD7E9D  bl 0x8222ced0
	ctx.lr = 0x83255038;
	sub_8222CED0(ctx, base);
	// 83255038: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325503C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255040: 3886080C  addi r4, r6, 0x80c
	ctx.r[4].s64 = ctx.r[6].s64 + 2060;
	// 83255044: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83255048: 4AFD7E89  bl 0x8222ced0
	ctx.lr = 0x8325504C;
	sub_8222CED0(ctx, base);
	// 8325504C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255050: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255054: 388407FC  addi r4, r4, 0x7fc
	ctx.r[4].s64 = ctx.r[4].s64 + 2044;
	// 83255058: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8325505C: 4AFD7E75  bl 0x8222ced0
	ctx.lr = 0x83255060;
	sub_8222CED0(ctx, base);
	// 83255060: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83255064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255068: 388307E8  addi r4, r3, 0x7e8
	ctx.r[4].s64 = ctx.r[3].s64 + 2024;
	// 8325506C: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83255070: 4AFD7E61  bl 0x8222ced0
	ctx.lr = 0x83255074;
	sub_8222CED0(ctx, base);
	// 83255074: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325507C: 388B07D8  addi r4, r11, 0x7d8
	ctx.r[4].s64 = ctx.r[11].s64 + 2008;
	// 83255080: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83255084: 4AFD7E4D  bl 0x8222ced0
	ctx.lr = 0x83255088;
	sub_8222CED0(ctx, base);
	// 83255088: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325508C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255090: 388A07C4  addi r4, r10, 0x7c4
	ctx.r[4].s64 = ctx.r[10].s64 + 1988;
	// 83255094: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83255098: 4AFD7E39  bl 0x8222ced0
	ctx.lr = 0x8325509C;
	sub_8222CED0(ctx, base);
	// 8325509C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832550A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550A4: 388907AC  addi r4, r9, 0x7ac
	ctx.r[4].s64 = ctx.r[9].s64 + 1964;
	// 832550A8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832550AC: 4AFD7E25  bl 0x8222ced0
	ctx.lr = 0x832550B0;
	sub_8222CED0(ctx, base);
	// 832550B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832550B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550B8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832550BC: 4AFD7E15  bl 0x8222ced0
	ctx.lr = 0x832550C0;
	sub_8222CED0(ctx, base);
	// 832550C0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832550C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550C8: 38880798  addi r4, r8, 0x798
	ctx.r[4].s64 = ctx.r[8].s64 + 1944;
	// 832550CC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832550D0: 4AFD7E01  bl 0x8222ced0
	ctx.lr = 0x832550D4;
	sub_8222CED0(ctx, base);
	// 832550D4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832550D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550DC: 38870780  addi r4, r7, 0x780
	ctx.r[4].s64 = ctx.r[7].s64 + 1920;
	// 832550E0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832550E4: 4AFD7DED  bl 0x8222ced0
	ctx.lr = 0x832550E8;
	sub_8222CED0(ctx, base);
	// 832550E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832550EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832550F0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832550F4: 4AFD7DDD  bl 0x8222ced0
	ctx.lr = 0x832550F8;
	sub_8222CED0(ctx, base);
	// 832550F8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832550FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255100: 38860768  addi r4, r6, 0x768
	ctx.r[4].s64 = ctx.r[6].s64 + 1896;
	// 83255104: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83255108: 4AFD7DC9  bl 0x8222ced0
	ctx.lr = 0x8325510C;
	sub_8222CED0(ctx, base);
	// 8325510C: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83255110: 38850754  addi r4, r5, 0x754
	ctx.r[4].s64 = ctx.r[5].s64 + 1876;
	// 83255114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255118: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8325511C: 4AFD7DB5  bl 0x8222ced0
	ctx.lr = 0x83255120;
	sub_8222CED0(ctx, base);
	// 83255120: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255124: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255128: 38840748  addi r4, r4, 0x748
	ctx.r[4].s64 = ctx.r[4].s64 + 1864;
	// 8325512C: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83255130: 4AFD7DA1  bl 0x8222ced0
	ctx.lr = 0x83255134;
	sub_8222CED0(ctx, base);
	// 83255134: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83255138: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325513C: 38830738  addi r4, r3, 0x738
	ctx.r[4].s64 = ctx.r[3].s64 + 1848;
	// 83255140: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83255144: 4AFD7D8D  bl 0x8222ced0
	ctx.lr = 0x83255148;
	sub_8222CED0(ctx, base);
	// 83255148: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8325514C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255150: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83255154: 4AFD7D7D  bl 0x8222ced0
	ctx.lr = 0x83255158;
	sub_8222CED0(ctx, base);
	// 83255158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8325515C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255160: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83255164: 4AFD7D6D  bl 0x8222ced0
	ctx.lr = 0x83255168;
	sub_8222CED0(ctx, base);
	// 83255168: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8325516C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255170: 388B0728  addi r4, r11, 0x728
	ctx.r[4].s64 = ctx.r[11].s64 + 1832;
	// 83255174: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83255178: 4AFD7D59  bl 0x8222ced0
	ctx.lr = 0x8325517C;
	sub_8222CED0(ctx, base);
	// 8325517C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255180: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255184: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83255188: 4AFD7D49  bl 0x8222ced0
	ctx.lr = 0x8325518C;
	sub_8222CED0(ctx, base);
	// 8325518C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255190: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255194: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83255198: 4AFD7D39  bl 0x8222ced0
	ctx.lr = 0x8325519C;
	sub_8222CED0(ctx, base);
	// 8325519C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832551A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551A4: 388A0718  addi r4, r10, 0x718
	ctx.r[4].s64 = ctx.r[10].s64 + 1816;
	// 832551A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 832551AC: 4AFD7D25  bl 0x8222ced0
	ctx.lr = 0x832551B0;
	sub_8222CED0(ctx, base);
	// 832551B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551B8: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 832551BC: 4AFD7D15  bl 0x8222ced0
	ctx.lr = 0x832551C0;
	sub_8222CED0(ctx, base);
	// 832551C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551C8: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 832551CC: 4AFD7D05  bl 0x8222ced0
	ctx.lr = 0x832551D0;
	sub_8222CED0(ctx, base);
	// 832551D0: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832551D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551D8: 38890708  addi r4, r9, 0x708
	ctx.r[4].s64 = ctx.r[9].s64 + 1800;
	// 832551DC: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 832551E0: 4AFD7CF1  bl 0x8222ced0
	ctx.lr = 0x832551E4;
	sub_8222CED0(ctx, base);
	// 832551E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551EC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 832551F0: 4AFD7CE1  bl 0x8222ced0
	ctx.lr = 0x832551F4;
	sub_8222CED0(ctx, base);
	// 832551F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832551F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832551FC: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 83255200: 4AFD7CD1  bl 0x8222ced0
	ctx.lr = 0x83255204;
	sub_8222CED0(ctx, base);
	// 83255204: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 83255208: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325520C: 388813B0  addi r4, r8, 0x13b0
	ctx.r[4].s64 = ctx.r[8].s64 + 5040;
	// 83255210: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83255214: 4AFD7CBD  bl 0x8222ced0
	ctx.lr = 0x83255218;
	sub_8222CED0(ctx, base);
	// 83255218: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8325521C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255220: 388706F4  addi r4, r7, 0x6f4
	ctx.r[4].s64 = ctx.r[7].s64 + 1780;
	// 83255224: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83255228: 4AFD7CA9  bl 0x8222ced0
	ctx.lr = 0x8325522C;
	sub_8222CED0(ctx, base);
	// 8325522C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255230: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255234: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83255238: 4AFD7C99  bl 0x8222ced0
	ctx.lr = 0x8325523C;
	sub_8222CED0(ctx, base);
	// 8325523C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 83255240: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255244: 388613A0  addi r4, r6, 0x13a0
	ctx.r[4].s64 = ctx.r[6].s64 + 5024;
	// 83255248: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 8325524C: 4AFD7C85  bl 0x8222ced0
	ctx.lr = 0x83255250;
	sub_8222CED0(ctx, base);
	// 83255250: 3CA0820C  lis r5, -0x7df4
	ctx.r[5].s64 = -2113142784;
	// 83255254: 388506E0  addi r4, r5, 0x6e0
	ctx.r[4].s64 = ctx.r[5].s64 + 1760;
	// 83255258: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325525C: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83255260: 4AFD7C71  bl 0x8222ced0
	ctx.lr = 0x83255264;
	sub_8222CED0(ctx, base);
	// 83255264: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83255268: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325526C: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 83255270: 4AFD7C61  bl 0x8222ced0
	ctx.lr = 0x83255274;
	sub_8222CED0(ctx, base);
	// 83255274: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83255278: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325527C: 388406CC  addi r4, r4, 0x6cc
	ctx.r[4].s64 = ctx.r[4].s64 + 1740;
	// 83255280: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83255284: 4AFD7C4D  bl 0x8222ced0
	ctx.lr = 0x83255288;
	sub_8222CED0(ctx, base);
	// 83255288: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 8325528C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255290: 388306B4  addi r4, r3, 0x6b4
	ctx.r[4].s64 = ctx.r[3].s64 + 1716;
	// 83255294: 387F0094  addi r3, r31, 0x94
	ctx.r[3].s64 = ctx.r[31].s64 + 148;
	// 83255298: 4AFD7C39  bl 0x8222ced0
	ctx.lr = 0x8325529C;
	sub_8222CED0(ctx, base);
	// 8325529C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832552A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552A4: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 832552A8: 4AFD7C29  bl 0x8222ced0
	ctx.lr = 0x832552AC;
	sub_8222CED0(ctx, base);
	// 832552AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832552B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552B4: 388B06A0  addi r4, r11, 0x6a0
	ctx.r[4].s64 = ctx.r[11].s64 + 1696;
	// 832552B8: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 832552BC: 4AFD7C15  bl 0x8222ced0
	ctx.lr = 0x832552C0;
	sub_8222CED0(ctx, base);
	// 832552C0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832552C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552C8: 388A0688  addi r4, r10, 0x688
	ctx.r[4].s64 = ctx.r[10].s64 + 1672;
	// 832552CC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 832552D0: 4AFD7C01  bl 0x8222ced0
	ctx.lr = 0x832552D4;
	sub_8222CED0(ctx, base);
	// 832552D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832552D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832552DC: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 832552E0: 4AFD7BF1  bl 0x8222ced0
	ctx.lr = 0x832552E4;
	sub_8222CED0(ctx, base);
	// 832552E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832552E8: 3869A5C0  addi r3, r9, -0x5a40
	ctx.r[3].s64 = ctx.r[9].s64 + -23104;
	// 832552EC: 4BA54C35  bl 0x82ca9f20
	ctx.lr = 0x832552F0;
	sub_82CA9F20(ctx, base);
	// 832552F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832552F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832552F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832552FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83255300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83255304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255308 size=64
    let mut pc: u32 = 0x83255308;
    'dispatch: loop {
        match pc {
            0x83255308 => {
    //   block [0x83255308..0x83255348)
	// 83255308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325530C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325531C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83255320: 386A8610  addi r3, r10, -0x79f0
	ctx.r[3].s64 = ctx.r[10].s64 + -31216;
	// 83255324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255328: 4AFD7BA9  bl 0x8222ced0
	ctx.lr = 0x8325532C;
	sub_8222CED0(ctx, base);
	// 8325532C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255330: 3869A628  addi r3, r9, -0x59d8
	ctx.r[3].s64 = ctx.r[9].s64 + -23000;
	// 83255334: 4BA54BED  bl 0x82ca9f20
	ctx.lr = 0x83255338;
	sub_82CA9F20(ctx, base);
	// 83255338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255348 size=64
    let mut pc: u32 = 0x83255348;
    'dispatch: loop {
        match pc {
            0x83255348 => {
    //   block [0x83255348..0x83255388)
	// 83255348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255354: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325535C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83255360: 386A8614  addi r3, r10, -0x79ec
	ctx.r[3].s64 = ctx.r[10].s64 + -31212;
	// 83255364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255368: 4AFD7B69  bl 0x8222ced0
	ctx.lr = 0x8325536C;
	sub_8222CED0(ctx, base);
	// 8325536C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255370: 3869A638  addi r3, r9, -0x59c8
	ctx.r[3].s64 = ctx.r[9].s64 + -22984;
	// 83255374: 4BA54BAD  bl 0x82ca9f20
	ctx.lr = 0x83255378;
	sub_82CA9F20(ctx, base);
	// 83255378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325537C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255388 size=64
    let mut pc: u32 = 0x83255388;
    'dispatch: loop {
        match pc {
            0x83255388 => {
    //   block [0x83255388..0x832553C8)
	// 83255388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325539C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832553A0: 386A8618  addi r3, r10, -0x79e8
	ctx.r[3].s64 = ctx.r[10].s64 + -31208;
	// 832553A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832553A8: 4AFD7B29  bl 0x8222ced0
	ctx.lr = 0x832553AC;
	sub_8222CED0(ctx, base);
	// 832553AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832553B0: 3869A648  addi r3, r9, -0x59b8
	ctx.r[3].s64 = ctx.r[9].s64 + -22968;
	// 832553B4: 4BA54B6D  bl 0x82ca9f20
	ctx.lr = 0x832553B8;
	sub_82CA9F20(ctx, base);
	// 832553B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832553BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832553C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832553C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832553C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832553C8 size=56
    let mut pc: u32 = 0x832553C8;
    'dispatch: loop {
        match pc {
            0x832553C8 => {
    //   block [0x832553C8..0x83255400)
	// 832553C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832553CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832553D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832553D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832553D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832553DC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832553E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832553E4: 4AF9E975  bl 0x821f3d58
	ctx.lr = 0x832553E8;
	sub_821F3D58(ctx, base);
	// 832553E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832553EC: 906A861C  stw r3, -0x79e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31204 as u32), ctx.r[3].u32 ) };
	// 832553F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832553F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832553F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832553FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255400 size=56
    let mut pc: u32 = 0x83255400;
    'dispatch: loop {
        match pc {
            0x83255400 => {
    //   block [0x83255400..0x83255438)
	// 83255400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325540C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255410: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255414: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83255418: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325541C: 4AF9E93D  bl 0x821f3d58
	ctx.lr = 0x83255420;
	sub_821F3D58(ctx, base);
	// 83255420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255424: 906A8620  stw r3, -0x79e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31200 as u32), ctx.r[3].u32 ) };
	// 83255428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325542C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255438 size=56
    let mut pc: u32 = 0x83255438;
    'dispatch: loop {
        match pc {
            0x83255438 => {
    //   block [0x83255438..0x83255470)
	// 83255438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255448: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325544C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83255450: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255454: 4AF9E905  bl 0x821f3d58
	ctx.lr = 0x83255458;
	sub_821F3D58(ctx, base);
	// 83255458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325545C: 906A8624  stw r3, -0x79dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31196 as u32), ctx.r[3].u32 ) };
	// 83255460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255470 size=56
    let mut pc: u32 = 0x83255470;
    'dispatch: loop {
        match pc {
            0x83255470 => {
    //   block [0x83255470..0x832554A8)
	// 83255470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325547C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255480: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255484: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83255488: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325548C: 4AF9E8CD  bl 0x821f3d58
	ctx.lr = 0x83255490;
	sub_821F3D58(ctx, base);
	// 83255490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255494: 906A8628  stw r3, -0x79d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31192 as u32), ctx.r[3].u32 ) };
	// 83255498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325549C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832554A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832554A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832554A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832554A8 size=56
    let mut pc: u32 = 0x832554A8;
    'dispatch: loop {
        match pc {
            0x832554A8 => {
    //   block [0x832554A8..0x832554E0)
	// 832554A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832554AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832554B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832554B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832554B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832554BC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832554C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832554C4: 4AF9E895  bl 0x821f3d58
	ctx.lr = 0x832554C8;
	sub_821F3D58(ctx, base);
	// 832554C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832554CC: 906A862C  stw r3, -0x79d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31188 as u32), ctx.r[3].u32 ) };
	// 832554D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832554D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832554D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832554DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832554E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832554E0 size=56
    let mut pc: u32 = 0x832554E0;
    'dispatch: loop {
        match pc {
            0x832554E0 => {
    //   block [0x832554E0..0x83255518)
	// 832554E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832554E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832554E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832554EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832554F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832554F4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832554F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832554FC: 4AF9E85D  bl 0x821f3d58
	ctx.lr = 0x83255500;
	sub_821F3D58(ctx, base);
	// 83255500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255504: 906A8630  stw r3, -0x79d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31184 as u32), ctx.r[3].u32 ) };
	// 83255508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325550C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255518 size=56
    let mut pc: u32 = 0x83255518;
    'dispatch: loop {
        match pc {
            0x83255518 => {
    //   block [0x83255518..0x83255550)
	// 83255518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325551C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325552C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83255530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255534: 4AF9E825  bl 0x821f3d58
	ctx.lr = 0x83255538;
	sub_821F3D58(ctx, base);
	// 83255538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325553C: 906A8634  stw r3, -0x79cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31180 as u32), ctx.r[3].u32 ) };
	// 83255540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325554C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255550 size=56
    let mut pc: u32 = 0x83255550;
    'dispatch: loop {
        match pc {
            0x83255550 => {
    //   block [0x83255550..0x83255588)
	// 83255550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325555C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255564: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83255568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325556C: 4AF9E7ED  bl 0x821f3d58
	ctx.lr = 0x83255570;
	sub_821F3D58(ctx, base);
	// 83255570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255574: 906A8638  stw r3, -0x79c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31176 as u32), ctx.r[3].u32 ) };
	// 83255578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325557C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255588 size=56
    let mut pc: u32 = 0x83255588;
    'dispatch: loop {
        match pc {
            0x83255588 => {
    //   block [0x83255588..0x832555C0)
	// 83255588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325559C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832555A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832555A4: 4AF9E7B5  bl 0x821f3d58
	ctx.lr = 0x832555A8;
	sub_821F3D58(ctx, base);
	// 832555A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832555AC: 906A863C  stw r3, -0x79c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31172 as u32), ctx.r[3].u32 ) };
	// 832555B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832555B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832555B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832555BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832555C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832555C0 size=56
    let mut pc: u32 = 0x832555C0;
    'dispatch: loop {
        match pc {
            0x832555C0 => {
    //   block [0x832555C0..0x832555F8)
	// 832555C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832555C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832555C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832555CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832555D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832555D4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832555D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832555DC: 4AF9E77D  bl 0x821f3d58
	ctx.lr = 0x832555E0;
	sub_821F3D58(ctx, base);
	// 832555E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832555E4: 906A8640  stw r3, -0x79c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31168 as u32), ctx.r[3].u32 ) };
	// 832555E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832555EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832555F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832555F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832555F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832555F8 size=56
    let mut pc: u32 = 0x832555F8;
    'dispatch: loop {
        match pc {
            0x832555F8 => {
    //   block [0x832555F8..0x83255630)
	// 832555F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832555FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325560C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83255610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255614: 4AF9E745  bl 0x821f3d58
	ctx.lr = 0x83255618;
	sub_821F3D58(ctx, base);
	// 83255618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325561C: 906A8644  stw r3, -0x79bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31164 as u32), ctx.r[3].u32 ) };
	// 83255620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325562C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255630 size=56
    let mut pc: u32 = 0x83255630;
    'dispatch: loop {
        match pc {
            0x83255630 => {
    //   block [0x83255630..0x83255668)
	// 83255630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325563C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255644: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83255648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325564C: 4AF9E70D  bl 0x821f3d58
	ctx.lr = 0x83255650;
	sub_821F3D58(ctx, base);
	// 83255650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255654: 906A8648  stw r3, -0x79b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31160 as u32), ctx.r[3].u32 ) };
	// 83255658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255668 size=56
    let mut pc: u32 = 0x83255668;
    'dispatch: loop {
        match pc {
            0x83255668 => {
    //   block [0x83255668..0x832556A0)
	// 83255668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325567C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83255680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255684: 4AF9E6D5  bl 0x821f3d58
	ctx.lr = 0x83255688;
	sub_821F3D58(ctx, base);
	// 83255688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325568C: 906A864C  stw r3, -0x79b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31156 as u32), ctx.r[3].u32 ) };
	// 83255690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325569C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832556A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832556A0 size=56
    let mut pc: u32 = 0x832556A0;
    'dispatch: loop {
        match pc {
            0x832556A0 => {
    //   block [0x832556A0..0x832556D8)
	// 832556A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832556A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832556A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832556AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832556B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832556B4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832556B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832556BC: 4AF9E69D  bl 0x821f3d58
	ctx.lr = 0x832556C0;
	sub_821F3D58(ctx, base);
	// 832556C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832556C4: 906A8650  stw r3, -0x79b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31152 as u32), ctx.r[3].u32 ) };
	// 832556C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832556CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832556D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832556D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832556D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832556D8 size=56
    let mut pc: u32 = 0x832556D8;
    'dispatch: loop {
        match pc {
            0x832556D8 => {
    //   block [0x832556D8..0x83255710)
	// 832556D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832556DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832556E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832556E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832556E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832556EC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832556F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832556F4: 4AF9E665  bl 0x821f3d58
	ctx.lr = 0x832556F8;
	sub_821F3D58(ctx, base);
	// 832556F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832556FC: 906A8654  stw r3, -0x79ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31148 as u32), ctx.r[3].u32 ) };
	// 83255700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325570C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255710 size=56
    let mut pc: u32 = 0x83255710;
    'dispatch: loop {
        match pc {
            0x83255710 => {
    //   block [0x83255710..0x83255748)
	// 83255710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325571C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255724: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83255728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325572C: 4AF9E62D  bl 0x821f3d58
	ctx.lr = 0x83255730;
	sub_821F3D58(ctx, base);
	// 83255730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255734: 906A8658  stw r3, -0x79a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31144 as u32), ctx.r[3].u32 ) };
	// 83255738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255748 size=56
    let mut pc: u32 = 0x83255748;
    'dispatch: loop {
        match pc {
            0x83255748 => {
    //   block [0x83255748..0x83255780)
	// 83255748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325575C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83255760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255764: 4AF9E5F5  bl 0x821f3d58
	ctx.lr = 0x83255768;
	sub_821F3D58(ctx, base);
	// 83255768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325576C: 906A865C  stw r3, -0x79a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31140 as u32), ctx.r[3].u32 ) };
	// 83255770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325577C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255780 size=56
    let mut pc: u32 = 0x83255780;
    'dispatch: loop {
        match pc {
            0x83255780 => {
    //   block [0x83255780..0x832557B8)
	// 83255780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325578C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255794: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83255798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325579C: 4AF9E5BD  bl 0x821f3d58
	ctx.lr = 0x832557A0;
	sub_821F3D58(ctx, base);
	// 832557A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832557A4: 906A8660  stw r3, -0x79a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31136 as u32), ctx.r[3].u32 ) };
	// 832557A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832557AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832557B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832557B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832557B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832557B8 size=56
    let mut pc: u32 = 0x832557B8;
    'dispatch: loop {
        match pc {
            0x832557B8 => {
    //   block [0x832557B8..0x832557F0)
	// 832557B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832557BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832557C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832557C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832557C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832557CC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832557D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832557D4: 4AF9E585  bl 0x821f3d58
	ctx.lr = 0x832557D8;
	sub_821F3D58(ctx, base);
	// 832557D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832557DC: 906A8664  stw r3, -0x799c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31132 as u32), ctx.r[3].u32 ) };
	// 832557E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832557E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832557E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832557EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832557F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832557F0 size=56
    let mut pc: u32 = 0x832557F0;
    'dispatch: loop {
        match pc {
            0x832557F0 => {
    //   block [0x832557F0..0x83255828)
	// 832557F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832557F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832557F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832557FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255804: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83255808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325580C: 4AF9E54D  bl 0x821f3d58
	ctx.lr = 0x83255810;
	sub_821F3D58(ctx, base);
	// 83255810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255814: 906A8668  stw r3, -0x7998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31128 as u32), ctx.r[3].u32 ) };
	// 83255818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325581C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255828 size=56
    let mut pc: u32 = 0x83255828;
    'dispatch: loop {
        match pc {
            0x83255828 => {
    //   block [0x83255828..0x83255860)
	// 83255828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325582C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325583C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83255840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255844: 4AF9E515  bl 0x821f3d58
	ctx.lr = 0x83255848;
	sub_821F3D58(ctx, base);
	// 83255848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325584C: 906A866C  stw r3, -0x7994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31124 as u32), ctx.r[3].u32 ) };
	// 83255850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325585C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255860 size=64
    let mut pc: u32 = 0x83255860;
    'dispatch: loop {
        match pc {
            0x83255860 => {
    //   block [0x83255860..0x832558A0)
	// 83255860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325586C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255874: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83255878: 386A8670  addi r3, r10, -0x7990
	ctx.r[3].s64 = ctx.r[10].s64 + -31120;
	// 8325587C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255880: 4AFD7651  bl 0x8222ced0
	ctx.lr = 0x83255884;
	sub_8222CED0(ctx, base);
	// 83255884: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255888: 3869A658  addi r3, r9, -0x59a8
	ctx.r[3].s64 = ctx.r[9].s64 + -22952;
	// 8325588C: 4BA54695  bl 0x82ca9f20
	ctx.lr = 0x83255890;
	sub_82CA9F20(ctx, base);
	// 83255890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325589C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832558A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832558A0 size=64
    let mut pc: u32 = 0x832558A0;
    'dispatch: loop {
        match pc {
            0x832558A0 => {
    //   block [0x832558A0..0x832558E0)
	// 832558A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832558A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832558A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832558AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832558B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832558B4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832558B8: 386A8674  addi r3, r10, -0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + -31116;
	// 832558BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832558C0: 4AFD7611  bl 0x8222ced0
	ctx.lr = 0x832558C4;
	sub_8222CED0(ctx, base);
	// 832558C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832558C8: 3869A668  addi r3, r9, -0x5998
	ctx.r[3].s64 = ctx.r[9].s64 + -22936;
	// 832558CC: 4BA54655  bl 0x82ca9f20
	ctx.lr = 0x832558D0;
	sub_82CA9F20(ctx, base);
	// 832558D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832558D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832558D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832558DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832558E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832558E0 size=64
    let mut pc: u32 = 0x832558E0;
    'dispatch: loop {
        match pc {
            0x832558E0 => {
    //   block [0x832558E0..0x83255920)
	// 832558E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832558E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832558E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832558EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832558F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832558F4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832558F8: 386A8678  addi r3, r10, -0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + -31112;
	// 832558FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255900: 4AFD75D1  bl 0x8222ced0
	ctx.lr = 0x83255904;
	sub_8222CED0(ctx, base);
	// 83255904: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255908: 3869A678  addi r3, r9, -0x5988
	ctx.r[3].s64 = ctx.r[9].s64 + -22920;
	// 8325590C: 4BA54615  bl 0x82ca9f20
	ctx.lr = 0x83255910;
	sub_82CA9F20(ctx, base);
	// 83255910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255920 size=64
    let mut pc: u32 = 0x83255920;
    'dispatch: loop {
        match pc {
            0x83255920 => {
    //   block [0x83255920..0x83255960)
	// 83255920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325592C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255934: 388B0BAC  addi r4, r11, 0xbac
	ctx.r[4].s64 = ctx.r[11].s64 + 2988;
	// 83255938: 386A867C  addi r3, r10, -0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + -31108;
	// 8325593C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255940: 4AFD7591  bl 0x8222ced0
	ctx.lr = 0x83255944;
	sub_8222CED0(ctx, base);
	// 83255944: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255948: 3869A688  addi r3, r9, -0x5978
	ctx.r[3].s64 = ctx.r[9].s64 + -22904;
	// 8325594C: 4BA545D5  bl 0x82ca9f20
	ctx.lr = 0x83255950;
	sub_82CA9F20(ctx, base);
	// 83255950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325595C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255960 size=64
    let mut pc: u32 = 0x83255960;
    'dispatch: loop {
        match pc {
            0x83255960 => {
    //   block [0x83255960..0x832559A0)
	// 83255960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325596C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255974: 388B0BCC  addi r4, r11, 0xbcc
	ctx.r[4].s64 = ctx.r[11].s64 + 3020;
	// 83255978: 386A8680  addi r3, r10, -0x7980
	ctx.r[3].s64 = ctx.r[10].s64 + -31104;
	// 8325597C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255980: 4AFD7551  bl 0x8222ced0
	ctx.lr = 0x83255984;
	sub_8222CED0(ctx, base);
	// 83255984: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255988: 3869A698  addi r3, r9, -0x5968
	ctx.r[3].s64 = ctx.r[9].s64 + -22888;
	// 8325598C: 4BA54595  bl 0x82ca9f20
	ctx.lr = 0x83255990;
	sub_82CA9F20(ctx, base);
	// 83255990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325599C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832559A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832559A0 size=64
    let mut pc: u32 = 0x832559A0;
    'dispatch: loop {
        match pc {
            0x832559A0 => {
    //   block [0x832559A0..0x832559E0)
	// 832559A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832559A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832559A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832559AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832559B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832559B4: 388B0BF0  addi r4, r11, 0xbf0
	ctx.r[4].s64 = ctx.r[11].s64 + 3056;
	// 832559B8: 386A8684  addi r3, r10, -0x797c
	ctx.r[3].s64 = ctx.r[10].s64 + -31100;
	// 832559BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832559C0: 4AFD7511  bl 0x8222ced0
	ctx.lr = 0x832559C4;
	sub_8222CED0(ctx, base);
	// 832559C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832559C8: 3869A6A8  addi r3, r9, -0x5958
	ctx.r[3].s64 = ctx.r[9].s64 + -22872;
	// 832559CC: 4BA54555  bl 0x82ca9f20
	ctx.lr = 0x832559D0;
	sub_82CA9F20(ctx, base);
	// 832559D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832559D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832559D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832559DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832559E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832559E0 size=64
    let mut pc: u32 = 0x832559E0;
    'dispatch: loop {
        match pc {
            0x832559E0 => {
    //   block [0x832559E0..0x83255A20)
	// 832559E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832559E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832559E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832559EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832559F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832559F4: 388B0C00  addi r4, r11, 0xc00
	ctx.r[4].s64 = ctx.r[11].s64 + 3072;
	// 832559F8: 386A8688  addi r3, r10, -0x7978
	ctx.r[3].s64 = ctx.r[10].s64 + -31096;
	// 832559FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255A00: 4AFD74D1  bl 0x8222ced0
	ctx.lr = 0x83255A04;
	sub_8222CED0(ctx, base);
	// 83255A04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255A08: 3869A6B8  addi r3, r9, -0x5948
	ctx.r[3].s64 = ctx.r[9].s64 + -22856;
	// 83255A0C: 4BA54515  bl 0x82ca9f20
	ctx.lr = 0x83255A10;
	sub_82CA9F20(ctx, base);
	// 83255A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255A20 size=64
    let mut pc: u32 = 0x83255A20;
    'dispatch: loop {
        match pc {
            0x83255A20 => {
    //   block [0x83255A20..0x83255A60)
	// 83255A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255A2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255A30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255A34: 388B0C20  addi r4, r11, 0xc20
	ctx.r[4].s64 = ctx.r[11].s64 + 3104;
	// 83255A38: 386A868C  addi r3, r10, -0x7974
	ctx.r[3].s64 = ctx.r[10].s64 + -31092;
	// 83255A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255A40: 4AFD7491  bl 0x8222ced0
	ctx.lr = 0x83255A44;
	sub_8222CED0(ctx, base);
	// 83255A44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255A48: 3869A6C8  addi r3, r9, -0x5938
	ctx.r[3].s64 = ctx.r[9].s64 + -22840;
	// 83255A4C: 4BA544D5  bl 0x82ca9f20
	ctx.lr = 0x83255A50;
	sub_82CA9F20(ctx, base);
	// 83255A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255A60 size=64
    let mut pc: u32 = 0x83255A60;
    'dispatch: loop {
        match pc {
            0x83255A60 => {
    //   block [0x83255A60..0x83255AA0)
	// 83255A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255A6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255A74: 388B0C34  addi r4, r11, 0xc34
	ctx.r[4].s64 = ctx.r[11].s64 + 3124;
	// 83255A78: 386A8690  addi r3, r10, -0x7970
	ctx.r[3].s64 = ctx.r[10].s64 + -31088;
	// 83255A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255A80: 4AFD7451  bl 0x8222ced0
	ctx.lr = 0x83255A84;
	sub_8222CED0(ctx, base);
	// 83255A84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255A88: 3869A6D8  addi r3, r9, -0x5928
	ctx.r[3].s64 = ctx.r[9].s64 + -22824;
	// 83255A8C: 4BA54495  bl 0x82ca9f20
	ctx.lr = 0x83255A90;
	sub_82CA9F20(ctx, base);
	// 83255A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255AA0 size=64
    let mut pc: u32 = 0x83255AA0;
    'dispatch: loop {
        match pc {
            0x83255AA0 => {
    //   block [0x83255AA0..0x83255AE0)
	// 83255AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255AAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255AB4: 388B0C48  addi r4, r11, 0xc48
	ctx.r[4].s64 = ctx.r[11].s64 + 3144;
	// 83255AB8: 386A8694  addi r3, r10, -0x796c
	ctx.r[3].s64 = ctx.r[10].s64 + -31084;
	// 83255ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255AC0: 4AFD7411  bl 0x8222ced0
	ctx.lr = 0x83255AC4;
	sub_8222CED0(ctx, base);
	// 83255AC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255AC8: 3869A6E8  addi r3, r9, -0x5918
	ctx.r[3].s64 = ctx.r[9].s64 + -22808;
	// 83255ACC: 4BA54455  bl 0x82ca9f20
	ctx.lr = 0x83255AD0;
	sub_82CA9F20(ctx, base);
	// 83255AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255AE0 size=64
    let mut pc: u32 = 0x83255AE0;
    'dispatch: loop {
        match pc {
            0x83255AE0 => {
    //   block [0x83255AE0..0x83255B20)
	// 83255AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255AEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255AF4: 388B0C64  addi r4, r11, 0xc64
	ctx.r[4].s64 = ctx.r[11].s64 + 3172;
	// 83255AF8: 386A8698  addi r3, r10, -0x7968
	ctx.r[3].s64 = ctx.r[10].s64 + -31080;
	// 83255AFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255B00: 4AFD73D1  bl 0x8222ced0
	ctx.lr = 0x83255B04;
	sub_8222CED0(ctx, base);
	// 83255B04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255B08: 3869A6F8  addi r3, r9, -0x5908
	ctx.r[3].s64 = ctx.r[9].s64 + -22792;
	// 83255B0C: 4BA54415  bl 0x82ca9f20
	ctx.lr = 0x83255B10;
	sub_82CA9F20(ctx, base);
	// 83255B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255B20 size=64
    let mut pc: u32 = 0x83255B20;
    'dispatch: loop {
        match pc {
            0x83255B20 => {
    //   block [0x83255B20..0x83255B60)
	// 83255B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255B2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255B34: 388B0C78  addi r4, r11, 0xc78
	ctx.r[4].s64 = ctx.r[11].s64 + 3192;
	// 83255B38: 386A869C  addi r3, r10, -0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + -31076;
	// 83255B3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255B40: 4AFD7391  bl 0x8222ced0
	ctx.lr = 0x83255B44;
	sub_8222CED0(ctx, base);
	// 83255B44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255B48: 3869A708  addi r3, r9, -0x58f8
	ctx.r[3].s64 = ctx.r[9].s64 + -22776;
	// 83255B4C: 4BA543D5  bl 0x82ca9f20
	ctx.lr = 0x83255B50;
	sub_82CA9F20(ctx, base);
	// 83255B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255B60 size=64
    let mut pc: u32 = 0x83255B60;
    'dispatch: loop {
        match pc {
            0x83255B60 => {
    //   block [0x83255B60..0x83255BA0)
	// 83255B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255B6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255B70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255B74: 388B0C8C  addi r4, r11, 0xc8c
	ctx.r[4].s64 = ctx.r[11].s64 + 3212;
	// 83255B78: 386A86A0  addi r3, r10, -0x7960
	ctx.r[3].s64 = ctx.r[10].s64 + -31072;
	// 83255B7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255B80: 4AFD7351  bl 0x8222ced0
	ctx.lr = 0x83255B84;
	sub_8222CED0(ctx, base);
	// 83255B84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255B88: 3869A718  addi r3, r9, -0x58e8
	ctx.r[3].s64 = ctx.r[9].s64 + -22760;
	// 83255B8C: 4BA54395  bl 0x82ca9f20
	ctx.lr = 0x83255B90;
	sub_82CA9F20(ctx, base);
	// 83255B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255BA0 size=64
    let mut pc: u32 = 0x83255BA0;
    'dispatch: loop {
        match pc {
            0x83255BA0 => {
    //   block [0x83255BA0..0x83255BE0)
	// 83255BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255BAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255BB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255BB4: 388B0C9C  addi r4, r11, 0xc9c
	ctx.r[4].s64 = ctx.r[11].s64 + 3228;
	// 83255BB8: 386A86A4  addi r3, r10, -0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + -31068;
	// 83255BBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255BC0: 4AFD7311  bl 0x8222ced0
	ctx.lr = 0x83255BC4;
	sub_8222CED0(ctx, base);
	// 83255BC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255BC8: 3869A728  addi r3, r9, -0x58d8
	ctx.r[3].s64 = ctx.r[9].s64 + -22744;
	// 83255BCC: 4BA54355  bl 0x82ca9f20
	ctx.lr = 0x83255BD0;
	sub_82CA9F20(ctx, base);
	// 83255BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255BE0 size=64
    let mut pc: u32 = 0x83255BE0;
    'dispatch: loop {
        match pc {
            0x83255BE0 => {
    //   block [0x83255BE0..0x83255C20)
	// 83255BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255BEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255BF4: 388B0CB0  addi r4, r11, 0xcb0
	ctx.r[4].s64 = ctx.r[11].s64 + 3248;
	// 83255BF8: 386A86A8  addi r3, r10, -0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + -31064;
	// 83255BFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255C00: 4AFD72D1  bl 0x8222ced0
	ctx.lr = 0x83255C04;
	sub_8222CED0(ctx, base);
	// 83255C04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255C08: 3869A738  addi r3, r9, -0x58c8
	ctx.r[3].s64 = ctx.r[9].s64 + -22728;
	// 83255C0C: 4BA54315  bl 0x82ca9f20
	ctx.lr = 0x83255C10;
	sub_82CA9F20(ctx, base);
	// 83255C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255C20 size=64
    let mut pc: u32 = 0x83255C20;
    'dispatch: loop {
        match pc {
            0x83255C20 => {
    //   block [0x83255C20..0x83255C60)
	// 83255C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255C2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255C34: 388B0CC8  addi r4, r11, 0xcc8
	ctx.r[4].s64 = ctx.r[11].s64 + 3272;
	// 83255C38: 386A86AC  addi r3, r10, -0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + -31060;
	// 83255C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255C40: 4AFD7291  bl 0x8222ced0
	ctx.lr = 0x83255C44;
	sub_8222CED0(ctx, base);
	// 83255C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255C48: 3869A748  addi r3, r9, -0x58b8
	ctx.r[3].s64 = ctx.r[9].s64 + -22712;
	// 83255C4C: 4BA542D5  bl 0x82ca9f20
	ctx.lr = 0x83255C50;
	sub_82CA9F20(ctx, base);
	// 83255C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255C60 size=64
    let mut pc: u32 = 0x83255C60;
    'dispatch: loop {
        match pc {
            0x83255C60 => {
    //   block [0x83255C60..0x83255CA0)
	// 83255C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255C6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255C74: 388B0CDC  addi r4, r11, 0xcdc
	ctx.r[4].s64 = ctx.r[11].s64 + 3292;
	// 83255C78: 386A86B0  addi r3, r10, -0x7950
	ctx.r[3].s64 = ctx.r[10].s64 + -31056;
	// 83255C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255C80: 4AFD7251  bl 0x8222ced0
	ctx.lr = 0x83255C84;
	sub_8222CED0(ctx, base);
	// 83255C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255C88: 3869A758  addi r3, r9, -0x58a8
	ctx.r[3].s64 = ctx.r[9].s64 + -22696;
	// 83255C8C: 4BA54295  bl 0x82ca9f20
	ctx.lr = 0x83255C90;
	sub_82CA9F20(ctx, base);
	// 83255C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255CA0 size=64
    let mut pc: u32 = 0x83255CA0;
    'dispatch: loop {
        match pc {
            0x83255CA0 => {
    //   block [0x83255CA0..0x83255CE0)
	// 83255CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255CAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255CB4: 388B0CF0  addi r4, r11, 0xcf0
	ctx.r[4].s64 = ctx.r[11].s64 + 3312;
	// 83255CB8: 386A86B4  addi r3, r10, -0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + -31052;
	// 83255CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255CC0: 4AFD7211  bl 0x8222ced0
	ctx.lr = 0x83255CC4;
	sub_8222CED0(ctx, base);
	// 83255CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255CC8: 3869A768  addi r3, r9, -0x5898
	ctx.r[3].s64 = ctx.r[9].s64 + -22680;
	// 83255CCC: 4BA54255  bl 0x82ca9f20
	ctx.lr = 0x83255CD0;
	sub_82CA9F20(ctx, base);
	// 83255CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255CE0 size=64
    let mut pc: u32 = 0x83255CE0;
    'dispatch: loop {
        match pc {
            0x83255CE0 => {
    //   block [0x83255CE0..0x83255D20)
	// 83255CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255CEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255CF4: 388B0D08  addi r4, r11, 0xd08
	ctx.r[4].s64 = ctx.r[11].s64 + 3336;
	// 83255CF8: 386A86B8  addi r3, r10, -0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + -31048;
	// 83255CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255D00: 4AFD71D1  bl 0x8222ced0
	ctx.lr = 0x83255D04;
	sub_8222CED0(ctx, base);
	// 83255D04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255D08: 3869A778  addi r3, r9, -0x5888
	ctx.r[3].s64 = ctx.r[9].s64 + -22664;
	// 83255D0C: 4BA54215  bl 0x82ca9f20
	ctx.lr = 0x83255D10;
	sub_82CA9F20(ctx, base);
	// 83255D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D20 size=64
    let mut pc: u32 = 0x83255D20;
    'dispatch: loop {
        match pc {
            0x83255D20 => {
    //   block [0x83255D20..0x83255D60)
	// 83255D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255D2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255D30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255D34: 388B0D1C  addi r4, r11, 0xd1c
	ctx.r[4].s64 = ctx.r[11].s64 + 3356;
	// 83255D38: 386A86BC  addi r3, r10, -0x7944
	ctx.r[3].s64 = ctx.r[10].s64 + -31044;
	// 83255D3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255D40: 4AFD7191  bl 0x8222ced0
	ctx.lr = 0x83255D44;
	sub_8222CED0(ctx, base);
	// 83255D44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255D48: 3869A788  addi r3, r9, -0x5878
	ctx.r[3].s64 = ctx.r[9].s64 + -22648;
	// 83255D4C: 4BA541D5  bl 0x82ca9f20
	ctx.lr = 0x83255D50;
	sub_82CA9F20(ctx, base);
	// 83255D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D60 size=56
    let mut pc: u32 = 0x83255D60;
    'dispatch: loop {
        match pc {
            0x83255D60 => {
    //   block [0x83255D60..0x83255D98)
	// 83255D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255D6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83255D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255D74: 386B25D4  addi r3, r11, 0x25d4
	ctx.r[3].s64 = ctx.r[11].s64 + 9684;
	// 83255D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255D7C: 4AF9DFDD  bl 0x821f3d58
	ctx.lr = 0x83255D80;
	sub_821F3D58(ctx, base);
	// 83255D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255D84: 906A86C0  stw r3, -0x7940(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31040 as u32), ctx.r[3].u32 ) };
	// 83255D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255D98 size=64
    let mut pc: u32 = 0x83255D98;
    'dispatch: loop {
        match pc {
            0x83255D98 => {
    //   block [0x83255D98..0x83255DD8)
	// 83255D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255DAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83255DB0: 386A86C4  addi r3, r10, -0x793c
	ctx.r[3].s64 = ctx.r[10].s64 + -31036;
	// 83255DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255DB8: 4AFD7119  bl 0x8222ced0
	ctx.lr = 0x83255DBC;
	sub_8222CED0(ctx, base);
	// 83255DBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255DC0: 3869A798  addi r3, r9, -0x5868
	ctx.r[3].s64 = ctx.r[9].s64 + -22632;
	// 83255DC4: 4BA5415D  bl 0x82ca9f20
	ctx.lr = 0x83255DC8;
	sub_82CA9F20(ctx, base);
	// 83255DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255DD8 size=64
    let mut pc: u32 = 0x83255DD8;
    'dispatch: loop {
        match pc {
            0x83255DD8 => {
    //   block [0x83255DD8..0x83255E18)
	// 83255DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255DEC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83255DF0: 386A86C8  addi r3, r10, -0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + -31032;
	// 83255DF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255DF8: 4AFD70D9  bl 0x8222ced0
	ctx.lr = 0x83255DFC;
	sub_8222CED0(ctx, base);
	// 83255DFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255E00: 3869A7A8  addi r3, r9, -0x5858
	ctx.r[3].s64 = ctx.r[9].s64 + -22616;
	// 83255E04: 4BA5411D  bl 0x82ca9f20
	ctx.lr = 0x83255E08;
	sub_82CA9F20(ctx, base);
	// 83255E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E18 size=64
    let mut pc: u32 = 0x83255E18;
    'dispatch: loop {
        match pc {
            0x83255E18 => {
    //   block [0x83255E18..0x83255E58)
	// 83255E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255E24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83255E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255E2C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83255E30: 386A86CC  addi r3, r10, -0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + -31028;
	// 83255E34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83255E38: 4AFD7099  bl 0x8222ced0
	ctx.lr = 0x83255E3C;
	sub_8222CED0(ctx, base);
	// 83255E3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83255E40: 3869A7B8  addi r3, r9, -0x5848
	ctx.r[3].s64 = ctx.r[9].s64 + -22600;
	// 83255E44: 4BA540DD  bl 0x82ca9f20
	ctx.lr = 0x83255E48;
	sub_82CA9F20(ctx, base);
	// 83255E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E58 size=56
    let mut pc: u32 = 0x83255E58;
    'dispatch: loop {
        match pc {
            0x83255E58 => {
    //   block [0x83255E58..0x83255E90)
	// 83255E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255E64: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255E68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255E6C: 386B0E58  addi r3, r11, 0xe58
	ctx.r[3].s64 = ctx.r[11].s64 + 3672;
	// 83255E70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255E74: 4AF9DEE5  bl 0x821f3d58
	ctx.lr = 0x83255E78;
	sub_821F3D58(ctx, base);
	// 83255E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255E7C: 906A86D0  stw r3, -0x7930(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31024 as u32), ctx.r[3].u32 ) };
	// 83255E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255E90 size=56
    let mut pc: u32 = 0x83255E90;
    'dispatch: loop {
        match pc {
            0x83255E90 => {
    //   block [0x83255E90..0x83255EC8)
	// 83255E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255E9C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255EA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255EA4: 386B0E68  addi r3, r11, 0xe68
	ctx.r[3].s64 = ctx.r[11].s64 + 3688;
	// 83255EA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255EAC: 4AF9DEAD  bl 0x821f3d58
	ctx.lr = 0x83255EB0;
	sub_821F3D58(ctx, base);
	// 83255EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255EB4: 906A86D4  stw r3, -0x792c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31020 as u32), ctx.r[3].u32 ) };
	// 83255EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255EC8 size=56
    let mut pc: u32 = 0x83255EC8;
    'dispatch: loop {
        match pc {
            0x83255EC8 => {
    //   block [0x83255EC8..0x83255F00)
	// 83255EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255ED4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255ED8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255EDC: 386B0E74  addi r3, r11, 0xe74
	ctx.r[3].s64 = ctx.r[11].s64 + 3700;
	// 83255EE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255EE4: 4AF9DE75  bl 0x821f3d58
	ctx.lr = 0x83255EE8;
	sub_821F3D58(ctx, base);
	// 83255EE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255EEC: 906A86D8  stw r3, -0x7928(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31016 as u32), ctx.r[3].u32 ) };
	// 83255EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F00 size=56
    let mut pc: u32 = 0x83255F00;
    'dispatch: loop {
        match pc {
            0x83255F00 => {
    //   block [0x83255F00..0x83255F38)
	// 83255F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F0C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F14: 386B0E84  addi r3, r11, 0xe84
	ctx.r[3].s64 = ctx.r[11].s64 + 3716;
	// 83255F18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F1C: 4AF9DE3D  bl 0x821f3d58
	ctx.lr = 0x83255F20;
	sub_821F3D58(ctx, base);
	// 83255F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F24: 906A86DC  stw r3, -0x7924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31012 as u32), ctx.r[3].u32 ) };
	// 83255F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F38 size=56
    let mut pc: u32 = 0x83255F38;
    'dispatch: loop {
        match pc {
            0x83255F38 => {
    //   block [0x83255F38..0x83255F70)
	// 83255F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F44: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F4C: 386B0E94  addi r3, r11, 0xe94
	ctx.r[3].s64 = ctx.r[11].s64 + 3732;
	// 83255F50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F54: 4AF9DE05  bl 0x821f3d58
	ctx.lr = 0x83255F58;
	sub_821F3D58(ctx, base);
	// 83255F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F5C: 906A86E0  stw r3, -0x7920(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31008 as u32), ctx.r[3].u32 ) };
	// 83255F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255F70 size=56
    let mut pc: u32 = 0x83255F70;
    'dispatch: loop {
        match pc {
            0x83255F70 => {
    //   block [0x83255F70..0x83255FA8)
	// 83255F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255F7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255F80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255F84: 386B0EA4  addi r3, r11, 0xea4
	ctx.r[3].s64 = ctx.r[11].s64 + 3748;
	// 83255F88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255F8C: 4AF9DDCD  bl 0x821f3d58
	ctx.lr = 0x83255F90;
	sub_821F3D58(ctx, base);
	// 83255F90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255F94: 906A86E4  stw r3, -0x791c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31004 as u32), ctx.r[3].u32 ) };
	// 83255F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255FA8 size=56
    let mut pc: u32 = 0x83255FA8;
    'dispatch: loop {
        match pc {
            0x83255FA8 => {
    //   block [0x83255FA8..0x83255FE0)
	// 83255FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255FB4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255FB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83255FBC: 386B0EB0  addi r3, r11, 0xeb0
	ctx.r[3].s64 = ctx.r[11].s64 + 3760;
	// 83255FC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83255FC4: 4AF9DD95  bl 0x821f3d58
	ctx.lr = 0x83255FC8;
	sub_821F3D58(ctx, base);
	// 83255FC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255FCC: 906A86E8  stw r3, -0x7918(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31000 as u32), ctx.r[3].u32 ) };
	// 83255FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83255FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83255FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83255FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83255FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83255FE0 size=64
    let mut pc: u32 = 0x83255FE0;
    'dispatch: loop {
        match pc {
            0x83255FE0 => {
    //   block [0x83255FE0..0x83256020)
	// 83255FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83255FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83255FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83255FEC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83255FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83255FF4: 388B0EC8  addi r4, r11, 0xec8
	ctx.r[4].s64 = ctx.r[11].s64 + 3784;
	// 83255FF8: 386A86EC  addi r3, r10, -0x7914
	ctx.r[3].s64 = ctx.r[10].s64 + -30996;
	// 83255FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256000: 4AFD6ED1  bl 0x8222ced0
	ctx.lr = 0x83256004;
	sub_8222CED0(ctx, base);
	// 83256004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256008: 3869A7C8  addi r3, r9, -0x5838
	ctx.r[3].s64 = ctx.r[9].s64 + -22584;
	// 8325600C: 4BA53F15  bl 0x82ca9f20
	ctx.lr = 0x83256010;
	sub_82CA9F20(ctx, base);
	// 83256010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325601C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256020 size=64
    let mut pc: u32 = 0x83256020;
    'dispatch: loop {
        match pc {
            0x83256020 => {
    //   block [0x83256020..0x83256060)
	// 83256020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325602C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256034: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256038: 386A86F0  addi r3, r10, -0x7910
	ctx.r[3].s64 = ctx.r[10].s64 + -30992;
	// 8325603C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256040: 4AFD6E91  bl 0x8222ced0
	ctx.lr = 0x83256044;
	sub_8222CED0(ctx, base);
	// 83256044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256048: 3869A7D8  addi r3, r9, -0x5828
	ctx.r[3].s64 = ctx.r[9].s64 + -22568;
	// 8325604C: 4BA53ED5  bl 0x82ca9f20
	ctx.lr = 0x83256050;
	sub_82CA9F20(ctx, base);
	// 83256050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325605C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256060 size=64
    let mut pc: u32 = 0x83256060;
    'dispatch: loop {
        match pc {
            0x83256060 => {
    //   block [0x83256060..0x832560A0)
	// 83256060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325606C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256074: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256078: 386A86F4  addi r3, r10, -0x790c
	ctx.r[3].s64 = ctx.r[10].s64 + -30988;
	// 8325607C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256080: 4AFD6E51  bl 0x8222ced0
	ctx.lr = 0x83256084;
	sub_8222CED0(ctx, base);
	// 83256084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256088: 3869A7E8  addi r3, r9, -0x5818
	ctx.r[3].s64 = ctx.r[9].s64 + -22552;
	// 8325608C: 4BA53E95  bl 0x82ca9f20
	ctx.lr = 0x83256090;
	sub_82CA9F20(ctx, base);
	// 83256090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325609C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832560A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832560A0 size=64
    let mut pc: u32 = 0x832560A0;
    'dispatch: loop {
        match pc {
            0x832560A0 => {
    //   block [0x832560A0..0x832560E0)
	// 832560A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832560A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832560A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832560AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832560B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832560B4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832560B8: 386A86F8  addi r3, r10, -0x7908
	ctx.r[3].s64 = ctx.r[10].s64 + -30984;
	// 832560BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832560C0: 4AFD6E11  bl 0x8222ced0
	ctx.lr = 0x832560C4;
	sub_8222CED0(ctx, base);
	// 832560C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832560C8: 3869A7F8  addi r3, r9, -0x5808
	ctx.r[3].s64 = ctx.r[9].s64 + -22536;
	// 832560CC: 4BA53E55  bl 0x82ca9f20
	ctx.lr = 0x832560D0;
	sub_82CA9F20(ctx, base);
	// 832560D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832560D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832560D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832560DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832560E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832560E0 size=64
    let mut pc: u32 = 0x832560E0;
    'dispatch: loop {
        match pc {
            0x832560E0 => {
    //   block [0x832560E0..0x83256120)
	// 832560E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832560E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832560E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832560EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832560F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832560F4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832560F8: 386A86FC  addi r3, r10, -0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + -30980;
	// 832560FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256100: 4AFD6DD1  bl 0x8222ced0
	ctx.lr = 0x83256104;
	sub_8222CED0(ctx, base);
	// 83256104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256108: 3869A808  addi r3, r9, -0x57f8
	ctx.r[3].s64 = ctx.r[9].s64 + -22520;
	// 8325610C: 4BA53E15  bl 0x82ca9f20
	ctx.lr = 0x83256110;
	sub_82CA9F20(ctx, base);
	// 83256110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325611C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256120 size=64
    let mut pc: u32 = 0x83256120;
    'dispatch: loop {
        match pc {
            0x83256120 => {
    //   block [0x83256120..0x83256160)
	// 83256120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325612C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256134: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256138: 386A8700  addi r3, r10, -0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + -30976;
	// 8325613C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256140: 4AFD6D91  bl 0x8222ced0
	ctx.lr = 0x83256144;
	sub_8222CED0(ctx, base);
	// 83256144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256148: 3869A818  addi r3, r9, -0x57e8
	ctx.r[3].s64 = ctx.r[9].s64 + -22504;
	// 8325614C: 4BA53DD5  bl 0x82ca9f20
	ctx.lr = 0x83256150;
	sub_82CA9F20(ctx, base);
	// 83256150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256160 size=64
    let mut pc: u32 = 0x83256160;
    'dispatch: loop {
        match pc {
            0x83256160 => {
    //   block [0x83256160..0x832561A0)
	// 83256160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325616C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256174: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256178: 386A8704  addi r3, r10, -0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30972;
	// 8325617C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256180: 4AFD6D51  bl 0x8222ced0
	ctx.lr = 0x83256184;
	sub_8222CED0(ctx, base);
	// 83256184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256188: 3869A828  addi r3, r9, -0x57d8
	ctx.r[3].s64 = ctx.r[9].s64 + -22488;
	// 8325618C: 4BA53D95  bl 0x82ca9f20
	ctx.lr = 0x83256190;
	sub_82CA9F20(ctx, base);
	// 83256190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325619C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832561A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832561A0 size=100
    let mut pc: u32 = 0x832561A0;
    'dispatch: loop {
        match pc {
            0x832561A0 => {
    //   block [0x832561A0..0x83256204)
	// 832561A0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832561A4: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832561A8: 392BD5C8  addi r9, r11, -0x2a38
	ctx.r[9].s64 = ctx.r[11].s64 + -10808;
	// 832561AC: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 832561B0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 832561B4: C00BD5C8  lfs f0, -0x2a38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832561B8: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832561BC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832561C0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832561C4: C009BEBC  lfs f0, -0x4144(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832561C8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832561CC: 38858710  addi r4, r5, -0x78f0
	ctx.r[4].s64 = ctx.r[5].s64 + -30960;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83256208 size=96
    let mut pc: u32 = 0x83256208;
    'dispatch: loop {
        match pc {
            0x83256208 => {
    //   block [0x83256208..0x83256268)
	// 83256208: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325620C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83256210: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83256214: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83256218: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8325621C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83256220: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83256224: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83256228: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256268 size=64
    let mut pc: u32 = 0x83256268;
    'dispatch: loop {
        match pc {
            0x83256268 => {
    //   block [0x83256268..0x832562A8)
	// 83256268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256274: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325627C: 388B1018  addi r4, r11, 0x1018
	ctx.r[4].s64 = ctx.r[11].s64 + 4120;
	// 83256280: 386A8708  addi r3, r10, -0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30968;
	// 83256284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256288: 4AFD6C49  bl 0x8222ced0
	ctx.lr = 0x8325628C;
	sub_8222CED0(ctx, base);
	// 8325628C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256290: 3869A838  addi r3, r9, -0x57c8
	ctx.r[3].s64 = ctx.r[9].s64 + -22472;
	// 83256294: 4BA53C8D  bl 0x82ca9f20
	ctx.lr = 0x83256298;
	sub_82CA9F20(ctx, base);
	// 83256298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325629C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832562A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832562A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832562A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832562A8 size=64
    let mut pc: u32 = 0x832562A8;
    'dispatch: loop {
        match pc {
            0x832562A8 => {
    //   block [0x832562A8..0x832562E8)
	// 832562A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832562AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832562B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832562B4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832562B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832562BC: 388B1024  addi r4, r11, 0x1024
	ctx.r[4].s64 = ctx.r[11].s64 + 4132;
	// 832562C0: 386A870C  addi r3, r10, -0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30964;
	// 832562C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832562C8: 4AFD6C09  bl 0x8222ced0
	ctx.lr = 0x832562CC;
	sub_8222CED0(ctx, base);
	// 832562CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832562D0: 3869A848  addi r3, r9, -0x57b8
	ctx.r[3].s64 = ctx.r[9].s64 + -22456;
	// 832562D4: 4BA53C4D  bl 0x82ca9f20
	ctx.lr = 0x832562D8;
	sub_82CA9F20(ctx, base);
	// 832562D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832562DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832562E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832562E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832562E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832562E8 size=88
    let mut pc: u32 = 0x832562E8;
    'dispatch: loop {
        match pc {
            0x832562E8 => {
    //   block [0x832562E8..0x83256340)
	// 832562E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832562EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832562F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832562F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832562F8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832562FC: 386BA858  addi r3, r11, -0x57a8
	ctx.r[3].s64 = ctx.r[11].s64 + -22440;
	// 83256300: 4BA53C21  bl 0x82ca9f20
	ctx.lr = 0x83256304;
	sub_82CA9F20(ctx, base);
	// 83256304: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83256308: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 8325630C: 3BEAC4C8  addi r31, r10, -0x3b38
	ctx.r[31].s64 = ctx.r[10].s64 + -15160;
	// 83256310: 38891030  addi r4, r9, 0x1030
	ctx.r[4].s64 = ctx.r[9].s64 + 4144;
	// 83256314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83256318: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325631C: 4AFD6BB5  bl 0x8222ced0
	ctx.lr = 0x83256320;
	sub_8222CED0(ctx, base);
	// 83256320: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 83256324: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83256328: 91688730  stw r11, -0x78d0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30928 as u32), ctx.r[11].u32 ) };
	// 8325632C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8325633C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256340 size=88
    let mut pc: u32 = 0x83256340;
    'dispatch: loop {
        match pc {
            0x83256340 => {
    //   block [0x83256340..0x83256398)
	// 83256340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325634C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256350: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256354: 386BA868  addi r3, r11, -0x5798
	ctx.r[3].s64 = ctx.r[11].s64 + -22424;
	// 83256358: 4BA53BC9  bl 0x82ca9f20
	ctx.lr = 0x8325635C;
	sub_82CA9F20(ctx, base);
	// 8325635C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83256360: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83256364: 3BEAC4CC  addi r31, r10, -0x3b34
	ctx.r[31].s64 = ctx.r[10].s64 + -15156;
	// 83256368: 38891030  addi r4, r9, 0x1030
	ctx.r[4].s64 = ctx.r[9].s64 + 4144;
	// 8325636C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83256370: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256374: 4AFD6B5D  bl 0x8222ced0
	ctx.lr = 0x83256378;
	sub_8222CED0(ctx, base);
	// 83256378: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8325637C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83256380: 91688734  stw r11, -0x78cc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30924 as u32), ctx.r[11].u32 ) };
	// 83256384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83256394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256398 size=88
    let mut pc: u32 = 0x83256398;
    'dispatch: loop {
        match pc {
            0x83256398 => {
    //   block [0x83256398..0x832563F0)
	// 83256398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832563A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832563A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832563A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832563AC: 386BA878  addi r3, r11, -0x5788
	ctx.r[3].s64 = ctx.r[11].s64 + -22408;
	// 832563B0: 4BA53B71  bl 0x82ca9f20
	ctx.lr = 0x832563B4;
	sub_82CA9F20(ctx, base);
	// 832563B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832563B8: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832563BC: 3BEAC4D0  addi r31, r10, -0x3b30
	ctx.r[31].s64 = ctx.r[10].s64 + -15152;
	// 832563C0: 3889104C  addi r4, r9, 0x104c
	ctx.r[4].s64 = ctx.r[9].s64 + 4172;
	// 832563C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832563C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832563CC: 4AFD6B05  bl 0x8222ced0
	ctx.lr = 0x832563D0;
	sub_8222CED0(ctx, base);
	// 832563D0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 832563D4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 832563D8: 91688738  stw r11, -0x78c8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-30920 as u32), ctx.r[11].u32 ) };
	// 832563DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832563E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832563E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832563E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832563EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832563F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832563F0 size=56
    let mut pc: u32 = 0x832563F0;
    'dispatch: loop {
        match pc {
            0x832563F0 => {
    //   block [0x832563F0..0x83256428)
	// 832563F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832563F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832563F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832563FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256404: 386B1068  addi r3, r11, 0x1068
	ctx.r[3].s64 = ctx.r[11].s64 + 4200;
	// 83256408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325640C: 4AF9D94D  bl 0x821f3d58
	ctx.lr = 0x83256410;
	sub_821F3D58(ctx, base);
	// 83256410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256414: 906A873C  stw r3, -0x78c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30916 as u32), ctx.r[3].u32 ) };
	// 83256418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256428 size=56
    let mut pc: u32 = 0x83256428;
    'dispatch: loop {
        match pc {
            0x83256428 => {
    //   block [0x83256428..0x83256460)
	// 83256428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256434: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325643C: 386BABCC  addi r3, r11, -0x5434
	ctx.r[3].s64 = ctx.r[11].s64 + -21556;
	// 83256440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256444: 4AF9D915  bl 0x821f3d58
	ctx.lr = 0x83256448;
	sub_821F3D58(ctx, base);
	// 83256448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325644C: 906A8740  stw r3, -0x78c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30912 as u32), ctx.r[3].u32 ) };
	// 83256450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325645C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256460 size=56
    let mut pc: u32 = 0x83256460;
    'dispatch: loop {
        match pc {
            0x83256460 => {
    //   block [0x83256460..0x83256498)
	// 83256460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325646C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83256470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256474: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 83256478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325647C: 4AF9D8DD  bl 0x821f3d58
	ctx.lr = 0x83256480;
	sub_821F3D58(ctx, base);
	// 83256480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256484: 906A8744  stw r3, -0x78bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30908 as u32), ctx.r[3].u32 ) };
	// 83256488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325648C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256498 size=56
    let mut pc: u32 = 0x83256498;
    'dispatch: loop {
        match pc {
            0x83256498 => {
    //   block [0x83256498..0x832564D0)
	// 83256498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832564A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564AC: 386BABD8  addi r3, r11, -0x5428
	ctx.r[3].s64 = ctx.r[11].s64 + -21544;
	// 832564B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564B4: 4AF9D8A5  bl 0x821f3d58
	ctx.lr = 0x832564B8;
	sub_821F3D58(ctx, base);
	// 832564B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564BC: 906A8748  stw r3, -0x78b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30904 as u32), ctx.r[3].u32 ) };
	// 832564C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832564C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832564CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832564D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832564D0 size=56
    let mut pc: u32 = 0x832564D0;
    'dispatch: loop {
        match pc {
            0x832564D0 => {
    //   block [0x832564D0..0x83256508)
	// 832564D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832564D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832564D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832564DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832564E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832564E4: 386B1074  addi r3, r11, 0x1074
	ctx.r[3].s64 = ctx.r[11].s64 + 4212;
	// 832564E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832564EC: 4AF9D86D  bl 0x821f3d58
	ctx.lr = 0x832564F0;
	sub_821F3D58(ctx, base);
	// 832564F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832564F4: 906A874C  stw r3, -0x78b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30900 as u32), ctx.r[3].u32 ) };
	// 832564F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832564FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256508 size=56
    let mut pc: u32 = 0x83256508;
    'dispatch: loop {
        match pc {
            0x83256508 => {
    //   block [0x83256508..0x83256540)
	// 83256508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256514: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256518: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325651C: 386B1080  addi r3, r11, 0x1080
	ctx.r[3].s64 = ctx.r[11].s64 + 4224;
	// 83256520: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256524: 4AF9D835  bl 0x821f3d58
	ctx.lr = 0x83256528;
	sub_821F3D58(ctx, base);
	// 83256528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325652C: 906A8750  stw r3, -0x78b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30896 as u32), ctx.r[3].u32 ) };
	// 83256530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325653C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256540 size=56
    let mut pc: u32 = 0x83256540;
    'dispatch: loop {
        match pc {
            0x83256540 => {
    //   block [0x83256540..0x83256578)
	// 83256540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325654C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256550: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256554: 386B108C  addi r3, r11, 0x108c
	ctx.r[3].s64 = ctx.r[11].s64 + 4236;
	// 83256558: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325655C: 4AF9D7FD  bl 0x821f3d58
	ctx.lr = 0x83256560;
	sub_821F3D58(ctx, base);
	// 83256560: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256564: 906A8754  stw r3, -0x78ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30892 as u32), ctx.r[3].u32 ) };
	// 83256568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256578 size=56
    let mut pc: u32 = 0x83256578;
    'dispatch: loop {
        match pc {
            0x83256578 => {
    //   block [0x83256578..0x832565B0)
	// 83256578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256584: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256588: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325658C: 386B109C  addi r3, r11, 0x109c
	ctx.r[3].s64 = ctx.r[11].s64 + 4252;
	// 83256590: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256594: 4AF9D7C5  bl 0x821f3d58
	ctx.lr = 0x83256598;
	sub_821F3D58(ctx, base);
	// 83256598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325659C: 906A8758  stw r3, -0x78a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30888 as u32), ctx.r[3].u32 ) };
	// 832565A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832565B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565B0 size=56
    let mut pc: u32 = 0x832565B0;
    'dispatch: loop {
        match pc {
            0x832565B0 => {
    //   block [0x832565B0..0x832565E8)
	// 832565B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565C4: 386B10AC  addi r3, r11, 0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + 4268;
	// 832565C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832565CC: 4AF9D78D  bl 0x821f3d58
	ctx.lr = 0x832565D0;
	sub_821F3D58(ctx, base);
	// 832565D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832565D4: 906A875C  stw r3, -0x78a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30884 as u32), ctx.r[3].u32 ) };
	// 832565D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832565DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832565E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832565E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832565E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832565E8 size=56
    let mut pc: u32 = 0x832565E8;
    'dispatch: loop {
        match pc {
            0x832565E8 => {
    //   block [0x832565E8..0x83256620)
	// 832565E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832565EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832565F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832565F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832565F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832565FC: 386B10B8  addi r3, r11, 0x10b8
	ctx.r[3].s64 = ctx.r[11].s64 + 4280;
	// 83256600: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256604: 4AF9D755  bl 0x821f3d58
	ctx.lr = 0x83256608;
	sub_821F3D58(ctx, base);
	// 83256608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325660C: 906A8760  stw r3, -0x78a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30880 as u32), ctx.r[3].u32 ) };
	// 83256610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325661C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256620 size=56
    let mut pc: u32 = 0x83256620;
    'dispatch: loop {
        match pc {
            0x83256620 => {
    //   block [0x83256620..0x83256658)
	// 83256620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325662C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256630: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83256634: 386B10D0  addi r3, r11, 0x10d0
	ctx.r[3].s64 = ctx.r[11].s64 + 4304;
	// 83256638: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325663C: 4AF9D71D  bl 0x821f3d58
	ctx.lr = 0x83256640;
	sub_821F3D58(ctx, base);
	// 83256640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256644: 906A8764  stw r3, -0x789c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30876 as u32), ctx.r[3].u32 ) };
	// 83256648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256658 size=56
    let mut pc: u32 = 0x83256658;
    'dispatch: loop {
        match pc {
            0x83256658 => {
    //   block [0x83256658..0x83256690)
	// 83256658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256664: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256668: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325666C: 386B10E4  addi r3, r11, 0x10e4
	ctx.r[3].s64 = ctx.r[11].s64 + 4324;
	// 83256670: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83256674: 4AF9D6E5  bl 0x821f3d58
	ctx.lr = 0x83256678;
	sub_821F3D58(ctx, base);
	// 83256678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325667C: 906A8768  stw r3, -0x7898(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30872 as u32), ctx.r[3].u32 ) };
	// 83256680: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325668C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256690 size=64
    let mut pc: u32 = 0x83256690;
    'dispatch: loop {
        match pc {
            0x83256690 => {
    //   block [0x83256690..0x832566D0)
	// 83256690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325669C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832566A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832566A4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832566A8: 386A876C  addi r3, r10, -0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + -30868;
	// 832566AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832566B0: 4AFD6821  bl 0x8222ced0
	ctx.lr = 0x832566B4;
	sub_8222CED0(ctx, base);
	// 832566B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832566B8: 3869A888  addi r3, r9, -0x5778
	ctx.r[3].s64 = ctx.r[9].s64 + -22392;
	// 832566BC: 4BA53865  bl 0x82ca9f20
	ctx.lr = 0x832566C0;
	sub_82CA9F20(ctx, base);
	// 832566C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832566C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832566C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832566CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832566D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832566D0 size=64
    let mut pc: u32 = 0x832566D0;
    'dispatch: loop {
        match pc {
            0x832566D0 => {
    //   block [0x832566D0..0x83256710)
	// 832566D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832566D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832566D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832566DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832566E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832566E4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832566E8: 386A8770  addi r3, r10, -0x7890
	ctx.r[3].s64 = ctx.r[10].s64 + -30864;
	// 832566EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832566F0: 4AFD67E1  bl 0x8222ced0
	ctx.lr = 0x832566F4;
	sub_8222CED0(ctx, base);
	// 832566F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832566F8: 3869A898  addi r3, r9, -0x5768
	ctx.r[3].s64 = ctx.r[9].s64 + -22376;
	// 832566FC: 4BA53825  bl 0x82ca9f20
	ctx.lr = 0x83256700;
	sub_82CA9F20(ctx, base);
	// 83256700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325670C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256710 size=64
    let mut pc: u32 = 0x83256710;
    'dispatch: loop {
        match pc {
            0x83256710 => {
    //   block [0x83256710..0x83256750)
	// 83256710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325671C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256720: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256724: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256728: 386A8774  addi r3, r10, -0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + -30860;
	// 8325672C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256730: 4AFD67A1  bl 0x8222ced0
	ctx.lr = 0x83256734;
	sub_8222CED0(ctx, base);
	// 83256734: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256738: 3869A8A8  addi r3, r9, -0x5758
	ctx.r[3].s64 = ctx.r[9].s64 + -22360;
	// 8325673C: 4BA537E5  bl 0x82ca9f20
	ctx.lr = 0x83256740;
	sub_82CA9F20(ctx, base);
	// 83256740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325674C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256750 size=64
    let mut pc: u32 = 0x83256750;
    'dispatch: loop {
        match pc {
            0x83256750 => {
    //   block [0x83256750..0x83256790)
	// 83256750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325675C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256760: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256764: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256768: 386A8778  addi r3, r10, -0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + -30856;
	// 8325676C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256770: 4AFD6761  bl 0x8222ced0
	ctx.lr = 0x83256774;
	sub_8222CED0(ctx, base);
	// 83256774: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256778: 3869A8B8  addi r3, r9, -0x5748
	ctx.r[3].s64 = ctx.r[9].s64 + -22344;
	// 8325677C: 4BA537A5  bl 0x82ca9f20
	ctx.lr = 0x83256780;
	sub_82CA9F20(ctx, base);
	// 83256780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325678C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256790 size=64
    let mut pc: u32 = 0x83256790;
    'dispatch: loop {
        match pc {
            0x83256790 => {
    //   block [0x83256790..0x832567D0)
	// 83256790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325679C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832567A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832567A4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832567A8: 386A877C  addi r3, r10, -0x7884
	ctx.r[3].s64 = ctx.r[10].s64 + -30852;
	// 832567AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832567B0: 4AFD6721  bl 0x8222ced0
	ctx.lr = 0x832567B4;
	sub_8222CED0(ctx, base);
	// 832567B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832567B8: 3869A8C8  addi r3, r9, -0x5738
	ctx.r[3].s64 = ctx.r[9].s64 + -22328;
	// 832567BC: 4BA53765  bl 0x82ca9f20
	ctx.lr = 0x832567C0;
	sub_82CA9F20(ctx, base);
	// 832567C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832567C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832567C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832567CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832567D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832567D0 size=64
    let mut pc: u32 = 0x832567D0;
    'dispatch: loop {
        match pc {
            0x832567D0 => {
    //   block [0x832567D0..0x83256810)
	// 832567D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832567D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832567D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832567DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832567E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832567E4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832567E8: 386A8780  addi r3, r10, -0x7880
	ctx.r[3].s64 = ctx.r[10].s64 + -30848;
	// 832567EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832567F0: 4AFD66E1  bl 0x8222ced0
	ctx.lr = 0x832567F4;
	sub_8222CED0(ctx, base);
	// 832567F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832567F8: 3869A8D8  addi r3, r9, -0x5728
	ctx.r[3].s64 = ctx.r[9].s64 + -22312;
	// 832567FC: 4BA53725  bl 0x82ca9f20
	ctx.lr = 0x83256800;
	sub_82CA9F20(ctx, base);
	// 83256800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325680C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256810 size=52
    let mut pc: u32 = 0x83256810;
    'dispatch: loop {
        match pc {
            0x83256810 => {
    //   block [0x83256810..0x83256844)
	// 83256810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325681C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256820: 386B8784  addi r3, r11, -0x787c
	ctx.r[3].s64 = ctx.r[11].s64 + -30844;
	// 83256824: 4B7B3D8D  bl 0x82a0a5b0
	ctx.lr = 0x83256828;
	sub_82A0A5B0(ctx, base);
	// 83256828: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325682C: 386AA8E8  addi r3, r10, -0x5718
	ctx.r[3].s64 = ctx.r[10].s64 + -22296;
	// 83256830: 4BA536F1  bl 0x82ca9f20
	ctx.lr = 0x83256834;
	sub_82CA9F20(ctx, base);
	// 83256834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325683C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256848 size=52
    let mut pc: u32 = 0x83256848;
    'dispatch: loop {
        match pc {
            0x83256848 => {
    //   block [0x83256848..0x8325687C)
	// 83256848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256854: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256858: 386B8790  addi r3, r11, -0x7870
	ctx.r[3].s64 = ctx.r[11].s64 + -30832;
	// 8325685C: 4B7B3D55  bl 0x82a0a5b0
	ctx.lr = 0x83256860;
	sub_82A0A5B0(ctx, base);
	// 83256860: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256864: 386AA8F8  addi r3, r10, -0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + -22280;
	// 83256868: 4BA536B9  bl 0x82ca9f20
	ctx.lr = 0x8325686C;
	sub_82CA9F20(ctx, base);
	// 8325686C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256880 size=52
    let mut pc: u32 = 0x83256880;
    'dispatch: loop {
        match pc {
            0x83256880 => {
    //   block [0x83256880..0x832568B4)
	// 83256880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325688C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256890: 386B879C  addi r3, r11, -0x7864
	ctx.r[3].s64 = ctx.r[11].s64 + -30820;
	// 83256894: 4B7B3D1D  bl 0x82a0a5b0
	ctx.lr = 0x83256898;
	sub_82A0A5B0(ctx, base);
	// 83256898: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325689C: 386AA908  addi r3, r10, -0x56f8
	ctx.r[3].s64 = ctx.r[10].s64 + -22264;
	// 832568A0: 4BA53681  bl 0x82ca9f20
	ctx.lr = 0x832568A4;
	sub_82CA9F20(ctx, base);
	// 832568A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832568A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832568AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832568B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832568B8 size=20
    let mut pc: u32 = 0x832568B8;
    'dispatch: loop {
        match pc {
            0x832568B8 => {
    //   block [0x832568B8..0x832568CC)
	// 832568B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832568BC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832568C0: C00B9F18  lfs f0, -0x60e8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832568C4: D00A87A8  stfs f0, -0x7858(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30808 as u32), tmp.u32 ) };
	// 832568C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832568D0 size=20
    let mut pc: u32 = 0x832568D0;
    'dispatch: loop {
        match pc {
            0x832568D0 => {
    //   block [0x832568D0..0x832568E4)
	// 832568D0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832568D4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832568D8: C00B9F1C  lfs f0, -0x60e4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24804 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832568DC: D00A87AC  stfs f0, -0x7854(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30804 as u32), tmp.u32 ) };
	// 832568E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832568E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832568E8 size=20
    let mut pc: u32 = 0x832568E8;
    'dispatch: loop {
        match pc {
            0x832568E8 => {
    //   block [0x832568E8..0x832568FC)
	// 832568E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832568EC: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 832568F0: 394B87B0  addi r10, r11, -0x7850
	ctx.r[10].s64 = ctx.r[11].s64 + -30800;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256900 size=20
    let mut pc: u32 = 0x83256900;
    'dispatch: loop {
        match pc {
            0x83256900 => {
    //   block [0x83256900..0x83256914)
	// 83256900: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256904: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256908: 394B87C0  addi r10, r11, -0x7840
	ctx.r[10].s64 = ctx.r[11].s64 + -30784;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256918 size=20
    let mut pc: u32 = 0x83256918;
    'dispatch: loop {
        match pc {
            0x83256918 => {
    //   block [0x83256918..0x8325692C)
	// 83256918: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8325691C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256920: 394B87D0  addi r10, r11, -0x7830
	ctx.r[10].s64 = ctx.r[11].s64 + -30768;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256930 size=20
    let mut pc: u32 = 0x83256930;
    'dispatch: loop {
        match pc {
            0x83256930 => {
    //   block [0x83256930..0x83256944)
	// 83256930: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256934: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83256938: 394B87E0  addi r10, r11, -0x7820
	ctx.r[10].s64 = ctx.r[11].s64 + -30752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256948 size=96
    let mut pc: u32 = 0x83256948;
    'dispatch: loop {
        match pc {
            0x83256948 => {
    //   block [0x83256948..0x8325696C)
	// 83256948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325694C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256954: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 83256958: 4AFC8901  bl 0x8221f258
	ctx.lr = 0x8325695C;
	sub_8221F258(ctx, base);
	// 8325695C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83256960: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83256964: 419A0008  beq cr6, 0x8325696c
	if ctx.cr[6].eq {
	pc = 0x8325696C; continue 'dispatch;
	}
	// 83256968: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8325696C; continue 'dispatch;
            }
            0x8325696C => {
    //   block [0x8325696C..0x83256978)
	// 8325696C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83256970: 41820008  beq 0x83256978
	if ctx.cr[0].eq {
	pc = 0x83256978; continue 'dispatch;
	}
	// 83256974: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83256978; continue 'dispatch;
            }
            0x83256978 => {
    //   block [0x83256978..0x832569A8)
	// 83256978: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8325697C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83256980: 390987F0  addi r8, r9, -0x7810
	ctx.r[8].s64 = ctx.r[9].s64 + -30736;
	// 83256984: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83256988: 3867A918  addi r3, r7, -0x56e8
	ctx.r[3].s64 = ctx.r[7].s64 + -22248;
	// 8325698C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83256990: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83256994: 4BA5358D  bl 0x82ca9f20
	ctx.lr = 0x83256998;
	sub_82CA9F20(ctx, base);
	// 83256998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325699C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832569A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832569A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832569A8 size=12
    let mut pc: u32 = 0x832569A8;
    'dispatch: loop {
        match pc {
            0x832569A8 => {
    //   block [0x832569A8..0x832569B4)
	// 832569A8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832569AC: 386BA9B0  addi r3, r11, -0x5650
	ctx.r[3].s64 = ctx.r[11].s64 + -22096;
	// 832569B0: 4BA53570  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569B8 size=64
    let mut pc: u32 = 0x832569B8;
    'dispatch: loop {
        match pc {
            0x832569B8 => {
    //   block [0x832569B8..0x832569F8)
	// 832569B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832569C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832569C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832569C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832569CC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832569D0: 386A880C  addi r3, r10, -0x77f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30708;
	// 832569D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832569D8: 4AFD64F9  bl 0x8222ced0
	ctx.lr = 0x832569DC;
	sub_8222CED0(ctx, base);
	// 832569DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832569E0: 3869AA08  addi r3, r9, -0x55f8
	ctx.r[3].s64 = ctx.r[9].s64 + -22008;
	// 832569E4: 4BA5353D  bl 0x82ca9f20
	ctx.lr = 0x832569E8;
	sub_82CA9F20(ctx, base);
	// 832569E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832569EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832569F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832569F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832569F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832569F8 size=64
    let mut pc: u32 = 0x832569F8;
    'dispatch: loop {
        match pc {
            0x832569F8 => {
    //   block [0x832569F8..0x83256A38)
	// 832569F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832569FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256A10: 386A8810  addi r3, r10, -0x77f0
	ctx.r[3].s64 = ctx.r[10].s64 + -30704;
	// 83256A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A18: 4AFD64B9  bl 0x8222ced0
	ctx.lr = 0x83256A1C;
	sub_8222CED0(ctx, base);
	// 83256A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256A20: 3869AA18  addi r3, r9, -0x55e8
	ctx.r[3].s64 = ctx.r[9].s64 + -21992;
	// 83256A24: 4BA534FD  bl 0x82ca9f20
	ctx.lr = 0x83256A28;
	sub_82CA9F20(ctx, base);
	// 83256A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A38 size=64
    let mut pc: u32 = 0x83256A38;
    'dispatch: loop {
        match pc {
            0x83256A38 => {
    //   block [0x83256A38..0x83256A78)
	// 83256A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256A50: 386A8814  addi r3, r10, -0x77ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30700;
	// 83256A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A58: 4AFD6479  bl 0x8222ced0
	ctx.lr = 0x83256A5C;
	sub_8222CED0(ctx, base);
	// 83256A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256A60: 3869AA28  addi r3, r9, -0x55d8
	ctx.r[3].s64 = ctx.r[9].s64 + -21976;
	// 83256A64: 4BA534BD  bl 0x82ca9f20
	ctx.lr = 0x83256A68;
	sub_82CA9F20(ctx, base);
	// 83256A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256A78 size=64
    let mut pc: u32 = 0x83256A78;
    'dispatch: loop {
        match pc {
            0x83256A78 => {
    //   block [0x83256A78..0x83256AB8)
	// 83256A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256A8C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256A90: 386A8818  addi r3, r10, -0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30696;
	// 83256A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256A98: 4AFD6439  bl 0x8222ced0
	ctx.lr = 0x83256A9C;
	sub_8222CED0(ctx, base);
	// 83256A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256AA0: 3869AA38  addi r3, r9, -0x55c8
	ctx.r[3].s64 = ctx.r[9].s64 + -21960;
	// 83256AA4: 4BA5347D  bl 0x82ca9f20
	ctx.lr = 0x83256AA8;
	sub_82CA9F20(ctx, base);
	// 83256AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AB8 size=64
    let mut pc: u32 = 0x83256AB8;
    'dispatch: loop {
        match pc {
            0x83256AB8 => {
    //   block [0x83256AB8..0x83256AF8)
	// 83256AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256ACC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256AD0: 386A881C  addi r3, r10, -0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30692;
	// 83256AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256AD8: 4AFD63F9  bl 0x8222ced0
	ctx.lr = 0x83256ADC;
	sub_8222CED0(ctx, base);
	// 83256ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256AE0: 3869AA48  addi r3, r9, -0x55b8
	ctx.r[3].s64 = ctx.r[9].s64 + -21944;
	// 83256AE4: 4BA5343D  bl 0x82ca9f20
	ctx.lr = 0x83256AE8;
	sub_82CA9F20(ctx, base);
	// 83256AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256AF8 size=64
    let mut pc: u32 = 0x83256AF8;
    'dispatch: loop {
        match pc {
            0x83256AF8 => {
    //   block [0x83256AF8..0x83256B38)
	// 83256AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256B0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256B10: 386A8820  addi r3, r10, -0x77e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30688;
	// 83256B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256B18: 4AFD63B9  bl 0x8222ced0
	ctx.lr = 0x83256B1C;
	sub_8222CED0(ctx, base);
	// 83256B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256B20: 3869AA58  addi r3, r9, -0x55a8
	ctx.r[3].s64 = ctx.r[9].s64 + -21928;
	// 83256B24: 4BA533FD  bl 0x82ca9f20
	ctx.lr = 0x83256B28;
	sub_82CA9F20(ctx, base);
	// 83256B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B38 size=44
    let mut pc: u32 = 0x83256B38;
    'dispatch: loop {
        match pc {
            0x83256B38 => {
    //   block [0x83256B38..0x83256B64)
	// 83256B38: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83256B40: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256B44: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256B48: C9AA0D30  lfd f13, 0xd30(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(3376 as u32) ) };
	// 83256B4C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256B50: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256B54: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256B58: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256B5C: 91698824  stw r11, -0x77dc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30684 as u32), ctx.r[11].u32 ) };
	// 83256B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B68 size=44
    let mut pc: u32 = 0x83256B68;
    'dispatch: loop {
        match pc {
            0x83256B68 => {
    //   block [0x83256B68..0x83256B94)
	// 83256B68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B6C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83256B70: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256B74: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256B78: C9AA1718  lfd f13, 0x1718(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(5912 as u32) ) };
	// 83256B7C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256B80: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256B84: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256B88: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256B8C: 91698828  stw r11, -0x77d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30680 as u32), ctx.r[11].u32 ) };
	// 83256B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256B98 size=32
    let mut pc: u32 = 0x83256B98;
    'dispatch: loop {
        match pc {
            0x83256B98 => {
    //   block [0x83256B98..0x83256BB8)
	// 83256B98: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256B9C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256BA0: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256BA4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256BA8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 83256BAC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256BB0: 916A882C  stw r11, -0x77d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30676 as u32), ctx.r[11].u32 ) };
	// 83256BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256BB8 size=52
    let mut pc: u32 = 0x83256BB8;
    'dispatch: loop {
        match pc {
            0x83256BB8 => {
    //   block [0x83256BB8..0x83256BEC)
	// 83256BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256BC4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256BC8: 386B9F34  addi r3, r11, -0x60cc
	ctx.r[3].s64 = ctx.r[11].s64 + -24780;
	// 83256BCC: 4AF3D26D  bl 0x82193e38
	ctx.lr = 0x83256BD0;
	sub_82193E38(ctx, base);
	// 83256BD0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256BD4: 386AAA68  addi r3, r10, -0x5598
	ctx.r[3].s64 = ctx.r[10].s64 + -21912;
	// 83256BD8: 4BA53349  bl 0x82ca9f20
	ctx.lr = 0x83256BDC;
	sub_82CA9F20(ctx, base);
	// 83256BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256BF0 size=12
    let mut pc: u32 = 0x83256BF0;
    'dispatch: loop {
        match pc {
            0x83256BF0 => {
    //   block [0x83256BF0..0x83256BFC)
	// 83256BF0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256BF4: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 83256BF8: 4BA53328  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256C00 size=12
    let mut pc: u32 = 0x83256C00;
    'dispatch: loop {
        match pc {
            0x83256C00 => {
    //   block [0x83256C00..0x83256C0C)
	// 83256C00: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256C04: 386BAAF8  addi r3, r11, -0x5508
	ctx.r[3].s64 = ctx.r[11].s64 + -21768;
	// 83256C08: 4BA53318  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256C10 size=12
    let mut pc: u32 = 0x83256C10;
    'dispatch: loop {
        match pc {
            0x83256C10 => {
    //   block [0x83256C10..0x83256C1C)
	// 83256C10: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83256C14: 386BAB08  addi r3, r11, -0x54f8
	ctx.r[3].s64 = ctx.r[11].s64 + -21752;
	// 83256C18: 4BA53308  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C20 size=64
    let mut pc: u32 = 0x83256C20;
    'dispatch: loop {
        match pc {
            0x83256C20 => {
    //   block [0x83256C20..0x83256C60)
	// 83256C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256C2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256C34: 388B0A88  addi r4, r11, 0xa88
	ctx.r[4].s64 = ctx.r[11].s64 + 2696;
	// 83256C38: 386A883C  addi r3, r10, -0x77c4
	ctx.r[3].s64 = ctx.r[10].s64 + -30660;
	// 83256C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256C40: 4AFD6291  bl 0x8222ced0
	ctx.lr = 0x83256C44;
	sub_8222CED0(ctx, base);
	// 83256C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256C48: 3869AB80  addi r3, r9, -0x5480
	ctx.r[3].s64 = ctx.r[9].s64 + -21632;
	// 83256C4C: 4BA532D5  bl 0x82ca9f20
	ctx.lr = 0x83256C50;
	sub_82CA9F20(ctx, base);
	// 83256C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256C60 size=64
    let mut pc: u32 = 0x83256C60;
    'dispatch: loop {
        match pc {
            0x83256C60 => {
    //   block [0x83256C60..0x83256CA0)
	// 83256C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256C6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256C74: 388B0E18  addi r4, r11, 0xe18
	ctx.r[4].s64 = ctx.r[11].s64 + 3608;
	// 83256C78: 386A8840  addi r3, r10, -0x77c0
	ctx.r[3].s64 = ctx.r[10].s64 + -30656;
	// 83256C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256C80: 4AFD6251  bl 0x8222ced0
	ctx.lr = 0x83256C84;
	sub_8222CED0(ctx, base);
	// 83256C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256C88: 3869AB90  addi r3, r9, -0x5470
	ctx.r[3].s64 = ctx.r[9].s64 + -21616;
	// 83256C8C: 4BA53295  bl 0x82ca9f20
	ctx.lr = 0x83256C90;
	sub_82CA9F20(ctx, base);
	// 83256C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CA0 size=64
    let mut pc: u32 = 0x83256CA0;
    'dispatch: loop {
        match pc {
            0x83256CA0 => {
    //   block [0x83256CA0..0x83256CE0)
	// 83256CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256CAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83256CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256CB4: 388B2EE0  addi r4, r11, 0x2ee0
	ctx.r[4].s64 = ctx.r[11].s64 + 12000;
	// 83256CB8: 386A8844  addi r3, r10, -0x77bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30652;
	// 83256CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256CC0: 4AFD6211  bl 0x8222ced0
	ctx.lr = 0x83256CC4;
	sub_8222CED0(ctx, base);
	// 83256CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256CC8: 3869ABA0  addi r3, r9, -0x5460
	ctx.r[3].s64 = ctx.r[9].s64 + -21600;
	// 83256CCC: 4BA53255  bl 0x82ca9f20
	ctx.lr = 0x83256CD0;
	sub_82CA9F20(ctx, base);
	// 83256CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256CE0 size=64
    let mut pc: u32 = 0x83256CE0;
    'dispatch: loop {
        match pc {
            0x83256CE0 => {
    //   block [0x83256CE0..0x83256D20)
	// 83256CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256CEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256CF4: 388B0E58  addi r4, r11, 0xe58
	ctx.r[4].s64 = ctx.r[11].s64 + 3672;
	// 83256CF8: 386A8848  addi r3, r10, -0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30648;
	// 83256CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256D00: 4AFD61D1  bl 0x8222ced0
	ctx.lr = 0x83256D04;
	sub_8222CED0(ctx, base);
	// 83256D04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256D08: 3869ABB0  addi r3, r9, -0x5450
	ctx.r[3].s64 = ctx.r[9].s64 + -21584;
	// 83256D0C: 4BA53215  bl 0x82ca9f20
	ctx.lr = 0x83256D10;
	sub_82CA9F20(ctx, base);
	// 83256D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D20 size=52
    let mut pc: u32 = 0x83256D20;
    'dispatch: loop {
        match pc {
            0x83256D20 => {
    //   block [0x83256D20..0x83256D54)
	// 83256D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256D2C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83256D30: 386B884C  addi r3, r11, -0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + -30644;
	// 83256D34: 4B229505  bl 0x82480238
	ctx.lr = 0x83256D38;
	sub_82480238(ctx, base);
	// 83256D38: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83256D3C: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 83256D40: 4BA531E1  bl 0x82ca9f20
	ctx.lr = 0x83256D44;
	sub_82CA9F20(ctx, base);
	// 83256D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D58 size=64
    let mut pc: u32 = 0x83256D58;
    'dispatch: loop {
        match pc {
            0x83256D58 => {
    //   block [0x83256D58..0x83256D98)
	// 83256D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256D64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256D68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256D6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256D70: 386A8858  addi r3, r10, -0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30632;
	// 83256D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256D78: 4AFD6159  bl 0x8222ced0
	ctx.lr = 0x83256D7C;
	sub_8222CED0(ctx, base);
	// 83256D7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256D80: 3869ABD0  addi r3, r9, -0x5430
	ctx.r[3].s64 = ctx.r[9].s64 + -21552;
	// 83256D84: 4BA5319D  bl 0x82ca9f20
	ctx.lr = 0x83256D88;
	sub_82CA9F20(ctx, base);
	// 83256D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256D98 size=64
    let mut pc: u32 = 0x83256D98;
    'dispatch: loop {
        match pc {
            0x83256D98 => {
    //   block [0x83256D98..0x83256DD8)
	// 83256D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256DAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256DB0: 386A885C  addi r3, r10, -0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30628;
	// 83256DB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256DB8: 4AFD6119  bl 0x8222ced0
	ctx.lr = 0x83256DBC;
	sub_8222CED0(ctx, base);
	// 83256DBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256DC0: 3869ABE0  addi r3, r9, -0x5420
	ctx.r[3].s64 = ctx.r[9].s64 + -21536;
	// 83256DC4: 4BA5315D  bl 0x82ca9f20
	ctx.lr = 0x83256DC8;
	sub_82CA9F20(ctx, base);
	// 83256DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256DD8 size=64
    let mut pc: u32 = 0x83256DD8;
    'dispatch: loop {
        match pc {
            0x83256DD8 => {
    //   block [0x83256DD8..0x83256E18)
	// 83256DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256DE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256DE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256DEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256DF0: 386A8860  addi r3, r10, -0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30624;
	// 83256DF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256DF8: 4AFD60D9  bl 0x8222ced0
	ctx.lr = 0x83256DFC;
	sub_8222CED0(ctx, base);
	// 83256DFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256E00: 3869ABF0  addi r3, r9, -0x5410
	ctx.r[3].s64 = ctx.r[9].s64 + -21520;
	// 83256E04: 4BA5311D  bl 0x82ca9f20
	ctx.lr = 0x83256E08;
	sub_82CA9F20(ctx, base);
	// 83256E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83256E18 size=44
    let mut pc: u32 = 0x83256E18;
    'dispatch: loop {
        match pc {
            0x83256E18 => {
    //   block [0x83256E18..0x83256E44)
	// 83256E18: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83256E1C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83256E20: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83256E24: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 83256E28: C9AA11A8  lfd f13, 0x11a8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(4520 as u32) ) };
	// 83256E2C: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 83256E30: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83256E34: D9A1FFF0  stfd f13, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[13].u64 ) };
	// 83256E38: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83256E3C: 91698864  stw r11, -0x779c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-30620 as u32), ctx.r[11].u32 ) };
	// 83256E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E48 size=64
    let mut pc: u32 = 0x83256E48;
    'dispatch: loop {
        match pc {
            0x83256E48 => {
    //   block [0x83256E48..0x83256E88)
	// 83256E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256E54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256E5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256E60: 386A8868  addi r3, r10, -0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + -30616;
	// 83256E64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256E68: 4AFD6069  bl 0x8222ced0
	ctx.lr = 0x83256E6C;
	sub_8222CED0(ctx, base);
	// 83256E6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256E70: 3869AC00  addi r3, r9, -0x5400
	ctx.r[3].s64 = ctx.r[9].s64 + -21504;
	// 83256E74: 4BA530AD  bl 0x82ca9f20
	ctx.lr = 0x83256E78;
	sub_82CA9F20(ctx, base);
	// 83256E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256E88 size=64
    let mut pc: u32 = 0x83256E88;
    'dispatch: loop {
        match pc {
            0x83256E88 => {
    //   block [0x83256E88..0x83256EC8)
	// 83256E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256E9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256EA0: 386A886C  addi r3, r10, -0x7794
	ctx.r[3].s64 = ctx.r[10].s64 + -30612;
	// 83256EA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256EA8: 4AFD6029  bl 0x8222ced0
	ctx.lr = 0x83256EAC;
	sub_8222CED0(ctx, base);
	// 83256EAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256EB0: 3869AC10  addi r3, r9, -0x53f0
	ctx.r[3].s64 = ctx.r[9].s64 + -21488;
	// 83256EB4: 4BA5306D  bl 0x82ca9f20
	ctx.lr = 0x83256EB8;
	sub_82CA9F20(ctx, base);
	// 83256EB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256EC8 size=64
    let mut pc: u32 = 0x83256EC8;
    'dispatch: loop {
        match pc {
            0x83256EC8 => {
    //   block [0x83256EC8..0x83256F08)
	// 83256EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256ED4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256ED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256EDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256EE0: 386A8870  addi r3, r10, -0x7790
	ctx.r[3].s64 = ctx.r[10].s64 + -30608;
	// 83256EE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256EE8: 4AFD5FE9  bl 0x8222ced0
	ctx.lr = 0x83256EEC;
	sub_8222CED0(ctx, base);
	// 83256EEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256EF0: 3869AC20  addi r3, r9, -0x53e0
	ctx.r[3].s64 = ctx.r[9].s64 + -21472;
	// 83256EF4: 4BA5302D  bl 0x82ca9f20
	ctx.lr = 0x83256EF8;
	sub_82CA9F20(ctx, base);
	// 83256EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F08 size=64
    let mut pc: u32 = 0x83256F08;
    'dispatch: loop {
        match pc {
            0x83256F08 => {
    //   block [0x83256F08..0x83256F48)
	// 83256F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83256F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F1C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83256F20: 386A8874  addi r3, r10, -0x778c
	ctx.r[3].s64 = ctx.r[10].s64 + -30604;
	// 83256F24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256F28: 4AFD5FA9  bl 0x8222ced0
	ctx.lr = 0x83256F2C;
	sub_8222CED0(ctx, base);
	// 83256F2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256F30: 3869AC30  addi r3, r9, -0x53d0
	ctx.r[3].s64 = ctx.r[9].s64 + -21456;
	// 83256F34: 4BA52FED  bl 0x82ca9f20
	ctx.lr = 0x83256F38;
	sub_82CA9F20(ctx, base);
	// 83256F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F48 size=64
    let mut pc: u32 = 0x83256F48;
    'dispatch: loop {
        match pc {
            0x83256F48 => {
    //   block [0x83256F48..0x83256F88)
	// 83256F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256F58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83256F60: 386A8878  addi r3, r10, -0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + -30600;
	// 83256F64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256F68: 4AFD5F69  bl 0x8222ced0
	ctx.lr = 0x83256F6C;
	sub_8222CED0(ctx, base);
	// 83256F6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256F70: 3869AC48  addi r3, r9, -0x53b8
	ctx.r[3].s64 = ctx.r[9].s64 + -21432;
	// 83256F74: 4BA52FAD  bl 0x82ca9f20
	ctx.lr = 0x83256F78;
	sub_82CA9F20(ctx, base);
	// 83256F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256F88 size=64
    let mut pc: u32 = 0x83256F88;
    'dispatch: loop {
        match pc {
            0x83256F88 => {
    //   block [0x83256F88..0x83256FC8)
	// 83256F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256F98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256F9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83256FA0: 386A887C  addi r3, r10, -0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + -30596;
	// 83256FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256FA8: 4AFD5F29  bl 0x8222ced0
	ctx.lr = 0x83256FAC;
	sub_8222CED0(ctx, base);
	// 83256FAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256FB0: 3869AC58  addi r3, r9, -0x53a8
	ctx.r[3].s64 = ctx.r[9].s64 + -21416;
	// 83256FB4: 4BA52F6D  bl 0x82ca9f20
	ctx.lr = 0x83256FB8;
	sub_82CA9F20(ctx, base);
	// 83256FB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83256FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83256FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83256FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83256FC8 size=64
    let mut pc: u32 = 0x83256FC8;
    'dispatch: loop {
        match pc {
            0x83256FC8 => {
    //   block [0x83256FC8..0x83257008)
	// 83256FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83256FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83256FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83256FD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83256FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83256FDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83256FE0: 386A8880  addi r3, r10, -0x7780
	ctx.r[3].s64 = ctx.r[10].s64 + -30592;
	// 83256FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83256FE8: 4AFD5EE9  bl 0x8222ced0
	ctx.lr = 0x83256FEC;
	sub_8222CED0(ctx, base);
	// 83256FEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83256FF0: 3869AC68  addi r3, r9, -0x5398
	ctx.r[3].s64 = ctx.r[9].s64 + -21400;
	// 83256FF4: 4BA52F2D  bl 0x82ca9f20
	ctx.lr = 0x83256FF8;
	sub_82CA9F20(ctx, base);
	// 83256FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83256FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257008 size=52
    let mut pc: u32 = 0x83257008;
    'dispatch: loop {
        match pc {
            0x83257008 => {
    //   block [0x83257008..0x8325703C)
	// 83257008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257014: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83257018: 386B8884  addi r3, r11, -0x777c
	ctx.r[3].s64 = ctx.r[11].s64 + -30588;
	// 8325701C: 4B22921D  bl 0x82480238
	ctx.lr = 0x83257020;
	sub_82480238(ctx, base);
	// 83257020: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83257024: 386AAC88  addi r3, r10, -0x5378
	ctx.r[3].s64 = ctx.r[10].s64 + -21368;
	// 83257028: 4BA52EF9  bl 0x82ca9f20
	ctx.lr = 0x8325702C;
	sub_82CA9F20(ctx, base);
	// 8325702C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257040 size=64
    let mut pc: u32 = 0x83257040;
    'dispatch: loop {
        match pc {
            0x83257040 => {
    //   block [0x83257040..0x83257080)
	// 83257040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325704C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257054: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257058: 386A8890  addi r3, r10, -0x7770
	ctx.r[3].s64 = ctx.r[10].s64 + -30576;
	// 8325705C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257060: 4AFD5E71  bl 0x8222ced0
	ctx.lr = 0x83257064;
	sub_8222CED0(ctx, base);
	// 83257064: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257068: 3869ACB8  addi r3, r9, -0x5348
	ctx.r[3].s64 = ctx.r[9].s64 + -21320;
	// 8325706C: 4BA52EB5  bl 0x82ca9f20
	ctx.lr = 0x83257070;
	sub_82CA9F20(ctx, base);
	// 83257070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325707C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257080 size=64
    let mut pc: u32 = 0x83257080;
    'dispatch: loop {
        match pc {
            0x83257080 => {
    //   block [0x83257080..0x832570C0)
	// 83257080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325708C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257094: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257098: 386A8894  addi r3, r10, -0x776c
	ctx.r[3].s64 = ctx.r[10].s64 + -30572;
	// 8325709C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832570A0: 4AFD5E31  bl 0x8222ced0
	ctx.lr = 0x832570A4;
	sub_8222CED0(ctx, base);
	// 832570A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832570A8: 3869ACC8  addi r3, r9, -0x5338
	ctx.r[3].s64 = ctx.r[9].s64 + -21304;
	// 832570AC: 4BA52E75  bl 0x82ca9f20
	ctx.lr = 0x832570B0;
	sub_82CA9F20(ctx, base);
	// 832570B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832570B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832570B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832570BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832570C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832570C0 size=64
    let mut pc: u32 = 0x832570C0;
    'dispatch: loop {
        match pc {
            0x832570C0 => {
    //   block [0x832570C0..0x83257100)
	// 832570C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832570C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832570C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832570CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832570D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832570D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832570D8: 386A8898  addi r3, r10, -0x7768
	ctx.r[3].s64 = ctx.r[10].s64 + -30568;
	// 832570DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832570E0: 4AFD5DF1  bl 0x8222ced0
	ctx.lr = 0x832570E4;
	sub_8222CED0(ctx, base);
	// 832570E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832570E8: 3869ACD8  addi r3, r9, -0x5328
	ctx.r[3].s64 = ctx.r[9].s64 + -21288;
	// 832570EC: 4BA52E35  bl 0x82ca9f20
	ctx.lr = 0x832570F0;
	sub_82CA9F20(ctx, base);
	// 832570F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832570F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832570F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832570FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257100 size=64
    let mut pc: u32 = 0x83257100;
    'dispatch: loop {
        match pc {
            0x83257100 => {
    //   block [0x83257100..0x83257140)
	// 83257100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325710C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257114: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257118: 386A889C  addi r3, r10, -0x7764
	ctx.r[3].s64 = ctx.r[10].s64 + -30564;
	// 8325711C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257120: 4AFD5DB1  bl 0x8222ced0
	ctx.lr = 0x83257124;
	sub_8222CED0(ctx, base);
	// 83257124: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257128: 3869ACE8  addi r3, r9, -0x5318
	ctx.r[3].s64 = ctx.r[9].s64 + -21272;
	// 8325712C: 4BA52DF5  bl 0x82ca9f20
	ctx.lr = 0x83257130;
	sub_82CA9F20(ctx, base);
	// 83257130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325713C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257140 size=64
    let mut pc: u32 = 0x83257140;
    'dispatch: loop {
        match pc {
            0x83257140 => {
    //   block [0x83257140..0x83257180)
	// 83257140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325714C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257154: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257158: 386A88A0  addi r3, r10, -0x7760
	ctx.r[3].s64 = ctx.r[10].s64 + -30560;
	// 8325715C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257160: 4AFD5D71  bl 0x8222ced0
	ctx.lr = 0x83257164;
	sub_8222CED0(ctx, base);
	// 83257164: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257168: 3869ACF8  addi r3, r9, -0x5308
	ctx.r[3].s64 = ctx.r[9].s64 + -21256;
	// 8325716C: 4BA52DB5  bl 0x82ca9f20
	ctx.lr = 0x83257170;
	sub_82CA9F20(ctx, base);
	// 83257170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257180 size=64
    let mut pc: u32 = 0x83257180;
    'dispatch: loop {
        match pc {
            0x83257180 => {
    //   block [0x83257180..0x832571C0)
	// 83257180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325718C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257194: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257198: 386A88A4  addi r3, r10, -0x775c
	ctx.r[3].s64 = ctx.r[10].s64 + -30556;
	// 8325719C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832571A0: 4AFD5D31  bl 0x8222ced0
	ctx.lr = 0x832571A4;
	sub_8222CED0(ctx, base);
	// 832571A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832571A8: 3869AD08  addi r3, r9, -0x52f8
	ctx.r[3].s64 = ctx.r[9].s64 + -21240;
	// 832571AC: 4BA52D75  bl 0x82ca9f20
	ctx.lr = 0x832571B0;
	sub_82CA9F20(ctx, base);
	// 832571B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832571B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832571B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832571BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832571C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832571C0 size=64
    let mut pc: u32 = 0x832571C0;
    'dispatch: loop {
        match pc {
            0x832571C0 => {
    //   block [0x832571C0..0x83257200)
	// 832571C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832571C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832571C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832571CC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832571D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832571D4: 388B3C3C  addi r4, r11, 0x3c3c
	ctx.r[4].s64 = ctx.r[11].s64 + 15420;
	// 832571D8: 386A88A8  addi r3, r10, -0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + -30552;
	// 832571DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832571E0: 4AFD5CF1  bl 0x8222ced0
	ctx.lr = 0x832571E4;
	sub_8222CED0(ctx, base);
	// 832571E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832571E8: 3869AD18  addi r3, r9, -0x52e8
	ctx.r[3].s64 = ctx.r[9].s64 + -21224;
	// 832571EC: 4BA52D35  bl 0x82ca9f20
	ctx.lr = 0x832571F0;
	sub_82CA9F20(ctx, base);
	// 832571F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832571F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832571F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832571FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257200 size=64
    let mut pc: u32 = 0x83257200;
    'dispatch: loop {
        match pc {
            0x83257200 => {
    //   block [0x83257200..0x83257240)
	// 83257200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325720C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257214: 388B3C48  addi r4, r11, 0x3c48
	ctx.r[4].s64 = ctx.r[11].s64 + 15432;
	// 83257218: 386A88AC  addi r3, r10, -0x7754
	ctx.r[3].s64 = ctx.r[10].s64 + -30548;
	// 8325721C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257220: 4AFD5CB1  bl 0x8222ced0
	ctx.lr = 0x83257224;
	sub_8222CED0(ctx, base);
	// 83257224: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257228: 3869AD28  addi r3, r9, -0x52d8
	ctx.r[3].s64 = ctx.r[9].s64 + -21208;
	// 8325722C: 4BA52CF5  bl 0x82ca9f20
	ctx.lr = 0x83257230;
	sub_82CA9F20(ctx, base);
	// 83257230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325723C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257240 size=96
    let mut pc: u32 = 0x83257240;
    'dispatch: loop {
        match pc {
            0x83257240 => {
    //   block [0x83257240..0x83257264)
	// 83257240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325724C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83257250: 4AFC8009  bl 0x8221f258
	ctx.lr = 0x83257254;
	sub_8221F258(ctx, base);
	// 83257254: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83257258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8325725C: 419A0008  beq cr6, 0x83257264
	if ctx.cr[6].eq {
	pc = 0x83257264; continue 'dispatch;
	}
	// 83257260: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83257264; continue 'dispatch;
            }
            0x83257264 => {
    //   block [0x83257264..0x83257270)
	// 83257264: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83257268: 41820008  beq 0x83257270
	if ctx.cr[0].eq {
	pc = 0x83257270; continue 'dispatch;
	}
	// 8325726C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x83257270; continue 'dispatch;
            }
            0x83257270 => {
    //   block [0x83257270..0x832572A0)
	// 83257270: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83257274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83257278: 390988B0  addi r8, r9, -0x7750
	ctx.r[8].s64 = ctx.r[9].s64 + -30544;
	// 8325727C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83257280: 3867AD38  addi r3, r7, -0x52c8
	ctx.r[3].s64 = ctx.r[7].s64 + -21192;
	// 83257284: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83257288: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8325728C: 4BA52C95  bl 0x82ca9f20
	ctx.lr = 0x83257290;
	sub_82CA9F20(ctx, base);
	// 83257290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325729C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832572A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572A0 size=64
    let mut pc: u32 = 0x832572A0;
    'dispatch: loop {
        match pc {
            0x832572A0 => {
    //   block [0x832572A0..0x832572E0)
	// 832572A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832572AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832572B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832572B4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832572B8: 386A88BC  addi r3, r10, -0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + -30532;
	// 832572BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832572C0: 4AFD5C11  bl 0x8222ced0
	ctx.lr = 0x832572C4;
	sub_8222CED0(ctx, base);
	// 832572C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832572C8: 3869ADC0  addi r3, r9, -0x5240
	ctx.r[3].s64 = ctx.r[9].s64 + -21056;
	// 832572CC: 4BA52C55  bl 0x82ca9f20
	ctx.lr = 0x832572D0;
	sub_82CA9F20(ctx, base);
	// 832572D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832572D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832572D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832572DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832572E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832572E0 size=64
    let mut pc: u32 = 0x832572E0;
    'dispatch: loop {
        match pc {
            0x832572E0 => {
    //   block [0x832572E0..0x83257320)
	// 832572E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832572E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832572E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832572EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832572F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832572F4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832572F8: 386A88C0  addi r3, r10, -0x7740
	ctx.r[3].s64 = ctx.r[10].s64 + -30528;
	// 832572FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257300: 4AFD5BD1  bl 0x8222ced0
	ctx.lr = 0x83257304;
	sub_8222CED0(ctx, base);
	// 83257304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257308: 3869ADD0  addi r3, r9, -0x5230
	ctx.r[3].s64 = ctx.r[9].s64 + -21040;
	// 8325730C: 4BA52C15  bl 0x82ca9f20
	ctx.lr = 0x83257310;
	sub_82CA9F20(ctx, base);
	// 83257310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325731C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257320 size=64
    let mut pc: u32 = 0x83257320;
    'dispatch: loop {
        match pc {
            0x83257320 => {
    //   block [0x83257320..0x83257360)
	// 83257320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325732C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257334: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257338: 386A88C4  addi r3, r10, -0x773c
	ctx.r[3].s64 = ctx.r[10].s64 + -30524;
	// 8325733C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257340: 4AFD5B91  bl 0x8222ced0
	ctx.lr = 0x83257344;
	sub_8222CED0(ctx, base);
	// 83257344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257348: 3869ADE0  addi r3, r9, -0x5220
	ctx.r[3].s64 = ctx.r[9].s64 + -21024;
	// 8325734C: 4BA52BD5  bl 0x82ca9f20
	ctx.lr = 0x83257350;
	sub_82CA9F20(ctx, base);
	// 83257350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257360 size=64
    let mut pc: u32 = 0x83257360;
    'dispatch: loop {
        match pc {
            0x83257360 => {
    //   block [0x83257360..0x832573A0)
	// 83257360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325736C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257374: 388B4034  addi r4, r11, 0x4034
	ctx.r[4].s64 = ctx.r[11].s64 + 16436;
	// 83257378: 386A88C8  addi r3, r10, -0x7738
	ctx.r[3].s64 = ctx.r[10].s64 + -30520;
	// 8325737C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257380: 4AFD5B51  bl 0x8222ced0
	ctx.lr = 0x83257384;
	sub_8222CED0(ctx, base);
	// 83257384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257388: 3869ADF0  addi r3, r9, -0x5210
	ctx.r[3].s64 = ctx.r[9].s64 + -21008;
	// 8325738C: 4BA52B95  bl 0x82ca9f20
	ctx.lr = 0x83257390;
	sub_82CA9F20(ctx, base);
	// 83257390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832573A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573A0 size=64
    let mut pc: u32 = 0x832573A0;
    'dispatch: loop {
        match pc {
            0x832573A0 => {
    //   block [0x832573A0..0x832573E0)
	// 832573A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832573AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832573B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832573B4: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 832573B8: 386A88CC  addi r3, r10, -0x7734
	ctx.r[3].s64 = ctx.r[10].s64 + -30516;
	// 832573BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832573C0: 4AFD5B11  bl 0x8222ced0
	ctx.lr = 0x832573C4;
	sub_8222CED0(ctx, base);
	// 832573C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832573C8: 3869AE00  addi r3, r9, -0x5200
	ctx.r[3].s64 = ctx.r[9].s64 + -20992;
	// 832573CC: 4BA52B55  bl 0x82ca9f20
	ctx.lr = 0x832573D0;
	sub_82CA9F20(ctx, base);
	// 832573D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832573D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832573D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832573DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832573E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832573E0 size=64
    let mut pc: u32 = 0x832573E0;
    'dispatch: loop {
        match pc {
            0x832573E0 => {
    //   block [0x832573E0..0x83257420)
	// 832573E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832573E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832573E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832573EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832573F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832573F4: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 832573F8: 386A88D0  addi r3, r10, -0x7730
	ctx.r[3].s64 = ctx.r[10].s64 + -30512;
	// 832573FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257400: 4AFD5AD1  bl 0x8222ced0
	ctx.lr = 0x83257404;
	sub_8222CED0(ctx, base);
	// 83257404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257408: 3869AE10  addi r3, r9, -0x51f0
	ctx.r[3].s64 = ctx.r[9].s64 + -20976;
	// 8325740C: 4BA52B15  bl 0x82ca9f20
	ctx.lr = 0x83257410;
	sub_82CA9F20(ctx, base);
	// 83257410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325741C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257420 size=64
    let mut pc: u32 = 0x83257420;
    'dispatch: loop {
        match pc {
            0x83257420 => {
    //   block [0x83257420..0x83257460)
	// 83257420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325742C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257434: 388B403C  addi r4, r11, 0x403c
	ctx.r[4].s64 = ctx.r[11].s64 + 16444;
	// 83257438: 386A88D4  addi r3, r10, -0x772c
	ctx.r[3].s64 = ctx.r[10].s64 + -30508;
	// 8325743C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257440: 4AFD5A91  bl 0x8222ced0
	ctx.lr = 0x83257444;
	sub_8222CED0(ctx, base);
	// 83257444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257448: 3869AE20  addi r3, r9, -0x51e0
	ctx.r[3].s64 = ctx.r[9].s64 + -20960;
	// 8325744C: 4BA52AD5  bl 0x82ca9f20
	ctx.lr = 0x83257450;
	sub_82CA9F20(ctx, base);
	// 83257450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325745C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257460 size=64
    let mut pc: u32 = 0x83257460;
    'dispatch: loop {
        match pc {
            0x83257460 => {
    //   block [0x83257460..0x832574A0)
	// 83257460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325746C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257474: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257478: 386A88D8  addi r3, r10, -0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + -30504;
	// 8325747C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257480: 4AFD5A51  bl 0x8222ced0
	ctx.lr = 0x83257484;
	sub_8222CED0(ctx, base);
	// 83257484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257488: 3869AE30  addi r3, r9, -0x51d0
	ctx.r[3].s64 = ctx.r[9].s64 + -20944;
	// 8325748C: 4BA52A95  bl 0x82ca9f20
	ctx.lr = 0x83257490;
	sub_82CA9F20(ctx, base);
	// 83257490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325749C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832574A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574A0 size=64
    let mut pc: u32 = 0x832574A0;
    'dispatch: loop {
        match pc {
            0x832574A0 => {
    //   block [0x832574A0..0x832574E0)
	// 832574A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832574AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832574B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832574B4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832574B8: 386A88DC  addi r3, r10, -0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + -30500;
	// 832574BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832574C0: 4AFD5A11  bl 0x8222ced0
	ctx.lr = 0x832574C4;
	sub_8222CED0(ctx, base);
	// 832574C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832574C8: 3869AE40  addi r3, r9, -0x51c0
	ctx.r[3].s64 = ctx.r[9].s64 + -20928;
	// 832574CC: 4BA52A55  bl 0x82ca9f20
	ctx.lr = 0x832574D0;
	sub_82CA9F20(ctx, base);
	// 832574D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832574D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832574D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832574DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832574E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832574E0 size=64
    let mut pc: u32 = 0x832574E0;
    'dispatch: loop {
        match pc {
            0x832574E0 => {
    //   block [0x832574E0..0x83257520)
	// 832574E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832574E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832574E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832574EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832574F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832574F4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832574F8: 386A88E0  addi r3, r10, -0x7720
	ctx.r[3].s64 = ctx.r[10].s64 + -30496;
	// 832574FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257500: 4AFD59D1  bl 0x8222ced0
	ctx.lr = 0x83257504;
	sub_8222CED0(ctx, base);
	// 83257504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257508: 3869AE50  addi r3, r9, -0x51b0
	ctx.r[3].s64 = ctx.r[9].s64 + -20912;
	// 8325750C: 4BA52A15  bl 0x82ca9f20
	ctx.lr = 0x83257510;
	sub_82CA9F20(ctx, base);
	// 83257510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257520 size=64
    let mut pc: u32 = 0x83257520;
    'dispatch: loop {
        match pc {
            0x83257520 => {
    //   block [0x83257520..0x83257560)
	// 83257520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325752C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257534: 388B475C  addi r4, r11, 0x475c
	ctx.r[4].s64 = ctx.r[11].s64 + 18268;
	// 83257538: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 8325753C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257540: 4AFD5991  bl 0x8222ced0
	ctx.lr = 0x83257544;
	sub_8222CED0(ctx, base);
	// 83257544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257548: 3869AE60  addi r3, r9, -0x51a0
	ctx.r[3].s64 = ctx.r[9].s64 + -20896;
	// 8325754C: 4BA529D5  bl 0x82ca9f20
	ctx.lr = 0x83257550;
	sub_82CA9F20(ctx, base);
	// 83257550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257560 size=64
    let mut pc: u32 = 0x83257560;
    'dispatch: loop {
        match pc {
            0x83257560 => {
    //   block [0x83257560..0x832575A0)
	// 83257560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325756C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257574: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257578: 386A88E8  addi r3, r10, -0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + -30488;
	// 8325757C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257580: 4AFD5951  bl 0x8222ced0
	ctx.lr = 0x83257584;
	sub_8222CED0(ctx, base);
	// 83257584: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257588: 3869AE70  addi r3, r9, -0x5190
	ctx.r[3].s64 = ctx.r[9].s64 + -20880;
	// 8325758C: 4BA52995  bl 0x82ca9f20
	ctx.lr = 0x83257590;
	sub_82CA9F20(ctx, base);
	// 83257590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325759C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832575A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575A0 size=64
    let mut pc: u32 = 0x832575A0;
    'dispatch: loop {
        match pc {
            0x832575A0 => {
    //   block [0x832575A0..0x832575E0)
	// 832575A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832575AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832575B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832575B4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832575B8: 386A88EC  addi r3, r10, -0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + -30484;
	// 832575BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832575C0: 4AFD5911  bl 0x8222ced0
	ctx.lr = 0x832575C4;
	sub_8222CED0(ctx, base);
	// 832575C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832575C8: 3869AE80  addi r3, r9, -0x5180
	ctx.r[3].s64 = ctx.r[9].s64 + -20864;
	// 832575CC: 4BA52955  bl 0x82ca9f20
	ctx.lr = 0x832575D0;
	sub_82CA9F20(ctx, base);
	// 832575D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832575D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832575D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832575DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832575E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832575E0 size=64
    let mut pc: u32 = 0x832575E0;
    'dispatch: loop {
        match pc {
            0x832575E0 => {
    //   block [0x832575E0..0x83257620)
	// 832575E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832575E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832575E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832575EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832575F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832575F4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832575F8: 386A88F0  addi r3, r10, -0x7710
	ctx.r[3].s64 = ctx.r[10].s64 + -30480;
	// 832575FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257600: 4AFD58D1  bl 0x8222ced0
	ctx.lr = 0x83257604;
	sub_8222CED0(ctx, base);
	// 83257604: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257608: 3869AE90  addi r3, r9, -0x5170
	ctx.r[3].s64 = ctx.r[9].s64 + -20848;
	// 8325760C: 4BA52915  bl 0x82ca9f20
	ctx.lr = 0x83257610;
	sub_82CA9F20(ctx, base);
	// 83257610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257620 size=64
    let mut pc: u32 = 0x83257620;
    'dispatch: loop {
        match pc {
            0x83257620 => {
    //   block [0x83257620..0x83257660)
	// 83257620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257628: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325762C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257634: 388B47A0  addi r4, r11, 0x47a0
	ctx.r[4].s64 = ctx.r[11].s64 + 18336;
	// 83257638: 386A88F4  addi r3, r10, -0x770c
	ctx.r[3].s64 = ctx.r[10].s64 + -30476;
	// 8325763C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257640: 4AFD5891  bl 0x8222ced0
	ctx.lr = 0x83257644;
	sub_8222CED0(ctx, base);
	// 83257644: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257648: 3869AEA0  addi r3, r9, -0x5160
	ctx.r[3].s64 = ctx.r[9].s64 + -20832;
	// 8325764C: 4BA528D5  bl 0x82ca9f20
	ctx.lr = 0x83257650;
	sub_82CA9F20(ctx, base);
	// 83257650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257660 size=64
    let mut pc: u32 = 0x83257660;
    'dispatch: loop {
        match pc {
            0x83257660 => {
    //   block [0x83257660..0x832576A0)
	// 83257660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325766C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257670: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257674: 388B47B0  addi r4, r11, 0x47b0
	ctx.r[4].s64 = ctx.r[11].s64 + 18352;
	// 83257678: 386A88F8  addi r3, r10, -0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + -30472;
	// 8325767C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257680: 4AFD5851  bl 0x8222ced0
	ctx.lr = 0x83257684;
	sub_8222CED0(ctx, base);
	// 83257684: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257688: 3869AEB0  addi r3, r9, -0x5150
	ctx.r[3].s64 = ctx.r[9].s64 + -20816;
	// 8325768C: 4BA52895  bl 0x82ca9f20
	ctx.lr = 0x83257690;
	sub_82CA9F20(ctx, base);
	// 83257690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325769C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832576A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576A0 size=64
    let mut pc: u32 = 0x832576A0;
    'dispatch: loop {
        match pc {
            0x832576A0 => {
    //   block [0x832576A0..0x832576E0)
	// 832576A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832576AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832576B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832576B4: 388B47C0  addi r4, r11, 0x47c0
	ctx.r[4].s64 = ctx.r[11].s64 + 18368;
	// 832576B8: 386A88FC  addi r3, r10, -0x7704
	ctx.r[3].s64 = ctx.r[10].s64 + -30468;
	// 832576BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832576C0: 4AFD5811  bl 0x8222ced0
	ctx.lr = 0x832576C4;
	sub_8222CED0(ctx, base);
	// 832576C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832576C8: 3869AEC0  addi r3, r9, -0x5140
	ctx.r[3].s64 = ctx.r[9].s64 + -20800;
	// 832576CC: 4BA52855  bl 0x82ca9f20
	ctx.lr = 0x832576D0;
	sub_82CA9F20(ctx, base);
	// 832576D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832576D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832576D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832576DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832576E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832576E0 size=64
    let mut pc: u32 = 0x832576E0;
    'dispatch: loop {
        match pc {
            0x832576E0 => {
    //   block [0x832576E0..0x83257720)
	// 832576E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832576E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832576E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832576EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832576F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832576F4: 388B47D4  addi r4, r11, 0x47d4
	ctx.r[4].s64 = ctx.r[11].s64 + 18388;
	// 832576F8: 386A8900  addi r3, r10, -0x7700
	ctx.r[3].s64 = ctx.r[10].s64 + -30464;
	// 832576FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257700: 4AFD57D1  bl 0x8222ced0
	ctx.lr = 0x83257704;
	sub_8222CED0(ctx, base);
	// 83257704: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257708: 3869AED0  addi r3, r9, -0x5130
	ctx.r[3].s64 = ctx.r[9].s64 + -20784;
	// 8325770C: 4BA52815  bl 0x82ca9f20
	ctx.lr = 0x83257710;
	sub_82CA9F20(ctx, base);
	// 83257710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325771C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257720 size=64
    let mut pc: u32 = 0x83257720;
    'dispatch: loop {
        match pc {
            0x83257720 => {
    //   block [0x83257720..0x83257760)
	// 83257720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325772C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257734: 388B47E0  addi r4, r11, 0x47e0
	ctx.r[4].s64 = ctx.r[11].s64 + 18400;
	// 83257738: 386A8904  addi r3, r10, -0x76fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30460;
	// 8325773C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257740: 4AFD5791  bl 0x8222ced0
	ctx.lr = 0x83257744;
	sub_8222CED0(ctx, base);
	// 83257744: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257748: 3869AEE0  addi r3, r9, -0x5120
	ctx.r[3].s64 = ctx.r[9].s64 + -20768;
	// 8325774C: 4BA527D5  bl 0x82ca9f20
	ctx.lr = 0x83257750;
	sub_82CA9F20(ctx, base);
	// 83257750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325775C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257760 size=64
    let mut pc: u32 = 0x83257760;
    'dispatch: loop {
        match pc {
            0x83257760 => {
    //   block [0x83257760..0x832577A0)
	// 83257760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325776C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257774: 388B47F4  addi r4, r11, 0x47f4
	ctx.r[4].s64 = ctx.r[11].s64 + 18420;
	// 83257778: 386A8908  addi r3, r10, -0x76f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30456;
	// 8325777C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257780: 4AFD5751  bl 0x8222ced0
	ctx.lr = 0x83257784;
	sub_8222CED0(ctx, base);
	// 83257784: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257788: 3869AEF0  addi r3, r9, -0x5110
	ctx.r[3].s64 = ctx.r[9].s64 + -20752;
	// 8325778C: 4BA52795  bl 0x82ca9f20
	ctx.lr = 0x83257790;
	sub_82CA9F20(ctx, base);
	// 83257790: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325779C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832577A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577A0 size=64
    let mut pc: u32 = 0x832577A0;
    'dispatch: loop {
        match pc {
            0x832577A0 => {
    //   block [0x832577A0..0x832577E0)
	// 832577A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832577AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832577B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832577B4: 388B4808  addi r4, r11, 0x4808
	ctx.r[4].s64 = ctx.r[11].s64 + 18440;
	// 832577B8: 386A890C  addi r3, r10, -0x76f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30452;
	// 832577BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832577C0: 4AFD5711  bl 0x8222ced0
	ctx.lr = 0x832577C4;
	sub_8222CED0(ctx, base);
	// 832577C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832577C8: 3869AF00  addi r3, r9, -0x5100
	ctx.r[3].s64 = ctx.r[9].s64 + -20736;
	// 832577CC: 4BA52755  bl 0x82ca9f20
	ctx.lr = 0x832577D0;
	sub_82CA9F20(ctx, base);
	// 832577D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832577D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832577D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832577DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832577E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832577E0 size=64
    let mut pc: u32 = 0x832577E0;
    'dispatch: loop {
        match pc {
            0x832577E0 => {
    //   block [0x832577E0..0x83257820)
	// 832577E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832577E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832577E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832577EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832577F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832577F4: 388B4820  addi r4, r11, 0x4820
	ctx.r[4].s64 = ctx.r[11].s64 + 18464;
	// 832577F8: 386A8910  addi r3, r10, -0x76f0
	ctx.r[3].s64 = ctx.r[10].s64 + -30448;
	// 832577FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257800: 4AFD56D1  bl 0x8222ced0
	ctx.lr = 0x83257804;
	sub_8222CED0(ctx, base);
	// 83257804: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257808: 3869AF10  addi r3, r9, -0x50f0
	ctx.r[3].s64 = ctx.r[9].s64 + -20720;
	// 8325780C: 4BA52715  bl 0x82ca9f20
	ctx.lr = 0x83257810;
	sub_82CA9F20(ctx, base);
	// 83257810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257820 size=64
    let mut pc: u32 = 0x83257820;
    'dispatch: loop {
        match pc {
            0x83257820 => {
    //   block [0x83257820..0x83257860)
	// 83257820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325782C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257830: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257834: 388B4838  addi r4, r11, 0x4838
	ctx.r[4].s64 = ctx.r[11].s64 + 18488;
	// 83257838: 386A8914  addi r3, r10, -0x76ec
	ctx.r[3].s64 = ctx.r[10].s64 + -30444;
	// 8325783C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257840: 4AFD5691  bl 0x8222ced0
	ctx.lr = 0x83257844;
	sub_8222CED0(ctx, base);
	// 83257844: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257848: 3869AF20  addi r3, r9, -0x50e0
	ctx.r[3].s64 = ctx.r[9].s64 + -20704;
	// 8325784C: 4BA526D5  bl 0x82ca9f20
	ctx.lr = 0x83257850;
	sub_82CA9F20(ctx, base);
	// 83257850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325785C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257860 size=64
    let mut pc: u32 = 0x83257860;
    'dispatch: loop {
        match pc {
            0x83257860 => {
    //   block [0x83257860..0x832578A0)
	// 83257860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325786C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257870: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257874: 388B4850  addi r4, r11, 0x4850
	ctx.r[4].s64 = ctx.r[11].s64 + 18512;
	// 83257878: 386A8918  addi r3, r10, -0x76e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30440;
	// 8325787C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257880: 4AFD5651  bl 0x8222ced0
	ctx.lr = 0x83257884;
	sub_8222CED0(ctx, base);
	// 83257884: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257888: 3869AF30  addi r3, r9, -0x50d0
	ctx.r[3].s64 = ctx.r[9].s64 + -20688;
	// 8325788C: 4BA52695  bl 0x82ca9f20
	ctx.lr = 0x83257890;
	sub_82CA9F20(ctx, base);
	// 83257890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325789C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832578A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578A0 size=64
    let mut pc: u32 = 0x832578A0;
    'dispatch: loop {
        match pc {
            0x832578A0 => {
    //   block [0x832578A0..0x832578E0)
	// 832578A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832578AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832578B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832578B4: 388B4868  addi r4, r11, 0x4868
	ctx.r[4].s64 = ctx.r[11].s64 + 18536;
	// 832578B8: 386A891C  addi r3, r10, -0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30436;
	// 832578BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832578C0: 4AFD5611  bl 0x8222ced0
	ctx.lr = 0x832578C4;
	sub_8222CED0(ctx, base);
	// 832578C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832578C8: 3869AF40  addi r3, r9, -0x50c0
	ctx.r[3].s64 = ctx.r[9].s64 + -20672;
	// 832578CC: 4BA52655  bl 0x82ca9f20
	ctx.lr = 0x832578D0;
	sub_82CA9F20(ctx, base);
	// 832578D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832578D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832578D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832578DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832578E0 size=64
    let mut pc: u32 = 0x832578E0;
    'dispatch: loop {
        match pc {
            0x832578E0 => {
    //   block [0x832578E0..0x83257920)
	// 832578E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832578E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832578E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832578EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832578F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832578F4: 388B4874  addi r4, r11, 0x4874
	ctx.r[4].s64 = ctx.r[11].s64 + 18548;
	// 832578F8: 386A8920  addi r3, r10, -0x76e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30432;
	// 832578FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257900: 4AFD55D1  bl 0x8222ced0
	ctx.lr = 0x83257904;
	sub_8222CED0(ctx, base);
	// 83257904: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257908: 3869AF50  addi r3, r9, -0x50b0
	ctx.r[3].s64 = ctx.r[9].s64 + -20656;
	// 8325790C: 4BA52615  bl 0x82ca9f20
	ctx.lr = 0x83257910;
	sub_82CA9F20(ctx, base);
	// 83257910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257920 size=64
    let mut pc: u32 = 0x83257920;
    'dispatch: loop {
        match pc {
            0x83257920 => {
    //   block [0x83257920..0x83257960)
	// 83257920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325792C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257934: 388B4878  addi r4, r11, 0x4878
	ctx.r[4].s64 = ctx.r[11].s64 + 18552;
	// 83257938: 386A8924  addi r3, r10, -0x76dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30428;
	// 8325793C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257940: 4AFD5591  bl 0x8222ced0
	ctx.lr = 0x83257944;
	sub_8222CED0(ctx, base);
	// 83257944: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257948: 3869AF60  addi r3, r9, -0x50a0
	ctx.r[3].s64 = ctx.r[9].s64 + -20640;
	// 8325794C: 4BA525D5  bl 0x82ca9f20
	ctx.lr = 0x83257950;
	sub_82CA9F20(ctx, base);
	// 83257950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325795C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257960 size=64
    let mut pc: u32 = 0x83257960;
    'dispatch: loop {
        match pc {
            0x83257960 => {
    //   block [0x83257960..0x832579A0)
	// 83257960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325796C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257974: 388B4880  addi r4, r11, 0x4880
	ctx.r[4].s64 = ctx.r[11].s64 + 18560;
	// 83257978: 386A8928  addi r3, r10, -0x76d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30424;
	// 8325797C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257980: 4AFD5551  bl 0x8222ced0
	ctx.lr = 0x83257984;
	sub_8222CED0(ctx, base);
	// 83257984: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257988: 3869AF70  addi r3, r9, -0x5090
	ctx.r[3].s64 = ctx.r[9].s64 + -20624;
	// 8325798C: 4BA52595  bl 0x82ca9f20
	ctx.lr = 0x83257990;
	sub_82CA9F20(ctx, base);
	// 83257990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325799C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832579A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579A0 size=64
    let mut pc: u32 = 0x832579A0;
    'dispatch: loop {
        match pc {
            0x832579A0 => {
    //   block [0x832579A0..0x832579E0)
	// 832579A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832579AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832579B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832579B4: 388B489C  addi r4, r11, 0x489c
	ctx.r[4].s64 = ctx.r[11].s64 + 18588;
	// 832579B8: 386A892C  addi r3, r10, -0x76d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30420;
	// 832579BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832579C0: 4AFD5511  bl 0x8222ced0
	ctx.lr = 0x832579C4;
	sub_8222CED0(ctx, base);
	// 832579C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832579C8: 3869AF80  addi r3, r9, -0x5080
	ctx.r[3].s64 = ctx.r[9].s64 + -20608;
	// 832579CC: 4BA52555  bl 0x82ca9f20
	ctx.lr = 0x832579D0;
	sub_82CA9F20(ctx, base);
	// 832579D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832579D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832579D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832579DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832579E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832579E0 size=64
    let mut pc: u32 = 0x832579E0;
    'dispatch: loop {
        match pc {
            0x832579E0 => {
    //   block [0x832579E0..0x83257A20)
	// 832579E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832579E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832579E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832579EC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832579F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832579F4: 388B48B8  addi r4, r11, 0x48b8
	ctx.r[4].s64 = ctx.r[11].s64 + 18616;
	// 832579F8: 386A8930  addi r3, r10, -0x76d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30416;
	// 832579FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A00: 4AFD54D1  bl 0x8222ced0
	ctx.lr = 0x83257A04;
	sub_8222CED0(ctx, base);
	// 83257A04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A08: 3869AF90  addi r3, r9, -0x5070
	ctx.r[3].s64 = ctx.r[9].s64 + -20592;
	// 83257A0C: 4BA52515  bl 0x82ca9f20
	ctx.lr = 0x83257A10;
	sub_82CA9F20(ctx, base);
	// 83257A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A20 size=64
    let mut pc: u32 = 0x83257A20;
    'dispatch: loop {
        match pc {
            0x83257A20 => {
    //   block [0x83257A20..0x83257A60)
	// 83257A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257A2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257A30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257A34: 388B48C4  addi r4, r11, 0x48c4
	ctx.r[4].s64 = ctx.r[11].s64 + 18628;
	// 83257A38: 386A8934  addi r3, r10, -0x76cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30412;
	// 83257A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A40: 4AFD5491  bl 0x8222ced0
	ctx.lr = 0x83257A44;
	sub_8222CED0(ctx, base);
	// 83257A44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A48: 3869AFA0  addi r3, r9, -0x5060
	ctx.r[3].s64 = ctx.r[9].s64 + -20576;
	// 83257A4C: 4BA524D5  bl 0x82ca9f20
	ctx.lr = 0x83257A50;
	sub_82CA9F20(ctx, base);
	// 83257A50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257A60 size=64
    let mut pc: u32 = 0x83257A60;
    'dispatch: loop {
        match pc {
            0x83257A60 => {
    //   block [0x83257A60..0x83257AA0)
	// 83257A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257A6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257A74: 388B48D4  addi r4, r11, 0x48d4
	ctx.r[4].s64 = ctx.r[11].s64 + 18644;
	// 83257A78: 386A8938  addi r3, r10, -0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30408;
	// 83257A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257A80: 4AFD5451  bl 0x8222ced0
	ctx.lr = 0x83257A84;
	sub_8222CED0(ctx, base);
	// 83257A84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257A88: 3869AFB0  addi r3, r9, -0x5050
	ctx.r[3].s64 = ctx.r[9].s64 + -20560;
	// 83257A8C: 4BA52495  bl 0x82ca9f20
	ctx.lr = 0x83257A90;
	sub_82CA9F20(ctx, base);
	// 83257A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AA0 size=64
    let mut pc: u32 = 0x83257AA0;
    'dispatch: loop {
        match pc {
            0x83257AA0 => {
    //   block [0x83257AA0..0x83257AE0)
	// 83257AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257AB4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257AB8: 386A893C  addi r3, r10, -0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + -30404;
	// 83257ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257AC0: 4AFD5411  bl 0x8222ced0
	ctx.lr = 0x83257AC4;
	sub_8222CED0(ctx, base);
	// 83257AC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257AC8: 3869AFC0  addi r3, r9, -0x5040
	ctx.r[3].s64 = ctx.r[9].s64 + -20544;
	// 83257ACC: 4BA52455  bl 0x82ca9f20
	ctx.lr = 0x83257AD0;
	sub_82CA9F20(ctx, base);
	// 83257AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257AE0 size=64
    let mut pc: u32 = 0x83257AE0;
    'dispatch: loop {
        match pc {
            0x83257AE0 => {
    //   block [0x83257AE0..0x83257B20)
	// 83257AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257AE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257AEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257AF4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257AF8: 386A8940  addi r3, r10, -0x76c0
	ctx.r[3].s64 = ctx.r[10].s64 + -30400;
	// 83257AFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B00: 4AFD53D1  bl 0x8222ced0
	ctx.lr = 0x83257B04;
	sub_8222CED0(ctx, base);
	// 83257B04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B08: 3869AFD0  addi r3, r9, -0x5030
	ctx.r[3].s64 = ctx.r[9].s64 + -20528;
	// 83257B0C: 4BA52415  bl 0x82ca9f20
	ctx.lr = 0x83257B10;
	sub_82CA9F20(ctx, base);
	// 83257B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B20 size=64
    let mut pc: u32 = 0x83257B20;
    'dispatch: loop {
        match pc {
            0x83257B20 => {
    //   block [0x83257B20..0x83257B60)
	// 83257B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257B2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257B34: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257B38: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 83257B3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B40: 4AFD5391  bl 0x8222ced0
	ctx.lr = 0x83257B44;
	sub_8222CED0(ctx, base);
	// 83257B44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B48: 3869AFE0  addi r3, r9, -0x5020
	ctx.r[3].s64 = ctx.r[9].s64 + -20512;
	// 83257B4C: 4BA523D5  bl 0x82ca9f20
	ctx.lr = 0x83257B50;
	sub_82CA9F20(ctx, base);
	// 83257B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257B60 size=64
    let mut pc: u32 = 0x83257B60;
    'dispatch: loop {
        match pc {
            0x83257B60 => {
    //   block [0x83257B60..0x83257BA0)
	// 83257B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257B6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257B70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257B74: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257B78: 386A8948  addi r3, r10, -0x76b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30392;
	// 83257B7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257B80: 4AFD5351  bl 0x8222ced0
	ctx.lr = 0x83257B84;
	sub_8222CED0(ctx, base);
	// 83257B84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257B88: 3869AFF0  addi r3, r9, -0x5010
	ctx.r[3].s64 = ctx.r[9].s64 + -20496;
	// 83257B8C: 4BA52395  bl 0x82ca9f20
	ctx.lr = 0x83257B90;
	sub_82CA9F20(ctx, base);
	// 83257B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BA0 size=64
    let mut pc: u32 = 0x83257BA0;
    'dispatch: loop {
        match pc {
            0x83257BA0 => {
    //   block [0x83257BA0..0x83257BE0)
	// 83257BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257BB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257BB4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257BB8: 386A894C  addi r3, r10, -0x76b4
	ctx.r[3].s64 = ctx.r[10].s64 + -30388;
	// 83257BBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257BC0: 4AFD5311  bl 0x8222ced0
	ctx.lr = 0x83257BC4;
	sub_8222CED0(ctx, base);
	// 83257BC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257BC8: 3869B000  addi r3, r9, -0x5000
	ctx.r[3].s64 = ctx.r[9].s64 + -20480;
	// 83257BCC: 4BA52355  bl 0x82ca9f20
	ctx.lr = 0x83257BD0;
	sub_82CA9F20(ctx, base);
	// 83257BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257BE0 size=64
    let mut pc: u32 = 0x83257BE0;
    'dispatch: loop {
        match pc {
            0x83257BE0 => {
    //   block [0x83257BE0..0x83257C20)
	// 83257BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257BEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257BF4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257BF8: 386A8950  addi r3, r10, -0x76b0
	ctx.r[3].s64 = ctx.r[10].s64 + -30384;
	// 83257BFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C00: 4AFD52D1  bl 0x8222ced0
	ctx.lr = 0x83257C04;
	sub_8222CED0(ctx, base);
	// 83257C04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C08: 3869B010  addi r3, r9, -0x4ff0
	ctx.r[3].s64 = ctx.r[9].s64 + -20464;
	// 83257C0C: 4BA52315  bl 0x82ca9f20
	ctx.lr = 0x83257C10;
	sub_82CA9F20(ctx, base);
	// 83257C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C20 size=64
    let mut pc: u32 = 0x83257C20;
    'dispatch: loop {
        match pc {
            0x83257C20 => {
    //   block [0x83257C20..0x83257C60)
	// 83257C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257C34: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257C38: 386A8954  addi r3, r10, -0x76ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30380;
	// 83257C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C40: 4AFD5291  bl 0x8222ced0
	ctx.lr = 0x83257C44;
	sub_8222CED0(ctx, base);
	// 83257C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C48: 3869B020  addi r3, r9, -0x4fe0
	ctx.r[3].s64 = ctx.r[9].s64 + -20448;
	// 83257C4C: 4BA522D5  bl 0x82ca9f20
	ctx.lr = 0x83257C50;
	sub_82CA9F20(ctx, base);
	// 83257C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257C60 size=64
    let mut pc: u32 = 0x83257C60;
    'dispatch: loop {
        match pc {
            0x83257C60 => {
    //   block [0x83257C60..0x83257CA0)
	// 83257C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257C6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257C74: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257C78: 386A8958  addi r3, r10, -0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30376;
	// 83257C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257C80: 4AFD5251  bl 0x8222ced0
	ctx.lr = 0x83257C84;
	sub_8222CED0(ctx, base);
	// 83257C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257C88: 3869B030  addi r3, r9, -0x4fd0
	ctx.r[3].s64 = ctx.r[9].s64 + -20432;
	// 83257C8C: 4BA52295  bl 0x82ca9f20
	ctx.lr = 0x83257C90;
	sub_82CA9F20(ctx, base);
	// 83257C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CA0 size=64
    let mut pc: u32 = 0x83257CA0;
    'dispatch: loop {
        match pc {
            0x83257CA0 => {
    //   block [0x83257CA0..0x83257CE0)
	// 83257CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257CAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257CB4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257CB8: 386A895C  addi r3, r10, -0x76a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30372;
	// 83257CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257CC0: 4AFD5211  bl 0x8222ced0
	ctx.lr = 0x83257CC4;
	sub_8222CED0(ctx, base);
	// 83257CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257CC8: 3869B040  addi r3, r9, -0x4fc0
	ctx.r[3].s64 = ctx.r[9].s64 + -20416;
	// 83257CCC: 4BA52255  bl 0x82ca9f20
	ctx.lr = 0x83257CD0;
	sub_82CA9F20(ctx, base);
	// 83257CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257CE0 size=56
    let mut pc: u32 = 0x83257CE0;
    'dispatch: loop {
        match pc {
            0x83257CE0 => {
    //   block [0x83257CE0..0x83257D18)
	// 83257CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257CEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257CF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257CF4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83257CF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257CFC: 4AF9C05D  bl 0x821f3d58
	ctx.lr = 0x83257D00;
	sub_821F3D58(ctx, base);
	// 83257D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D04: 906A8960  stw r3, -0x76a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30368 as u32), ctx.r[3].u32 ) };
	// 83257D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D18 size=56
    let mut pc: u32 = 0x83257D18;
    'dispatch: loop {
        match pc {
            0x83257D18 => {
    //   block [0x83257D18..0x83257D50)
	// 83257D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D24: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257D28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D2C: 386B4DBC  addi r3, r11, 0x4dbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19900;
	// 83257D30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D34: 4AF9C025  bl 0x821f3d58
	ctx.lr = 0x83257D38;
	sub_821F3D58(ctx, base);
	// 83257D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D3C: 906A8964  stw r3, -0x769c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30364 as u32), ctx.r[3].u32 ) };
	// 83257D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D50 size=56
    let mut pc: u32 = 0x83257D50;
    'dispatch: loop {
        match pc {
            0x83257D50 => {
    //   block [0x83257D50..0x83257D88)
	// 83257D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D64: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83257D68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257D6C: 4AF9BFED  bl 0x821f3d58
	ctx.lr = 0x83257D70;
	sub_821F3D58(ctx, base);
	// 83257D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257D74: 906A8968  stw r3, -0x7698(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30360 as u32), ctx.r[3].u32 ) };
	// 83257D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257D88 size=56
    let mut pc: u32 = 0x83257D88;
    'dispatch: loop {
        match pc {
            0x83257D88 => {
    //   block [0x83257D88..0x83257DC0)
	// 83257D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257D94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257D98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257D9C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83257DA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DA4: 4AF9BFB5  bl 0x821f3d58
	ctx.lr = 0x83257DA8;
	sub_821F3D58(ctx, base);
	// 83257DA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DAC: 906A896C  stw r3, -0x7694(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30356 as u32), ctx.r[3].u32 ) };
	// 83257DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DC0 size=56
    let mut pc: u32 = 0x83257DC0;
    'dispatch: loop {
        match pc {
            0x83257DC0 => {
    //   block [0x83257DC0..0x83257DF8)
	// 83257DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257DCC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257DD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257DD4: 386B4DD4  addi r3, r11, 0x4dd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19924;
	// 83257DD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257DDC: 4AF9BF7D  bl 0x821f3d58
	ctx.lr = 0x83257DE0;
	sub_821F3D58(ctx, base);
	// 83257DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257DE4: 906A8970  stw r3, -0x7690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30352 as u32), ctx.r[3].u32 ) };
	// 83257DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257DF8 size=56
    let mut pc: u32 = 0x83257DF8;
    'dispatch: loop {
        match pc {
            0x83257DF8 => {
    //   block [0x83257DF8..0x83257E30)
	// 83257DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257E08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83257E0C: 386B4DEC  addi r3, r11, 0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + 19948;
	// 83257E10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83257E14: 4AF9BF45  bl 0x821f3d58
	ctx.lr = 0x83257E18;
	sub_821F3D58(ctx, base);
	// 83257E18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E1C: 906A8974  stw r3, -0x768c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30348 as u32), ctx.r[3].u32 ) };
	// 83257E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E30 size=64
    let mut pc: u32 = 0x83257E30;
    'dispatch: loop {
        match pc {
            0x83257E30 => {
    //   block [0x83257E30..0x83257E70)
	// 83257E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E44: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257E48: 386A8978  addi r3, r10, -0x7688
	ctx.r[3].s64 = ctx.r[10].s64 + -30344;
	// 83257E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257E50: 4AFD5081  bl 0x8222ced0
	ctx.lr = 0x83257E54;
	sub_8222CED0(ctx, base);
	// 83257E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257E58: 3869B050  addi r3, r9, -0x4fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -20400;
	// 83257E5C: 4BA520C5  bl 0x82ca9f20
	ctx.lr = 0x83257E60;
	sub_82CA9F20(ctx, base);
	// 83257E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257E70 size=64
    let mut pc: u32 = 0x83257E70;
    'dispatch: loop {
        match pc {
            0x83257E70 => {
    //   block [0x83257E70..0x83257EB0)
	// 83257E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257E84: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257E88: 386A897C  addi r3, r10, -0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + -30340;
	// 83257E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257E90: 4AFD5041  bl 0x8222ced0
	ctx.lr = 0x83257E94;
	sub_8222CED0(ctx, base);
	// 83257E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257E98: 3869B060  addi r3, r9, -0x4fa0
	ctx.r[3].s64 = ctx.r[9].s64 + -20384;
	// 83257E9C: 4BA52085  bl 0x82ca9f20
	ctx.lr = 0x83257EA0;
	sub_82CA9F20(ctx, base);
	// 83257EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EB0 size=64
    let mut pc: u32 = 0x83257EB0;
    'dispatch: loop {
        match pc {
            0x83257EB0 => {
    //   block [0x83257EB0..0x83257EF0)
	// 83257EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257EC4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257EC8: 386A8980  addi r3, r10, -0x7680
	ctx.r[3].s64 = ctx.r[10].s64 + -30336;
	// 83257ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257ED0: 4AFD5001  bl 0x8222ced0
	ctx.lr = 0x83257ED4;
	sub_8222CED0(ctx, base);
	// 83257ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257ED8: 3869B070  addi r3, r9, -0x4f90
	ctx.r[3].s64 = ctx.r[9].s64 + -20368;
	// 83257EDC: 4BA52045  bl 0x82ca9f20
	ctx.lr = 0x83257EE0;
	sub_82CA9F20(ctx, base);
	// 83257EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257EF0 size=64
    let mut pc: u32 = 0x83257EF0;
    'dispatch: loop {
        match pc {
            0x83257EF0 => {
    //   block [0x83257EF0..0x83257F30)
	// 83257EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F04: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83257F08: 386A8984  addi r3, r10, -0x767c
	ctx.r[3].s64 = ctx.r[10].s64 + -30332;
	// 83257F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F10: 4AFD4FC1  bl 0x8222ced0
	ctx.lr = 0x83257F14;
	sub_8222CED0(ctx, base);
	// 83257F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F18: 3869B080  addi r3, r9, -0x4f80
	ctx.r[3].s64 = ctx.r[9].s64 + -20352;
	// 83257F1C: 4BA52005  bl 0x82ca9f20
	ctx.lr = 0x83257F20;
	sub_82CA9F20(ctx, base);
	// 83257F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F30 size=64
    let mut pc: u32 = 0x83257F30;
    'dispatch: loop {
        match pc {
            0x83257F30 => {
    //   block [0x83257F30..0x83257F70)
	// 83257F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257F3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F44: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83257F48: 386A8988  addi r3, r10, -0x7678
	ctx.r[3].s64 = ctx.r[10].s64 + -30328;
	// 83257F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F50: 4AFD4F81  bl 0x8222ced0
	ctx.lr = 0x83257F54;
	sub_8222CED0(ctx, base);
	// 83257F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F58: 3869B090  addi r3, r9, -0x4f70
	ctx.r[3].s64 = ctx.r[9].s64 + -20336;
	// 83257F5C: 4BA51FC5  bl 0x82ca9f20
	ctx.lr = 0x83257F60;
	sub_82CA9F20(ctx, base);
	// 83257F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257F70 size=64
    let mut pc: u32 = 0x83257F70;
    'dispatch: loop {
        match pc {
            0x83257F70 => {
    //   block [0x83257F70..0x83257FB0)
	// 83257F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83257F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257F84: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83257F88: 386A898C  addi r3, r10, -0x7674
	ctx.r[3].s64 = ctx.r[10].s64 + -30324;
	// 83257F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257F90: 4AFD4F41  bl 0x8222ced0
	ctx.lr = 0x83257F94;
	sub_8222CED0(ctx, base);
	// 83257F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257F98: 3869B0A0  addi r3, r9, -0x4f60
	ctx.r[3].s64 = ctx.r[9].s64 + -20320;
	// 83257F9C: 4BA51F85  bl 0x82ca9f20
	ctx.lr = 0x83257FA0;
	sub_82CA9F20(ctx, base);
	// 83257FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FB0 size=64
    let mut pc: u32 = 0x83257FB0;
    'dispatch: loop {
        match pc {
            0x83257FB0 => {
    //   block [0x83257FB0..0x83257FF0)
	// 83257FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257FBC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83257FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83257FC4: 388B5304  addi r4, r11, 0x5304
	ctx.r[4].s64 = ctx.r[11].s64 + 21252;
	// 83257FC8: 386A8990  addi r3, r10, -0x7670
	ctx.r[3].s64 = ctx.r[10].s64 + -30320;
	// 83257FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83257FD0: 4AFD4F01  bl 0x8222ced0
	ctx.lr = 0x83257FD4;
	sub_8222CED0(ctx, base);
	// 83257FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83257FD8: 3869B0B0  addi r3, r9, -0x4f50
	ctx.r[3].s64 = ctx.r[9].s64 + -20304;
	// 83257FDC: 4BA51F45  bl 0x82ca9f20
	ctx.lr = 0x83257FE0;
	sub_82CA9F20(ctx, base);
	// 83257FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83257FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83257FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83257FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83257FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83257FF0 size=64
    let mut pc: u32 = 0x83257FF0;
    'dispatch: loop {
        match pc {
            0x83257FF0 => {
    //   block [0x83257FF0..0x83258030)
	// 83257FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83257FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83257FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83257FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258004: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258008: 386A8994  addi r3, r10, -0x766c
	ctx.r[3].s64 = ctx.r[10].s64 + -30316;
	// 8325800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258010: 4AFD4EC1  bl 0x8222ced0
	ctx.lr = 0x83258014;
	sub_8222CED0(ctx, base);
	// 83258014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258018: 3869B0C0  addi r3, r9, -0x4f40
	ctx.r[3].s64 = ctx.r[9].s64 + -20288;
	// 8325801C: 4BA51F05  bl 0x82ca9f20
	ctx.lr = 0x83258020;
	sub_82CA9F20(ctx, base);
	// 83258020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258030 size=64
    let mut pc: u32 = 0x83258030;
    'dispatch: loop {
        match pc {
            0x83258030 => {
    //   block [0x83258030..0x83258070)
	// 83258030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325803C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258044: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258048: 386A8998  addi r3, r10, -0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + -30312;
	// 8325804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258050: 4AFD4E81  bl 0x8222ced0
	ctx.lr = 0x83258054;
	sub_8222CED0(ctx, base);
	// 83258054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258058: 3869B0D0  addi r3, r9, -0x4f30
	ctx.r[3].s64 = ctx.r[9].s64 + -20272;
	// 8325805C: 4BA51EC5  bl 0x82ca9f20
	ctx.lr = 0x83258060;
	sub_82CA9F20(ctx, base);
	// 83258060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258070 size=64
    let mut pc: u32 = 0x83258070;
    'dispatch: loop {
        match pc {
            0x83258070 => {
    //   block [0x83258070..0x832580B0)
	// 83258070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325807C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258084: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258088: 386A899C  addi r3, r10, -0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + -30308;
	// 8325808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258090: 4AFD4E41  bl 0x8222ced0
	ctx.lr = 0x83258094;
	sub_8222CED0(ctx, base);
	// 83258094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258098: 3869B0E0  addi r3, r9, -0x4f20
	ctx.r[3].s64 = ctx.r[9].s64 + -20256;
	// 8325809C: 4BA51E85  bl 0x82ca9f20
	ctx.lr = 0x832580A0;
	sub_82CA9F20(ctx, base);
	// 832580A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832580A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832580A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832580AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832580B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832580B0 size=12
    let mut pc: u32 = 0x832580B0;
    'dispatch: loop {
        match pc {
            0x832580B0 => {
    //   block [0x832580B0..0x832580BC)
	// 832580B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832580B4: 386B89A0  addi r3, r11, -0x7660
	ctx.r[3].s64 = ctx.r[11].s64 + -30304;
	// 832580B8: 4AFE3CF8  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832580C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832580C0 size=64
    let mut pc: u32 = 0x832580C0;
    'dispatch: loop {
        match pc {
            0x832580C0 => {
    //   block [0x832580C0..0x83258100)
	// 832580C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832580C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832580C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832580CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832580D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832580D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832580D8: 386A89E0  addi r3, r10, -0x7620
	ctx.r[3].s64 = ctx.r[10].s64 + -30240;
	// 832580DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832580E0: 4AFD4DF1  bl 0x8222ced0
	ctx.lr = 0x832580E4;
	sub_8222CED0(ctx, base);
	// 832580E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832580E8: 3869B0F0  addi r3, r9, -0x4f10
	ctx.r[3].s64 = ctx.r[9].s64 + -20240;
	// 832580EC: 4BA51E35  bl 0x82ca9f20
	ctx.lr = 0x832580F0;
	sub_82CA9F20(ctx, base);
	// 832580F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832580F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832580F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832580FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258100 size=64
    let mut pc: u32 = 0x83258100;
    'dispatch: loop {
        match pc {
            0x83258100 => {
    //   block [0x83258100..0x83258140)
	// 83258100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325810C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258114: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258118: 386A89E4  addi r3, r10, -0x761c
	ctx.r[3].s64 = ctx.r[10].s64 + -30236;
	// 8325811C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258120: 4AFD4DB1  bl 0x8222ced0
	ctx.lr = 0x83258124;
	sub_8222CED0(ctx, base);
	// 83258124: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258128: 3869B100  addi r3, r9, -0x4f00
	ctx.r[3].s64 = ctx.r[9].s64 + -20224;
	// 8325812C: 4BA51DF5  bl 0x82ca9f20
	ctx.lr = 0x83258130;
	sub_82CA9F20(ctx, base);
	// 83258130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258140 size=64
    let mut pc: u32 = 0x83258140;
    'dispatch: loop {
        match pc {
            0x83258140 => {
    //   block [0x83258140..0x83258180)
	// 83258140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325814C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258150: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258154: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258158: 386A89E8  addi r3, r10, -0x7618
	ctx.r[3].s64 = ctx.r[10].s64 + -30232;
	// 8325815C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258160: 4AFD4D71  bl 0x8222ced0
	ctx.lr = 0x83258164;
	sub_8222CED0(ctx, base);
	// 83258164: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258168: 3869B110  addi r3, r9, -0x4ef0
	ctx.r[3].s64 = ctx.r[9].s64 + -20208;
	// 8325816C: 4BA51DB5  bl 0x82ca9f20
	ctx.lr = 0x83258170;
	sub_82CA9F20(ctx, base);
	// 83258170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325817C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258180 size=12
    let mut pc: u32 = 0x83258180;
    'dispatch: loop {
        match pc {
            0x83258180 => {
    //   block [0x83258180..0x8325818C)
	// 83258180: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83258184: 386BB120  addi r3, r11, -0x4ee0
	ctx.r[3].s64 = ctx.r[11].s64 + -20192;
	// 83258188: 4BA51D98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258190 size=12
    let mut pc: u32 = 0x83258190;
    'dispatch: loop {
        match pc {
            0x83258190 => {
    //   block [0x83258190..0x8325819C)
	// 83258190: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83258194: 386BB188  addi r3, r11, -0x4e78
	ctx.r[3].s64 = ctx.r[11].s64 + -20088;
	// 83258198: 4BA51D88  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832581A0 size=12
    let mut pc: u32 = 0x832581A0;
    'dispatch: loop {
        match pc {
            0x832581A0 => {
    //   block [0x832581A0..0x832581AC)
	// 832581A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832581A4: 386BB1F0  addi r3, r11, -0x4e10
	ctx.r[3].s64 = ctx.r[11].s64 + -19984;
	// 832581A8: 4BA51D78  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581B0 size=64
    let mut pc: u32 = 0x832581B0;
    'dispatch: loop {
        match pc {
            0x832581B0 => {
    //   block [0x832581B0..0x832581F0)
	// 832581B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832581BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832581C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832581C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832581C8: 386A8A1C  addi r3, r10, -0x75e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30180;
	// 832581CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832581D0: 4AFD4D01  bl 0x8222ced0
	ctx.lr = 0x832581D4;
	sub_8222CED0(ctx, base);
	// 832581D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832581D8: 3869B248  addi r3, r9, -0x4db8
	ctx.r[3].s64 = ctx.r[9].s64 + -19896;
	// 832581DC: 4BA51D45  bl 0x82ca9f20
	ctx.lr = 0x832581E0;
	sub_82CA9F20(ctx, base);
	// 832581E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832581E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832581E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832581EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832581F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832581F0 size=64
    let mut pc: u32 = 0x832581F0;
    'dispatch: loop {
        match pc {
            0x832581F0 => {
    //   block [0x832581F0..0x83258230)
	// 832581F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832581F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832581F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832581FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258204: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258208: 386A8A20  addi r3, r10, -0x75e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30176;
	// 8325820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258210: 4AFD4CC1  bl 0x8222ced0
	ctx.lr = 0x83258214;
	sub_8222CED0(ctx, base);
	// 83258214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258218: 3869B258  addi r3, r9, -0x4da8
	ctx.r[3].s64 = ctx.r[9].s64 + -19880;
	// 8325821C: 4BA51D05  bl 0x82ca9f20
	ctx.lr = 0x83258220;
	sub_82CA9F20(ctx, base);
	// 83258220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258230 size=64
    let mut pc: u32 = 0x83258230;
    'dispatch: loop {
        match pc {
            0x83258230 => {
    //   block [0x83258230..0x83258270)
	// 83258230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325823C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258244: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258248: 386A8A24  addi r3, r10, -0x75dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30172;
	// 8325824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258250: 4AFD4C81  bl 0x8222ced0
	ctx.lr = 0x83258254;
	sub_8222CED0(ctx, base);
	// 83258254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258258: 3869B268  addi r3, r9, -0x4d98
	ctx.r[3].s64 = ctx.r[9].s64 + -19864;
	// 8325825C: 4BA51CC5  bl 0x82ca9f20
	ctx.lr = 0x83258260;
	sub_82CA9F20(ctx, base);
	// 83258260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258270 size=64
    let mut pc: u32 = 0x83258270;
    'dispatch: loop {
        match pc {
            0x83258270 => {
    //   block [0x83258270..0x832582B0)
	// 83258270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325827C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258284: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258288: 386A8A28  addi r3, r10, -0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30168;
	// 8325828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258290: 4AFD4C41  bl 0x8222ced0
	ctx.lr = 0x83258294;
	sub_8222CED0(ctx, base);
	// 83258294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258298: 3869B278  addi r3, r9, -0x4d88
	ctx.r[3].s64 = ctx.r[9].s64 + -19848;
	// 8325829C: 4BA51C85  bl 0x82ca9f20
	ctx.lr = 0x832582A0;
	sub_82CA9F20(ctx, base);
	// 832582A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832582A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832582A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832582AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832582B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582B0 size=64
    let mut pc: u32 = 0x832582B0;
    'dispatch: loop {
        match pc {
            0x832582B0 => {
    //   block [0x832582B0..0x832582F0)
	// 832582B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832582BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832582C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832582C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832582C8: 386A8A2C  addi r3, r10, -0x75d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30164;
	// 832582CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832582D0: 4AFD4C01  bl 0x8222ced0
	ctx.lr = 0x832582D4;
	sub_8222CED0(ctx, base);
	// 832582D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832582D8: 3869B288  addi r3, r9, -0x4d78
	ctx.r[3].s64 = ctx.r[9].s64 + -19832;
	// 832582DC: 4BA51C45  bl 0x82ca9f20
	ctx.lr = 0x832582E0;
	sub_82CA9F20(ctx, base);
	// 832582E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832582E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832582E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832582EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832582F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832582F0 size=64
    let mut pc: u32 = 0x832582F0;
    'dispatch: loop {
        match pc {
            0x832582F0 => {
    //   block [0x832582F0..0x83258330)
	// 832582F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832582F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832582F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832582FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258304: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258308: 386A8A30  addi r3, r10, -0x75d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30160;
	// 8325830C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258310: 4AFD4BC1  bl 0x8222ced0
	ctx.lr = 0x83258314;
	sub_8222CED0(ctx, base);
	// 83258314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258318: 3869B298  addi r3, r9, -0x4d68
	ctx.r[3].s64 = ctx.r[9].s64 + -19816;
	// 8325831C: 4BA51C05  bl 0x82ca9f20
	ctx.lr = 0x83258320;
	sub_82CA9F20(ctx, base);
	// 83258320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258330 size=64
    let mut pc: u32 = 0x83258330;
    'dispatch: loop {
        match pc {
            0x83258330 => {
    //   block [0x83258330..0x83258370)
	// 83258330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325833C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258344: 388B2EF4  addi r4, r11, 0x2ef4
	ctx.r[4].s64 = ctx.r[11].s64 + 12020;
	// 83258348: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 8325834C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258350: 4AFD4B81  bl 0x8222ced0
	ctx.lr = 0x83258354;
	sub_8222CED0(ctx, base);
	// 83258354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258358: 3869B2A8  addi r3, r9, -0x4d58
	ctx.r[3].s64 = ctx.r[9].s64 + -19800;
	// 8325835C: 4BA51BC5  bl 0x82ca9f20
	ctx.lr = 0x83258360;
	sub_82CA9F20(ctx, base);
	// 83258360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258370 size=696
    let mut pc: u32 = 0x83258370;
    'dispatch: loop {
        match pc {
            0x83258370 => {
    //   block [0x83258370..0x83258628)
	// 83258370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325837C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83258384: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83258388: 3BEB8A38  addi r31, r11, -0x75c8
	ctx.r[31].s64 = ctx.r[11].s64 + -30152;
	// 8325838C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83258390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83258394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258398: 4AFD4B39  bl 0x8222ced0
	ctx.lr = 0x8325839C;
	sub_8222CED0(ctx, base);
	// 8325839C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832583A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583A4: 3889674C  addi r4, r9, 0x674c
	ctx.r[4].s64 = ctx.r[9].s64 + 26444;
	// 832583A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832583AC: 4AFD4B25  bl 0x8222ced0
	ctx.lr = 0x832583B0;
	sub_8222CED0(ctx, base);
	// 832583B0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832583B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583B8: 38886748  addi r4, r8, 0x6748
	ctx.r[4].s64 = ctx.r[8].s64 + 26440;
	// 832583BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832583C0: 4AFD4B11  bl 0x8222ced0
	ctx.lr = 0x832583C4;
	sub_8222CED0(ctx, base);
	// 832583C4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832583C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583CC: 38876744  addi r4, r7, 0x6744
	ctx.r[4].s64 = ctx.r[7].s64 + 26436;
	// 832583D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832583D4: 4AFD4AFD  bl 0x8222ced0
	ctx.lr = 0x832583D8;
	sub_8222CED0(ctx, base);
	// 832583D8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832583DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583E0: 38866740  addi r4, r6, 0x6740
	ctx.r[4].s64 = ctx.r[6].s64 + 26432;
	// 832583E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832583E8: 4AFD4AE9  bl 0x8222ced0
	ctx.lr = 0x832583EC;
	sub_8222CED0(ctx, base);
	// 832583EC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832583F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832583F4: 3884673C  addi r4, r4, 0x673c
	ctx.r[4].s64 = ctx.r[4].s64 + 26428;
	// 832583F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832583FC: 4AFD4AD5  bl 0x8222ced0
	ctx.lr = 0x83258400;
	sub_8222CED0(ctx, base);
	// 83258400: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258408: 38836738  addi r4, r3, 0x6738
	ctx.r[4].s64 = ctx.r[3].s64 + 26424;
	// 8325840C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83258410: 4AFD4AC1  bl 0x8222ced0
	ctx.lr = 0x83258414;
	sub_8222CED0(ctx, base);
	// 83258414: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258418: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325841C: 388B6734  addi r4, r11, 0x6734
	ctx.r[4].s64 = ctx.r[11].s64 + 26420;
	// 83258420: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83258424: 4AFD4AAD  bl 0x8222ced0
	ctx.lr = 0x83258428;
	sub_8222CED0(ctx, base);
	// 83258428: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325842C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258430: 388A6730  addi r4, r10, 0x6730
	ctx.r[4].s64 = ctx.r[10].s64 + 26416;
	// 83258434: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83258438: 4AFD4A99  bl 0x8222ced0
	ctx.lr = 0x8325843C;
	sub_8222CED0(ctx, base);
	// 8325843C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258440: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258444: 3889672C  addi r4, r9, 0x672c
	ctx.r[4].s64 = ctx.r[9].s64 + 26412;
	// 83258448: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325844C: 4AFD4A85  bl 0x8222ced0
	ctx.lr = 0x83258450;
	sub_8222CED0(ctx, base);
	// 83258450: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258458: 38886728  addi r4, r8, 0x6728
	ctx.r[4].s64 = ctx.r[8].s64 + 26408;
	// 8325845C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83258460: 4AFD4A71  bl 0x8222ced0
	ctx.lr = 0x83258464;
	sub_8222CED0(ctx, base);
	// 83258464: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325846C: 38876724  addi r4, r7, 0x6724
	ctx.r[4].s64 = ctx.r[7].s64 + 26404;
	// 83258470: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83258474: 4AFD4A5D  bl 0x8222ced0
	ctx.lr = 0x83258478;
	sub_8222CED0(ctx, base);
	// 83258478: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325847C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258480: 38866720  addi r4, r6, 0x6720
	ctx.r[4].s64 = ctx.r[6].s64 + 26400;
	// 83258484: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83258488: 4AFD4A49  bl 0x8222ced0
	ctx.lr = 0x8325848C;
	sub_8222CED0(ctx, base);
	// 8325848C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258490: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258494: 3884671C  addi r4, r4, 0x671c
	ctx.r[4].s64 = ctx.r[4].s64 + 26396;
	// 83258498: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8325849C: 4AFD4A35  bl 0x8222ced0
	ctx.lr = 0x832584A0;
	sub_8222CED0(ctx, base);
	// 832584A0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832584A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584A8: 38836718  addi r4, r3, 0x6718
	ctx.r[4].s64 = ctx.r[3].s64 + 26392;
	// 832584AC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832584B0: 4AFD4A21  bl 0x8222ced0
	ctx.lr = 0x832584B4;
	sub_8222CED0(ctx, base);
	// 832584B4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832584B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584BC: 388B6714  addi r4, r11, 0x6714
	ctx.r[4].s64 = ctx.r[11].s64 + 26388;
	// 832584C0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832584C4: 4AFD4A0D  bl 0x8222ced0
	ctx.lr = 0x832584C8;
	sub_8222CED0(ctx, base);
	// 832584C8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832584CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584D0: 388A6710  addi r4, r10, 0x6710
	ctx.r[4].s64 = ctx.r[10].s64 + 26384;
	// 832584D4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832584D8: 4AFD49F9  bl 0x8222ced0
	ctx.lr = 0x832584DC;
	sub_8222CED0(ctx, base);
	// 832584DC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832584E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584E4: 38896708  addi r4, r9, 0x6708
	ctx.r[4].s64 = ctx.r[9].s64 + 26376;
	// 832584E8: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 832584EC: 4AFD49E5  bl 0x8222ced0
	ctx.lr = 0x832584F0;
	sub_8222CED0(ctx, base);
	// 832584F0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832584F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832584F8: 38886700  addi r4, r8, 0x6700
	ctx.r[4].s64 = ctx.r[8].s64 + 26368;
	// 832584FC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83258500: 4AFD49D1  bl 0x8222ced0
	ctx.lr = 0x83258504;
	sub_8222CED0(ctx, base);
	// 83258504: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83258508: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325850C: 388766F8  addi r4, r7, 0x66f8
	ctx.r[4].s64 = ctx.r[7].s64 + 26360;
	// 83258510: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83258514: 4AFD49BD  bl 0x8222ced0
	ctx.lr = 0x83258518;
	sub_8222CED0(ctx, base);
	// 83258518: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325851C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258520: 388666F0  addi r4, r6, 0x66f0
	ctx.r[4].s64 = ctx.r[6].s64 + 26352;
	// 83258524: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83258528: 4AFD49A9  bl 0x8222ced0
	ctx.lr = 0x8325852C;
	sub_8222CED0(ctx, base);
	// 8325852C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83258530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258534: 388466E8  addi r4, r4, 0x66e8
	ctx.r[4].s64 = ctx.r[4].s64 + 26344;
	// 83258538: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8325853C: 4AFD4995  bl 0x8222ced0
	ctx.lr = 0x83258540;
	sub_8222CED0(ctx, base);
	// 83258540: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83258544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258548: 388366E0  addi r4, r3, 0x66e0
	ctx.r[4].s64 = ctx.r[3].s64 + 26336;
	// 8325854C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83258550: 4AFD4981  bl 0x8222ced0
	ctx.lr = 0x83258554;
	sub_8222CED0(ctx, base);
	// 83258554: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258558: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325855C: 388B66D8  addi r4, r11, 0x66d8
	ctx.r[4].s64 = ctx.r[11].s64 + 26328;
	// 83258560: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83258564: 4AFD496D  bl 0x8222ced0
	ctx.lr = 0x83258568;
	sub_8222CED0(ctx, base);
	// 83258568: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325856C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258570: 388A66D0  addi r4, r10, 0x66d0
	ctx.r[4].s64 = ctx.r[10].s64 + 26320;
	// 83258574: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83258578: 4AFD4959  bl 0x8222ced0
	ctx.lr = 0x8325857C;
	sub_8222CED0(ctx, base);
	// 8325857C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83258580: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258584: 388966C8  addi r4, r9, 0x66c8
	ctx.r[4].s64 = ctx.r[9].s64 + 26312;
	// 83258588: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8325858C: 4AFD4945  bl 0x8222ced0
	ctx.lr = 0x83258590;
	sub_8222CED0(ctx, base);
	// 83258590: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83258594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258598: 388866C0  addi r4, r8, 0x66c0
	ctx.r[4].s64 = ctx.r[8].s64 + 26304;
	// 8325859C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 832585A0: 4AFD4931  bl 0x8222ced0
	ctx.lr = 0x832585A4;
	sub_8222CED0(ctx, base);
	// 832585A4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832585A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585AC: 388766B8  addi r4, r7, 0x66b8
	ctx.r[4].s64 = ctx.r[7].s64 + 26296;
	// 832585B0: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 832585B4: 4AFD491D  bl 0x8222ced0
	ctx.lr = 0x832585B8;
	sub_8222CED0(ctx, base);
	// 832585B8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832585BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585C0: 388666B0  addi r4, r6, 0x66b0
	ctx.r[4].s64 = ctx.r[6].s64 + 26288;
	// 832585C4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 832585C8: 4AFD4909  bl 0x8222ced0
	ctx.lr = 0x832585CC;
	sub_8222CED0(ctx, base);
	// 832585CC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832585D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585D4: 388466A8  addi r4, r4, 0x66a8
	ctx.r[4].s64 = ctx.r[4].s64 + 26280;
	// 832585D8: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 832585DC: 4AFD48F5  bl 0x8222ced0
	ctx.lr = 0x832585E0;
	sub_8222CED0(ctx, base);
	// 832585E0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832585E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585E8: 388366A0  addi r4, r3, 0x66a0
	ctx.r[4].s64 = ctx.r[3].s64 + 26272;
	// 832585EC: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 832585F0: 4AFD48E1  bl 0x8222ced0
	ctx.lr = 0x832585F4;
	sub_8222CED0(ctx, base);
	// 832585F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832585F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832585FC: 388B6698  addi r4, r11, 0x6698
	ctx.r[4].s64 = ctx.r[11].s64 + 26264;
	// 83258600: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83258604: 4AFD48CD  bl 0x8222ced0
	ctx.lr = 0x83258608;
	sub_8222CED0(ctx, base);
	// 83258608: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8325860C: 386AB2B8  addi r3, r10, -0x4d48
	ctx.r[3].s64 = ctx.r[10].s64 + -19784;
	// 83258610: 4BA51911  bl 0x82ca9f20
	ctx.lr = 0x83258614;
	sub_82CA9F20(ctx, base);
	// 83258614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325861C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83258624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83258628 size=12
    let mut pc: u32 = 0x83258628;
    'dispatch: loop {
        match pc {
            0x83258628 => {
    //   block [0x83258628..0x83258634)
	// 83258628: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325862C: 386BB320  addi r3, r11, -0x4ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -19680;
	// 83258630: 4BA518F0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258638 size=56
    let mut pc: u32 = 0x83258638;
    'dispatch: loop {
        match pc {
            0x83258638 => {
    //   block [0x83258638..0x83258670)
	// 83258638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325863C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325864C: 386B6E20  addi r3, r11, 0x6e20
	ctx.r[3].s64 = ctx.r[11].s64 + 28192;
	// 83258650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258654: 4AF9B705  bl 0x821f3d58
	ctx.lr = 0x83258658;
	sub_821F3D58(ctx, base);
	// 83258658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325865C: 906A8ACC  stw r3, -0x7534(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-30004 as u32), ctx.r[3].u32 ) };
	// 83258660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325866C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258670 size=64
    let mut pc: u32 = 0x83258670;
    'dispatch: loop {
        match pc {
            0x83258670 => {
    //   block [0x83258670..0x832586B0)
	// 83258670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325867C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258684: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258688: 386A8AD0  addi r3, r10, -0x7530
	ctx.r[3].s64 = ctx.r[10].s64 + -30000;
	// 8325868C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258690: 4AFD4841  bl 0x8222ced0
	ctx.lr = 0x83258694;
	sub_8222CED0(ctx, base);
	// 83258694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258698: 3869B378  addi r3, r9, -0x4c88
	ctx.r[3].s64 = ctx.r[9].s64 + -19592;
	// 8325869C: 4BA51885  bl 0x82ca9f20
	ctx.lr = 0x832586A0;
	sub_82CA9F20(ctx, base);
	// 832586A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832586A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832586A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832586AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832586B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586B0 size=64
    let mut pc: u32 = 0x832586B0;
    'dispatch: loop {
        match pc {
            0x832586B0 => {
    //   block [0x832586B0..0x832586F0)
	// 832586B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832586BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832586C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832586C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832586C8: 386A8AD4  addi r3, r10, -0x752c
	ctx.r[3].s64 = ctx.r[10].s64 + -29996;
	// 832586CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832586D0: 4AFD4801  bl 0x8222ced0
	ctx.lr = 0x832586D4;
	sub_8222CED0(ctx, base);
	// 832586D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832586D8: 3869B388  addi r3, r9, -0x4c78
	ctx.r[3].s64 = ctx.r[9].s64 + -19576;
	// 832586DC: 4BA51845  bl 0x82ca9f20
	ctx.lr = 0x832586E0;
	sub_82CA9F20(ctx, base);
	// 832586E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832586E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832586E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832586EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832586F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832586F0 size=64
    let mut pc: u32 = 0x832586F0;
    'dispatch: loop {
        match pc {
            0x832586F0 => {
    //   block [0x832586F0..0x83258730)
	// 832586F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832586F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832586F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832586FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258704: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83258708: 386A8AD8  addi r3, r10, -0x7528
	ctx.r[3].s64 = ctx.r[10].s64 + -29992;
	// 8325870C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258710: 4AFD47C1  bl 0x8222ced0
	ctx.lr = 0x83258714;
	sub_8222CED0(ctx, base);
	// 83258714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258718: 3869B398  addi r3, r9, -0x4c68
	ctx.r[3].s64 = ctx.r[9].s64 + -19560;
	// 8325871C: 4BA51805  bl 0x82ca9f20
	ctx.lr = 0x83258720;
	sub_82CA9F20(ctx, base);
	// 83258720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325872C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258730 size=64
    let mut pc: u32 = 0x83258730;
    'dispatch: loop {
        match pc {
            0x83258730 => {
    //   block [0x83258730..0x83258770)
	// 83258730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325873C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258744: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83258748: 386A8ADC  addi r3, r10, -0x7524
	ctx.r[3].s64 = ctx.r[10].s64 + -29988;
	// 8325874C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258750: 4AFD4781  bl 0x8222ced0
	ctx.lr = 0x83258754;
	sub_8222CED0(ctx, base);
	// 83258754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258758: 3869B3A8  addi r3, r9, -0x4c58
	ctx.r[3].s64 = ctx.r[9].s64 + -19544;
	// 8325875C: 4BA517C5  bl 0x82ca9f20
	ctx.lr = 0x83258760;
	sub_82CA9F20(ctx, base);
	// 83258760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325876C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258770 size=64
    let mut pc: u32 = 0x83258770;
    'dispatch: loop {
        match pc {
            0x83258770 => {
    //   block [0x83258770..0x832587B0)
	// 83258770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83258774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325877C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83258780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258784: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83258788: 386A8AE0  addi r3, r10, -0x7520
	ctx.r[3].s64 = ctx.r[10].s64 + -29984;
	// 8325878C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83258790: 4AFD4741  bl 0x8222ced0
	ctx.lr = 0x83258794;
	sub_8222CED0(ctx, base);
	// 83258794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83258798: 3869B3B8  addi r3, r9, -0x4c48
	ctx.r[3].s64 = ctx.r[9].s64 + -19528;
	// 8325879C: 4BA51785  bl 0x82ca9f20
	ctx.lr = 0x832587A0;
	sub_82CA9F20(ctx, base);
	// 832587A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832587A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832587A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832587AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832587B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587B0 size=64
    let mut pc: u32 = 0x832587B0;
    'dispatch: loop {
        match pc {
            0x832587B0 => {
    //   block [0x832587B0..0x832587F0)
	// 832587B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832587BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832587C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832587C4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832587C8: 386A8AE4  addi r3, r10, -0x751c
	ctx.r[3].s64 = ctx.r[10].s64 + -29980;
	// 832587CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832587D0: 4AFD4701  bl 0x8222ced0
	ctx.lr = 0x832587D4;
	sub_8222CED0(ctx, base);
	// 832587D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832587D8: 3869B3C8  addi r3, r9, -0x4c38
	ctx.r[3].s64 = ctx.r[9].s64 + -19512;
	// 832587DC: 4BA51745  bl 0x82ca9f20
	ctx.lr = 0x832587E0;
	sub_82CA9F20(ctx, base);
	// 832587E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832587E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832587E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832587EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832587F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832587F0 size=56
    let mut pc: u32 = 0x832587F0;
    'dispatch: loop {
        match pc {
            0x832587F0 => {
    //   block [0x832587F0..0x83258828)
	// 832587F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832587F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832587F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832587FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83258804: 386B7198  addi r3, r11, 0x7198
	ctx.r[3].s64 = ctx.r[11].s64 + 29080;
	// 83258808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325880C: 4AF9B54D  bl 0x821f3d58
	ctx.lr = 0x83258810;
	sub_821F3D58(ctx, base);
	// 83258810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83258814: 906A8AE8  stw r3, -0x7518(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29976 as u32), ctx.r[3].u32 ) };
	// 83258818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83258824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83258828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83258828 size=56
    let mut pc: u32 = 0x83258828;
    'dispatch: loop {
        match pc {
            0x83258828 => {
    //   block [0x83258828..0x83258860)
	// 83258828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83258830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83258834: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83258838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325883C: 386B71A8  addi r3, r11, 0x71a8
	ctx.r[3].s64 = ctx.r[11].s64 + 29096;
	// 83258840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83258844: 4AF9B515  bl 0x821f3d58
	ctx.lr = 0x83258848;
	sub_821F3D58(ctx, base);
	// 83258848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325884C: 906A8AEC  stw r3, -0x7514(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-29972 as u32), ctx.r[3].u32 ) };
	// 83258850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83258854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83258858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325885C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


